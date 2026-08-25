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
   is finalized;
8. the active validator set changes only at an election boundary or through a
   removal-only revocation transition; the set committed at an election boundary
   is the exact output of the derivation of
   [validator election and rotation](#validator-election-and-rotation); entry is
   capped and contraction is floored. A quorum authorizes the successor set and
   **cannot name its members** — but it does decide which candidacies are
   finalized, so the accurate statement is that it can narrow the field and not
   that it is powerless over composition. What bounds that power is the
   contraction floor, not the derivation.

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
        |"revoke_identity"|"validator_candidacy",
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
  "eligible_set_root":sha256-string,              // existence only
  "work_kind":"availability"|"storage"|"compute", // work only
  "app_id":sha256-string,                         // publisher only
  "active_subscriber_count":u64-string,           // publisher only
  "active_subscription_root":sha256-string,       // publisher only
  "counted_subscription_burn_microtokens":u64-string // publisher only
}
MintAuthorization = {"quorum_certificate":TransactionQuorumCertificate}
```

For `existence_income`, `work_kind` is absent, `eligible_node_count` and
`eligible_set_root` are required, and evidence MUST establish the configured availability threshold for
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

`E` is a divisor, so a quorum that inflates it silently reduces every honest
node's income and the difference is simply never minted, appearing nowhere as
inflation. The count is therefore committed to the set it counts, using the same
construction as the subscription tree below. Entries are the `account_key`s of
the nodes that met the threshold for that epoch, unique and sorted bytewise:

```text
eligible_leaf  = H(0x24 || u64be(reward_epoch) || account_key_32)
eligible_node  = H(0x25 || left_32 || right_32)
eligible_empty = H(0x26)
```

The tree preserves sorted order and pads to a power of two with
`eligible_empty`; zero entries use `H(0x27)` as the root, which cannot appear in
a valid mint because `E > 0`. Full validators already recompute `E` from the
finalized evidence of that epoch, so they hold the exact set: they MUST also
recompute `eligible_set_root` from it and reject a mint whose root differs,
exactly as they reject a mint whose count or quotient differs. `E` MUST equal
the number of leaves. The commitment costs a validator nothing it was not
already computing.

Declared limit, narrowed but not closed: the root makes the count **falsifiable**
— any full node, and any auditor replaying finalized evidence, can now contradict
an inflated `E` with a recomputation instead of an assertion. A light client
still verifies the arithmetic and the quorum rather than independently
recomputing eligibility, because it has the root but not the leaves; serving
per-epoch eligibility proofs is M-02 work. The field and its serialization are
fixed **now** for the same reason the challenge-assignment fields were fixed
before their algorithm existed: adding a commitment to `MintBody` after launch is
a breaking format change, and a migration costs more than a reserved field.

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
{"authorization":{"quorum_certificate":{"signatures":[{"signature":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA","validator_id":"val-001"}],"validator_set_hash":"sha256:1df0a6454faaa5985b7f98c48d3c60d2ed62d5b3b24fe8e97d3dca1dd36f1120"}},"body":{"amount_microtokens":"250000","beneficiary_node_id":"cblx1ci6q36gqm6u3spknxzr7p5r2y4xw7n25d5icm7rsoq7lq6ka","eligible_node_count":"4000","eligible_set_root":"sha256:2f0e2c8a9d4b6f1c3e5a7b9d0f2468ace13579bdf02468ace13579bdf02468ac","evidence_tx_ids":["sha256:313eb3d86d8c049838543325910bccb953b828da764b5f18bff11d7a123b0e68"],"policy_hash":"sha256:7df04d03b60f62852f0d76c847d0181a2b17b43a50f987c0b9f814e70f064bcc","reason":"existence_income","reward_epoch":"17"},"created_at_ms":"1787654500000","expires_at_ms":"1787740900000","kind":"mint","network_id":"coblox-devnet-0","schema_version":"0.1"}
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

Declared limit, quantified. Two facts, and they point in opposite directions.

**What the commitment does close.** `challenge_randomness` is a function of
`beacon_block_id` *and* `issuer_secret_32`, and the secret is not revealed when
the beacon is produced. A proposer that is **not** colluding with the issuer
therefore cannot compute the randomness any candidate block would yield, and so
cannot grind it at all, however many candidates it tries. This is the whole
purpose of the commit-before-beacon ordering and it is met.

**What remains open, with its cost.** With a colluding issuer that hands over
its committed secret, the pair can search jointly. The search is cheaper than
"discard a candidate block and try another" suggests: `BlockHeader` carries
`timestamp_ms`, constrained only to exceed the median of the previous 11
finalized blocks and not to exceed the maximum clock drift. At millisecond
granularity that window admits on the order of **10³–10⁶ legal values**, each
producing a different `block_id` for the cost of **one SHA-256** over a
few-hundred-byte header. The proposer does not discard blocks; it enumerates a
field. The pair looks for a beacon satisfying two conditions at once — that the
epoch's assignment function pairs that issuer with the target subject, and that
the resulting randomness selects a chunk the subject actually kept.

**Why this is a reduction in detection rate and not a bypass.** The two-issuer
coverage rule of [wire.md](wire.md#challenge_request) — every subject covered by
at least two distinct issuers per epoch, none of them the subject — is
**the mitigation of this grinding**, not a redundancy measure, and the two rules
are stated in different documents so the link is made explicit here. The honest
second issuer still queries the subject from an unground beacon, so a successful
search degrades the attack from "pass the challenge" to "pass one of two".

Two reductions are available and are not taken in v0: quantizing `timestamp_ms`
to the consensus slot, or deriving beacon material from the `block_id`s of `K`
consecutive blocks so that grinding requires `K` consecutive proposals by the
same attacker. Both belong with the dedicated randomness beacon, which is M-02
work under [DEBT-005]. The residual is stated with its order of magnitude rather
than assumed away, and the word "bounded" without a number is not a bound.

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
  "election":ElectionRecord,   // absent only for the genesis set
  "validators":[{
    "validator_id":string,
    "node_id":string,
    "consensus_public_key":base64url(32 bytes),
    "key_binding_signature":base64url(64 bytes),
    "seated_since_epoch":u64-string,
    "term_expiry_epoch":u64-string,
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

`election` and `seated_since_epoch` are specified in
[validator election and rotation](#validator-election-and-rotation); they are
what constrains *which* members the old quorum is permitted to commit. The
genesis set is the only set without an `election` record, because it is a trust
anchor rather than a derived object; its entries carry `seated_since_epoch:"0"`
and **staggered** `term_expiry_epoch` values, for the reason given in
[the genesis cohort](#the-genesis-cohort-and-why-its-terms-must-be-staggered).

For each entry, the identity public key from the finalized enrollment
certificate MUST verify `key_binding_signature` over the global chain-bound
domain `coblox-consensus-key-binding-v0` and JCS of
`{"activation_height":...,"consensus_public_key":...,"node_id":...,"validator_id":...}`.
The consensus key MUST differ from the identity key and is never independently
enrolled. Full nodes and light clients verify every binding before accepting a
set or any vote from it. On leaving the active set, operators MUST destroy or
rotate the old consensus private key; re-entry requires a fresh binding.

Continuity authenticates a transition. It does not by itself say **when** a
transition may happen, and the schema above admits one at every height. v0
therefore adds a second condition, checkable from the header alone and stated
here because it is the precondition for everything in
[validator election and rotation](#validator-election-and-rotation):

> At every finalized height `h` that is neither an **election boundary** nor a
> **revocation-forced transition**, `next_validator_set_hash` MUST equal
> `validator_set_hash`. A block that changes the committed successor set outside
> those two occasions is invalid.

A light client checks this with two fields it already reads, without seeing a
single transaction. Which members may appear at an election boundary, and which
may appear at a revocation-forced transition, are the subjects of the next two
sections.

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

Rules 1–3 are complete **for a full node**, which sees the `revoke_identity`
transaction and can therefore evaluate them. They are not complete for a light
client, and this document previously claimed otherwise. The claim was false and
is corrected here, because a wrong safety statement is worse than a missing one:

> A light client checks set-hash continuity and `key_binding_signature`s and
> **never sees transactions**, so it does not know that a revocation exists or
> whom it names. It observes *a* transition if one happens; it cannot establish
> that a transition was *due*. A chain on which the transition never happened
> satisfies continuity and every binding, and is therefore indistinguishable to
> it.

The consequence is concrete. If the consensus keys of validators summing to more
than two thirds of the power leak — the exact scenario this rule exists for —
honest full nodes stall as declared below, while the attacker signs a parallel
chain from `effective_height` with the *old* set. That chain is hash-continuous,
every binding verifies, and a light client passes every step on it. Safety
would then protect whoever runs a server and not whoever installed the app.

The missing anchor is not a `BlockHeader` field — adding one would cost every
block forever and would still be authenticated by the very set that is
compromised. It is in the trust anchor. The closure is therefore:

5. the weak subjectivity checkpoint carries `revoked_validators` and
   `revocation_root`, defined in
   [README.md](README.md#weak-subjectivity-checkpoint);
6. a light client MUST reject any block at height `>= effective_height` whose
   active validator set contains a `node_id` listed in its checkpoint, applying
   rule 2 with the data the checkpoint gives it, and MUST reject any set
   containing such a `node_id` with `activation_height >= effective_height`,
   applying rule 1.

**Declared limit, stated with its bound.** This closes the gap only for
revocations known when the checkpoint was issued. A revocation finalized
afterwards is invisible to a client running on that checkpoint, so its exposure
window is at most `max_weak_subjectivity_age_ms` and it then fails closed. That
window is exactly the emergency window, and the two parameters pull against each
other: `min_revocation_effective_delay_blocks` is chosen **long** to make the
stall below rare, which lengthens the interval during which a revocation is
finalized but not yet effective. Governance MUST therefore choose
`max_weak_subjectivity_age_ms` no greater than the expected wall-clock duration
of `min_revocation_effective_delay_blocks`, so that a checkpoint a client still
accepts is never older than the window granted to commit a compliant successor
set. Choosing them independently silently widens the hole.

A network MUST publish a fresh checkpoint on any validator revocation rather
than waiting for its ordinary release cadence.

Declared consequence, stated rather than discovered later: if the remaining
validators fail to commit a compliant successor set within the delay window, the
chain **stalls** at `effective_height` instead of finalizing blocks signed by a
set containing a revoked key. That is a deliberate choice of safety over
liveness, and `min_revocation_effective_delay_blocks` exists to make the window
long enough that the choice is rarely exercised.

**A revocation-forced transition removes; it never admits.** This is the second
validity condition on off-boundary transitions, and it exists because an
"emergency replacement" clause would reopen exactly the hole that
[validator election and rotation](#validator-election-and-rotation) closes: an
unelected member seated by the sitting quorum, under a pretext the quorum
itself creates. A set whose `activation_height` is not an election boundary is
valid only if:

7. its `validators` array is a **strict subset** of the array of the set it
   replaces, entry-for-entry identical in `validator_id`, `node_id`,
   `consensus_public_key`, `seated_since_epoch`, `term_expiry_epoch`, and
   `voting_power`, with only `key_binding_signature` re-issued for the new
   `activation_height`;
8. every removed `node_id` has a finalized `revoke_identity` whose
   `effective_height` is at most that `activation_height`;
9. its `election` record is copied verbatim from the set it replaces, except
   `member_count`, which MUST equal the new, smaller array length;
10. it satisfies the **contraction floor**
    `3 * member_count(new) > 2 * member_count(old)` of
    [rotation: the cap and the floor](#rotation-the-cap-and-the-floor).

The vacancies stay empty until the next election boundary, where they are
refilled under the ordinary rotation cap. Consequently a quorum cannot launder
mass revocation into mass admission. Nor — and this needed rule 10 to be true,
having previously been asserted without it — can it launder mass revocation into
**concentration**: for an attacker that is already inside the set, removing the
others *is* choosing the set, and it was reachable while the only lower bound on
size was `validator_min_set_size`, which the constraint block permits to be very
small. With rule 10, shrinking the set far enough reaches the stall rather than a
set of the attacker's choosing **in one step**. A coalition contracts to itself in
a single transition only if it already holds more than two thirds, at which point
BFT safety has failed for reasons no set-composition rule can repair; below that
it can still get there over several transitions, each of them published, as
[the contraction floor](#the-contraction-floor-a-cap-on-entry-is-only-half-a-rule)
sets out. What a light client can and cannot
conclude from an off-boundary transition is stated in
[what a light client can establish](#what-a-light-client-can-establish-about-set-composition).

## Validator election and rotation

Continuity says the outgoing quorum authorizes its successor. Until v0 wrote
this section, nothing said what it was allowed to authorize, so a quorum reached
once could commit a successor identical to itself, for ever, along a chain that
satisfied every check in this document at every step. That is the defect this
section closes, and closing it is not only a matter of preventing capture: it is
a matter of making the active set verifiable as **the right one** rather than
merely as **a continuous one**. The two are different claims, and the second is
the one continuity already provided.

The rule is built in two layers on purpose, because they fail differently.

- **Layer 1 — shape and turnover.** Term limits, uniform voting power, a
  rotation cap, and the boundary rule above. Layer 1 is a function of the
  validator-set documents alone, so a light client that never sees a transaction
  verifies all of it. Layer 1 is what makes self-perpetuation *impossible*
  rather than *unlikely*, and it does not depend on the quality of any
  randomness.
- **Layer 2 — composition.** Which eligible candidates fill the seats Layer 1
  vacates, derived from a committed candidate set and a finalized seed. Layer 2
  is verifiable in full by any node that replays finalized transactions, and only
  partially by a light client. Its residuals are stated in
  [what a light client can establish](#what-a-light-client-can-establish-about-set-composition).

Splitting them this way is a deliberate answer to the risk that a rule designed
for full nodes leaves light clients where they were. The anti-capture invariant
lives entirely in the layer a light client can check.

### Election epochs and the boundary

Election epochs are counted in block heights, not wall-clock time, so that every
quantity a verifier needs is a header quantity. With
`election_epoch_blocks = L` from the active consensus parameters, epoch `e`
begins at

```text
election_boundary_height(e) = e * L
```

Epoch 0 is the genesis set. An **election boundary** is any height of that form
with `e >= 1`. The set activating at `election_boundary_height(e)` MUST carry an
`election` record with `election_epoch` equal to `e` and `activation_height`
equal to that height.

Two derived heights bound the inputs:

```text
candidacy_close_height(e) = election_boundary_height(e) - candidacy_close_blocks
entropy_window(e)         = [ election_boundary_height(e) - election_entropy_blocks,
                              election_boundary_height(e) - 1 ]
```

`candidacy_close_blocks > election_entropy_blocks` is a validity rule on the
consensus-parameters document, so the candidate set is finalized and fixed
strictly before the first block whose identity feeds the seed exists. Without
that ordering a proposer inside the entropy window could watch the seed forming
and add or withhold a candidacy in response.

### Candidacy is an explicit, per-epoch act

A node cannot be conscripted into the set, and no member is retained implicitly.
Serving in epoch `e` requires a finalized `validator_candidacy` for that exact
epoch, from incumbents and newcomers alike. There are three reasons and all
three are load-bearing: the consensus key binding of
[validator-set continuity](#validator-set-continuity) is signed over a specific
`activation_height`, which only a per-epoch declaration can supply in advance;
consent to hold consensus keys is not something a quorum should be able to
assert on someone else's behalf; and a member that stops declaring leaves
without any need for a removal mechanism.

```text
ValidatorCandidacyBody = {
  "node_id":string,
  "election_epoch":u64-string,
  "consensus_public_key":base64url(32 bytes),
  "key_binding_signature":base64url(64 bytes)
}
ValidatorCandidacyAuthorization = {
  "public_key":base64url(32 bytes),
  "signature":base64url(64 bytes)
}
```

The authorization key MUST derive the enrolled, unrevoked `node_id`.
`key_binding_signature` is the signature of the **consensus** key over the
binding object of [validator-set continuity](#validator-set-continuity) with
`activation_height = election_boundary_height(election_epoch)` and
`validator_id = node_id`; validators verify it when the candidacy executes, so a
candidacy carrying a binding that would not verify inside a set is invalid on
arrival rather than at the boundary. The consensus key MUST differ from the
identity key. At most one candidacy exists per `(node_id, election_epoch)`; a
second is invalid, so a node cannot hold several consensus keys for one epoch
and let the outgoing set choose which one to seat. `election_epoch` MUST be an
epoch whose `candidacy_close_height` is strictly above the height of the block
proposing the candidacy. This transaction moves no value and touches no balance
or nonce.

Canonical serialized example:

```json
{"authorization":{"public_key":"11qYAYdk9J0L5Z-6hB4qMTPBSAE5nK1G0IU2n6z1V9g","signature":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"},"body":{"consensus_public_key":"IjIiMiIyIjIiMiIyIjIiMiIyIjIiMiIyIjIiMiIyIjI","election_epoch":"3","key_binding_signature":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA","node_id":"cblx1ci6q36gqm6u3spknxzr7p5r2y4xw7n25d5icm7rsoq7lq6ka"},"created_at_ms":"1787654530000","expires_at_ms":"1787740930000","kind":"validator_candidacy","network_id":"coblox-devnet-0","schema_version":"0.1"}
```

**Elected sets use `validator_id = node_id` and uniform voting power.** Every
entry of a set carrying an `election` record MUST have `validator_id` equal to
its `node_id` and `voting_power` equal to `1`. Distinct validator identifiers
bought nothing and cost an ambiguity — two candidates claiming one
`validator_id` — that the election would then have to resolve. Uniform power is
not cosmetic either: it is the reason a node cannot buy influence by
contributing more, which is the first point of the [ADR-008] test. The quorum
predicate is unchanged and still operates on summed power; over an elected set
that sum happens to equal a member count, which is a consequence of the weights
and not a change to the predicate.

### Eligibility: demonstrated storage and compute, never availability

Per [ADR-007], eligibility is anchored to work that is hard to forge, and never
to uptime or availability, which a rented VPS with an SLA beats by construction
against any real phone. The protocol expresses this as a **contribution score**
computed from evidence that already exists on the ledger for another purpose.

Let `W(e)` be the finalized `challenge_evidence` transactions with
`outcome:"passed"` and `kind` in `{"storage","compute"}` whose
`subject_node_id` is the node, finalized below `candidacy_close_height(e)` and
belonging to one of the last `validator_eligibility_window_epochs` reward
epochs. With `storage_units_per_contribution_unit` and
`compute_units_per_contribution_unit` from the active reward policy, both
strictly positive:

```text
contribution_score(n, e) =
    sum over W(e) of  measured_units / divisor(kind)     // integer division per item
```

Intermediates are checked `u128`. Evidence of kind `availability` contributes
**zero**: this is [ADR-007] point 2 written as arithmetic rather than as an
intention, and it is the single line that makes the rule unattractive to a
datacenter fleet whose only real advantage is being switched on.

A node is **eligible** for epoch `e` when all of the following hold, each of them
a fact a replaying verifier settles without judgement:

1. it is enrolled and not revoked as of `candidacy_close_height(e)`;
2. a valid `validator_candidacy` for `(n, e)` is finalized strictly below
   `candidacy_close_height(e)`;
3. `contribution_score(n, e) >= validator_eligibility_threshold_units`;
4. the evidence counted in that score comes from at least
   `validator_eligibility_min_issuers` **distinct** `issuer_node_id`s, none of
   them the subject itself. A score built from a single issuer does not qualify
   however large it is;
5. it is not in cooldown: it did not **leave a seat** in any of the
   `validator_cooldown_epochs` epochs before `e`. Leaving a seat means having
   been a member of the set active at a boundary and not retained at the next
   one, **for any reason whatsoever** — term expiry, a lapsed candidacy, a
   contribution score fallen below the threshold, or revocation.

Condition 5 says "for any reason whatsoever" because an earlier version of it
said "through term expiry", and that version was **evadable by playing well**.
An incumbent one epoch short of its term limit could simply file no candidacy,
leave as a voluntary exit with no cooldown attached, re-file for the next epoch
and return with `seated_since_epoch` reset — an effective absence of one epoch
against however many the network had chosen. The occupancy arithmetic made the
evasion dominant rather than marginal: `T/(T+k)` for a member that serves out its
term against `(T-1)/T` for one that leaves early, and the second exceeds the
first for every `validator_cooldown_epochs = k >= 2`. A parameter that the
diligent obey and the strategic ignore is worse than no parameter, because it is
budgeted for as though it worked. Under condition 5 the answer to "how long must
a departing member that plays well stay out?" is `validator_cooldown_epochs`,
with no qualification.

**Declared limit, inherited and not created here.** Condition 3 is only as sound
as the evidence it counts, and this document already quantifies a way to obtain
`outcome:"passed"` evidence more cheaply than by doing the work: a colluding
issuer that hands over its committed secret lets a proposer enumerate the legal
`timestamp_ms` values — of order `10^3` to `10^6`, one SHA-256 each — until the
beacon both assigns that issuer to the target subject and selects a chunk the
subject actually kept. See
[challenge evidence](#challenge-evidence), where the residual is stated for the
reward channel.

This section put that evidence to a **new use**, as a credential for entry into
consensus rather than as a metric for payment, and the mitigation declared there
does not transfer. The two-issuer coverage rule of
[wire.md](wire.md#challenge_request) degrades the attack from "pass the
challenge" to "pass one of two", which is an effective answer for a *detection
rate*, where a failure counts against the subject. `contribution_score` is a
**sum of successes** and subtracts nothing: passing one of two still adds
`measured_units`, and the honest issuer's failed challenge has no effect on
eligibility at all. Left unaddressed, a pair of enrolled identities — one playing
issuer, one playing subject — could clear the threshold while holding a single
chunk instead of the objects the score claims to measure, at the price of two
enrollments plus grinding rather than the price of storage.

Condition 4 is the answer, and it is a counting condition on data that is already
finalized rather than a change to how challenges are issued: the score must draw
on at least `validator_eligibility_min_issuers` distinct issuers. **It raises the
price; it does not remove the residual.** An attacker willing to enroll
`validator_eligibility_min_issuers` colluding issuer identities per fabricated
candidate still clears the threshold, and the cost of doing so is the cost of
enrollment — which [ADR-007] has already declared cannot price a perpetual flow.
So the honest formulation is that eligibility is anchored to work that is
**expensive to fake and not impossible to fake**, that the price is linear in
`validator_eligibility_min_issuers`, and that what ultimately bounds the attack
is the same numerosity residual `alpha` governs, plus the churn cap on how fast
tickets become seats. Writing "cannot be faked without spending real resources"
would be the overstated safety claim this document refuses to make.

**Eligibility is a predicate, not a ranking, and this is the whole design.**
Above the threshold, additional storage or compute buys nothing: not a seat, not
a weight, not a better draw, because every eligible node contributes exactly one
ticket and every seated node exactly one unit of power. A ranking by contributed
work would have been the obvious rule and is rejected here for precisely the
reason [ADR-008] gives: a rule that pays more for spending more, without the
network needing more, is mining wearing a different hat, and it would have
reproduced under the eligibility heading the spending race that the exclusion
forbids. The test is answered in full in
[the ADR-008 test, answered](#the-adr-008-test-answered).

### The committed candidate set

The eligible nodes of epoch `e` are committed as a Merkle tree over their
`account_key`s, unique and sorted bytewise, in the same construction the
existence-income eligible set already uses:

```text
candidate_leaf  = H(0x40 || u64be(election_epoch) || account_key_32)
candidate_node  = H(0x41 || left_32 || right_32)
candidate_empty = H(0x42)
```

The tree preserves sorted order and pads to a power of two with
`candidate_empty`; zero entries use `H(0x43)` as the root, which cannot appear
in a valid election because an epoch with no eligible candidate and no retained
member has no valid set at all. `candidate_count` MUST equal the number of
leaves. Full validators already recompute eligibility from finalized evidence,
so they hold the exact leaf set and MUST reject an `election` record whose root
or count differs from their own recomputation.

Sorting the leaves by `account_key` is not presentation. It is what makes
**non-membership** provable in two leaves, which is how a candidate unlawfully
left out contradicts the record that omitted it; see
[what a light client can establish](#what-a-light-client-can-establish-about-set-composition).

### The seed, and why the rule does not depend on it

```text
election_entropy = H("coblox-election-entropy-v0\0" || chain_id_32
                     || u64be(election_epoch) || u64be(election_entropy_blocks)
                     || raw_32_bytes(block_id[first]) || ... || raw_32_bytes(block_id[last]))
election_seed    = H("coblox-election-seed-v0\0" || chain_id_32
                     || u64be(election_epoch) || raw_32_bytes(election_entropy))
election_ticket  = H("coblox-election-ticket-v0\0" || chain_id_32
                     || raw_32_bytes(election_seed) || account_key_32)
```

The block IDs are the canonical finalized IDs at every height of
`entropy_window(e)`, in ascending height order, exactly
`election_entropy_blocks` of them. They are carried in the `election` record so
that a client which joined at a later checkpoint can still recompute the seed; a
client that walked those heights MUST additionally check that they are the IDs
it saw.

The seed depends on the entropy window **and on nothing else**. An earlier draft
of this section also hashed `candidate_root` and `candidate_count` into the seed,
to stop a seed being reused against a different candidate set. That binding is
not needed and has been removed: `ElectionRecord` commits both quantities, full
validators recompute both from finalized evidence and reject a record whose
values differ, and the seed is already bound to its epoch by
`u64be(election_epoch)` and to its chain by `chain_id`. Keeping them in the
preimage bought no validity and gave the outgoing set a **second lever** on the
seed, because the composition of the candidate set is a function of which
candidacy transactions get finalized, which the outgoing set controls. Removing
them reduces what a proposer can steer to exactly one thing — the block IDs it
proposes — which is what makes the paragraphs below stateable without
qualification.

**Now the honest part, and it is the part that decides whether this rule is worth
anything.** A beacon derived from block IDs is *grindable by whoever proposes
those blocks*. The proposer of a block in the entropy window chooses its
transaction set and its `timestamp_ms`; the timestamp alone admits on the order
of 10^3 to 10^6 legal values, as [challenge evidence](#challenge-evidence)
already quantifies, and each is one SHA-256 away from a different `block_id`.
Aggregating `election_entropy_blocks` consecutive blocks raises the cost of
controlling the *whole* window to holding consecutive proposal slots — the
reduction this document deferred to "the dedicated randomness beacon" and takes
here — but it does not remove best-of-`G` resampling by whoever proposes the
**last** block of the window. An unbiasable beacon in v0 would need a verifiable
delay function or a threshold signature scheme, and v0 has Ed25519 and nothing
else.

So the seed is **not** trusted to be unbiasable, and the security of this section
does not rest on it:

- grinding yields *bias*, never *choice*: a resample redraws the tickets, it does
  not let the grinder name a winner;
- the number of seats a grinder can win at one boundary is bounded by
  `validator_churn_cap_seats` however favourable the seed is, because the cap is
  applied after the draw;
- an incumbent cannot grind itself a longer term at any price, because term
  expiry is not a function of the seed.

The residual, stated with its shape rather than assumed away: with `c` seats
filled per boundary, an attacker holding a fraction `p` of the committed
candidates and `G` grinding attempts wins on the order of
`c*p + O(sqrt(c*p*(1-p)*2*ln G))` seats instead of `c*p`. The excess grows with
the square root of the logarithm of the attacker's effort and is capped at `c`
regardless. That is a fairness loss inside a bounded rotation, not a capture
path.

#### The second lever: the pool itself, and what is honestly claimable about it

An earlier version of this section claimed that "every resample is still a draw
over a committed candidate set the grinder does not control". **That claim was
false for the only adversary that matters** and is retracted here rather than
softened. A grinder is by construction a sitting validator, since only a proposer
can grind; a sitting validator controls **transaction inclusion**; a candidacy
that is not included is not finalized; therefore the outgoing set controls which
nodes are in `C` at all. It does not control a committed set — it controls what
gets committed.

What limits that lever is an **ordering** property rather than an assertion, and
it is worth stating exactly because it is the only thing standing there.
`candidacy_close_blocks > election_entropy_blocks` puts
`candidacy_close_height(e)` strictly **below** the entropy window, so the set of
candidacies is frozen before the first block whose ID feeds the seed exists.
A proposer choosing which of `q` withheld candidacies to finalize therefore
chooses **blind**: it cannot yet compute any ticket, so it cannot pick the subset
that produces the ticket vector it wants. Combined with the removal of
`candidate_root` from the seed preimage, subset choice now yields no seed
advantage at all — before the removal it yielded `2^q` seed families, chosen
blind and therefore of little use, but the surface existed and no longer does.

**What the lever still buys, and it is not nothing: exclusion.** Withholding a
candidacy removes a competitor from the pool outright, which is worth more than
any amount of grinding and does not need the seed. Two properties of that
residual must be stated:

- it is **bounded in effect** by the same quantities as everything else in this
  section: excluding competitors cannot seat more than
  `validator_churn_cap_seats` members at a boundary, cannot extend a term, and
  cannot shrink the set past the contraction floor of
  [rotation: the cap and the floor](#rotation-the-cap-and-the-floor);
- it is **worse in visibility** than the omission case (a) of
  [what a light client can establish](#what-a-light-client-can-establish-about-set-composition).
  There the omitted node contradicts the record by exhibiting its own *finalized*
  candidacy against a sorted-tree non-membership proof. Here the candidacy was
  never finalized, so there is no compact proof and no long one either: a
  censored candidacy and a candidacy never submitted are the same object, which
  is to say no object. Delayed inclusion is indistinguishable from absence and
  leaves no trace. This is recorded in the "cannot establish" list as its own
  entry rather than folded into (a), because it is the one composition failure
  that not even a full node replaying the whole chain can detect.

**Two alternatives were considered and rejected.** Candidate commit-reveal, in
which each candidacy carries a commitment and the seed mixes the reveals, moves
the bias from the proposer to the candidates and makes it *worse*: a candidate
that dislikes the outcome simply withholds its reveal, and withholding is free
because a candidate that is not seated loses nothing. Deriving the seed from the
finalized `issuer_reveal` values of that epoch's challenge evidence looks
attractive, because those secrets are committed before their beacons, but the
outgoing set still controls which evidence transactions are *included*, so the
bias returns through inclusion rather than through hashing, and it arrives harder
to see.
### The derivation

Every step below is a total function of finalized data. Two verifiers holding the
same finalized chain derive byte-identical sets, or the block at the boundary is
invalid. Let `P` be the validator set active at
`election_boundary_height(e) - 1` and let `T = validator_max_consecutive_terms`.

1. **Retain.** An entry `n` of `P` is retained when *all* of these hold: a valid
   candidacy for `(n, e)` is finalized below `candidacy_close_height(e)`;
   `e < term_expiry_epoch(n)`; `contribution_score(n, e)` meets the threshold;
   and `n` is not revoked. A retained entry keeps its `seated_since_epoch` **and
   its `term_expiry_epoch`** unchanged, and takes its consensus key and binding
   from its epoch-`e` candidacy. Call the result `R`.
2. **Commit the candidates.** `C` is the eligible set of the section above —
   which contains the retained members too — committed as `candidate_root` with
   `candidate_count` equal to `|C|`.
3. **Form the fill pool.** `Nw = C \ R`, the eligible nodes not already seated. A
   member of `P` that failed step 1 is **not** in `Nw` for this epoch, and is not
   in `C` either: leaving a seat starts the cooldown of eligibility condition 5
   whatever the reason for leaving, so term expiry, a lapsed candidacy, a fallen
   contribution score and revocation are alike in this respect.
4. **Derive the seed** from `entropy_window(e)` alone, per
   [the seed](#the-seed-and-why-the-rule-does-not-depend-on-it).
5. **Rank.** Compute `election_ticket` for every node of `Nw` and order the pool
   by `(ticket ascending, account_key ascending)`, both compared as unsigned
   bytes over the raw 32 bytes. The second key makes the order **total**: equal
   tickets would require a SHA-256 collision, but a derivation with an
   unspecified case is not deterministic even when the case is unreachable, and
   an unreachable case left open is how a future hash change becomes a chain
   split.
6. **Fill, under the cap.**

   ```text
   fills = min( max(0, validator_target_set_size - |R|),
                validator_churn_cap_seats,
                |Nw| )
   ```

   The first `fills` nodes of the ordered pool are seated with
   `seated_since_epoch` equal to `e`, `term_expiry_epoch` equal to
   `e + validator_max_consecutive_terms` evaluated against the parameters active
   at `e`, `voting_power` 1 and `validator_id = node_id`.
7. **Assemble.** The new set is `R` together with the fills, sorted by
   `validator_id`, carrying the `election` record below. It MUST satisfy the
   **contraction floor** `3 * member_count(new) > 2 * member_count(P)` and hold
   at least `validator_min_set_size` members. If either fails, **no valid set
   exists for epoch `e`** and the chain stalls at the boundary; see
   [degenerate cases](#degenerate-cases-and-what-the-protocol-does-instead-of-improvising).

```text
ElectionRecord = {
  "election_epoch":u64-string,
  "previous_validator_set_hash":sha256-string,
  "candidate_root":sha256-string,
  "candidate_count":u64-string,
  "entropy_first_height":u64-string,
  "entropy_block_ids":[sha256-string],
  "election_seed":sha256-string,
  "retained_count":u64-string,
  "filled_count":u64-string,
  "member_count":u64-string
}
```

`entropy_block_ids` holds exactly `election_entropy_blocks` entries in ascending
height order starting at `entropy_first_height`, which MUST equal
`election_boundary_height(election_epoch) - election_entropy_blocks`.
`previous_validator_set_hash` MUST equal the hash of `P`. `candidate_root` and
`candidate_count` are bound here by **validity** rather than through the seed
preimage: full validators recompute both from finalized evidence and reject a
record whose values differ, which is a stronger check than hashing them into a
seed and costs the outgoing set one lever less. `retained_count` and
`filled_count` MUST equal the number of entries whose `seated_since_epoch` is
respectively below and equal to `election_epoch`, and `member_count` the array
length. Those three counts are redundant with the array on purpose: a light
client checks the cap against `filled_count` and `filled_count` against the
array, so a set that lies about either contradicts itself.

**Where the commitment lives, and why not in the header.** `ElectionRecord` is
part of `ValidatorSet`, so it is committed by `validator_set_hash`, which the
previous height already commits as `next_validator_set_hash` — a `BlockHeader`
field. The header therefore commits every input of the derivation, transitively
and exactly, and after-the-fact recomputation is possible from headers plus the
set documents a client already fetches. A dedicated header field was the obvious
alternative and is rejected: it would cost every block for ever to carry a
quantity that changes once per epoch, and it would be authenticated by the same
quorum anyway, so it would buy no independence.

### Rotation: the cap and the floor

A cap alone does not close this defect. A set merely forbidden to change *fast*
is still permitted to change *never*, which is the self-perpetuating chain with
extra steps. v0 therefore states both ends.

**The floor is a term limit, stamped and not derived.** Every entry carries a
`term_expiry_epoch`, and a set is invalid if any entry has
`election_epoch >= term_expiry_epoch`. A seat filled at boundary `e` is stamped
`term_expiry_epoch = e + validator_max_consecutive_terms`; a retained entry keeps
the stamp it was given, unchanged, for the whole of its tenure.

Stamping rather than recomputing `e - seated_since_epoch < T` at every boundary is
not a presentational choice. With the derived form, a quorum that raised `T`
inside the genesis ceiling would extend **its own sitting members' terms
retroactively**, because their expiry would be recomputed against the new value —
a smaller version of the same manoeuvre the genesis bounds exist to stop, and one
those bounds do not by themselves prevent. With the stamped form a change to `T`
governs only seats filled after it activates, and no document a sitting set signs
can lengthen a term already running. `seated_since_epoch` remains, because the
retained/filled distinction and the cross-set consistency check are stated on it.

Turnover is consequently not a target but an arithmetic certainty: a set of `V`
members whose terms are capped at `T` vacates at least `ceil(V / T)` seats per
epoch on average, whatever anyone intends. A retiring member enters cooldown for
`validator_cooldown_epochs` and then competes for re-entry from the pool like
anyone else. Cooldown does not banish a genuinely contributing node; it forces
every seat to be **re-won by derivation** instead of retained by inertia, which
is the property [DEBT-005] found missing.

#### The genesis cohort, and why its terms must be staggered

The three rules of this section — term expiry, the entry cap, the contraction
floor — are each satisfiable alone and were, for one draft, jointly unsatisfiable
on **every conformant network**, at a boundary that arrives by the calendar and
not by anybody's choice. The interaction is worth writing out, because it is
exactly the kind that no rule sees from inside itself.

A genesis set is a trust anchor: `V` entries installed at once. If all of them
carry the same expiry, they expire **together**. At that boundary `R` is empty,
so the new set is whatever the fill step can supply, which is at most `c`; the
contraction floor then demands `3c > 2V`, that is `c > 2V/3`, while the capture
constraint demands `3 * c * m <= V` with `m >= 1`, that is `c <= V/3`. The
interval is empty for every `V`. No valid set exists, and the chain halts at
height `validator_max_consecutive_terms * election_epoch_blocks` with recovery
only out of band.

**The tempting repair is a trap and is refused.** Exempting the floor when `R` is
empty reopens the attrition capture of the contraction floor above, because a
quorum that controls inclusion can *manufacture* an empty `R` by censoring every
candidacy and then walk through its own exemption. It is the same objection this
document already makes twice — to continuing the previous set when no lawful
successor exists, and to suspending an election for want of a seed. An exception
clause is worth exactly as much as the difficulty of fabricating its trigger, and
this one is free to fabricate.

The cause is not the floor. It is the **synchronization**, and synchronization is
introduced at genesis, where no quorum has any say:

> **Genesis stagger.** In the genesis set, every entry's `term_expiry_epoch` lies
> in `[1, validator_max_consecutive_terms]`, and no more than
> `validator_churn_cap_seats` entries share the same value. A genesis set
> violating either condition is not a valid trust anchor and a client MUST refuse
> it.

Thereafter the property maintains itself, because expiries at boundary `e` are
the stamps written at boundary `e - T`, and at most `c` seats are filled per
boundary — **provided `T` does not shrink**, which is the subject of
[a term limit may not shrink](#a-term-limit-may-not-shrink) below and is a
condition of this argument rather than a detail of it. So at most `c` seats
expire at any boundary, ever, and
the guarantee to want is that a boundary at which a whole cohort expires and
**nobody at all is seated** still yields a valid set: the survivors must exceed
two thirds on their own, that is `3 * (V - c) > 2V`, which is `3c < V`. It is
added to the constraint block below as a rule rather than left as an observation.
A network that seats replacements keeps its size and clears the floor trivially;
`3c < V` is what stops a single empty candidate window from turning an ordinary
retirement into a halt. A staggered genesis introduces no exception, no
special case in the derivation, and nothing a quorum can trigger — the stagger is
fixed in an object no on-chain document can rewrite.

#### A term limit may not shrink

The self-maintenance argument above is exact when `T` is fixed and **false when
`T` decreases**, and the arithmetic says so precisely. A seat filled at boundary
`e` is stamped `e + T(e)`, so two seats filled at distinct boundaries
`e1 < e2` collide when `e1 + T(e1) = e2 + T(e2)`, which happens if and only if
`T(e2) < T(e1)`. **Collisions exist exactly when the term limit is shortened**,
and every collision puts more than one cohort on the same boundary, which is the
one thing `3c < V` is not sized for: that bound covers a single cohort.

The consequence is a halt, and it needs no adversary. Shortening terms on the
economic simulator's advice is the most ordinary act of governance imaginable.
With `V = 12` and `c = 3`, walking `T` down one step per boundary from 12 to 4 —
every intermediate value satisfying `ceil(V/T) <= c` and `3c < V`, every document
inside the change ratio and the activation gap — sends the seat filled at each of
those boundaries to the **same** expiry epoch, because `1 + 11`, `2 + 10`, and so
on down to `8 + 4` are all 12. Nine of twelve seats then expire at boundary 12:
`R` is three, `fills` is capped at three, the new set is six against a previous
twelve, and `3 * 6 > 2 * 12` is false. **A full candidate pool does not save
it**, because what limits the rebuild is the entry cap and not a shortage of
candidates.

> **Monotonic term limit.** A `consensus_parameters` document is accepted only if
> its `validator_max_consecutive_terms` is greater than or equal to the value in
> the currently active document. On a live chain the term limit never decreases.

Raising it is unrestricted beyond the genesis ceiling and the change ratio, and
costs nothing, precisely because the limit is **stamped**: a longer `T` governs
only seats filled after it activates and cannot lengthen a term already running.
That is the same property that made the stamp worth having, paying for itself a
second time.

**Why the more permissive rule is not taken, with the argument rather than a
preference.** The obvious liberalization is to allow a reduction whenever it
cannot collide — accept `T_new < T_old` only when
`e + T_new > max(term_expiry_epoch)` over the active set. It is sound, and it is
**not evaluable when a document is accepted**. Acceptance happens at some height;
activation happens later; and a seat filled at the last boundary before
activation is stamped with the *old* limit, so the condition an acceptance-time
check can actually guarantee is
`e_a + T_new > (e_a - 1) + T_old`, which is `T_new >= T_old` — the monotonic rule
again. The permissive version has bite only if it is evaluated **at activation**
against the set then active, which means a governed document whose activation is
conditional on chain state. v0 does not have that concept and this section is not
the place to introduce it.

**This is a rejection on cost, not an impossibility.** If a later version gives
protocol documents conditional activation, the permissive rule becomes available
and is strictly better, because a term limit that can only ever grow is a
one-way door on a safety-relevant quantity: a network that starts with terms too
long cannot correct them without the out-of-band recovery this document reserves
for stalls. That cost is real and is declared here rather than discovered by the
operator who first needs to shorten a term.

**The cap is `validator_churn_cap_seats`,** the maximum number of seats filled
at one boundary. Reasoning at both extremes, because the parameter is
meaningless without it:

- **Too low.** Genuinely failed validators are replaced slowly; a correlated
  failure — one hosting provider going down — leaves the set degraded for
  several epochs, and with the term floor still forcing exits the set shrinks
  toward the stall threshold. A cap below `ceil(V / T)` is worse than unhelpful:
  it makes term expiry unsatisfiable, so the chain stalls by construction rather
  than by accident.
- **Too high.** An adversary that wins eligibility in a single epoch flips the
  set in a single transition. With a cap of `c` seats the adversary needs at
  least `ceil((V / 3) / c)` boundaries to reach the BFT safety threshold **by
  admission** — the qualification is load-bearing and is explained in the
  contraction floor below, because seats can also be taken without admitting
  anyone. Every one of those boundaries publishes a set document whose
  composition drift any observer can compute. The cap does not prevent capture;
  it converts capture from an event into a process with a declared minimum
  duration and a public signal at every step. It is worth having only where
  someone is looking, which is why the drift is a light-client-computable
  quantity rather than an operator dashboard.

#### The contraction floor: a cap on entry is only half a rule

The cap bounds **admissions**. For a while nothing in this document bounded
**departures**, and that asymmetry was a capture path in its own right, reached
without breaking a single rule. It is written out here in full, because the
correction is only intelligible next to the attack it answers.

A coalition holding `k` seats with `k > V/3` — below the BFT safety threshold, so
by hypothesis unable to capture anything — has more than one third of the voting
power and can therefore withhold quorum from any block it dislikes. During the
candidacy window it finalizes its own `k` candidacies and then refuses to
finalize any block carrying somebody else's. After `candidacy_close_height(e)`
those candidacies are void by construction, since a candidacy must be finalized
strictly below that height. At the boundary the derivation, run honestly by every
full node, yields `R` equal to the coalition, `C` equal to the coalition,
`Nw` empty and `fills = 0`. **Under the cap and not at it.** The coalition is now
the entire set, holds all the voting power, and every check a light client
performs passes: no off-boundary change, uniform power, terms respected,
`seated_since_epoch` consistent, the entry cap satisfied with room to spare. The
coalition never admitted anybody. It simply outlasted everybody.

The rule that closes it is the quorum predicate applied to membership, and it
needs no new parameter:

> **Contraction floor.** For any validator set `S_new` replacing `S_old` —
> at an election boundary or through a removal-only revocation transition —
> `3 * member_count(S_new) > 2 * member_count(S_old)`. A set that does not
> satisfy it is invalid, and if no valid set exists the chain stalls at that
> height.

The shape is deliberate: it is the same strict `signed * 3 > total * 2` used for
every quorum in v0, applied to seats instead of power, so a reviewer reading it
recognizes the arithmetic and its boundary cases without new fixtures of its own.
Its effect on the attack above is exact. A coalition at `k` just above `V/3`
gives `3k` barely above `V`, which is not above `2V`, so the set it would produce
is **invalid** and the chain stalls instead of handing it everything. What the
coalition can obtain by censoring is therefore a halt — which anyone above one
third could already cause simply by not voting — and never a set of its own
choosing **at that boundary**.

**What the floor does not buy, stated before what it does.** An earlier version
of this paragraph concluded that the effective capture threshold of the network
was therefore two thirds rather than one third plus epsilon. That was wrong, and
the refutation was already three paragraphs further down in this same section:
residual (g) of
[what a light client can establish](#what-a-light-client-can-establish-about-set-composition)
says a lawful contraction and a capture by attrition are indistinguishable, which
is only worth saying if capture by attrition survives the floor. It does. Total
censorship is refused by the floor; **selective** censorship is not. A coalition
holding `k > V/3` lets through exactly the honest candidacies needed to land on
the smallest set the floor permits, and repeats:

```text
V  ->  2V/3  ->  4V/9  ->  ... ->  k        boundaries = ceil(log(V/k) / log(3/2))
```

For `k` near `V/3` that is **three boundaries**, not one and not never. Honest
nodes sign each of those blocks, because each is valid: the derivation is
deterministic, and the candidacies that were censored were never finalized, so
nothing distinguishes the block from an honest one. The effective capture
threshold of this network therefore remains **just above one third**, and above
two thirds a coalition satisfies the floor in a single step — which is not a
regression, since past two thirds the BFT safety assumption has already failed
and no set-composition rule can repair it.

**What the floor does buy is worth having and is claimed exactly.** It converts a
capture that took **one invisible boundary** into one that takes **three, each of
which publishes its own contraction in a signed document any light client can
diff**. That is the same standard the entry cap is held to — an event converted
into a process with a public signal at every step — and it is claimed here on the
same terms, neither more nor less.

**A stronger rule exists and is refused on its cost, which is not the same as
there being none.** The per-boundary floor has a cumulative sibling:

```text
3 * member_count(e)  >  2 * member_count(e - validator_min_capture_epochs)
```

It would tie the attrition horizon to `m` exactly as `3 * c * m <= V` ties the
admission horizon, and it is available: it never asks *why* a member left, any
more than the per-boundary floor does, so it does not need the censored-versus-
never-sent distinction that residual (h) declares impossible for every verifier;
and it is computed from `member_count` values a light client already retains.
What it costs is liveness, and the cost is paid by honest networks: a network
that legitimately shrinks by more than a third across `m` boundaries — a
withdrawal of operators, a run of correlated failures, a set deliberately being
reduced — stalls, and the wider `m` is chosen the more ordinary attrition it
forbids. v0 declines it for that reason and for that reason only.

The distinction matters beyond this rule. An earlier version of this paragraph
justified the refusal by asserting that no honest rule could be written here at
all. That was wrong, and it is the more damaging kind of wrong: a cost is a
judgement a later version can revisit when the trade changes, while an
impossibility is a proof that tells the next reader to stop looking. The
cumulative floor is on the table, it is sound, and it is not taken today.

**Declared asymmetry between the two paths.** The admission horizon is
**tunable**: `ceil((V/3)/c)` boundaries, and `validator_min_capture_epochs` lets a
network buy as many of them as it wants by lowering `c`. The attrition horizon is
**fixed**: `ceil(log(V/k)/log(3/2))` falls out of the floor's ratio and no
parameter moves it. A rule is only as strong as its weakest path, so the honest
figure to quote for this network is three boundaries, not the tuned one — and
anyone tempted to buy safety by lowering `c` should know they are buying it on
the path that was already the slower of the two.

Contraction is now the mirror of admission rather than its blind spot: entry is
capped at `c` seats per boundary, exit at one third of the set per boundary, and
both are computed by a light client from the two set documents it already holds.
A coalition that wants to shrink the network to itself needs
`ceil(log(V/k) / log(3/2))` boundaries to do it, every one of them publishing the
shrinkage in a signed document. Two consequences are declared rather than
discovered: a network that loses more than a third of its validators to genuine
correlated failure between two boundaries **stalls**, which is the same
safety-over-liveness trade taken everywhere else in this section and is in any
case the state such a network is already in, since a set missing more than a
third of its power cannot reach quorum either; and mass revocation cannot be
laundered into concentration **in a single transition**, because the removal-only
transition is subject to the same floor — over several transitions it can, on the
same terms and with the same publicity as any other contraction.

#### Magnitudes, not only relations: the bounds are fixed at genesis

Everything above constrains election parameters **against each other**. That is
not sufficient, and the reason is one this project has already met and solved
once: `identity.md` justifies making the Argon2id cost floor a validity rule by
observing that a governed parameter set could otherwise have removed the
memory-hard floor entirely while remaining fully conformant and leaving no
on-chain trace. The election parameters are governed by exactly the same
mechanism — a `consensus_parameters` document signed by a validator quorum, which
is to say by the sitting set — so the same reasoning applies and had to be
applied here too.

Concretely, and this was reachable under the relational constraints alone: a
sitting set publishes a document with `election_epoch_blocks` set to `2^60` and
`validator_max_consecutive_terms` set to `2^60`. Every relational constraint is
satisfied — `ceil(V / 2^60) = 1 <= c` holds, `3 * c * m <= V` is untouched,
`election_epoch_blocks > candidacy_close_blocks` holds comfortably. The document
is accepted. From that height the next election boundary never arrives, so the
boundary rule requires `next_validator_set_hash` to equal `validator_set_hash` at
every height for ever; no term ever expires; and the light client's checks not
only pass but actively *enforce* the freeze. The invariant would have been
switched off by a document the invariant itself does not govern.

v0 therefore anchors the magnitudes outside the chain's own governance, in the
`ElectionBounds` object of the genesis trust anchor
([README.md](README.md#election-bounds)). Those values ship with the signed
network distribution, are not discoverable from the network, and cannot be
changed by any on-chain document — changing them is a new distribution and a new
chain-level decision, exactly like rotating a trust key.

Those two ends are not independent, and the protocol makes their relation a
validity rule on the consensus-parameters document rather than advice, with the
same mechanism as the enrollment cost floor of
[README.md](README.md#the-enrollment-cost-floor-is-a-validity-rule-not-a-recommendation)
and the creator-share cap above. With `V = validator_target_set_size`,
`T = validator_max_consecutive_terms`, `c = validator_churn_cap_seats` and
`m = validator_min_capture_epochs` — the number of boundaries the network
declares an adversary must need in order to reach one third of the power — a
consensus-parameters document is accepted only if:

```text
0 < validator_min_set_size <= V <= validator_max_set_size
election_entropy_blocks >= 2
candidacy_close_blocks  > election_entropy_blocks
election_epoch_blocks   > candidacy_close_blocks
T >= 1  and  validator_cooldown_epochs >= 1
validator_cooldown_epochs <= T       // cooldown cannot outlast a full term
validator_eligibility_window_epochs >= 1
ceil(V / T) <= c                     // the term floor must be satisfiable
3 * c      <  V                      // the contraction floor must survive a
                                     // full cohort of expiries at one boundary
3 * c * m   <= V                     // capture must take at least m boundaries
storage_units_per_contribution_unit > 0
compute_units_per_contribution_unit > 0
validator_eligibility_min_issuers >= 2

// magnitude bounds, taken from the genesis ElectionBounds and never from the
// document under evaluation:
election_epoch_blocks <= election_epoch_blocks_max
T                     <= validator_max_consecutive_terms_max
validator_max_set_size<= validator_max_set_size_max
validator_min_set_size>= validator_min_set_size_min
m                     >= validator_min_capture_epochs_min

// rate of change, against the currently active document, for every election
// parameter x above, with num > den > 0 from the genesis ElectionBounds and
// checked u128 intermediates:
x_new * den <= x_old * num   and   x_old * den <= x_new * num

// minimum spacing, so that the rate limit is a limit per unit of chain and not
// merely per document:
activation_height(new) >= activation_height(active)
                          + election_parameter_min_activation_gap_blocks

// direction, for the one parameter whose reduction desynchronizes the stamps:
T_new >= T_active
```

Three of those bounds squeeze `c` from both sides and their joint satisfiability
is itself a constraint, which is why they are written together rather than
discovered together on a running chain:

```text
ceil(V / T) <= c < V / 3        requires   T >= 4
3 * c * m   <= V                requires   T >= 3 * m
                                so         T >= max(4, 3 * m)
```

Both couplings are real and neither is obvious. **The number of boundaries a
capture must take is bounded by the term limit**, so a network that wants capture
to take at least `m` boundaries cannot also want short terms; and **a term limit
of three or fewer is unsatisfiable at any set size**, because sustaining `V` seats
would then need a fill rate of at least `V/3` while the contraction floor needs
one strictly below `V/3`. A document that violates either is rejected on
acceptance, so the impossibility surfaces when the parameters are chosen and not
at the boundary where the chain would otherwise have stopped.

The `validator_cooldown_epochs <= T` bound belongs to the same family and is
there for a narrower reason. Cooldown attaches to every departure, so an
adversary that censors an honest node's candidacy for one epoch removes it for
`1 + validator_cooldown_epochs` epochs: the censorship lever is multiplied by the
cooldown, and cooldown is the one election quantity whose **increase helps an
adversary**. Every other magnitude in the block is bounded above because a large
value is dangerous; this one is bounded above for the same reason, and `T` is the
natural ceiling because a member barred for longer than a full term is
effectively barred.

The rate-of-change rule matters as much as the ceilings. Without it a set could
walk a parameter to its ceiling in one step at the moment it needed to, which is
the same manoeuvre performed more slowly; with it, any move toward the edge is a
sequence of signed documents, each of them public and each of them a signal. It
is the parameter-space analogue of the churn cap, and it exists for the identical
reason: to convert an event into a process that somebody can watch.

The spacing rule is what makes that reason true rather than merely intended. A
ratio applied **per document** bounds nothing in time: `sequence` is only required
to increase, so a quorum can publish as many documents as it likes in as many
consecutive blocks, and a parameter reaches its genesis ceiling in as many blocks
as the ratio needs steps. The absolute ceiling still holds — that is what makes
this a bound on observability rather than on magnitude — but a process nobody has
time to observe is an event with extra paperwork. Requiring
`election_parameter_min_activation_gap_blocks` between the activation heights of
consecutive election-parameter changes prices the walk in chain time, which is
the quantity an observer actually has.

Every symbol in this section is a governance parameter whose **value** comes from
the economic simulator of M-02 and is deliberately not fixed here, with the
exception of the `ElectionBounds` magnitudes, whose values are a genesis decision
of the network operator and not a simulator output. The constraints among them
are not parameters and are fixed now.

**Declared limit of the bounds themselves.** `ElectionBounds` is configuration
carried by the signed distribution, so a client running an outdated distribution
enforces the bounds that distribution carries. If a network later ships wider
bounds, such a client rejects chains the network considers valid and fails
closed, reporting that it needs a newer distribution; if it ships narrower ones,
the older client is more permissive than the network, but the network will not
produce the sets it would have wrongly accepted. Neither direction lets an
attacker widen the bounds a given client enforces, which is the property that
matters; what it does mean is that the bounds are only as trustworthy as the
release channel, which is the same footing as the trust key and is stated in the
same terms in [README.md](README.md#the-network-release-trust-key).

### Degenerate cases, and what the protocol does instead of improvising

**Fewer eligible candidates than seats.** `fills` is a minimum over three
quantities, one of which is `|Nw|`, so a short pool simply produces a smaller
set. Nothing is relaxed to fill it: not the threshold, not the term limit, not
the cooldown. The set shrinks, and if it shrinks below `validator_min_set_size`
**or past the contraction floor** the chain stalls at the boundary.

**Many members leaving at once, and the cooldown they all enter.** Because
eligibility condition 5 attaches cooldown to *any* departure, a boundary at which
many members are not retained puts all of them out of the pool for
`validator_cooldown_epochs`. Combined with the contraction floor this is the
sharpest liveness edge in the section: a network cannot shed more than a third of
its set at a boundary, and the members it sheds cannot come straight back to
repair the shortfall. The trade is taken deliberately and in the same direction
as everywhere else — the alternative, waiving cooldown when the pool is short, is
attacker-triggerable in exactly the way the emergency-continuation clause below
is, since a coalition that can censor candidacies can manufacture the shortage
that waives the rule. `validator_cooldown_epochs` and the eligibility threshold
must be chosen together and against a simulated pool size, which is M-02 work.

**Stalling is a choice, and the alternative was considered.** The obvious
alternative — let the previous set continue for one more epoch when no lawful
successor exists — is rejected because it is *attacker-triggerable*: a quorum
able to censor candidacy transactions can manufacture the emptiness that
authorizes its own continuation, and the rule meant for emergencies becomes the
self-perpetuation path in a better disguise. v0 makes the same trade the
revocation rule makes, for the same reason: safety over liveness, with recovery
out of band through an authenticated release rather than through a mechanism a
quorum can turn against the network.

**Ties in the derivation.** Resolved by `account_key` ascending after the ticket,
giving a total order; see step 5.

**An epoch with no valid randomness.** It cannot occur, and that is by
construction rather than by fallback. The window
`[e*L - election_entropy_blocks, e*L - 1]` lies entirely below the boundary, and
`election_epoch_blocks > candidacy_close_blocks > election_entropy_blocks >= 2`
places every one of those heights above genesis and below the boundary, so all
of them are finalized before the boundary is reached and epoch 1 is the earliest
election. **No rule may make a missing or unusable seed suspend an election.** A
rule of that shape would hand the outgoing set a way to skip rotation by
damaging its own beacon, which is the defect of this section restored through the
exception clause.

**Revocation between two boundaries.** A revocation-forced transition removes
members and cannot admit any, per
[revocation forces a validator set transition](#revocation-forces-a-validator-set-transition).
At the next boundary the derivation runs with the interim set as `P`, so the
vacancies revocation created are refilled **under the ordinary cap**, not in one
step. A quorum therefore cannot revoke its way to a large lawful admission. The
cost of that choice is explicit: a network that revokes many validators at once
recovers its target size over several epochs, and if the interim set falls below
`validator_min_set_size` it stalls immediately rather than at the boundary.

**Revocation of a node in cooldown or in the candidate set.** A revoked identity
fails eligibility condition 1 permanently. A candidacy finalized before the
revocation does not survive it.

### The [ADR-008] test, answered

[ADR-008] requires every specification introducing a form of rewarded or
remunerated work to declare the outcome of its three-part test. The declaration
for this section:

1. **Limit — passed, and it is the reason the rule is a threshold.** There is a
   ceiling set by real network need, and above it spending more earns nothing.
   Eligibility is a binary predicate; every eligible node contributes exactly one
   ticket; every seated member has voting power 1. The evidence the score reads
   is bounded by demand at its source — storage challenges exist only for objects
   someone stored, compute challenges only for tasks someone invoked — so the
   total contribution the network can absorb is set by its own need, and the
   number of nodes that can clear the threshold is bounded by that need divided
   by the threshold. A ranking by contributed work, the obvious alternative,
   fails this point and was rejected for that reason.
   **Residual, declared rather than argued away:** numerosity still converts into
   draws, because `N` distinct identities each clearing the threshold hold `N`
   tickets. Each of them must supply real, demand-bounded, separately verified
   work, and the churn cap bounds how fast tickets become seats — but this is the
   same residual [ADR-007] names as governed by `alpha`, and it is not closed
   here.
2. **Waste — passed.** The work the score reads is proof of retrievability over
   stored objects and re-execution of WASM tasks. If it stopped being performed,
   user-visible services degrade directly: hosted apps stop running and stored
   data stops being retrievable. The election adds no work of its own beyond one
   `validator_candidacy` transaction per candidate per epoch.
3. **Battery — passed, with a consequence that must be stated.** This section
   introduces **no new sampled work at all**: it consumes challenge evidence that
   [ADR-002] already produces under the wide-response-window discipline of
   `SEC-REQ-17`, so the mobile admissibility question is answered where that
   evidence is defined and is not reopened here. The consequence to state is a
   different one: anchoring eligibility to storage and compute means a node
   offering only availability — most phones — is not a validator candidate. That
   is an intended effect of [ADR-007] rather than a side effect, running a
   validator on a phone is a poor idea independently, and phones remain full
   participants in existence income, in work compensation for storage and compute
   they do provide, and in light-client verification. It is written here because a
   rule that quietly excludes the project's characteristic device would otherwise
   be discovered by its users instead of declared by its authors.
### What a light client can establish about set composition

This document has already had to correct one overstated safety claim, in
[revocation forces a validator set transition](#revocation-forces-a-validator-set-transition),
and the correction stands as its standard: a wrong safety statement is worse than
a missing one. The perimeter below is therefore given as two closed lists.

A light client holds a validated checkpoint, the headers it has walked, the
`ElectionBounds` of its signed distribution, the active `consensus_parameters`
document authenticated against the `consensus_parameters_hash` of a header it
already trusts, and the full `ValidatorSet` document for every set it accepted —
it fetches and hashes each one already. It sees **no transactions**. Naming the
parameter source is not a formality: every check below compares against
`election_epoch_blocks`, `validator_max_consecutive_terms`,
`validator_churn_cap_seats` and the size bounds, and a client that took those
values from an unauthenticated source, or fell back to defaults when the document
was unavailable, would be enforcing the attacker's numbers. On that data alone it
MUST check, and can establish:

1. that the set changed only where it was permitted to: at every non-boundary,
   non-revocation height, `next_validator_set_hash` equals `validator_set_hash`;
2. that every elected set activates exactly at
   `election_epoch * election_epoch_blocks` and carries an `election` record
   whose `election_epoch` agrees, and whose `previous_validator_set_hash` equals
   the hash of the set it is replacing;
3. that every member has `voting_power` 1 and `validator_id` equal to `node_id`,
   and that `member_count` lies within
   `[validator_min_set_size, validator_max_set_size]`;
4. **the term limit**, from a single set document:
   `election_epoch < term_expiry_epoch` for every entry. No member serves beyond
   its term on any chain the client accepts, and because the stamp is carried
   rather than recomputed, no later parameter change can extend a term already
   running;
5. **`seated_since_epoch` and `term_expiry_epoch` consistency** across two
   adjacent sets: a member present in both keeps both values unchanged; a member
   present only in the newer set has `seated_since_epoch` exactly `election_epoch`
   and `term_expiry_epoch` exactly
   `election_epoch + validator_max_consecutive_terms`;
6. **the rotation cap**, that `filled_count <= validator_churn_cap_seats`, and
   that `retained_count`, `filled_count` and `member_count` agree with the array
   they describe;
7. that the committed `election_seed` is the correct hash of the committed
   entropy block IDs **and of nothing else** — `candidate_root` and
   `candidate_count` are committed by the `election` record but are deliberately
   not seed inputs, per
   [the seed](#the-seed-and-why-the-rule-does-not-depend-on-it) — and, for entropy
   heights it walked itself, that those block IDs are the ones it saw;
8. the composition **drift** of the set at every boundary, since it holds both
   sets in full;
9. **the contraction floor**, that `3 * member_count(new) > 2 * member_count(old)`
   across every transition it accepts, boundary or removal-only. This is the
   check that bounds capture by attrition, and like the term limit it is pure
   arithmetic over two documents it holds;
10. that the election parameters it is using are **within the genesis
    `ElectionBounds`** and come from a `consensus_parameters` document whose hash
    matches the header. A sitting quorum cannot switch the invariant off
    underneath the client by publishing a document with an unreachable epoch
    length or an unreachable term limit: such a document is invalid to full nodes
    and out of bounds to the client independently;
11. given a candidate-membership Merkle proof against `candidate_root`, that a
    given seated member was in the committed candidate set. Serving those proofs
    is M-02 work, exactly as per-epoch existence-income eligibility proofs already
    are.

It **cannot** establish, and no combination of the above amounts to it:

- **(a)** that `candidate_root` contains every node that was genuinely eligible.
  A quorum that censors candidacy transactions, or that omits eligible leaves,
  produces a record a light client cannot tell apart from an honest one;
- **(b)** that every committed candidate actually met the contribution threshold.
  The score is a function of transactions the client never sees;
- **(c)** that the fills are the lowest-ticket members of the pool. It can
  recompute the ticket of a member it was shown, but establishing that no omitted
  candidate had a lower ticket requires the whole leaf set;
- **(d)** that the seed was not ground, which no verifier of any kind can
  establish, because a ground beacon is a legal beacon;
- **(e)** cooldown, beyond the boundaries it observed itself. A client that joined
  at a recent checkpoint does not know who retired before it;
- **(f)** as already declared for revocation, that an off-boundary transition was
  *due*. It sees a removal-only transition and checks that it removes only; it
  cannot see the `revoke_identity` that authorizes it, and relies on its
  checkpoint's `revoked_validators` for the part that is covered;
- **(g)** that a **lawful contraction is not a capture by attrition**. The
  contraction floor bounds how far a set may shrink at one transition, and the
  client enforces that bound; it cannot tell a network genuinely losing
  validators from a coalition that is **selectively** censoring — letting through
  exactly the candidacies that land the set on the smallest size the floor
  permits, and repeating. Both produce a smaller set within the floor, signed by
  a valid quorum, and neither looks different from the other. Total censorship,
  where the coalition withholds quorum from *every* block carrying somebody
  else's candidacy, is the variant the floor **does** refuse, since it produces a
  set below the floor and therefore no valid set at all; it is named here only to
  say that it is not the vector. The vector is the selective one, it reaches the
  coalition in `ceil(log(V/k) / log(3/2))` boundaries, and what the floor buys
  against it is that those boundaries are several and each is published — not
  that any of them is distinguishable from honest attrition. See
  [what the floor does not buy](#the-contraction-floor-a-cap-on-entry-is-only-half-a-rule);
- **(h)** that no candidacy was **excluded by never being finalized**. This is
  the one composition failure that is invisible to *every* verifier, full node
  included, because a censored candidacy and a candidacy never submitted are the
  same absence of a transaction. It is listed separately from (a) precisely
  because (a) is compactly falsifiable and this is not falsifiable at all; see
  [the second lever](#the-second-lever-the-pool-itself-and-what-is-honestly-claimable-about-it).

**How much of that residual is falsifiable, which is not the same as
verifiable.** Any node replaying finalized transactions verifies (a), (b) and (c)
completely — (g) and (h) it cannot, and neither can anyone else, which is why
they are stated as limits of the protocol rather than as limits of the light
client. Of the three a replaying node does verify, two are contradictable by a
**compact** proof that needs no replay:

- **(a)** is falsifiable by the omitted node itself. The candidate tree is sorted
  by `account_key`, so non-membership is proved by the two adjacent leaves plus
  the omitted node's own finalized candidacy;
- **(c)** is falsifiable by anyone holding the leaf set: a Merkle proof of a
  candidate with a lower ticket that was not seated contradicts the record in a
  few hundred bytes;
- **(b)** is **not** compactly falsifiable, because it asserts the *absence* of
  qualifying evidence and absence has no short proof. Contradicting it requires
  replaying the eligibility window. The asymmetry is declared rather than smoothed
  over: of the three composition failures, two can be shouted down with a single
  message and one needs a node that keeps history.

**The claim this section is entitled to make**, deliberately narrower than "a
light client verifies the election", and stated in full because a shorter version
of it was wrong twice:

> Within the election parameter limits fixed at genesis, a light client
> establishes that the active set is **of lawful shape and in lawful rotation** —
> bounded terms, bounded entry, floored contraction, no off-schedule change, no
> member seated beyond its term — and does not establish that it is the set the
> eligibility rule should have produced. Of the three ways of composing the set
> wrongly that a replaying node can detect, two are contradictable with a short
> message and one requires a node that keeps history; two further ways —
> contraction indistinguishable from attrition, and exclusion by non-finalization
> — are detectable by nobody, and are bounded rather than observed. The bound on
> capture is a **number of published boundaries**, not a share of voting power:
> the effective threshold remains just above one third, and what the rules buy is
> that reaching it takes several transitions, each of which the client can see.

The first sentence closes [DEBT-005]; the rest is what remains. Two earlier
versions of this paragraph promised more: one omitted "within the parameter
limits fixed at genesis", at a time when no such limits existed and the property
could therefore be switched off by a document the sitting quorum signs; the other
spoke only about who **enters** the set, at a time when nothing bounded who
**leaves**. A third claimed that closing the second gap moved the effective
capture threshold to two thirds; selective censorship refutes that in three
boundaries, and the claim above is the corrected one, which promises observable
delay rather than a raised threshold. The wording keeps every qualification
visible, because the property is exactly as strong as the bounds and the floor,
and a reader is entitled to know where to look. The honest summary is unchanged in kind: this section moves the
light client from checking that a transition was *authenticated* to checking that
it was *lawful*, without promising that it was *correct*.

One clause belongs beside that claim rather than inside it, because it describes
the ground the claim stands on rather than the claim itself: **the magnitudes
that hold the property up are fixed at genesis, and the ones that move, move
under a ratio, under a spacing measured in chain height, and — for the term limit
— in one direction only.** Every part of that sentence is a rule in
[rotation: the cap and the floor](#rotation-the-cap-and-the-floor), and together
they are what stops the property being switched off by the same quorum it
constrains.

### Worked example of the derivation

The example is normative in form and not in values: every parameter below is
instantiated only to make the derivation reproducible, and none of these numbers
is a proposal. `chain_id` is 32 zero bytes, as in the `HASH-0` fixture of
[README.md](README.md#hash-conformance-fixtures). Account keys are written as a
byte repeated 32 times so that a reviewer can build every preimage by hand; real
account keys are hashes.

Example parameters: `election_epoch_blocks` 100, `election_entropy_blocks` 3,
`candidacy_close_blocks` 10, `validator_target_set_size` 8,
`validator_min_set_size` 3, `validator_churn_cap_seats` 2,
`validator_max_consecutive_terms` 4, `validator_cooldown_epochs` 1,
`validator_min_capture_epochs` 1. They satisfy the constraint block, which is part
of what the example is for: `ceil(8/4) = 2 <= 2`, `3*2 = 6 < 8`, `3*2*1 = 6 <= 8`,
`4 >= max(4, 3)`, `1 <= 4`. The epoch under election is `e = 3`, so the boundary
is height 300, candidacy closed at height 290, and the entropy window is heights
297, 298 and 299.

The previous set `P`, active at height 299, has four members. Their
`term_expiry_epoch` values are **staggered** rather than shared, which is the
genesis stagger rule propagating itself: `01` is a genesis member whose stamp lies
in `[1, 4]`, and every later stamp is the boundary that seated the entry plus `T`.

| account key | `seated_since_epoch` | `term_expiry_epoch` | outcome at `e = 3` |
| --- | --- | --- | --- |
| `01`×32 | 0 | 3 | **term expired** (3 is not below 3): removed, cooldown starts |
| `02`×32 | 2 | 6 | retained |
| `03`×32 | 2 | 6 | filed no candidacy for epoch 3: **voluntary exit** |
| `04`×32 | 1 | 5 | retained |

So `R = {02, 04}` and `|R| = 2`. Three further nodes filed valid candidacies and
clear the threshold: `05`, `06` and `08`. Node `07` filed a candidacy but its
`contribution_score` is below `validator_eligibility_threshold_units`; node `01`
is in cooldown; node `03` filed nothing. The eligible set and the fill pool are
therefore

```text
C  = { 02, 04, 05, 06, 08 }        candidate_count = 5
Nw = C \ R = { 05, 06, 08 }
```

Leaves, `candidate_leaf = H(0x40 || u64be(3) || account_key_32)`:

```text
02  cd3950bac9c60b73523a0f8157e5da8384515d2e9c4ef4127be2d910b878158c
04  004e3b02570032774d181a20dbb4d5e23f6fe83092374635189f42c513db97d9
05  154a73c742c2f580aafaf97401ef5633898862d761fc13ed230db7817b0111fd
06  9ecce0184a6b8d6d3a1257add0af3a16cc64809fe2cf723cfe91b570abe07f44
08  c9e67f59e0d4a6c08cc588bb906b17c8bddba657def06d6c2f111c64420796ca
candidate_empty = H(0x42) =
    df7e70e5021544f4834bbee64a9e3789febc4be81470df629cad6ddb03320a5c
```

Leaves stay in `account_key` order and pad to eight with `candidate_empty`.
Internal nodes are `H(0x41 || left || right)`:

```text
level 1
  H(leaf02, leaf04) = 00d36c1eb2fc336cb635735bbae46cf3e2a32322f4761db3d29a297644e45bb2
  H(leaf05, leaf06) = db1bd3372cea52438df608862c3dcb3c0a2b1c16b152c296c2a8effe9005bc20
  H(leaf08, empty)  = b65c2a34ededd92d98721c11a620cc6f16e87a03f512e54fd35b35cd70c3bf39
  H(empty,  empty)  = a7ee32ed571f99897d698653a14d292cfea8e8633f95a9db217ea993c7773e91
level 2
  H(n1_0, n1_1)     = a2bb9ac490112abad3c2af7197fdf6efe62af01b7c251f8d3443b20fa145a060
  H(n1_2, n1_3)     = 5d426fa91db2b6acb77c0980837dea9e54380afc25eede062e05e2cab2dc6c2b
candidate_root      = 42e4f6b1f01af3b69ba154aa464738829635a3ed7facf65e652d9712b461924a
```

The entropy window holds block IDs `aa`×32 at height 297, `bb`×32 at 298 and
`cc`×32 at 299:

```text
election_entropy = H("coblox-election-entropy-v0\0" || 00×32 || u64be(3) || u64be(3)
                     || aa×32 || bb×32 || cc×32)
                 = 29a63c10926e75ed418c6f3c7670b0edfeb0b021868d1342b788327f53767e42
election_seed    = H("coblox-election-seed-v0\0" || 00×32 || u64be(3)
                     || election_entropy)
                 = 9e2aa2621f957279e4bdf1c4ccea5629ce429892ac3f9af10c9456a3c78dad85
```

`candidate_root` does not enter the seed; it is bound by validity, as
[the derivation](#the-derivation) explains. The example still computes it,
because `ElectionRecord` carries it and a verifier recomputes it.

Tickets, `H("coblox-election-ticket-v0\0" || 00×32 || election_seed || account_key_32)`:

```text
05  a10e8ec4a79c2defa40f869c68c2a1570bbb4a5b12b597a28d94294bf7582f21
06  547132161f56f2361faf3caae90224e1b5c04ae1986ab871703b7003693c5fdd
08  9d04ef2f66f5403d275fa1f80819ce40a17af81931d263d3c390e0291e0b19c9
```

Ascending by ticket: `06` (`5471…`), `08` (`9d04…`), `05` (`a10e…`). The pool
holds three nodes and the target would admit three, but

```text
fills = min( max(0, 8 - 2), 2, 3 ) = 2
```

so the cap binds and `05` is not seated this epoch — the case worth exercising,
because a cap that never binds is no evidence that it works. The elected set,
sorted by `validator_id` (equal to `node_id`, shown here by account key):

| account key | `seated_since_epoch` | `term_expiry_epoch` | `voting_power` | how |
| --- | --- | --- | --- | --- |
| `02`×32 | 2 | 6 | 1 | retained, stamp carried unchanged |
| `04`×32 | 1 | 5 | 1 | retained, stamp carried unchanged |
| `06`×32 | 3 | 7 | 1 | filled, ticket rank 1, stamped `3 + 4` |
| `08`×32 | 3 | 7 | 1 | filled, ticket rank 2, stamped `3 + 4` |

`member_count` 4, `retained_count` 2, `filled_count` 2, which is at the cap. Two
size conditions are then checked and both hold: four is at or above
`validator_min_set_size` 3, and the contraction floor `3 * 4 > 2 * 4` holds,
since the previous set also had four members. Had the two newcomers been censored
out of the candidate window, the set would have been `R` alone — two members —
and `3 * 2 > 2 * 4` is false, so **that set would have been invalid and the chain
would have stalled** instead of leaving the two survivors in sole possession.
That is the contraction floor doing the work it exists for, on the same example.

The set being valid, `validator_set_hash` is the existing formula over its JCS.
A reviewer redoing this needs SHA-256 for five leaves, one empty leaf, six
internal nodes, two domain-separated hashes and three tickets — every preimage is
given above — and nothing else: the retention table, the ordering, the `fills`
minimum, the two size conditions and the assembly are integer comparisons.

The two new entries share `term_expiry_epoch` 7, which equals
`validator_churn_cap_seats` and is therefore admissible: at most `c` seats are
stamped at any boundary, so at most `c` expire at any later one. That is the
invariant the genesis stagger rule starts and the entry cap maintains, and it is
what keeps the contraction floor satisfiable for ever after — at boundary 7 two
of eight seats retire, and `3 * 6 > 2 * 8` holds with room.


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

1. **Validate the external checkpoint.** Load a `WeakSubjectivityCheckpoint`
   exactly as specified in [README.md](README.md#weak-subjectivity-checkpoint):
   verify its signature under a trust key the client already holds, require its
   `chain_id` to equal the configured chain ID, and require
   `now - issued_at_ms` to be at most the `max_weak_subjectivity_age_ms` carried
   **in the checkpoint itself**. Genesis and the following steps alone are not
   sufficient after that window. Missing, stale, chain-mismatched, or
   unknown-key checkpoints fail closed. Retain `revoked_validators` for step 4.
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
   **Apply the checkpoint's revocations**: for every `(node_id,
   effective_height)` in `revoked_validators`, reject any set whose
   `activation_height >= effective_height` that contains that `node_id`, and
   reject any header at height `>= effective_height` whose active set contains
   it — including the set inherited from the checkpoint. Without this the client
   would follow a chain signed by keys the network has already revoked; see
   [revocation forces a validator set transition](#revocation-forces-a-validator-set-transition).
5. **Obtain and authenticate the election parameters, then check that the set is
   lawfully shaped and lawfully rotating.** The checks below compare against
   election parameters, so the client MUST first establish where those values
   come from, in this order and with no fallback at any step: load
   `ElectionBounds` from the configured network distribution, exactly as it loads
   the genesis configuration at step 2 — it is a trust anchor and MUST NOT be
   learned from a peer, a header, or a chain document; fetch the active
   `consensus_parameters` `SignedProtocolDocument`, recompute
   `consensus_parameters_hash` over it and require the result to equal the
   `consensus_parameters_hash` of the trusted header being verified; verify its
   quorum signatures against the currently trusted validator set with the strict
   quorum predicate; and require every election parameter it carries to lie
   within `ElectionBounds`. **A missing, unverifiable, hash-mismatched, or
   out-of-bounds parameter document fails closed**: the client rejects the header
   and reports that it cannot verify set composition. It MUST NOT proceed with
   defaults, with values from an earlier document, or with values supplied by a
   peer — an implementation that fills the gap from whatever the chain currently
   says is enforcing the attacker's numbers and is the reason this step names its
   sources rather than assuming them.

   Then, for every header, require `next_validator_set_hash` to equal
   `validator_set_hash` unless the next height is an election boundary or the
   transition is removal-only; and for every accepted set apply checks 1 to 10 of
   [what a light client can establish](#what-a-light-client-can-establish-about-set-composition):
   activation height, `election_epoch`, `previous_validator_set_hash`, uniform
   voting power, `validator_id` equal to `node_id`, size bounds, the term limit,
   `seated_since_epoch` consistency with the previous set, the rotation cap, the
   contraction floor, the internal agreement of the three counts, and the seed
   derivation from the committed entropy IDs. Reject the set on any failure. This
   step establishes that the set rotates lawfully; it does **not** establish that
   its composition is the one the eligibility rule should have produced, and the
   boundary between the two is stated in that section.
6. **Corroborate freshness.** Query independently operated enrolled peers,
   reject tips older than `max_current_balance_age_ms`, and require the selected
   finalized height to be consistent with the recent checkpoint. Peer agreement
   is an availability/fork alarm, never a substitute for proof verification.
7. **Select final state.** Require the proof response header height to equal the
   requested height exactly, never below persisted trust, and retain its `state_root`.
8. **Bind the account.** Recompute `account_key` from the requested account kind
   and subject ID and
   compare all 32 bytes. Reject malformed bitmap or sibling count.
9. **Create the leaf.** If `present` is true, compute the type-specific leaf. If
   false, require balance and nonce both zero, app-only fields absent, and use
   `empty[256]`.
10. **Rebuild and decide.** Iterate depths 255 down to 0. Obtain that depth's sibling
   from the proof (or the corresponding default). If the key bit is 0 hash
   `branch(current, sibling)`; if 1 hash `branch(sibling, current)`.
   Compare the final 32-byte value to `state_root` in constant time. Only on
   equality display the balance, lifecycle if applicable, and finalized height.

TLS, a signed peer envelope, or a proof from several servers cannot replace any
step above. Clients SHOULD query independent peers for availability and fork
alerts, but cryptographic acceptance depends on the authenticated header.

## State transition order

Within a block, transactions execute in this deterministic order after all
static checks: (0) `challenge_commitment`, `challenge_evidence`,
`revoke_identity`, and `validator_candidacy`, ordered by raw
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

## DRAFT: economic values

The committee-selection question that stood here is closed: the election rule,
its eligibility basis, its randomness source, its rotation cap and its
commitment are specified in
[validator election and rotation](#validator-election-and-rotation), and the
constraints among their parameters are validity rules rather than guidance. What
remains open is the numeric value of those parameters, which belongs with the
economic values below.

One matter is intentionally open but fully bounded:

- reward and price values, including the publisher-reward curve and the
  per-epoch existence fund, come from the economic simulator, either as fixed
  epoch tables or bounded governance curves. AGENT-002 and the Project Lead own
  the decision under ADR-005, ADR-006, and ADR-007. The curve is free only
  within the creator-share cap above, which is a validity rule and not a
  tuning parameter.

The open values do not change transaction kinds, mint/burn separation, signed
policy hashes, validator-set continuity, the revocation transition rule, the
election derivation, or the light-client proof algorithm. The election
parameters additionally cannot be chosen independently of one another: the
consensus-parameters document is rejected on acceptance unless it satisfies the
constraint block of
[rotation: the cap and the floor](#rotation-the-cap-and-the-floor).
