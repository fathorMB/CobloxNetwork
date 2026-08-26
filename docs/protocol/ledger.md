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

### What `enrolled, unrevoked` means, and as of which height

Four authorization structures of this document are satisfied by a single key
that MUST derive an **enrolled, unrevoked** node ID. Until this section those
words carried no definition, and this document was using two readings of them:
one anchored to a revocation that is *finalized*, one anchored to a revocation
that is *effective as of* a stated height. Between the two lies exactly the
interval that
[revocation forces a validator set transition](#revocation-forces-a-validator-set-transition)
declares it keeps **long** on purpose. A qualification with two readings that
disagree across a deliberately long interval is not a rule with a gap in it: it
is two conformant implementations returning opposite verdicts on the validity of
a block, which is a partition of the chain and costs an attacker nothing but one
transaction.

**Definition.** For a transaction included in the block at height `h`, a
`node_id` is **enrolled, unrevoked** when both of the following hold against the
finalized state that block builds on:

1. a finalized enrollment certificate names `node_id` and its
   `valid_from_height` is at most `h`; and
2. no finalized `revoke_identity` naming `node_id` carries an `effective_height`
   at most `h`.

Both clauses are facts about the block being validated and its ancestors.
Neither is a fact about the verifier: not the height its own view has reached,
not which quorum certificates it happens to hold, not wall-clock time.

**What this definition reaches, stated as a scope and not as a spelling.** It
governs the qualification wherever it authorizes a **transaction**, in any of
the wordings the v0 documents use for it — *enrolled, unrevoked*, *enrolled and
unrevoked*, *finalized, unrevoked* — and `h` is always the height of the block
including that transaction. That includes the publisher key rule of
[app-manifest.md](app-manifest.md#manifest-schema), where `h` is the height of
the block finalizing the catalog record carrying the manifest. Naming the
wordings rather than one spelling is deliberate: the words differ from rule to
rule, and a scope declared as a literal string would have covered the phrasing
this section happens to use and missed the others.

**What it does not reach, and this is a limitation rather than an exclusion.**
[Validator-set continuity](#validator-set-continuity) requires the members of a
`ValidatorSet` to be *enrolled and unrevoked* and states no height. A
`ValidatorSet` is not a transaction, so the anchor above does not apply to it,
and this section does not silently supply one: `activation_height` is the
plausible anchor and choosing it is a decision about set continuity, not about
what the qualification means. The four quorum-authorized rules below reach the
qualification through that rule, and so do the finality vote and the auditor
signatures of challenge evidence, so the gap is inherited rather than introduced
here — and it is written down instead of being left to look closed.

**Why this reading and not the finalized one.** The obstacle to the finalized
reading is structural rather than a matter of difficulty. Finality is carried by
a `QuorumCertificate` over a block, and **no block carries one**: a
`BlockHeader` commits `previous_block_id`, `state_root` and `transactions_root`,
and nothing that records when any earlier block became final. The chain
therefore contains no height at which a revocation *became finalized*. A
verifier replaying the chain can establish that a `revoke_identity` was
**included** below `h`; that it was **final** below `h` it can establish only
from certificates it holds outside the chain, and two verifiers holding
different certificates then disagree about the same block. A validity rule whose
verdict depends on which certificates a node happens to have collected is not a
strict rule with a wide margin — it is a fork with a specification.

`effective_height` has the opposite shape. It is committed in the body of the
`revoke_identity` transaction itself, it MUST be later than the block proposing
the revocation, and every verifier therefore reads it from the same bytes.
Anchoring the qualification to it makes the predicate a total function of the
block and its ancestors, monotone in `h`, and identical for every verifier at
every later head. That is the property
[eligibility](#eligibility-demonstrated-storage-and-compute-never-availability)
already states of its own conditions — *"each of them a fact a replaying
verifier settles without judgement"* — it is the reading the two
height-anchored rules of this document already use, and it is the height at
which
[revocation and key replacement](identity.md#revocation-and-key-replacement)
already stops revocation from reopening signatures behind it. Choosing the other
reading would give this protocol two meanings of *revoked* at the same height:
one for spending and one for validating.

**The cost of this reading, declared, and it is larger than one interval.**
Between the finalization of a `revoke_identity` and its `effective_height` the
key still authorizes every transaction in the table below, a subscription burn
against the node balance included. That interval is at least
`min_revocation_effective_delay_blocks` blocks, which this protocol keeps long
deliberately, **and it has no upper bound at all**. Nothing in v0 caps
`effective_height`: the body carries a `u64`, [identity
revocation](#identity-revocation) requires only that it be later than the block
proposing the revocation, and rule 4 of
[revocation forces a validator set transition](#revocation-forces-a-validator-set-transition)
adds the floor. A `revoke_identity` naming an absurdly distant
`effective_height` satisfies every MUST of this document, is finalized, appears
in the `revoked_validators` of a checkpoint, and never bites. **Until that field
is bounded, how much a revocation protects a balance is chosen by the quorum
that revokes**, and this section is where that has to be said, because the
definition above is what gave the field that reach: before it, `effective_height`
governed the validator set transition and nothing else.

None of that is an argument for the other reading, which does not shorten the
exposure but makes it verifier-dependent, and a verifier-dependent window is
worse than a declared one. **But the trade is not "a declared window against a
fork", and stating it that way would be too flattering.** A third reading —
*no `revoke_identity` naming `node_id` is included at a height at or below `h`*
— has every property the argument above invokes: a fact about the block and its
ancestors, monotone in `h`, read from the same bytes by every verifier. It also
closes the window. It is not adopted because it contradicts
[revocation and key replacement](identity.md#revocation-and-key-replacement),
which keeps signatures valid below `effective_height`: adopting it means
redefining what `effective_height` is, which is revocation mechanics. **The
trade is therefore a declared window against redefining `effective_height`**,
and the second term is work nobody has done rather than a thing that cannot be
done. How short the window should be, whether the floor should depend on
`reason`, and what bounds `effective_height` from above are all questions about
revocation mechanics and not about what the qualification means.

**One rule this definition does not govern.**
[Authentication on a connection](identity.md#authentication-on-a-connection)
requires a receiver to reject a peer when a revocation exists *at the receiver's
own finalized height*, and to re-evaluate the connections it already holds
whenever its finalized revocation set changes. That is a receiver-local
acceptance rule about **reachability**, not a validity rule about a block: no
block is accepted or rejected by it, nothing replays it, and it is free to be
anchored to the receiver's own view for that reason. It is named here because
its wording is the second reading, and a reader who met it first would take it
for the definition.

**But reachability is not nothing, and inside the window the two rules diverge
observably.** A challenge travels on a protected stream, and a protected stream
requires the check above; so within the interval, two auditors at different
finalized heights reach opposite conclusions about the same subject — one must
close the connection and records `no_response`, the other is answered and
records `passed`. That outcome enters a quorum-signed `challenge_evidence`, and
from there it reaches `contribution_score`, eligibility and
`work_compensation`. The receiver-local reading is therefore not confined to
each receiver: it reaches the chain through the contents of an object rather
than through the validity of a block. A node inside the window is at once
authorized to spend under the definition above, counted at full voting power
until `effective_height`, and unreachable by every conformant peer. **This
paragraph says which rule governs what; it does not say that the combination is
settled.**

**The rules this definition governs.**

| Authorization | Key MUST derive | Qualified |
| --- | --- | --- |
| `FundAppAuthorization` | `payer_node_id` | yes |
| `SubscriptionBurnAuthorization` | `payer_node_id` | yes |
| `ChallengeCommitmentAuthorization` | `issuer_node_id` | yes |
| `ValidatorCandidacyAuthorization` | `node_id` | yes |

The remaining four — `MintAuthorization`, `HostingBurnAuthorization`,
`ChallengeEvidenceAuthorization` and `RevokeIdentityAuthorization` — are
satisfied by a validator quorum certificate instead of a single key, and
the qualification reaches them through
[validator-set continuity](#validator-set-continuity), which already requires
every member of the referenced set to be enrolled and unrevoked.

**Fixture `AUTH-0`: the case on which the two readings disagree, and the two
boundaries.** Both identities below hold a finalized enrollment certificate with
`valid_from_height` `5`. A `revoke_identity` naming `cblx1revokedfixture` with
`effective_height` `50` is finalized in the block at height `20`. For a
subscription burn authorized by each key:

| `node_id` | including height `h` | enrolled by `h` | revocation final below `h` | `effective_height <= h` | verdict |
| --- | --- | --- | --- | --- | --- |
| `cblx1revokedfixture` | `4` | no | no | no | invalid |
| `cblx1revokedfixture` | `5` | **yes** | no | no | **valid** |
| `cblx1revokedfixture` | `19` | yes | no | no | valid |
| `cblx1revokedfixture` | `21` | yes | yes | no | **valid** |
| `cblx1revokedfixture` | `49` | yes | yes | no | **valid** |
| `cblx1revokedfixture` | `50` | yes | yes | **yes** | invalid |
| `cblx1revokedfixture` | `51` | yes | yes | yes | invalid |
| `cblx1ci6q36gqm6u3spknxzr7p5r2y4xw7n25d5icm7rsoq7lq6ka` | `51` | yes | no revocation exists | no | valid |

**Rows `21` and `49` are the divergent ones.** They are the only rows *of this
table* on which the finalized reading says *invalid* and this definition says
*valid* — the divergent heights are the whole interval `[20, 49]`, of which the
table samples the first interior height and the last, and the table is a sample
rather than an enumeration. Those are the heights on which two implementations
would have partitioned the chain. The remaining rows are present so that what
they do *not* prove stays visible: `19`, `50` and `51` are agreed on by both
readings, and a conformance case built only from them would have been green
before this section existed.

**The two boundaries are one row each, and each is the first height at which its
clause flips.** Row `5` is `h = valid_from_height`: a certificate authorizes
*at* the height it becomes valid, so clause 1 is `<=` and not `<`. Row `50` is
`h = effective_height`: a revocation bites *at* its effective height, so clause 2
is `<=` and not `<`. Both are here because a clause stated with an inclusive
comparison and exercised only away from the boundary is a clause whose boundary
is a guess.

The last row varies the one quantity the revoked rows hold constant — whether a
revocation for the key exists at all — so that an implementation answering
*invalid* for every key, or reading clause 1 as clause 2, does not pass.
`effective_height` `50` and the finalization height `20` are deliberately
different: an implementation comparing `h` against the height at which the
revocation was included rather than against `effective_height` fails rows `21`
and `49`.

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

#### `reward_epoch` is derived from height

`reward_epoch` is an index that appears in every mint, in three Merkle leaves,
and in three uniqueness rules. Until this section nothing derived it, and the
consequence was that `reward_epoch_ms_min` — the floor
[reward bounds](README.md#reward-bounds) places under the **declared** duration
of an epoch — bounded a number in a signed document and not the speed at which
the index advances. A conforming quorum that incremented `reward_epoch` at every
block violated nothing, and multiplied real issuance by the ratio between a
block and an epoch.

The derivation binds the index to `height`, which is the one quantity of this
chain a validator cannot write freely: `height` is `previous + 1`, and any
observer can recheck that from headers alone, at any time, forever. Let
`reward_epoch_ms` be the value carried by the `reward_policy` document the mint
names through its own `policy_hash`, and `block_interval_ms` the genesis
constant of [README.md](README.md#genesis-constants):

```text
reward_epoch_blocks = ceil(reward_epoch_ms / block_interval_ms)
```

> A `mint` naming `reward_epoch` `e` is valid only in a finalized block at
> height `h` satisfying `(e + 1) * reward_epoch_blocks <= h`. A block containing
> a mint that violates this is invalid.

The ceiling and not the floor: the quantity is a lower bound on how much chain
must pass before an epoch may be settled, so rounding it down would widen the
permission. It is a settlement **floor** and not an equality, because a mint for
an epoch is finalized after that epoch has ended and no rule can say how long
after. A quorum may settle late, and may settle a backlog at once.

**What the rule bounds, stated as narrowly as it is true.** Cumulative existence
emission through height `h` is at most
`floor(h / reward_epoch_blocks) * existence_fund_microtokens_per_epoch_max`,
the ceiling fixed in [reward bounds](README.md#reward-bounds), since epoch `e`
is unmintable below its floor and at most one epoch's fund is mintable within
one epoch. The bound is stated against the genesis ceiling and not against `F`
itself because `F` is a governed quantity: a policy document may move it between
one epoch and the next, so `floor(h / reward_epoch_blocks) * F` holds only while
the policy carrying that `F` is in force, and a bound that holds while nothing
changes is not held by a rule. That is a bound **per block**. It is **not** a bound per unit of real
time, and must not be read as one: how many real milliseconds a block takes is
the gap of [block format](#block-format), which this protocol measures and does
not constrain. The two halves are one closure — the index is paced by the chain,
and the chain's own pace is measured from outside — and neither half is
sufficient alone. The derivation is what gives the fast side of the
[cadence band](README.md#cadence-band) something to protect.

**The opposite direction is not closed by a rule, and cannot be.** An index that
does not advance freezes existence income without violating anything, and it is
the twin of the case [reward bounds](README.md#reward-bounds) already declares
invalid for a `reward_epoch_ms` above its ceiling. No validity rule internal to
this chain can compel a quorum to mint — a rule can reject an act, never require
one — so this direction is closed in the same shape as the cadence: it is made
**computable**. The highest index the floor already permits at height `h` is
`floor(h / reward_epoch_blocks) - 1`, so the number of epochs whose floor has
passed unsettled is a quantity any full node or auditor recomputes from the
headers and the finalized mints, without trusting anyone's account of it.

A settlement **deadline** — an epoch becoming permanently unmintable after some
window — was considered and is deliberately not adopted. It would not compel a
quorum to mint either, and it would convert an honest outage into permanently
lost income, while the cumulative bound above already holds whether or not a
backlog is settled at once.

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

#### Availability tariff: zero as a validity rule

A `reward_policy` document MUST have `availability_microtokens_per_unit == 0`,
enforced as a validity rule on acceptance ([ADR-010]).

The reason is structural: `work_compensation` for `availability` is the only
channel that pays per node without an aggregate cap. If positive, an adversary
controlling `N` emulated identities increases total epoch emission linearly,
violating criterion (a) of [ADR-007] by construction. A document with
`availability_microtokens_per_unit > 0` is **rejected on acceptance**. If
availability is to be rewarded, it MUST flow through the capped existence fund
`F`, never an uncapped per-unit rate.

Furthermore, `existence_fund_microtokens_per_epoch` is bounded by
`existence_fund_microtokens_per_epoch_max` in the genesis `RewardBounds` trust
anchor ([ADR-010]), and changes in `reward_policy` parameters between
consecutive sequences are constrained by the change ratio and activation gap
defined in `RewardBounds`. A cap proportional to eligible nodes (`F = k * E`)
is explicitly rejected ([ADR-011]).

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

For a subscription, the key MUST derive the enrolled, unrevoked `payer_node_id`;
the signature is required and the node balance is debited. The service period is
half-open and end MUST be greater than start. For `app_hosting`, `payer_node_id` and
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

Canonical serialized example. Its hash-valued fields are illustrative
placeholders as described in
[README.md](README.md#inline-examples-are-not-conformance-oracles), with one
exception that is **not** free: `request_hash` MUST equal `challenge_id`, so
the example carries the same value in both. Until 2026-08-25 it did not, and
the example asserted a shape no conformant network can produce.

```json
{"authorization":{"quorum_certificate":{"signatures":[{"signature":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA","validator_id":"val-001"}],"validator_set_hash":"sha256:1df0a6454faaa5985b7f98c48d3c60d2ed62d5b3b24fe8e97d3dca1dd36f1120"}},"body":{"auditor_signatures":[{"signature":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA","validator_id":"val-001"}],"challenge_id":"sha256:3d56e5dd5104a2ad5c733fa4f0b6d8f35de2f68509e9c10a3d473128eaec0b21","completed_at_ms":"1787654416000","issuer_node_id":"cblx1issuerfixture","issuer_reveal":"REREREREREREREREREREREREREREREREREREREREREQ","kind":"availability","measured_units":"1","outcome":"passed","request":{"assignment":{"response_bytes":"32"},"challenge_id":"sha256:3d56e5dd5104a2ad5c733fa4f0b6d8f35de2f68509e9c10a3d473128eaec0b21","deadline_ms":"1787654420000","issued_at_ms":"1787654415000","issuer_commitment":"sha256:19556b209c36de1940340bd3ada4a4c821fe70cde0fd3906af2b71f31445e4d5","issuer_node_id":"cblx1issuerfixture","issuer_signature":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA","kind":"availability","randomness":"AAECAwQFBgcICQoLDA0ODxAREhMUFRYXGBkaGxwdHh8","randomness_source":{"beacon_block_id":"sha256:7e0694f564afa2d047db4eb58f4f2b3d322d71db808f6bbf5313ee2d2a4a95af","beacon_height":"40","commitment_epoch":"17"},"subject_node_id":"cblx1ci6q36gqm6u3spknxzr7p5r2y4xw7n25d5icm7rsoq7lq6ka"},"request_hash":"sha256:3d56e5dd5104a2ad5c733fa4f0b6d8f35de2f68509e9c10a3d473128eaec0b21","response":{"challenge_id":"sha256:3d56e5dd5104a2ad5c733fa4f0b6d8f35de2f68509e9c10a3d473128eaec0b21","completed_at_ms":"1787654416000","result":{"kind":"availability","response":"MzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzM"},"subject_node_id":"cblx1ci6q36gqm6u3spknxzr7p5r2y4xw7n25d5icm7rsoq7lq6ka","subject_signature":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"},"response_hash":"sha256:8bc23b6277b0892c0eea482c835359a2ad975ac18af9832b727738a880f2400f","subject_node_id":"cblx1ci6q36gqm6u3spknxzr7p5r2y4xw7n25d5icm7rsoq7lq6ka"},"created_at_ms":"1787654420000","expires_at_ms":"1787740820000","kind":"challenge_evidence","network_id":"coblox-devnet-0","schema_version":"0.1"}
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

Genesis has height 0, carries **no transactions**, so its `transactions_root`
is the empty-block root `H(0x03)`, and its `previous_block_id` is 32 zero bytes. That value is
fixed by this sentence and is not configurable: it is an input to
`genesis_block_id` and therefore to `chain_id`, so a distribution free to choose
it would leave `chain_id` underdetermined, which is the defect
[Genesis derivation and the placeholder chain ID](README.md#genesis-derivation-and-the-placeholder-chain-id)
exists to close. The earlier wording — *the configured all-zero previous ID* —
admitted both readings. Timestamps
MUST be greater than the median of the previous 11 finalized blocks and no more
than the active maximum clock drift after the proposal is received.

**The target block interval is 5 seconds, and v0 does not enforce it.** The
value is a genesis constant, declared in
[README.md](README.md#genesis-constants) with the reason it is not a governed
parameter ([ADR-013]). It is what gives a real-time meaning to every quantity
this protocol denominates in blocks — `election_epoch_blocks`,
`candidacy_close_blocks`, `election_entropy_blocks`,
`min_revocation_effective_delay_blocks` and
`election_parameter_min_activation_gap_blocks` — and that meaning is the whole
of its normative content. **No v0 validity rule constrains the distance between
consecutive `timestamp_ms` values.** The two constraints above impose
monotonicity against the median of eleven and an upper bound against the
receiver's clock; neither imposes a step. A set of validators that produces blocks more slowly
therefore lengthens, in real time, every quantity denominated in blocks —
including its own terms — without violating anything. It is named here rather
than left for a reader to discover, because a declared cadence reads like an
enforced one unless the difference is written down.

**No rule here will ever impose it, and the reason is general.** Every clock
this chain carries is written by the validators, so a validity rule can only
compare a validator-written number to a validator-written number. In particular
a rule on the distance between consecutive `timestamp_ms` values is **rejected**
and not merely absent: it would oblige a set to *write* a cadence, not to
*produce* one, and would buy a false closure at the full price of a
specification change ([ADR-013]).

**What this document does instead is remove the last clause of the paragraph
above** — *without a light client being able to say it was deliberate*. The real
production rate is now **measured**, against the one clock no validator writes:
the `issued_at_ms` of a weak subjectivity checkpoint. The measurement is step 4b
of [light-client balance verification](#light-client-balance-verification), its
tolerance is the [cadence band](README.md#cadence-band) of the genesis trust
anchor, and the checkpoint release procedure applies the same band before it
signs. The slowdown is not prevented. It is made visible, and given a threshold
that was declared before anyone had a reason to argue about it.

The two directions are not symmetric in what they cost. Slowdown stretches
incumbency and every revocation delay; acceleration multiplies real issuance,
because [`reward_epoch` is derived from height](#reward_epoch-is-derived-from-height).
The band is two-sided for that reason and not for tidiness.
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

**This preimage carries no `chain_id`, unlike every other domain-separated
preimage over a chain-specific consensus object referenced by hash, and the
asymmetry is deliberate.** The word *domain-separated* is load-bearing: the
tagged-tree preimages of this document — `node_leaf`, `eligible_leaf`,
`revocation_leaf`, `candidate_leaf` and the rest — are separated by tag byte,
carry no `chain_id` either, and are exempt for the same reason given next.

The reason is that every object which names a set by hash is itself bound to its
chain, and it holds separately on each of the three surfaces where a set is
named. A **quorum certificate** carries signatures taken over
`coblox-block-vote-v0` with `chain_id_32` ([what validators sign](#what-validators-sign)),
so a replayed certificate fails on the signature before its set hash matters. A
**weak subjectivity checkpoint** is itself a chain-bound preimage and is
rejected outright when its `chain_id` is not the client's. A **set transition**
is only ever seen through `next_validator_set_hash` in a `BlockHeader`, and
`block_id` carries `chain_id_32`.

A second binding exists in the set's own bytes — `election.election_seed` and
every `election_ticket` are computed through `chain_id_32`
([the derivation](#the-derivation)), and every `key_binding_signature` is taken
over the global chain-bound signature procedure — and it is corroboration rather
than the argument. On the **genesis** set, the only set without an `election`
record, the first two do not exist and the third is taken over the
**placeholder** chain ID rather than the derived one, because the set's bytes
are an input to `genesis_block_id`
([genesis derivation](README.md#genesis-derivation-and-the-placeholder-chain-id));
on the genesis set it binds the `network_id` its object carries, which is the
network name and not the chain.
The argument above covers the genesis set without depending on it.

Binding `chain_id` here would restate a binding that is already present, and
would change every published value that depends on this hash. The full statement
of the exception, with the six domain-separated preimages that omit `chain_id`
for reasons of their own, is in
[README.md](README.md#hash-preimage-registry).

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
`{"activation_height":...,"consensus_public_key":...,"network_id":...,"node_id":...,"validator_id":...}`.

**`network_id` is in that object, and it is the only signed payload of this
protocol that needed adding to.** The genesis set's bytes are an input to
`validator_set_hash`, a field of the height-0 header, so a genesis binding is
signed under the
[placeholder chain ID](README.md#genesis-derivation-and-the-placeholder-chain-id)
— the same 32 zero bytes on every network. Without `network_id` the signed
payload of a genesis entry would be byte-identical across two networks, and the
signature published in one distribution would seat that validator in another
genesis it never consented to, which is the one thing this signature exists to
prove. It is present at every height and not only at genesis, because a shape
that changes at one height is a shape to get wrong; above genesis it is
redundant with `chain_id_32` and harmless. `network_id` is **not** a field of the
`ValidatorSet`: a verifier takes it from the same trust anchor it takes
`chain_id` from. [REVIEW-029] RF-002.
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
nothing distinguishes the block from an honest one.

With the contraction floor alone, this attrition path would stop only at
`validator_min_set_size`. If `min_set` were permitted to remain small while `V`
grew, a coalition holding half the set could outlast the rest under the floor.
The relational constraint `3 * validator_min_set_size >= 2 * V`, enforced on
acceptance ([ADR-010]), closes that specific path: `validator_min_set_size` is
at least `ceil(2V/3)` in every valid parameter document, so a coalition holding
`k < 2/3 * V` can never **hold every seat** — the contraction stops at a set in
which honest members still sit.

#### Owning the set and controlling it are different thresholds

**This is the fourth version of this paragraph, and the third time the claim of
"two thirds" has been refuted. It is refused here in favour of the smaller,
correct number.** The constraint above bounds **possession**: how much of the
set a coalition can end up holding. Possession is not the property this document
promises anywhere else. The property that matters is the **quorum predicate**,
`3 * signed > 2 * total` over the *active* set — and a coalition obtains that
long before it owns anything, because the set it must reach quorum over is the
**contracted** one, not the original.

The arithmetic, on the recommended values `V = 27`, `min_set = 18`. A coalition
of **13 seats, 48.1 %** of the set:

1. it holds more than one third, so it can withhold quorum and censor. During
   the candidacy window it finalizes its own 13 candidacies and lets through
   exactly 6 honest ones;
2. at the boundary the derivation yields `S_new = 19`. The contraction floor is
   **strict**, so `27` contracts to `19` and not to `18`: `3 * 19 = 57 > 54`.
   `min_set = 18 <= 19`. The entry cap is untouched, `fills = 0`. No rule is
   violated and honest nodes sign the block, because the block is valid;
3. the coalition now holds 13 of 19, and `13 * 3 = 39 > 19 * 2 = 38`. **It has
   quorum.** From here it signs blocks, mints, revocations, and governed
   documents alone;
4. with quorum it lowers `V` and `min_set` together inside the rate limit — the
   relational rule constrains their *ratio*, not their magnitude — and repeats.
   Full possession follows in three boundaries.

The smallest coalition that reaches quorum this way, for a parameter set at the
tight end of the relational rule, is

```text
S_new  = max( floor(2V/3) + 1, validator_min_set_size )
k_min  = max( floor(2 * S_new / 3) + 1, floor(V/3) + 1 )
```

| `V` | `min_set` | `S_new` | `k_min` | fraction of `V` |
| --- | --- | --- | --- | --- |
| 12 | 8 | 9 | 7 | 58.3 % |
| 27 | 18 | 19 | 13 | 48.1 % |
| 36 | 24 | 25 | 17 | 47.2 % |
| 60 | 40 | 41 | 28 | 46.7 % |
| 600 | 400 | 401 | 268 | 44.7 % |

The fraction decreases with set size and approaches **`4/9`, 44.4 %**, from
above. It is never two thirds at any set size.

**What the constraint therefore buys, claimed exactly and not more.** Before
[ADR-010] the effective threshold against attrition was *just above one third*.
After it, control of the set requires **about four ninths** of it — `k_min`
above, bounded below by `4V/9` — and possession of every seat requires two
thirds. The gain is real and is worth the rule: it is roughly a third more of
the network that an attacker must hold. It is not two thirds, and the argument
that "above two thirds BFT safety has already failed" **does not apply to the
quorum threshold**, because at `4V/9` BFT safety has not failed at all. That
argument was what made the previous claim look harmless, and it is exactly the
step that was wrong in each of the three refutations.

**What the floor and the constraint buy together is worth having and is claimed
exactly.** They convert a capture that took **one invisible boundary** into a
process that requires about four ninths of the set and publishes a signed
contraction document any light client can diff at every step. That is the same
standard the entry cap is held to — an event converted into an observable
process with a public signal at every step — and it is claimed here on the same
terms, neither more nor less.

Boundary conformance for the threshold above; the first row is the attack:

| `V` | `min_set` | coalition | `S_new` | successor verdict | coalition holds quorum |
| --- | --- | --- | --- | --- | --- |
| 27 | 18 | 13 | 19 | **valid** | **yes**, `39 > 38` |
| 27 | 18 | 12 | 19 | **valid** | no, `36 < 38` |
| 27 | 18 | 13 | 18 | **invalid**, `54` is not `> 54` | — |
| 27 | 18 | 9 | 19 | **valid** | no, `27 < 38` |

The last row needs its reason stated precisely, because an earlier version of it
said "cannot censor" and that is wrong. A coalition of 9 of 27 **can already**
deny quorum: the honest 18 need `3 * 18 > 2 * 27`, which is `54 > 54` and false, so
blocking a boundary needs only `3k >= V` and not `3k > V`. What 9 cannot do is
obtain quorum for **itself** on any lawful successor — `3 * 9 = 27` is not above
`2 * 19 = 38` — so what it gets is a **halt**, which is the outcome the contraction
floor already grants anyone at one third. The conclusion of the row is unchanged;
only its reason was wrong. For the same reason the `floor(V/3) + 1` term of `k_min`
above is a **conservative** statement of the censoring requirement — the true
condition is `3k >= V` — and it never binds, because the first term of the maximum
dominates at every set size in the table.

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
3 * validator_min_set_size >= 2 * V    // min_set must be at least 2/3 of V to prevent attrition capture
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

**The relational bound on `validator_min_set_size` and what it prevents.** The
rule `3 * validator_min_set_size >= 2 * V` ([ADR-010]) ties the minimum set size
floor directly to `V` rather than leaving it an uncoupled choice. Without this
rule, governance could raise `V` (e.g. `27 -> 33 -> 36`) while leaving `min_set`
at 18. At `V = 36` and `min_set = 18`, `min_set / V` drops to exactly 50%:
selective censorship can then contract the set `36 -> 25 -> 18` across two
boundaries, delivering 100% of the active seats to an 18-seat coalition (50% of
`V`), where BFT safety has not failed and the network believes it has a
security guarantee. Enforcing `3 * validator_min_set_size >= 2 * V` makes
`min_set >= ceil(2V/3)` a validity rule for every conformant document, so no
coalition below two thirds can come to **hold every seat**. It does not prevent
a coalition from obtaining **quorum** over a lawfully contracted set, which
takes about four ninths; see
[owning the set and controlling it are different thresholds](#owning-the-set-and-controlling-it-are-different-thresholds).

Boundary conformance fixtures for `3 * validator_min_set_size >= 2 * V`:

| `V` | `validator_min_set_size` | `3 * min_set >= 2 * V` | Verdict | Reason |
| --- | --- | --- | --- | --- |
| 12 | 8 | `24 >= 24` | valid | exact floor (`PD-0` fixture) |
| 12 | 7 | `21 < 24` | **invalid** | below 2/3 floor |
| 27 | 18 | `54 >= 54` | valid | exact equality for recommended values |
| 27 | 17 | `51 < 54` | **invalid** | below 2/3 floor |
| 36 | 24 | `72 >= 72` | valid | exact floor at `V = 36` |
| 36 | 18 | `54 < 72` | **invalid** | 50% ratio rejected on acceptance |

**The relational bound on `validator_min_set_size` and what it costs.** The
paragraph above states what the rule prevents. A rule that is only described by
what it prevents is half-documented, and this one is paid for in liveness by
honest networks:

- **the contraction margin is spent once.** The floor permits losing up to a
  third of the set at a boundary; `min_set >= ceil(2V/3)` permits it **once**.
  At `V = 27, min_set = 18` the largest lawful contraction is `27 -> 19`, and
  from 19 the smallest lawful successor is **18**, which is the floor itself.
  After a single maximal contraction the set sits one seat above the floor and
  the boundary after that tolerates no unreplaced departure at all: the
  successor is invalid and the chain stalls. The same holds at every size,
  because `min_set` is pinned to the same ratio the floor uses;
- **cooldown compounds it.** Members that left are out of the pool for
  `validator_cooldown_epochs` boundaries and cannot repair the shortage they
  created. This section already calls that "the sharpest liveness edge"; the
  relational bound sharpens it further;
- **measured on the recommended parameters** — `V = 27`, `T = 9`, `c = 3`,
  `cooldown = 2`, `min_set = 18` — a network whose candidate pool goes empty
  loses three seats per boundary and survives **three boundaries**; at the
  fourth the successor would be 15, below `min_set`, and the chain stalls. The
  figure is measured against these values rather than inherited from an earlier
  study that assumed a different minimum;
- **`min_set = V` is admissible and is a trap.** `3V >= 2V` holds for every `V`,
  so a document setting `validator_min_set_size = validator_target_set_size` is
  accepted. Such a network stalls at the **first** unreplaced departure. Nothing
  in this block rejects it, and an operator choosing `min_set` should treat `V`
  as an upper bound to stay well below rather than as a permitted value;
- **the ramp is where this binds hardest.** The phase with the smallest
  candidate pool is the network's first, and it is now also the phase that must
  keep `ceil(2V/3)` validators alive at every boundary.

Recovery from a stall is the out-of-band path already declared in
[degenerate cases](#degenerate-cases-and-what-the-protocol-does-instead-of-improvising):
an authenticated release, not a mechanism a quorum can trigger. The trade is the
same one this section takes everywhere — safety over liveness — and it is
recorded here so that the cost is read next to the rule instead of discovered by
the first network that pays it.

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
> — are detectable by nobody, and are bounded rather than observed. Two distinct
> bounds apply to attrition, and conflating them is the error this paragraph has
> made three times. **Possession** of every seat requires **two thirds of the
> target set size**, guaranteed by the joint enforcement of the contraction floor
> and `3 * validator_min_set_size >= 2 * V` ([ADR-010]). **Control of the set
> through the quorum predicate** requires only
> `k_min = max(floor(2 * S_new / 3) + 1, floor(V/3) + 1)` with
> `S_new = max(floor(2V/3) + 1, validator_min_set_size)` — about **four ninths**
> of the set, 48.1 % at `V = 27` and tending to 44.4 % as `V` grows. The argument
> that above two thirds BFT safety has already failed applies to the first bound
> and **not** to the second. What the rules buy is that reaching any lawful
> contraction publishes a signed document any light client can diff, and that the
> effective threshold moves from just above one third to about four ninths across
> all valid parameter documents.

The first sentence closes [DEBT-005]; the rest is what remains. Two earlier
versions of this paragraph promised more: one omitted "within the parameter
limits fixed at genesis", at a time when no such limits existed and the property
could therefore be switched off by a document the sitting quorum signs; the other
spoke only about who **enters** the set, at a time when nothing bounded who
**leaves**. A third version claimed that closing the second gap moved the effective capture
threshold to two thirds. **That version was wrong, not premature**, and the
retraction is recorded here in that form on purpose: selective censorship
defeated it then, exactly as it defeats the fourth version, which repeated the
claim after `3 * validator_min_set_size >= 2 * V` was enacted and differed only
in where the contraction stops. Calling the third version premature would
convert a retraction into a rehabilitation and remove the precedent that should
have prevented the fourth — which is how the fourth came to be written. The
common defect in all three is the same: a bound on **possession** was stated as
a bound on **capture**, and the quorum predicate needs far less than possession.
This fifth version separates the two thresholds and claims four ninths for the
one that matters, with the arithmetic beside it. The wording keeps every
qualification visible, because the property is exactly as strong as the bounds,
the floor, and the relational constraints, and a reader is entitled to know
where to look.
The honest summary is unchanged in kind: this section moves the light client from
checking that a transition was *authenticated* to checking that it was *lawful*,
without promising that it was *correct*.

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

### `lifecycle_u8`, and why zero is not `active`

`lifecycle_u8` is a single byte and its value is **normative**, because it
enters a preimage that determines `state_root`. Two implementations that
disagree about it compute different `app_leaf` values for the same app account
and split the chain at the first account that is not `active`.

| Lifecycle | `lifecycle_u8` |
| --- | --- |
| — reserved, never assigned — | `0x00` |
| `active` | `0x01` |
| `grace` | `0x02` |
| `suspended` | `0x03` |

**Every other value, `0x00` included, is invalid.** An implementation that
encounters one MUST reject the object that carried it; it MUST NOT substitute a
default, and there is no default to substitute. This is stated as a rule rather
than left implicit because a default is exactly how the divergence above comes
back: two implementations that mis-handle the same *known* value disagree
visibly and loudly, while two that apply different defaults to an *unknown*
value agree on nothing and say nothing.

The obvious encoding is declaration order starting at zero, `active = 0`, and
that is the encoding this document does **not** use. The reason is the
direction of the danger. `app_leaf` is reconstructed from stored state, and the
zero byte is what an uninitialized, truncated, or zero-filled record yields for
free in every language a node might be written in. If zero meant `active`, that
accident would produce the *permissive* state — an app treated as serving when
its record says nothing — and it would produce a leaf indistinguishable from a
legitimately active one, so nothing downstream could contradict it. With `0x00`
reserved and invalid the same accident is a rejection at the point it happens.
The cost of the choice is one byte of intuition: implementers who assume
`0/1/2` are wrong, and the published `APP-0` fixture in
[README.md](README.md#hash-conformance-fixtures) is there to tell them so on
the first run instead of on the first suspension.

The textual spelling in `AccountProof` below is the lowercase name, not the
number; the number appears only in the preimage. An unknown spelling is
rejected on the same terms.

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
4b. **Measure the real cadence.** Load the `CadenceBand` from the configured
   network distribution, exactly as step 5 loads `ElectionBounds` — it is a
   trust anchor and MUST NOT be learned from a peer, a header, or a chain
   document — and require its `chain_id` to equal the configured chain ID and
   its own constraints to hold. A band whose `min_ms_per_block` or
   `min_measured_blocks` is zero does not fail: it silently admits every rate,
   so it is **rejected before it is used** rather than applied. Then, once step
   4 has authenticated a header the client is treating as the chain's finalized
   tip, compute
   `blocks = tip.height - checkpoint.height` and
   `elapsed_ms = now - checkpoint.issued_at_ms`, and compare against the
   [cadence band](README.md#cadence-band) of the genesis trust anchor exactly:
   the chain is **faster than the band** when
   `elapsed_ms + max_external_clock_slack_ms < blocks * min_ms_per_block` and
   **slower** when `elapsed_ms > blocks * max_ms_per_block`. Compute the
   comparison without dividing; a client that divides first will report a chain
   just outside the band as inside it. When `blocks < min_measured_blocks` the
   measurement is **not made**, and that is reported as its own outcome and
   never as a pass.

   The client MUST fail closed when the chain is faster than the band, and MUST
   report — not reject — when it is slower.

   **Why the slack is on the fast comparison and only there.** Both readings use
   only clocks outside the chain: `issued_at_ms` is signed by a release key that
   belongs to no validator, and `now` is the client's own. But both ends of the
   ratio are biased, and they push it in **opposite** directions. A client that
   has not caught up counts fewer blocks than the chain produced, which drags
   the reading slow. And `issued_at_ms` is when the checkpoint was *produced*,
   not when the height it names was finalized
   ([README.md](README.md#weak-subjectivity-checkpoint) states this), so blocks
   produced during release latency are counted **without their time**, which
   drags the reading fast — as does a client clock that is behind, or a release
   clock that is ahead. Neither verdict is attributable to the chain on its own,
   and an earlier revision of this step said otherwise.

   What separates the two directions is what lies past the tolerance. Nothing
   honest makes blocks appear, so a fast reading beyond
   `max_external_clock_slack_ms` has no innocent explanation. A slow reading is
   indistinguishable from the client's own lag **at any magnitude**, and no
   tolerance would change that — which is why the slow side is reported rather
   than tolerated. Rejecting on a reading the client's own position produces
   would be a guard that cries wolf, and a guard nobody believes is a guard
   nobody runs.

   **To whom the client reports, because a report with no recipient is a word.**
   The slow verdict MUST be surfaced to whoever asked the client for the balance
   — carried out of the verification routine and displayed or logged alongside
   the result, not discarded inside it. An implementation whose verification
   function computes the verdict and drops it has not performed this step. The
   chain is not rejected: the answer is delivered together with the observation
   that the chain is producing more slowly than its declared band, which is the
   whole of what the observation is worth.

   `timestamp_ms` is **not** an input to this step and MUST NOT be used in it,
   here or in any implementation of it. It is written by the same validators
   whose production rate is being measured; a client that reached for it would
   be timing the validators with the validators' own clock, and would get
   whatever answer they chose to write. This is the same reason a validity rule
   on the distance between consecutive `timestamp_ms` values is rejected
   ([ADR-013] and [block format](#block-format)).

   **What this step establishes, and what it does not.** It establishes the rate
   at which blocks arrived between two external points in time. It establishes
   nothing about *why*: an honest network under a partition and a set stretching
   its own incumbency produce the same reading, and no client of any kind can
   separate them. It belongs with the eight facts of
   [what a light client can establish](#what-a-light-client-can-establish-about-set-composition)
   in that respect — a quantity the client can compute, not a verdict it can
   reach.

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
