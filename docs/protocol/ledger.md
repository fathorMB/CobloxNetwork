# Federated ledger and light-client protocol

This document defines consensus-authenticated ledger objects, their state
transitions, and the proof path used by resource-constrained light clients.

## Model and invariants

Coblox v0 is an account-state ledger finalized by a rotating BFT validator
federation. It is not a transferable currency. Consensus validation MUST
preserve these invariants:

1. balances and nonces are unsigned 64-bit integers and arithmetic overflow or
   underflow invalidates the block;
2. `mint` is the only supply-increasing transaction, `burn` the only
   supply-decreasing transaction, and `fund_app` only reallocates existing
   balance from one node account to one app escrow account;
3. no transaction has both a user-controlled source and a user-controlled
   destination. A direct user-to-user transfer is therefore unrepresentable;
4. every mint is linked to finalized, validator-verifiable eligibility evidence;
   a subscription burn is authorized by the debited node, while hosting burns
   are consensus-authorized debits from the app escrow;
5. a burn destroys tokens. A resource provider is paid only by a separate mint
   backed by proof of work; the burn never names or credits a provider;
6. a transaction ID occurs at most once in chain history and debit nonces are
   strictly consecutive per node or app account;
7. only a block with a valid quorum certificate from the active validator set
   is finalized.

## Hashing primitives

`H(x)` is SHA-256. Concatenated integers are unsigned big-endian. Domain bytes
include the shown zero terminator.

```text
tx_id       = H("coblox-tx-id-v0\0" || chain_id_32 || JCS(unsigned_transaction))
tx_leaf     = H(0x00 || raw_32_bytes(tx_id))
merkle_node = H(0x01 || left_32 || right_32)
block_id    = H("coblox-block-id-v0\0" || chain_id_32 || JCS(block_header))
```

The transaction Merkle tree preserves block order. It is padded to the next
power of two with `H(0x02)` leaves; an empty block root is `H(0x03)`. A block
MUST NOT contain more than 16,384 transactions.

## Unsigned transaction and authorization

All transaction objects share:

```text
{
  "schema_version":"0.1",
  "network_id":string,
  "kind":"mint"|"burn"|"fund_app"|"challenge_commitment"|"challenge_evidence"
        |"revoke_identity",
  "created_at_ms":u64-string,
  "expires_at_ms":u64-string,
  "body":object,
  "authorization":object
}
```

The unsigned transaction used for its ID is the object with `authorization`
removed. Authorization signatures use the global chain-bound procedure with
domain `coblox-ledger-transaction-v0` and payload `raw_32_bytes(tx_id)`.

```text
ValidatorSignature = {"validator_id":string,"signature":base64url(64 bytes)}
TransactionQuorumCertificate = {
  "validator_set_hash":sha256-string,
  "signatures":[ValidatorSignature]
}
```

Transaction quorum signatures are unique, sorted by validator ID, and use the
quorum predicate below. The referenced set MUST be active at transaction execution.

## Quorum predicate

Every v0 quorum (block finality, transaction authorization, enrollment
certificate, validator-set document, and protocol document) uses exactly:

```text
quorum(signed_power, total_power) := signed_power * 3 > total_power * 2
```

Both multiplications use checked `u128`; overflow or zero total power rejects.
This strict predicate is not `>=`, not a rounded fraction, and not a validator
count. Boundary fixtures: for total power 100, 66 rejects and 67 accepts; for
101, 67 rejects and 68 accepts; for 102, 68 rejects and 69 accepts.

### Mint: existence income, work compensation, and publisher reward

```text
MintBody = {
  "reason":"existence_income"|"work_compensation"|"publisher_reward",
  "beneficiary_node_id":string,
  "amount_microtokens":u64-string,
  "reward_epoch":u64-string,
  "policy_hash":sha256-string,
  "evidence_tx_ids":[sha256-string],              // existence/work only
  "eligible_node_count":u64-string,               // existence only
  "work_kind":"availability"|"storage"|"compute", // work only
  "app_id":sha256-string,                         // publisher only
  "active_subscriber_count":u64-string,           // publisher only
  "active_subscription_root":sha256-string,       // publisher only
  "counted_subscription_burn_microtokens":u64-string // publisher only
}
MintAuthorization = {"quorum_certificate":TransactionQuorumCertificate}
```

For `existence_income`, `work_kind` is absent, `eligible_node_count` is
required, and evidence MUST establish the configured availability threshold for
that node and epoch. For `work_compensation`, `work_kind` is required and
evidence MUST establish the measured resource contribution. Evidence IDs are
unique and sorted bytewise. The reward function in the signed `policy_hash`
deterministically yields the amount; validators recompute it. Evidence cannot be
consumed by two mints.

#### Existence income is a share of a capped fund

Existence income is **not** a fixed amount per node. Per [ADR-007] it is a fund
whose size is fixed per epoch by governance, divided among the nodes that met
the threshold. For an epoch with `existence_fund_microtokens_per_epoch = F` from
the active reward policy and `eligible_node_count = E`:

```text
E > 0
amount_microtokens = F / E          // integer division, remainder discarded
```

The remainder is **not** minted and is not carried forward; total existence
emission for an epoch is therefore at most `F` by construction, not by
convention. Validators recompute `E` from the finalized evidence of that epoch
and reject a mint whose `eligible_node_count` differs or whose amount is not the
exact quotient. At most one `existence_income` mint exists per
`(beneficiary_node_id, reward_epoch)`, and the sum of existence mints for an
epoch MUST NOT exceed `F`.

This is the format-level consequence of the anti-Sybil position, and it is worth
stating why it matters: with a per-node amount, `N` emulated identities
**increase** total emission, so a fleet mints. With a capped fund, the same
fleet can only dilute the share of honest nodes — the attack degrades from
forgery to redistribution, and the fraction of total emission reachable that way
is bounded by how much of emission flows through this channel at all. That
fraction is a governance quantity monitored under `SEC-REQ-18`, owned by M-02
and M-03, and is deliberately not a schema field here: it is an observed ratio
between channels, not a knob.

Declared limit: `eligible_node_count` is committed in the mint and recomputed by
full validators from finalized evidence, but v0 does not commit a per-epoch
eligible-set root, so a light client verifies the arithmetic and the quorum
rather than independently recomputing eligibility. A per-epoch eligibility
commitment is M-02 work.

For `publisher_reward`, `evidence_tx_ids`, `eligible_node_count`, and
`work_kind` are absent; `app_id`, `active_subscriber_count`,
`active_subscription_root`, and `counted_subscription_burn_microtokens` are
required. The
beneficiary MUST equal the enrolled publisher committed by that app's finalized
catalog record. For an epoch, validators select finalized `app_subscription`
burns for the app whose half-open paid service period contains the entire reward
epoch, group them by payer node ID, and retain the lowest raw transaction ID for
each payer. The publisher's own node ID is excluded. The remaining entries are
sorted by `account_key` and committed as:

```text
subscription_leaf = H(0x20 || raw_32_bytes(app_id) || u64be(reward_epoch)
                          || account_key_32 || raw_32_bytes(subscription_burn_tx_id))
subscription_node = H(0x21 || left_32 || right_32)
subscription_empty = H(0x22)
```

The tree preserves sorted order and pads to a power of two with
`subscription_empty`; zero entries use `H(0x23)` as the root and are not reward
eligible. `active_subscriber_count` is the number of retained leaves and
`counted_subscription_burn_microtokens` is the checked `u128` sum of
`amount_microtokens` over exactly those retained burns, rejected if it exceeds
`u64::MAX`. Full validators recompute the count, the sum, and the root from
finalized burns, then apply the creator-reward curve selected by `policy_hash`.
At most one publisher-reward mint exists per `(app_id, reward_epoch)`. The
commitment prevents a proposer from inventing or double-counting subscribers.

#### Creator-share cap: a validity rule, not a policy note

The reward curve alone permits a publisher to be its own subscribers and collect more
than it burns, which is a token-printing cycle. v0 closes it with a consensus
constraint rather than a recommendation. Let `kn` and `kd` be
`publisher_reward_cap_numerator` and `publisher_reward_cap_denominator` from the
active reward policy, with `kd > 0` and `kn < kd` enforced when the document is
accepted. For every `(app_id, reward_epoch)`:

```text
amount_microtokens * kd  <=  kn * counted_subscription_burn_microtokens
```

Both products use checked `u128` intermediates; overflow rejects the block. A
mint violating the inequality is **invalid** — validators recompute and enforce
it exactly like any other validity rule, and the entire proposed block fails.

Because `kn < kd` is strict, the marginal effect of one self-owned subscriber per
period is `-S + P <= -S(1 - kn/kd) < 0`: strictly negative whatever curve the
economic simulator later selects. The cycle is structurally lossy, so the
constraint does not depend on tuning to be sound. Boundary conformance: with
`kn/kd` and a counted burn sum `B`, a mint of exactly `floor(kn * B / kd)` is
valid and that value plus one is invalid.

Declared limit, because the cap does not close everything: `active_subscriber_count`
remains a public finalized number and therefore a popularity signal, so a
publisher can still buy *reputation* at a cost of `S(1 - kn/kd)` per fake
subscriber per period, funded by existence income. Weighting subscribers by
demonstrated contribution, or not exposing the count in discovery, are the
candidate answers; both are economic and catalog decisions owned by the Project
Lead and AGENT-002 under [ADR-006], and the relation is on the economic
simulator's mandatory checklist. What this document guarantees is only that the
*ledger* cycle cannot be net positive.

Canonical existence-income mint:

```json
{"authorization":{"quorum_certificate":{"signatures":[{"signature":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA","validator_id":"val-001"}],"validator_set_hash":"sha256:1df0a6454faaa5985b7f98c48d3c60d2ed62d5b3b24fe8e97d3dca1dd36f1120"}},"body":{"amount_microtokens":"250000","beneficiary_node_id":"cblx1ci6q36gqm6u3spknxzr7p5r2y4xw7n25d5icm7rsoq7lq6ka","eligible_node_count":"4000","evidence_tx_ids":["sha256:313eb3d86d8c049838543325910bccb953b828da764b5f18bff11d7a123b0e68"],"policy_hash":"sha256:7df04d03b60f62852f0d76c847d0181a2b17b43a50f987c0b9f814e70f064bcc","reason":"existence_income","reward_epoch":"17"},"created_at_ms":"1787654500000","expires_at_ms":"1787740900000","kind":"mint","network_id":"coblox-devnet-0","schema_version":"0.1"}
```

Canonical storage-work mint:

```json
{"authorization":{"quorum_certificate":{"signatures":[{"signature":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA","validator_id":"val-001"}],"validator_set_hash":"sha256:1df0a6454faaa5985b7f98c48d3c60d2ed62d5b3b24fe8e97d3dca1dd36f1120"}},"body":{"amount_microtokens":"900000","beneficiary_node_id":"cblx1ci6q36gqm6u3spknxzr7p5r2y4xw7n25d5icm7rsoq7lq6ka","evidence_tx_ids":["sha256:9ee7dba4de0a88de35a8813c32c6d8cce0a86766c0ee65db3d26f519164b750e"],"policy_hash":"sha256:7df04d03b60f62852f0d76c847d0181a2b17b43a50f987c0b9f814e70f064bcc","reason":"work_compensation","reward_epoch":"17","work_kind":"storage"},"created_at_ms":"1787654501000","expires_at_ms":"1787740901000","kind":"mint","network_id":"coblox-devnet-0","schema_version":"0.1"}
```

Availability and compute use the identical serialized schema with
`work_kind` set accordingly.

Canonical publisher-reward mint:

```json
{"authorization":{"quorum_certificate":{"signatures":[{"signature":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA","validator_id":"val-001"}],"validator_set_hash":"sha256:1df0a6454faaa5985b7f98c48d3c60d2ed62d5b3b24fe8e97d3dca1dd36f1120"}},"body":{"active_subscriber_count":"128","active_subscription_root":"sha256:fc9cd19c4f7b32970a7c870e821dbca915d204c09a496d60b17f66ec8790ad3a","amount_microtokens":"6400000","app_id":"sha256:77a1d5d603f675f8b8a3f63ac94d14f9ea04c86d5e216ac4f0e1bd5ebac0ecf8","beneficiary_node_id":"cblx1ci6q36gqm6u3spknxzr7p5r2y4xw7n25d5icm7rsoq7lq6ka","counted_subscription_burn_microtokens":"38400000","policy_hash":"sha256:7df04d03b60f62852f0d76c847d0181a2b17b43a50f987c0b9f814e70f064bcc","reason":"publisher_reward","reward_epoch":"17"},"created_at_ms":"1787654502000","expires_at_ms":"1787740902000","kind":"mint","network_id":"coblox-devnet-0","schema_version":"0.1"}
```

### Fund app escrow

`fund_app` realizes ADR-006 without introducing a transferable destination:
the destination is the deterministic escrow account of a finalized app, never
an arbitrary user account.

```text
FundAppBody = {
  "payer_node_id":string,
  "payer_account_nonce":u64-string,
  "app_id":sha256-string,
  "amount_microtokens":u64-string
}
FundAppAuthorization = {
  "public_key":base64url(32 bytes),
  "signature":base64url(64 bytes)
}
```

The key MUST derive the enrolled, unrevoked `payer_node_id`. Execution subtracts
the amount and increments the node nonce, then adds the same amount to the app
escrow using checked arithmetic. Total supply is unchanged. The app MUST have a
finalized catalog record. Funding a suspended app moves it to `grace`; it
becomes `active` only when its balance covers the next deterministic billing
epoch.

Canonical app-funding transaction:

```json
{"authorization":{"public_key":"11qYAYdk9J0L5Z-6hB4qMTPBSAE5nK1G0IU2n6z1V9g","signature":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"},"body":{"amount_microtokens":"2400000","app_id":"sha256:77a1d5d603f675f8b8a3f63ac94d14f9ea04c86d5e216ac4f0e1bd5ebac0ecf8","payer_account_nonce":"8","payer_node_id":"cblx1ci6q36gqm6u3spknxzr7p5r2y4xw7n25d5icm7rsoq7lq6ka"},"created_at_ms":"1787654505000","expires_at_ms":"1787654805000","kind":"fund_app","network_id":"coblox-devnet-0","schema_version":"0.1"}
```

### Burn: hosting and subscription spending

```text
BurnBody = {
  "reason":"app_hosting"|"app_subscription",
  "amount_microtokens":u64-string,
  "app_id":sha256-string,
  "service_period_start_ms":u64-string,
  "service_period_end_ms":u64-string,
  "pricing_hash":sha256-string,
  "payer_node_id":string,       // subscription only
  "account_nonce":u64-string,   // subscription only
  "payer_app_id":sha256-string, // hosting only; MUST equal app_id
  "app_account_nonce":u64-string// hosting only
}
SubscriptionBurnAuthorization = {
  "public_key":base64url(32 bytes),
  "signature":base64url(64 bytes)
}
HostingBurnAuthorization = {"quorum_certificate":TransactionQuorumCertificate}
```

For a subscription, the key MUST derive `payer_node_id`; the signature is
required and the node balance is debited. The service period is half-open and
end MUST be greater than start. For `app_hosting`, `payer_node_id` and
`account_nonce` are absent, `payer_app_id` MUST equal `app_id`, the validator
quorum authorizes the deterministic charge, and the app escrow is debited. The
`pricing_hash` MUST identify the signed protocol hosting rate card active for
the billed epoch; validators derive the charge from that rate card, requested
replicas, declared per-replica resources, service period, and metered usage. A
publisher cannot supply or lower a hosting rate. For `app_subscription`,
`pricing_hash` commits to the app manifest's publisher-declared subscription and
invocation pricing. The amount MUST equal the deterministic quoted charge and
be available at execution. No provider ID exists in this schema.

An app account has a balance, consecutive `app_account_nonce`, and lifecycle
`active | grace | suspended`. At each billing epoch consensus first records a
`grace` transition when the next charge is unavailable. Suspension becomes
effective only after `app_suspension_notice_epochs` from the active signed
consensus parameters; a timely `fund_app` cancels the pending suspension. A
suspended app remains in state and in the catalog and can later be funded and
reactivated; it is never silently deleted.

Canonical hosting burn:

```json
{"authorization":{"quorum_certificate":{"signatures":[{"signature":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA","validator_id":"val-001"}],"validator_set_hash":"sha256:1df0a6454faaa5985b7f98c48d3c60d2ed62d5b3b24fe8e97d3dca1dd36f1120"}},"body":{"amount_microtokens":"1200000","app_account_nonce":"3","app_id":"sha256:77a1d5d603f675f8b8a3f63ac94d14f9ea04c86d5e216ac4f0e1bd5ebac0ecf8","payer_app_id":"sha256:77a1d5d603f675f8b8a3f63ac94d14f9ea04c86d5e216ac4f0e1bd5ebac0ecf8","pricing_hash":"sha256:2d1e35bf61f89fc50cb9cafe158f44ad63d522898971e0211d59708331c4b404","reason":"app_hosting","service_period_end_ms":"1790332800000","service_period_start_ms":"1787654400000"},"created_at_ms":"1787654510000","expires_at_ms":"1787654810000","kind":"burn","network_id":"coblox-devnet-0","schema_version":"0.1"}
```

Canonical subscription burn:

```json
{"authorization":{"public_key":"11qYAYdk9J0L5Z-6hB4qMTPBSAE5nK1G0IU2n6z1V9g","signature":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"},"body":{"account_nonce":"9","amount_microtokens":"300000","app_id":"sha256:77a1d5d603f675f8b8a3f63ac94d14f9ea04c86d5e216ac4f0e1bd5ebac0ecf8","payer_node_id":"cblx1ci6q36gqm6u3spknxzr7p5r2y4xw7n25d5icm7rsoq7lq6ka","pricing_hash":"sha256:2d1e35bf61f89fc50cb9cafe158f44ad63d522898971e0211d59708331c4b404","reason":"app_subscription","service_period_end_ms":"1790332800000","service_period_start_ms":"1787654400000"},"created_at_ms":"1787654520000","expires_at_ms":"1787654820000","kind":"burn","network_id":"coblox-devnet-0","schema_version":"0.1"}
```

### Challenge issuer commitment

A challenge is only meaningful if nobody could steer it. The commitment that
makes `randomness` verifiable is itself a ledger object, so that "committed
before subject selection" is a checkable fact rather than a claim.

```text
ChallengeCommitmentBody = {
  "issuer_node_id":string,
  "commitment_epoch":u64-string,
  "issuer_commitment":sha256-string
}
ChallengeCommitmentAuthorization = {
  "public_key":base64url(32 bytes),
  "signature":base64url(64 bytes)
}
```

The key MUST derive the enrolled, unrevoked `issuer_node_id`. At most one
commitment exists per `(issuer_node_id, commitment_epoch)`; a second is invalid,
so an issuer cannot hold several secrets and reveal whichever suits the outcome.
This transaction moves no value and touches no balance or nonce.

Canonical serialized example:

```json
{"authorization":{"public_key":"11qYAYdk9J0L5Z-6hB4qMTPBSAE5nK1G0IU2n6z1V9g","signature":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"},"body":{"commitment_epoch":"17","issuer_commitment":"sha256:19556b209c36de1940340bd3ada4a4c821fe70cde0fd3906af2b71f31445e4d5","issuer_node_id":"cblx1issuerfixture"},"created_at_ms":"1787654400000","expires_at_ms":"1787654700000","kind":"challenge_commitment","network_id":"coblox-devnet-0","schema_version":"0.1"}
```

### Challenge evidence

```text
ChallengeEvidenceBody = {
  "challenge_id":sha256-string,
  "kind":"availability"|"storage"|"compute",
  "issuer_node_id":string,
  "subject_node_id":string,
  "request":ChallengeRequest,
  "request_hash":sha256-string,
  "response":ChallengeResponse,              // absent only for no_response
  "response_hash":sha256-string,             // absent only for no_response
  "issuer_reveal":base64url(32 bytes),
  "outcome":"passed"|"failed"|"late"|"no_response",
  "measured_units":u64-string,
  "completed_at_ms":u64-string,
  "auditor_signatures":[ValidatorSignature]
}
ChallengeEvidenceAuthorization = {"quorum_certificate":TransactionQuorumCertificate}
```

Auditor signatures use domain `coblox-challenge-evidence-v0`, are unique and
sorted, and meet the policy's independent-auditor threshold. The transaction
quorum attests consensus acceptance; it does not replace raw request/response
verification by validators or light auditors. Validators recompute both hashes;
embedding the raw objects makes them retrievable with the finalized transaction.
`measured_units` is 1 for availability, verified
bytes for storage, and verified fuel units for compute.

`issuer_reveal` is the issuer secret behind the commitment. Every verifier —
not only validators — MUST recompute, from the registry formulas and the
embedded request:

1. `issuer_commitment` from `issuer_reveal`, `request.issuer_node_id`, and
   `request.randomness_source.commitment_epoch`, and require it to equal
   `request.issuer_commitment`;
2. that a finalized `challenge_commitment` transaction carries exactly that
   `(issuer_node_id, commitment_epoch, issuer_commitment)` triple, and that it
   was finalized at a height **strictly below** `beacon_height`, so the secret
   was fixed before the beacon existed;
3. `challenge_randomness` from the beacon, the commitment, the reveal, and the
   subject, and require it to equal `request.randomness`;
4. that `beacon_block_id` is the finalized canonical block at `beacon_height`;
5. that the `(issuer, subject)` pair is the pair the epoch's assignment
   function produces from that beacon.

Evidence failing any of these is invalid and cannot back a mint. This is what
turns "randomness MUST derive from finalized consensus randomness" from an
unenforceable instruction into a rule: previously no verifier held the data
needed to contradict a colluding issuer who picked the one chunk its subject had
kept.

Declared limit: in v0 the beacon value is a finalized `block_id`, so the
proposer of the beacon block has a bounded grinding advantage — it can discard a
candidate block and try another. The commit-before-beacon ordering removes the
issuer's freedom but not the proposer's, and a dedicated randomness beacon is
M-02 work. The residual is stated rather than assumed away.

Canonical serialized example:

```json
{"authorization":{"quorum_certificate":{"signatures":[{"signature":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA","validator_id":"val-001"}],"validator_set_hash":"sha256:1df0a6454faaa5985b7f98c48d3c60d2ed62d5b3b24fe8e97d3dca1dd36f1120"}},"body":{"auditor_signatures":[{"signature":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA","validator_id":"val-001"}],"challenge_id":"sha256:3d56e5dd5104a2ad5c733fa4f0b6d8f35de2f68509e9c10a3d473128eaec0b21","completed_at_ms":"1787654416000","issuer_node_id":"cblx1issuerfixture","issuer_reveal":"REREREREREREREREREREREREREREREREREREREREREQ","kind":"availability","measured_units":"1","outcome":"passed","request":{"assignment":{"response_bytes":"32"},"challenge_id":"sha256:3d56e5dd5104a2ad5c733fa4f0b6d8f35de2f68509e9c10a3d473128eaec0b21","deadline_ms":"1787654420000","issued_at_ms":"1787654415000","issuer_commitment":"sha256:19556b209c36de1940340bd3ada4a4c821fe70cde0fd3906af2b71f31445e4d5","issuer_node_id":"cblx1issuerfixture","issuer_signature":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA","kind":"availability","randomness":"AAECAwQFBgcICQoLDA0ODxAREhMUFRYXGBkaGxwdHh8","randomness_source":{"beacon_block_id":"sha256:7e0694f564afa2d047db4eb58f4f2b3d322d71db808f6bbf5313ee2d2a4a95af","beacon_height":"40","commitment_epoch":"17"},"subject_node_id":"cblx1ci6q36gqm6u3spknxzr7p5r2y4xw7n25d5icm7rsoq7lq6ka"},"request_hash":"sha256:e14d4c02c41a950c9f4f4464e9f98a6652c64e6c992efc36c97f01d2f4ca2dc2","response":{"challenge_id":"sha256:3d56e5dd5104a2ad5c733fa4f0b6d8f35de2f68509e9c10a3d473128eaec0b21","completed_at_ms":"1787654416000","result":{"kind":"availability","response":"MzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzM"},"subject_node_id":"cblx1ci6q36gqm6u3spknxzr7p5r2y4xw7n25d5icm7rsoq7lq6ka","subject_signature":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"},"response_hash":"sha256:8bc23b6277b0892c0eea482c835359a2ad975ac18af9832b727738a880f2400f","subject_node_id":"cblx1ci6q36gqm6u3spknxzr7p5r2y4xw7n25d5icm7rsoq7lq6ka"},"created_at_ms":"1787654420000","expires_at_ms":"1787740820000","kind":"challenge_evidence","network_id":"coblox-devnet-0","schema_version":"0.1"}
```

### Identity revocation

```text
RevokeIdentityBody = {
  "node_id":string,
  "reason":"key_compromise"|"validator_misconduct"|"operator_request",
  "effective_height":u64-string,
  "replacement_node_id":string // optional, informational only
}
RevokeIdentityAuthorization = {"quorum_certificate":TransactionQuorumCertificate}
```

The effective height MUST be later than the block proposing the revocation.
`replacement_node_id` receives no balance, nonce, or privileges from the old
identity. Revocation authority and signatures are validator-governed; a node's
self-signature alone cannot erase evidence or consensus history.

Canonical serialized example:

```json
{"authorization":{"quorum_certificate":{"signatures":[{"signature":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA","validator_id":"val-001"}],"validator_set_hash":"sha256:1df0a6454faaa5985b7f98c48d3c60d2ed62d5b3b24fe8e97d3dca1dd36f1120"}},"body":{"effective_height":"50","node_id":"cblx1revokedfixture","reason":"key_compromise","replacement_node_id":"cblx1replacementfixture"},"created_at_ms":"1787654550000","expires_at_ms":"1787740950000","kind":"revoke_identity","network_id":"coblox-devnet-0","schema_version":"0.1"}
```

## Block format

```text
BlockHeader = {
  "schema_version":"0.1",
  "protocol_version":"0.1",
  "network_id":string,
  "height":u64-string,
  "round":u64-string,
  "timestamp_ms":u64-string,
  "previous_block_id":sha256-string,
  "transactions_root":sha256-string,
  "state_root":sha256-string,
  "validator_set_hash":sha256-string,
  "next_validator_set_hash":sha256-string,
  "consensus_parameters_hash":sha256-string
}
Block = {
  "header":BlockHeader,
  "transactions":[Transaction],
  "quorum_certificate":QuorumCertificate
}
```

Genesis has height 0 and uses the configured all-zero previous ID. Timestamps
MUST be greater than the median of the previous 11 finalized blocks and no more
than the active maximum clock drift after the proposal is received.
`transactions_root` is recomputed in the canonical execution order defined
below. `state_root` is the result
after all transactions execute atomically.

Canonical header example:

```json
{"consensus_parameters_hash":"sha256:7df04d03b60f62852f0d76c847d0181a2b17b43a50f987c0b9f814e70f064bcc","height":"42","network_id":"coblox-devnet-0","next_validator_set_hash":"sha256:1df0a6454faaa5985b7f98c48d3c60d2ed62d5b3b24fe8e97d3dca1dd36f1120","previous_block_id":"sha256:7e0694f564afa2d047db4eb58f4f2b3d322d71db808f6bbf5313ee2d2a4a95af","protocol_version":"0.1","round":"0","schema_version":"0.1","state_root":"sha256:993b24bf6115fbf5651d615ca57a1baa825baf304b1dcc4d52debbc7fa3bd6d8","timestamp_ms":"1787654600000","transactions_root":"sha256:00811b3f6ae09c7acdb2e5c92fb273a05481f75fd477901fd43f76a9290b19b7","validator_set_hash":"sha256:1df0a6454faaa5985b7f98c48d3c60d2ed62d5b3b24fe8e97d3dca1dd36f1120"}
```

## What validators sign

Each finality vote signs exactly:

```text
"coblox-block-vote-v0\0" || chain_id_32 || u64be(height)
|| u64be(round) || raw_32_bytes(block_id)
```

```text
QuorumCertificate = {
  "height":u64-string,
  "round":u64-string,
  "block_id":sha256-string,
  "validator_set_hash":sha256-string,
  "signatures":[{"validator_id":string,"signature":base64url(64 bytes)}]
}
```

Signatures are unique and sorted by validator ID. Their summed voting power
MUST satisfy [the strict quorum predicate](#quorum-predicate). An empty or
duplicate signature entry invalidates the certificate. Aggregating Ed25519
signatures is not defined in v0.

## Validator-set continuity

```text
ValidatorSet = {
  "schema_version":"0.1",
  "activation_height":u64-string,
  "validators":[{
    "validator_id":string,
    "node_id":string,
    "consensus_public_key":base64url(32 bytes),
    "key_binding_signature":base64url(64 bytes),
    "voting_power":u64-string
  }]
}
validator_set_hash = H("coblox-validator-set-v0\0" || JCS(ValidatorSet))
```

Validators are sorted by ID, unique, enrolled and unrevoked; voting power is
positive and its sum cannot overflow `u64`. Genesis embeds the first trusted
set. At height `h`, `validator_set_hash` MUST equal the set committed as
`next_validator_set_hash` by finalized height `h-1` (genesis is the exception).
Thus the old quorum authorizes the next set before that set can sign blocks.
A light client obtains the full set, hashes it, and retains it with the header.

For each entry, the identity public key from the finalized enrollment
certificate MUST verify `key_binding_signature` over the global chain-bound
domain `coblox-consensus-key-binding-v0` and JCS of
`{"activation_height":...,"consensus_public_key":...,"node_id":...,"validator_id":...}`.
The consensus key MUST differ from the identity key and is never independently
enrolled. Full nodes and light clients verify every binding before accepting a
set or any vote from it. On leaving the active set, operators MUST destroy or
rotate the old consensus private key; re-entry requires a fresh binding.

This continuity rule specifies safe authentication but not how members are
elected or rotated.

### Revocation forces a validator set transition

Continuity alone has a hole: the active set is pinned by hash and changes only
through `next_validator_set_hash`, so a finalized `revoke_identity` naming a
sitting validator would remove nothing. The compromised consensus key would keep
voting, with its full weight counted toward quorum, until some later transition
that no rule obliges anyone to make — and a light client, which checks only
set-hash continuity and never sees transactions, could not even detect it.

The rule is therefore a validity condition on the set itself:

1. a `ValidatorSet` with `activation_height >= effective_height` that contains a
   `node_id` revoked with that `effective_height` is **invalid**;
2. a block at height `>= effective_height` whose active validator set contains
   that `node_id` is **invalid**, and so is any quorum certificate counted
   against such a set;
3. the revoked entry's voting power is never counted in either `signed_power` or
   `total_power` — it is not reweighted, the set is simply rejected;
4. `effective_height` MUST be at least `min_revocation_effective_delay_blocks`
   above the height of the block proposing the revocation, so the surviving
   members have a bounded, declared window in which to commit a compliant
   successor set.

A light client needs no new field to see this. Because the set must change, it
observes the transition through the mechanism it already verifies: it fetches the
new set, hashes it, and checks every `key_binding_signature`. That is why this
rule is preferable to adding a revocation commitment to `BlockHeader` — it costs
nothing on the wire and reuses a verification path the client already performs.

Declared consequence, stated rather than discovered later: if the remaining
validators fail to commit a compliant successor set within the delay window, the
chain **stalls** at `effective_height` instead of finalizing blocks signed by a
set containing a revoked key. That is a deliberate choice of safety over
liveness, and `min_revocation_effective_delay_blocks` exists to make the window
long enough that the choice is rarely exercised. Which members replace the
revoked one is an election question, out of scope here and open in M-02.

## Sparse Merkle account state

The account tree is a depth-256 binary sparse Merkle tree.

```text
account_key = H("coblox-account-key-v0\0" || 0x00 || node_id_utf8) // node
account_key = H("coblox-account-key-v0\0" || 0x01 || app_id_32)    // app
node_leaf   = H(0x10 || account_key || u64be(balance_microtokens)
                    || u64be(account_nonce))
app_leaf    = H(0x13 || account_key || u64be(balance_microtokens)
                    || u64be(account_nonce) || lifecycle_u8
                    || u64be(suspension_effective_epoch))
empty[256]  = H(0x12)
empty[d]    = H(0x11 || empty[d+1] || empty[d+1]) for d = 255 down to 0
branch      = H(0x11 || left || right)
```

Bits of `account_key` are traversed most-significant bit first; bit 0 chooses
left and bit 1 right. A present zero-balance account still has a leaf so its
spend nonce remains committed. An absent account uses `empty[256]` and has
implicit balance/nonce zero.

```text
AccountProof = {
  "account_kind":"node"|"app",
  "subject_id":string,
  "account_key":base64url(32 bytes),
  "balance_microtokens":u64-string,
  "account_nonce":u64-string,
  "present":boolean,
  "lifecycle":"active"|"grace"|"suspended", // app only
  "suspension_effective_epoch":u64-string,     // app only
  "sibling_bitmap":base64url(32 bytes),
  "siblings":[sha256-string]
}
```

The bitmap contains 256 bits in root-to-leaf order, MSB first in each byte. A 1
MUST appear if and only if the corresponding sibling differs from
`empty[d+1]`; a 0 means use that default. `siblings` are ordered root-to-leaf
and their count MUST equal the bitmap population count. An explicitly supplied
default hash (bit 1 with sibling equal to `empty[d+1]`) is non-canonical and
MUST be rejected even if it reconstructs the root. Negative fixture `SMT-1`:
all-default absent proof with bit 0 set to 1 and `siblings:[empty[1]]` rejects.

Canonical proof example (an absent account with all-default siblings):

```json
{"account_key":"AAECAwQFBgcICQoLDA0ODxAREhMUFRYXGBkaGxwdHh8","account_kind":"node","account_nonce":"0","balance_microtokens":"0","present":false,"sibling_bitmap":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA","siblings":[],"subject_id":"cblx1absentfixture"}
```

## Light-client balance verification

A light client can verify an account using a recent externally supplied weak-
subjectivity checkpoint, the trusted genesis configuration, the
validator sets/headers connecting it to a finalized header, that header's
quorum certificate, and one `AccountProof`:

1. **Validate the external checkpoint.** Load a signed checkpoint containing
   chain ID, finalized height/block ID, validator-set hash, and issued time.
   Require its age to be at most `max_weak_subjectivity_age_ms`; genesis and the
   following steps alone are not sufficient after that window. Missing, stale,
   or chain-mismatched checkpoints fail closed.
2. **Anchor the chain.** Load the configured network ID, derived chain ID,
   genesis block ID, and genesis validator set; recompute the set hash.
3. **Enforce non-regression.** Persist the highest trusted height and block ID.
   Reject a checkpoint, response, or restart state below it, and reject a
   different block ID at the same height.
4. **Advance trust.** For every newer header, recompute `block_id`, check chain,
   version, height, previous ID, and validator-set continuity. Verify the quorum
   certificate over the exact vote bytes using the currently trusted set and
   the strict quorum predicate. Fetch and hash a changed next
   set before using it. Never skip an untrusted set transition.
5. **Corroborate freshness.** Query independently operated enrolled peers,
   reject tips older than `max_current_balance_age_ms`, and require the selected
   finalized height to be consistent with the recent checkpoint. Peer agreement
   is an availability/fork alarm, never a substitute for proof verification.
6. **Select final state.** Require the proof response header height to equal the
   requested height exactly, never below persisted trust, and retain its `state_root`.
7. **Bind the account.** Recompute `account_key` from the requested account kind
   and subject ID and
   compare all 32 bytes. Reject malformed bitmap or sibling count.
8. **Create the leaf.** If `present` is true, compute the type-specific leaf. If
   false, require balance and nonce both zero, app-only fields absent, and use
   `empty[256]`.
9. **Rebuild and decide.** Iterate depths 255 down to 0. Obtain that depth's sibling
   from the proof (or the corresponding default). If the key bit is 0 hash
   `branch(current, sibling)`; if 1 hash `branch(sibling, current)`.
   Compare the final 32-byte value to `state_root` in constant time. Only on
   equality display the balance, lifecycle if applicable, and finalized height.

TLS, a signed peer envelope, or a proof from several servers cannot replace any
step above. Clients SHOULD query independent peers for availability and fork
alerts, but cryptographic acceptance depends on the authenticated header.

## State transition order

Within a block, transactions execute in this deterministic order after all
static checks: (0) `challenge_commitment`, `challenge_evidence`, and
`revoke_identity`, ordered by raw
transaction ID; (1) `fund_app` and `burn`, ordered by
`(account_kind, raw_account_key, debit_nonce, raw_tx_id)`; then (2) `mint`,
ordered by raw transaction ID. A mint may reference evidence from class 0 of
the same block or an earlier finalized block. For each account, class-1 nonces
MUST be exactly consecutive from pre-state; ordering solely by transaction ID
is invalid. Fixture `ORDER-1` has two otherwise-valid debits for one account
whose nonce 8 transaction ID sorts after nonce 9: canonical order is nonce 8
then 9 and the block is valid. Each debit increments its payer nonce exactly
once after subtracting its amount. Failed
execution invalidates the entire proposed block; there are no partially applied
transactions or fees in v0.

## DRAFT: committee selection and economic values

Two matters are intentionally open but fully bounded:

- committee selection alternatives are a weighted rotation or a
  finalized-randomness lottery, in both cases over an eligibility set. The
  selection algorithm is open; the **eligibility basis is not**. Per [ADR-007],
  eligibility MUST be anchored to finalized, hard-to-forge work — demonstrated
  storage and compute — and MUST NOT rest on uptime or availability alone, which
  a rented VPS with an SLA beats by construction against any real phone. The
  election rule itself, its randomness source, and its rotation cap are owned by
  AGENT-002 in M-02 and tracked in [DEBT-005]; nothing in this document fixes
  them;
- reward and price values, including the publisher-reward curve and the
  per-epoch existence fund, come from the economic simulator, either as fixed
  epoch tables or bounded governance curves. AGENT-002 and the Project Lead own
  the decision under ADR-005, ADR-006, and ADR-007. The curve is free only
  within the creator-share cap above, which is a validity rule and not a
  tuning parameter.

Neither open decision changes transaction kinds, mint/burn separation, signed
policy hashes, validator-set continuity, the revocation transition rule, or the
light-client proof algorithm.
