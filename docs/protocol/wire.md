# Coblox P2P wire protocol

This document defines connectivity and the complete Coblox v0 application
message catalog carried over authenticated libp2p connections.

## Network stack

Coblox nodes use libp2p. The v0 interoperability baseline is:

- QUIC-v1 over UDP as the preferred encrypted, multiplexed transport;
- TCP with Noise and Yamux as the required fallback;
- multistream-select for application protocol negotiation;
- Identify (`/ipfs/id/1.0.0` and push), Ping, and Kademlia DHT for WAN peer
  routing; mDNS for optional same-LAN discovery;
- AutoNAT v1 (`/libp2p/autonat/1.0.0`) for reachability classification,
  Circuit Relay v2 for the relayed fallback, and DCUtR (`/libp2p/dcutr`) for
  attempts to upgrade a relayed connection to a direct TCP/QUIC connection;
- GossipSub 1.1 or later for authenticated block and evidence announcements.

AutoNAT v2 remains an active libp2p working draft and is an optional negotiated
optimization, not the v0 interoperability baseline. A node MUST retain a relay
connection when hole punching fails: NAT traversal improves reachability but
does not guarantee a direct path. Relay service is opt-in for publicly reachable
nodes and MUST apply reservation, byte, duration, and connection limits.

**Choice rationale.** QUIC was preferred over TCP for connection migration and
fewer handshakes on mobile networks. WebSocket was not selected for native v0
nodes because it adds HTTP/TLS deployment constraints; it can be added under a
new negotiated transport profile. The behavior above follows the current
[libp2p specifications](https://github.com/libp2p/specs), including
[Circuit Relay v2](https://github.com/libp2p/specs/blob/master/relay/circuit-v2.md)
and [DCUtR](https://github.com/libp2p/specs/blob/master/relay/DCUtR.md).

### Devnet transport subset

For the devnet seed validator topology, nodes implement the required baseline fallback: TCP with Noise and Yamux, plus GossipSub 1.1 for broadcast of consensus messages (`/coblox/<network_id>/consensus/0.1`) and blocks (`/coblox/<network_id>/blocks/0.1`). Seed validators are configured with reachable peer addresses and do not require WAN NAT traversal or peer discovery.

The following elements of the full WAN baseline are explicitly excluded from the devnet seed validator scope:
- QUIC-v1 over UDP (connection migration and 0-RTT);
- Kademlia DHT and mDNS (WAN/LAN peer routing and discovery);
- AutoNAT v1, Circuit Relay v2, and DCUtR (reachability classification and NAT hole punching).

Full WAN NAT traversal and peer discovery are deferred to milestone M-04.

## Discovery

Bootstrap distributions provide at least three seed multiaddresses from
independent failure domains. Nodes:

1. discover LAN peers with mDNS service `_p2p._udp.local` when enabled;
2. connect to seeds and run Identify;
3. join the Kademlia DHT namespace whose genesis-derived key is
   `SHA-256("coblox-dht-v0\0" || genesis_block_id)`;
4. advertise the rendezvous namespace `/coblox/<network_id>/v0` when a
   rendezvous service is configured;
5. discard addresses whose authenticated Peer ID does not match Identify.

Private, loopback, and link-local addresses learned from untrusted WAN peers
MUST NOT be dialed unless they are valid for an existing local connection
scope. Nodes SHOULD randomize reconnect backoff and bound address-book entries.

## Coblox protocol IDs and topics

| Purpose | Identifier | Pattern |
| --- | --- | --- |
| identity enrollment | `/coblox/enrollment/0.1.0` | request/response |
| availability/storage/compute challenge | `/coblox/challenge/0.1.0` | request/response |
| ledger status and synchronization | `/coblox/ledger-sync/0.1.0` | request/response |
| block announcements | `/coblox/<network_id>/blocks/0.1` | GossipSub |
| challenge-evidence announcements | `/coblox/<network_id>/evidence/0.1` | GossipSub |
| consensus proposals and votes | `/coblox/<network_id>/consensus/0.1` | GossipSub |

The consensus topic is separate from `blocks` and the separation is normative,
not organizational: a `block_announcement` carries a block that is **already
finalized** and is explicitly a hint, while the consensus topic carries the
messages that decide whether a block will exist at all. A node that dropped
consensus messages under the backpressure rules written for hints would stop
participating in consensus while appearing healthy.

The enrollment stream accepts unauthenticated transport peers; all other
Coblox protocols require a valid finalized enrollment certificate and a valid
`TransportKeyAttestation` presented in-session ([identity.md](identity.md#authentication-on-a-connection)).
Topic names are exact UTF-8 strings. A node MUST NOT bridge messages between
network IDs.

## Framing

Request/response streams carry repeated frames:

```text
unsigned-varint payload_length || payload_length bytes of JCS JSON
```

The varint is the minimal unsigned LEB128 encoding and is limited to five
bytes. A zero-length frame, non-minimal varint, oversized length, truncated
payload, invalid UTF-8, or non-canonical JSON closes the stream. The limits in
[README.md](README.md#security-and-resource-limits) apply before allocation.
GossipSub message data is exactly one `SignedEnvelope` without this length
prefix because GossipSub already frames messages.

## Signed envelope

Domain: `coblox-wire-envelope-v0`. The envelope signature covers the object
with `signature` removed using the global chain-bound signature procedure.
`payload` is an object, never an embedded JSON string.

```text
SignedEnvelope = {
  "schema_version": "0.1",
  "network_id": string,
  "message_type": enum,
  "message_id": sha256-string,
  "sender_node_id": string,
  "created_at_ms": u64-string,
  "expires_at_ms": u64-string,
  "nonce": base64url(16 bytes),
  "payload": object,
  "signature": base64url(64 bytes)
}
```

`message_id` is SHA-256 of `"coblox-message-id-v0\0" || chain_id_32` plus JCS
of the envelope without `message_id` and `signature`. Receivers recompute it,
reject an expiry before
creation, and require `expires_at_ms - created_at_ms <= max_envelope_validity_ms`
from the active signed consensus parameters. They cache message IDs and
`(sender_node_id, nonce)` until expiry. The cache has protocol caps
`replay_cache_entries_global` and `replay_cache_entries_per_peer`; an insertion
that would exceed either cap rejects the new envelope as `rate_limited` and
MUST NOT evict a still-live entry. Clock rollback, unavailable durable cache,
or loss of cache integrity fails closed for protected protocols. Either
duplicate is a replay. GossipSub's message-ID function MUST use this verified ID.

The complete v0 enum is `enrollment_admission_request`,
`enrollment_admission_challenge`, `enrollment_request`, `enrollment_response`,
`challenge_request`, `challenge_response`, `ledger_status_request`,
`ledger_status_response`, `ledger_range_request`, `ledger_range_response`,
`balance_proof_request`, `balance_proof_response`, `block_proposal`, `prevote`,
`precommit`, `block_announcement`, and
`challenge_evidence_announcement`. Any other value is `unsupported_version`.

Canonical serialized example:

```json
{"created_at_ms":"1787654410000","expires_at_ms":"1787654470000","message_id":"sha256:56d2aa0cd4c2ff0b06c47b478b6bfc2dff88b2c162c6cff1e33f9bf3284c7308","message_type":"ledger_status_request","network_id":"coblox-devnet-0","nonce":"AAECAwQFBgcICQoLDA0ODw","payload":{"finalized_height":"41","want_validator_set":true},"schema_version":"0.1","sender_node_id":"cblx1ci6q36gqm6u3spknxzr7p5r2y4xw7n25d5icm7rsoq7lq6ka","signature":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"}
```

## Message catalog

The `message_type` value determines the exact payload schema. No other custom
message types exist in v0.

### `enrollment_admission_request` / `enrollment_admission_challenge`

The enrollment stream is a three-message exchange, not a single submission,
because it is the only stream open to unauthenticated peers and the only one
whose validation costs a 64 MiB memory-hard evaluation. The reasoning and the
normative bounds are in
[identity.md](identity.md#the-admission-shield-and-why-bounded-memory-is-not-enough);
only the formats are here.

```text
EnrollmentAdmissionRequest = {
  "public_key": base64url(32 bytes)
}
EnrollmentAdmissionChallenge = {
  "admission_nonce": base64url(16 bytes),
  "admission_difficulty_bits": u64-string,
  "expires_at_ms": u64-string
}
```

The validator generates `admission_nonce` from a cryptographically secure random
generator, binds it to the authenticated libp2p Peer ID and observed remote
address of the connection that asked for it, and accepts it exactly once before
`expires_at_ms`. A nonce MUST NOT be accepted on another connection, from
another public key, or by another validator. `admission_difficulty_bits` MAY be
`"0"`, and is under normal load.

This exchange is a resource shield, not an authorization step: satisfying it
proves nothing about identity and grants nothing.

### `enrollment_request` / `enrollment_response`

```text
EnrollmentSubmission = {
  "admission_nonce": base64url(16 bytes),
  "admission_solution": u64-string,
  "request": EnrollmentRequest
}
```

`enrollment_request` payload is an `EnrollmentSubmission` wrapping the
`EnrollmentRequest` defined in
[identity.md](identity.md#enrollment-request). The wrapper carries the shield
solution **outside** the signed object on purpose: `admission_nonce` is chosen
by the validator and differs per validator, so putting it inside
`EnrollmentRequest` would force one signature and one
`enrollment_request_hash` per validator and break the single-certificate model.
`admission_tag` is computed from the registry formula over
`request.public_key`, which binds the solution to the enrolling key.

Because the sender is not yet enrolled, its envelope `sender_node_id` and
envelope signature are checked against the request key, not a certificate.

```text
EnrollmentResponse = {
  "enrollment_request_hash": sha256-string,
  "status": "accepted" | "pending" | "rejected",
  "certificate": EnrollmentCertificate,   // required only when accepted
  "error_code": enum                      // required only when rejected
}
```

Allowed error codes are `invalid_request`, `invalid_admission`, `invalid_pow`,
`stale_parameters`, `duplicate_identity`, `rate_limited`, and
`internal_unavailable`. `invalid_admission` covers an unknown, expired, reused,
or wrongly bound `admission_nonce` and an `admission_tag` below the issued
difficulty; a requester that receives it MUST obtain a fresh challenge rather
than retrying the same submission. The response
hash MUST equal the registry `enrollment_request_hash` of the received request.
The response MUST NOT echo the proof or expose validation internals.

### `challenge_request`

```text
{
  "challenge_id": sha256-string,
  "kind": "availability" | "storage" | "compute",
  "issuer_node_id": string,
  "subject_node_id": string,
  "issued_at_ms": u64-string,
  "deadline_ms": u64-string,
  "randomness": base64url(32 bytes),
  "randomness_source": {
    "beacon_height": u64-string,
    "beacon_block_id": sha256-string,
    "commitment_epoch": u64-string
  },
  "issuer_commitment": sha256-string,
  "assignment": AvailabilityAssignment | StorageAssignment | ComputeAssignment,
  "issuer_signature": base64url(64 bytes)
}
```

```text
AvailabilityAssignment = {"response_bytes": u64-string}
StorageAssignment = {
  "object_id": sha256-string,
  "chunk_index": u64-string,
  "chunk_length": u64-string,
  "expected_root": sha256-string
}
ComputeAssignment = {
  "app_id": sha256-string,
  "module_hash": sha256-string,
  "input_hash": sha256-string,
  "input": base64url(bytes),
  "fuel_limit": u64-string
}
```

The `challenge_id` is `request_hash` from the hash registry over the JCS request
without `challenge_id` and without `issuer_signature`. The issuer signature uses
domain `coblox-challenge-request-v0`, the global chain binding, and payload
`raw_32_bytes(challenge_id)`, so the issuer is bound to the request itself and
not merely to the envelope that carried it. Only an assigned validator or
ledger-selected auditor may issue a challenge.

`randomness` MUST equal the `challenge_randomness` of the hash registry, derived
from the finalized beacon named by `randomness_source`, the issuer's committed
secret, and the subject. A subject-chosen nonce is invalid, and so is an
issuer-chosen one: the value is a function of published data plus one secret
that was committed on-chain beforehand, so anyone can recompute it once the
secret is revealed in the challenge evidence. `issuer_commitment` MUST match a
finalized `challenge_commitment` transaction for `(issuer_node_id,
commitment_epoch)` whose height is strictly below `beacon_height`.

The subject-to-issuer assignment is likewise derived, not chosen: the
`(issuer_node_id, subject_node_id)` pair MUST be one the epoch's assignment
function produces from the same finalized beacon over the eligible sets, and
every subject MUST be covered by at least two distinct issuers per epoch, none
of which is the subject. **That coverage rule is the mitigation of beacon
grinding, not a redundancy measure**: a proposer colluding with one issuer can
search the legal `timestamp_ms` values for a favourable beacon, but the second
independent issuer still queries the subject from an unground assignment, so the
attack degrades from passing the challenge to passing one of two. The cost of
that search is quantified in
[ledger.md](ledger.md#challenge-evidence). The assignment function is deterministic, recomputable
by any observer from finalized data, and specified with the challenge engine in
M-03; the fields it needs are fixed here so that adding it later is not a
breaking format change.

Verification of all of the above happens against the finalized evidence and is
specified in [ledger.md](ledger.md#challenge-evidence). A request whose
randomness a verifier cannot reproduce backs no reward.

Canonical availability example:

```json
{"assignment":{"response_bytes":"32"},"challenge_id":"sha256:3d56e5dd5104a2ad5c733fa4f0b6d8f35de2f68509e9c10a3d473128eaec0b21","deadline_ms":"1787654420000","issued_at_ms":"1787654415000","issuer_commitment":"sha256:19556b209c36de1940340bd3ada4a4c821fe70cde0fd3906af2b71f31445e4d5","issuer_node_id":"cblx1issuerfixture","issuer_signature":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA","kind":"availability","randomness":"AAECAwQFBgcICQoLDA0ODxAREhMUFRYXGBkaGxwdHh8","randomness_source":{"beacon_block_id":"sha256:7e0694f564afa2d047db4eb58f4f2b3d322d71db808f6bbf5313ee2d2a4a95af","beacon_height":"40","commitment_epoch":"17"},"subject_node_id":"cblx1ci6q36gqm6u3spknxzr7p5r2y4xw7n25d5icm7rsoq7lq6ka"}
```

### `challenge_response`

```text
{
  "challenge_id": sha256-string,
  "subject_node_id": string,
  "completed_at_ms": u64-string,
  "result": AvailabilityResult | StorageResult | ComputeResult,
  "subject_signature": base64url(64 bytes)
}
AvailabilityResult = {"kind":"availability", "response":base64url(bytes)}
StorageResult = {
  "kind":"storage", "chunk":base64url(bytes),
  "leaf_index":u64-string, "merkle_siblings":[sha256-string]
}
ComputeResult = {
  "kind":"compute", "output":base64url(bytes), "output_hash":sha256-string,
  "fuel_consumed":u64-string
}
```

The subject signature domain is `coblox-challenge-response-v0` and uses the
global chain binding. `response_hash` is the registry hash of the response with
`subject_signature` removed. Storage proof
verification reconstructs `expected_root`; compute verification hashes the
exact output and MUST reject fuel above the assignment limit. Responses after
the deadline remain auditable but are not reward-eligible.

Canonical response example:

```json
{"challenge_id":"sha256:3d56e5dd5104a2ad5c733fa4f0b6d8f35de2f68509e9c10a3d473128eaec0b21","completed_at_ms":"1787654416000","result":{"kind":"availability","response":"MzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzM"},"subject_node_id":"cblx1ci6q36gqm6u3spknxzr7p5r2y4xw7n25d5icm7rsoq7lq6ka","subject_signature":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"}
```

### `ledger_status_request` / `ledger_status_response`

```text
LedgerStatusRequest = {
  "finalized_height":u64-string,
  "want_validator_set":boolean,
  "document_hashes":[sha256-string]
}
LedgerStatusResponse = {
  "genesis_block_id":sha256-string,
  "finalized_header":BlockHeader,
  "quorum_certificate":QuorumCertificate,
  "validator_set":ValidatorSet,            // omitted unless requested/needed
  "protocol_documents":[SignedProtocolDocument]
}
```

`document_hashes` are unique, bytewise sorted, and capped at 32. The response
includes each requested active or historical signed document, capped at 2 MiB;
unknown hashes return `not_found`. This is the canonical retrievability path for
enrollment parameters, reward policies, hosting rate cards, and consensus
parameters committed by ledger hashes.

### `ledger_range_request` / `ledger_range_response`

```text
LedgerRangeRequest = {
  "from_height":u64-string, "max_blocks":u64-string,
  "expected_previous_block_id":sha256-string
}
LedgerRangeResponse = {
  "blocks":[Block], "more":boolean,
  "next_height":u64-string
}
```

`max_blocks` is 1–128. Blocks are contiguous, finalized, and capped by the 8 MiB
response limit. The requester validates every link and certificate before
advancing its trusted height; a response is never trusted because it arrived
over an authenticated peer connection.

Canonical range request example:

```json
{"expected_previous_block_id":"sha256:7e0694f564afa2d047db4eb58f4f2b3d322d71db808f6bbf5313ee2d2a4a95af","from_height":"42","max_blocks":"64"}
```

### `balance_proof_request` / `balance_proof_response`

```text
BalanceProofRequest = {
  "account_kind":"node"|"app", "subject_id":string,
  "at_height":u64-string
}
BalanceProofResponse = {
  "header":BlockHeader,
  "quorum_certificate":QuorumCertificate,
  "validator_set":ValidatorSet,
  "proof":AccountProof
}
```

Servers MUST return only finalized heights and MUST return a header whose
height equals `at_height` exactly; substituting an older or merely available
height is invalid. A client rejects any height below its persisted trusted
height, requires the tip/checkpoint freshness bounds from active consensus
parameters, and queries independently operated enrolled peers for corroboration.
Proof validation is defined step by
step in [ledger.md](ledger.md#light-client-balance-verification).

### The three consensus messages

Gossip topic: `consensus`. Three payloads — `block_proposal`, `prevote`,
`precommit` — carry the two-phase consensus protocol of [ADR-018]. They travel
on the `SignedEnvelope` above like every other message, so none of them repeats
`network_id`, `nonce`, `created_at_ms` or `expires_at_ms`; those are the
envelope's fields, and a payload that carried its own copy would give a receiver
two answers to one question.

Only a member of the active validator set may send any of the three. A receiver
establishes the sender from the envelope's `sender_node_id` and resolves it to a
`validator_id` in the set named by the height's `validator_set_hash`; a message
from a non-member is `unauthorized`.

#### Who proposes

The proposer of `(height, round)` is fixed by a rule and is not announced, so
every node computes it and no node is told it:

> Walk the active validator set in `validator_id` order, accumulating
> `voting_power`. The proposer is the member whose accumulated range contains
> `(height + round) mod total_voting_power`, with the sum and the modulus taken
> over an integer width that cannot wrap.

The index is `(height, round)` and **nothing else**. In particular it is not
derived from any value a participant supplies: [ADR-018] §3 requires this, and
gives the reason — the ledger has already established, in the revocation
analysis, that a participant able to grind an input it chooses will grind it, and
the prize here is the right to propose.

Two consecutive rounds at the same height therefore step one unit along the
power ladder. **At uniform voting power** — one unit each, which is the shape
an elected set is required to have — two consecutive rounds cannot name the
same member while an unvisited one remains, and that is what makes a height
survive a proposer that says nothing.

**At weighted voting power the ladder is walked in units and not in members, so
a member with voting power `w` is the proposer of `w` consecutive rounds of a
height** while a member nobody has reached yet waits. With powers
`1, 1, 1, 7` the heavy member proposes seven rounds in a row. The consequence
is on liveness and is stated rather than left to be discovered: a silent member
of power `w` costs a height `w` rounds instead of one, and since each round's
timeout grows with the round number, the wait grows quadratically in `w`.
Safety is untouched — this rule authorizes proposing, a proposal decides
nothing on its own, and every rule that can finalize a block counts signed
votes.

#### `block_proposal`

```text
{
  "height": u64-string,
  "round": u64-string,
  "valid_round": u64-string,        // omitted at the first proposal of a value
  "header": BlockHeader,
  "transactions": [Transaction]
}
```

A proposal is **not** a `Block`: it carries no `quorum_certificate`, because
nothing has certified it yet. That asymmetry is the protocol's, not this
message's — a `Block` carries its own certificate and is therefore always
already final.

`header.height` MUST equal `height`. `header.round` is the round at which this
value was **first proposed** and MUST NOT be rewritten when the value is
re-proposed in a later round: `block_id` covers the header, and a rewritten
header would be a different block, which would strand every prevote already cast
for the value and make a locked validator unable to vote for the only block it is
allowed to vote for.

That rule binds the proposer. The receiver's half of it is this, and it is
stated separately because it is what a receiver can actually check:

> **A receiver MUST reject a proposal that omits `valid_round` and whose
> `header.round` is not `round`.** A receiver MUST NOT apply that comparison
> when `valid_round` is present: a re-proposal carries the header of the round
> the value was first proposed at, so `header.round` is below `round` there, and
> a receiver that rejected it would refuse every re-proposal and stall every
> height that needs a second round.

Nothing further is required of `header.round` on a re-proposal, and the reason
is not an omission: `block_id` covers every byte of the header, and a receiver
acts on a re-proposal only once it has itself seen more than two thirds of
prevotes for that same `block_id` at `valid_round` — a quorum in its own log,
which the proposer cannot manufacture.

> **A receiver MUST reject a proposal whose `transactions` do not reproduce
> `header.transactions_root`**, recomputed as
> [ledger.md](ledger.md#hashing-primitives) defines it: `tx_id` over each
> transaction with `authorization` removed, then the transaction Merkle tree in
> block order.

The check needs no executor and no account state, so it is not part of the
executing validity of a block: it is the binding between the value the protocol
agrees on and the bytes the block publishes. Without it one proposer can send
one `header` to two honest receivers with two different `transactions` arrays;
both prevote and precommit the same `block_id`, both finalize, and the two
`Block` artifacts they publish differ — a divergence in the ledger produced by a
single participant well inside the fault budget, and one that no rule stated in
terms of `block_id` can see.

`valid_round` is present exactly when the proposer is re-proposing a value it
has seen more than two thirds of prevotes for at an earlier round; it MUST then
be strictly below `round`, and a receiver acts on the proposal only once it has
itself seen more than two thirds of prevotes for `block_id` at `valid_round`.
Its absence means the value is fresh.

A proposal carries no signature of its own. Its authenticity is the envelope's,
which binds `sender_node_id`. A consequence a reader should not have to derive:
a proposer that sends two different proposals in one round is **detectable** by
anyone who receives both and is **not attributable** from a payload in
isolation, so proposal equivocation does not become evidence the way a double
precommit does. Nothing decides on a proposal alone, so a forged or doubled
proposal costs a round and no more.

#### `prevote`

```text
{
  "height": u64-string,
  "round": u64-string,
  "block_id": sha256-string,
  "validator_id": string,
  "signature": base64url(64 bytes)
}
```

The prevote signature domain is `coblox-block-prevote-v0` and uses the global
chain binding. Each prevote signs exactly:

```text
"coblox-block-prevote-v0\0" || chain_id_32 || u64be(height)
|| u64be(round) || raw_32_bytes(block_id)
```

This is the same six fields, in the same order and the same widths, as the
finality vote of [ledger.md](ledger.md#what-validators-sign), under a different
domain separator. The repetition is deliberate: the two phases must be
impossible to confuse, and one separator is the whole of what distinguishes
them. More than two thirds of the voting power prevoting one `block_id` at a
round is what makes a validator **lock**, and a signature that could be
presented as either phase would let one message both lock a validator and count
towards finalizing a block.

#### `precommit`

```text
{
  "height": u64-string,
  "round": u64-string,
  "block_id": sha256-string,
  "validator_id": string,
  "signature": base64url(64 bytes)
}
```

The precommit signature domain is `coblox-block-vote-v0` — the finality vote
that [ledger.md](ledger.md#what-validators-sign) has always specified, unchanged
in every byte. A `QuorumCertificate` is the set of precommits over one
`(height, round, block_id)`, so a `precommit` payload's `validator_id` and
`signature` are exactly one entry of one, and a node assembling a certificate
copies them rather than re-deriving anything.

A validator MUST NOT sign two different precommits for the same
`(height, round)`. A receiver counts the **first** precommit it accepted from a
validator at a round and discards a later one for a different block, so a
validator's power reaches at most one `block_id` per round in any honest node's
tally, whether or not the equivocation is ever noticed.

There is **no nil vote** in either phase. A validator that will not vote for a
block sends nothing, and a round that produces no quorum ends on a timeout.

#### The consensus timeouts are local

Three durations govern how long a node waits at each step —
`propose_timeout_ms`, `prevote_timeout_ms`, `precommit_timeout_ms`, each growing
with the round number.

**They are local parameters of a node, and this document fixes neither their
values nor a band for them.** They are not carried by any signed document and
are not genesis constants, so no validity rule of this protocol can compare them:
a rule on them would have to compare a node's private setting to something, and
there is nothing on-chain to compare it to. Two nodes running different values
are both conformant, and a node whose values are too small for its network
wastes rounds rather than producing an invalid one. They are named here, with
that consequence, rather than left for a reader to infer from their absence.

### `block_announcement`

Gossip topic: `blocks`. Payload:

```text
{"block_id":sha256-string,"height":u64-string,"header":BlockHeader,
 "quorum_certificate":QuorumCertificate}
```

The announcement is a hint. Receivers validate it and fetch the full block via
ledger sync; gossip is not a finality mechanism.

### `challenge_evidence_announcement`

Gossip topic: `evidence`. Payload:

```text
{"evidence_tx_id":sha256-string,"challenge_id":sha256-string,
 "subject_node_id":string,"result":"passed"|"failed"|"late"}
```

The full evidence transaction is fetched from a finalized block. Announcements
do not independently create rewards or penalties.

## Error response

Any request/response stream may return:

```json
{"error_code":"invalid_request","message_id":"sha256:56d2aa0cd4c2ff0b06c47b478b6bfc2dff88b2c162c6cff1e33f9bf3284c7308","retry_after_ms":"0"}
```

Codes are `invalid_request`, `unauthorized`, `not_found`, `conflict`,
`rate_limited`, `unsupported_version`, `too_large`, and
`internal_unavailable`. Human text is intentionally absent from the wire.

## Gossip validation and backpressure
 
Nodes validate topic/network, canonical envelope, size, ID, signature,
certificate, expiry, replay cache, and payload schema before accepting gossip.
Block announcements additionally require a valid quorum certificate. Evidence
announcements require an enrolled validator sender but are treated as hints.
Consensus messages additionally require a sender that is a member of the active
validator set, a `prevote` or `precommit` signature that verifies under its own
domain against that member's `consensus_public_key`, and — for a
`block_proposal` — a sender that is the proposer of its `(height, round)`, a
`transactions` array that reproduces `header.transactions_root`, and, when
`valid_round` is absent, a `header.round` equal to `round`.
Unlike a block announcement, a consensus message is **not** a hint and MUST NOT
be shed as one.
Application objects MUST NOT use libp2p's anonymous author mode.

### Transport rotation, attribution, and rate limits

Because transport keys are decoupled from node identities ([ADR-015]), a node may
rotate its libp2p Peer ID without re-enrolling on the ledger. Protocol accounting
and backpressure interact with transport rotation as follows:

1. **Identity-bound attribution:** Gossip envelopes (`SignedEnvelope`) carry
   `sender_node_id` in the signed cleartext. Gossip authorization, spam scoring,
   and validator status checks bind to `sender_node_id`, never to ephemeral
   transport Peer IDs.
2. **Replay prevention:** The replay cache indexes `(sender_node_id, nonce)` pairs
   up to `replay_cache_entries_per_peer`. An attacker cannot bypass replay limits
   or flood duplicates by reconnecting under a freshly generated transport Peer ID.
3. **Queue lifecycle and rotation backpressure:** Transport disconnects tear down
   the per-connection transport queue. When a node reconnects under a new
   `TransportKeyAttestation`, a fresh transport queue is established, but
   node-level rate limiters and replay caches remain active. Nodes enforce a
   minimum rotation interval (rate limit on verifying new `TransportKeyAttestation`
   presentations per `node_id`) to bound the computational cost of session
   re-establishment.
4. **The enrollment stream is the exception, and it is named rather than left
   to exclusion.** Points 1 to 3 all anchor to `sender_node_id`, and the
   enrolling peer is by definition the one that has no verified `node_id`: its
   envelope and signature are checked against the key inside the request, not
   against a certificate, and it presents no `TransportKeyAttestation` at all.
   Transport rotation is therefore free on that stream, and nothing in this
   section limits it. What limits it is
   [identity.md](identity.md#the-admission-shield-and-why-bounded-memory-is-not-enough):
   the admission shield counts against the **observed remote address** — never
   against the libp2p Peer ID, which [ADR-015] made free — and the memory-hard
   stage admits at most one in-flight evaluation per enrolling public key.
   Neither anchor is reset by a `keygen`.

Per-peer queues are bounded. When full, nodes drop duplicate/low-priority hints
before finalized headers or direct responses. Enrollment, challenge request, and
ledger sync streams use timeouts and explicit concurrency limits; no untrusted
peer can cause an unbounded task, allocation, or retained response. The
enrollment stream is named first deliberately: it is the only one that accepts
unauthenticated transport peers, so it is the only one to which "no untrusted
peer" fully applies, and it is the most expensive to validate. Its specific
bounds — the per-key and global caps on the memory-hard stage, the shedding
queue, and the admission shield — are normative in
[identity.md](identity.md#validation-order-and-its-reason) and are not restated
here. An implementer building the transport layer from this document alone MUST
follow that link; implementing the limits only for the streams historically
listed here would leave the one stream that needs them unprotected.
