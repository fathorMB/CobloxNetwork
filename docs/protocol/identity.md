# Node identity and enrollment

This document defines the cryptographic identity lifecycle required before a
node can participate in authenticated Coblox protocols or receive ledger value.

## Key hierarchy

A Coblox node uses three distinct keys with clear hierarchical roles:

1. **Identity key.** Each node has one Ed25519 identity key pair. The private
   key MUST be generated with a cryptographically secure random generator,
   stored in the platform's protected credential facility where available, never
   transmitted, and never derived from a password. The public key is 32 bytes.
   This key defines the permanent node identity and derives `node_id`. It signs
   the node's enrollment request and in-session transport key attestations.

2. **Transport key.** The libp2p transport layer MUST use an Ed25519 key pair
   **distinct from the identity key**: the transport public key MUST NOT equal
   the node's enrolled identity public key, and an attestation that names the
   enrolled identity public key as its `transport_public_key` MUST be rejected
   (§[Transport key attestation](#transport-key-attestation)). This is a
   validity rule and not a recommendation, because the privacy property of
   [ADR-015] depends on it entirely: the enrollment certificate publishes the
   identity public key on the ledger, and the canonical Peer ID derivation of
   §[Canonical libp2p Peer ID](#canonical-libp2p-peer-id) is fully specified
   here, so a node that reuses one key for both roles hands every offline reader
   of the ledger its Peer ID for free, retroactively and without connecting to
   anything. The link would not be *published*, but it would be
   **recomputable**, which for TM-28 is the same thing. The rule is enforceable
   by the receiver, which holds both keys at verification time.

   The transport key derives the node's libp2p Peer ID and authenticates Noise
   or QUIC transport handshakes. It is subordinate to the identity key,
   rotatable, and bound to the enrolled identity via a signed
   `TransportKeyAttestation` presented in-session. The transport key and its
   binding are **never published on the ledger** ([ADR-015]).

3. **Validator consensus key.** A validator MUST use a distinct Ed25519 consensus
   key. That key is subordinate to, and bound by a proof of possession from, its
   enrolled identity key; it is not a second enrolled identity. The binding and
   mandatory verification rules are specified in
   [ledger.md](ledger.md#validator-set-continuity). The consensus key is published
   by the node itself, ahead of the election epoch it is bound to, through the
   `validator_candidacy` transaction of
   [ledger.md](ledger.md#candidacy-is-an-explicit-per-epoch-act): an enrolled
   identity is never conscripted into the validator set, and a quorum cannot
   assert a consensus key on someone else's behalf.

## Canonical libp2p Peer ID

Parsers MUST accept both legal libp2p textual forms (legacy base58btc
multihash and CIDv1 base32) when parsing peer identifiers from connection
metadata, decode them, and compare the resulting multihash. Signed on-chain
objects no longer embed `libp2p_peer_id`.

For an Ed25519 transport public key, the embedded libp2p `PublicKey` protobuf
MUST be deterministic: varints are minimal, fields appear in ascending tag order,
all required fields appear exactly once, and unknown/duplicate fields are
rejected. The identity multihash is computed from those exact canonical protobuf
bytes.

Conformance fixture: the **transport** public key
`n0lDnp2wlbxBEe0l01eV2DG8VaBH9LHX9q7jd3u0EiA` — the same key the `TKA-0`
attestation of §[Transport key attestation](#transport-key-attestation) carries
— has canonical protobuf hex
`080112209f49439e9db095bc4111ed25d35795d831bc55a047f4b1d7f6aee3777bb41220`
and Peer ID
`12D3KooWLY9nerKo6xGVcRVjDRdqLh7oMgz3tJk61oSgCo5kKWmM`. The equivalent CID
`bafzaajaiaejcbh2jiopj3mevxrard3jf2nlzlwbrxrk2ar7uwhl7nlxdo553iera` parses to
the same multihash for connection comparison.

The fixture is deliberately built on a transport key and not on the identity
fixture key of the enrollment request. This section publishes a complete,
executable derivation; the enrollment certificate publishes an identity key on
the ledger. Demonstrating the one on the other would hand every offline reader
of the ledger a worked example of the very correlation [ADR-015] removes — for
the fixture's own node, and by imitation for every implementation that copies
it.

## Node identifier

`node_id` is derived as specified in [README.md](README.md#identifiers-and-cryptographic-conventions).
It is stable for the life of the key, case-sensitive, and contains no account or
device metadata. The 256-bit digest makes collisions cryptographically
negligible. A node MUST reject a certificate whose public key does not derive
the certificate's `node_id`.

Example public key (base64url):

```text
L_o1qZ06PPuxe7fB3FVhsYqNzKTfONxhPqhZw36xM2s
```

Example identifier shape:

```text
cblx176fmuouuc5v2xyqqxgef5uwrdqt53yqazdlxwcfl6a63bxarnuyq
```

The identifier above is illustrative; conformance fixtures MUST recompute the
identifier from the fixture key rather than treating prose as a trust anchor.

## Signed object procedure

For a schema with domain `D`, the signer:

1. removes the `signature` field and any fields the schema marks unsigned;
2. validates and JCS-serializes the resulting object;
3. signs the chain-bound preimage defined by the global signature procedure in
   [README.md](README.md#identifiers-and-cryptographic-conventions);
4. inserts the unpadded base64url signature and re-canonicalizes for transport.

The verifier reverses those steps, verifies field lengths and canonical form,
derives the signer's identifier from the embedded or trusted public key, and
only then performs replay and authorization checks. A valid signature proves
control of a key; it does not by itself prove enrollment or authorization.

## Enrollment request

Domain: `coblox-enrollment-request-v0`.

```text
EnrollmentRequest = {
  "schema_version": "0.1",
  "network_id": string,
  "node_id": string,
  "public_key": base64url(32 bytes),
  "pow": {
    "algorithm": "argon2id-leading-zero-bits-v0",
    "difficulty_bits": u64-string,
    "memory_kib": u64-string,
    "iterations": u64-string,
    "lanes": u64-string,
    "nonce": u64-string,
    "parameter_set_hash": sha256-string,
    "recent_block_height": u64-string,
    "recent_block_id": sha256-string
  },
  "created_at_ms": u64-string,
  "signature": base64url(64 bytes)
}
```

Canonical serialized example (signature and IDs are fixture values):

```json
{"created_at_ms":"1787654400000","network_id":"coblox-devnet-0","node_id":"cblx176fmuouuc5v2xyqqxgef5uwrdqt53yqazdlxwcfl6a63bxarnuyq","pow":{"algorithm":"argon2id-leading-zero-bits-v0","difficulty_bits":"4","iterations":"3","lanes":"4","memory_kib":"65536","nonce":"11","parameter_set_hash":"sha256:7df04d03b60f62852f0d76c847d0181a2b17b43a50f987c0b9f814e70f064bcc","recent_block_height":"41","recent_block_id":"sha256:7e0694f564afa2d047db4eb58f4f2b3d322d71db808f6bbf5313ee2d2a4a95af"},"public_key":"L_o1qZ06PPuxe7fB3FVhsYqNzKTfONxhPqhZw36xM2s","schema_version":"0.1","signature":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"}
```

Fixture strings are structurally valid but are not production credentials. Test
suites MUST generate a key, proof, and signature and include negative vectors.

## One-time anti-Sybil proof of work

The primitive is **Argon2id** ([RFC 9106](https://www.rfc-editor.org/rfc/rfc9106.html)),
version `0x13`, not SHA-256. The choice and its consequences are recorded in
[ADR-007]. SHA-256 is compute-bound, tiny-state, and perfectly parallel, so the
cost ratio between a phone and a commodity GPU is roughly three orders of
magnitude; no difficulty is then simultaneously tolerable on Android and
expensive for an attacker. Argon2id moves the per-attempt cost to memory
capacity, which an attacker cannot buy at that ratio.

The active signed parameter set supplies `difficulty_bits`, the Argon2id cost
profile (`memory_kib`, `iterations`, `lanes`, `tag_length_bytes`), and its own
`parameter_set_hash`. The request echoes `difficulty_bits`, `memory_kib`,
`iterations`, and `lanes`; each MUST exactly equal the active parameter set, so
a validator can reject a mismatched cost before allocating any memory.

That cost profile is not free to choose. The RFC 9106 parameter ranges are the
domain of the function and are **not** a security floor: read as one, they would
let a signed, fully conformant parameter set drop `memory_kib` to 8 KiB and
revoke the memory-hard floor that [ADR-007] rests on, leaving no trace on chain.
v0 therefore enforces a cost floor as a **validity rule on document acceptance**,
specified once in
[README.md](README.md#the-enrollment-cost-floor-is-a-validity-rule-not-a-recommendation).
`memory_kib` and `iterations` are security parameters, not performance
parameters.

`difficulty_bits` is in the inclusive range **2–6**. That range is normative and
replaces the 18–40 range of the SHA-256 design, which was a shape built around a
cheap primitive and is meaningless here: with a memory-hard function the cost
lives in the single evaluation, so the expected number of evaluations must be
small. A production network MUST choose the pair (cost profile, difficulty) by
benchmark and security review against a **declared reference device**, and MUST
publish both the reference device and the measured onboarding time; a difficulty
expressed without the device it was measured on is not a meaningful bound.
`4` in the example is not a mainnet default.

For nonce `n`, compute:

```text
pow_password = "coblox-enrollment-pow-v0\0"
             || chain_id_32
             || public_key_32
             || parameter_set_hash_32
             || u64be(recent_block_height)
             || raw_32_bytes(recent_block_id)
             || u64be(n)
pow_salt     = enrollment_pow_salt   // 16 bytes, see the hash preimage registry
pow_tag      = Argon2id(version = 0x13,
                        password = pow_password,
                        salt = pow_salt,
                        secret = empty,
                        associated_data = empty,
                        m = memory_kib, t = iterations, p = lanes,
                        tag_length = tag_length_bytes)
```

The salt is derived deterministically rather than chosen, because verification
must reproduce the evaluation exactly; it varies per identity and per recent
block, so two identities never share a memory-filling schedule. The nonce enters
the password, not the salt.

The proof is valid when the first `difficulty_bits` of `pow_tag`, read most
significant bit first, are zero. Nonce search begins at a random `u64` to avoid
identical search paths; wraparound is allowed exactly once. Expected work is
`2^difficulty_bits` Argon2id evaluations and success probability after `h`
attempts is `1 - (1 - 2^-difficulty_bits)^h`.

### Validation order and its reason

With a memory-hard function **verifying costs what generating costs**. A single
Argon2id evaluation at the recommended profile occupies 64 MiB and hundreds of
milliseconds, so an unordered validator would hand any anonymous peer a
denial-of-service primitive. The proof is therefore checked **last**. Validators
accept an enrollment only when all of these hold, evaluated strictly in this
order, and abandon the request at the first failure:

1. connection-level admission and rate limits are satisfied;
2. size limits, canonical JCS form, and schema are valid;
3. `network_id` matches and `created_at_ms` is within the parameterized
   enrollment window (`max_request_age_ms`, `max_future_skew_ms`);
4. the signature verifies under the [consensus-critical Ed25519 rule](README.md#consensus-critical-ed25519-verification),
   and the public key derives `node_id`;
5. neither `node_id` nor public key is already enrolled or revoked, and no
   pending request with the same key is being processed;
6. the referenced parameter set is active at the referenced height, and the
   echoed difficulty and cost profile exactly equal it;
7. `recent_block_id` is the locally finalized canonical block at
   `recent_block_height`, and that height is no more than the active parameter
   `recent_block_window` behind the validator's latest finalized height;
8. the **admission shield** below is satisfied: the submitted `admission_nonce`
   is one this validator issued to this transport peer, is unexpired and unused,
   and `admission_tag` meets the difficulty that was issued with it;
9. **last**, and only if every check above passed, the Argon2id tag is computed
   and MUST meet the target.

Ordering alone is necessary but **not sufficient**, and the specification says so
rather than assuming it: an attacker who holds a key can produce a request that
passes the cheap checks and fails only at the memory-hard step, so each such
request still costs a validator one full evaluation. Validators therefore MUST
bound the memory-hard stage itself:

- at most one in-flight step-9 evaluation per public key;
- a declared maximum number of concurrent step-9 evaluations, whose product with
  `memory_kib` is the enrollment subsystem's declared peak memory budget;
- a bounded admission queue that sheds with `rate_limited` when saturated,
  never by accepting unverified requests and never by unbounded queueing;
- a failed step 9 counted against the source connection for rate limiting.

These bounds are local operational policy, not consensus, and are deliberately
not signed network parameters: they govern a validator's own resources and must
be tunable per deployment. What is normative is that a bound exists, is
declared, and fails closed.

### The admission shield, and why bounded memory is not enough

Bounding the memory-hard stage converts an exhaustion attack into a **starvation**
attack, and v0 states the conversion rather than presenting the bounds as a
solution. Signing a syntactically perfect request costs an attacker roughly
20 µs on one core; refusing it at step 9 costs a validator a 64 MiB slot for
hundreds of milliseconds. The asymmetry is of order **10⁴:1**. A validator with
a 4 GiB peak budget sustains a few dozen evaluations per second and is saturated
by a single attacking core with three orders of magnitude to spare. Because an
enrollment certificate needs a quorum, an attacker does not have to reach every
validator: saturating a little more than **one third of the voting power** — on
a 100-member set, about 34 cores — means no request ever reaches the threshold,
and onboarding stops network-wide at negligible cost, without the attacker
holding a single token or enrolled identity.

The shield therefore sits between step 7 and step 9, and it has two parts. Both
are required; neither is sufficient alone.

**Part 1 — a bound source.** A validator issues an `admission_nonce` only over
an established, authenticated transport connection, and the nonce is bound to
that libp2p Peer ID and the observed remote address, single-use, and short-lived
(seconds, not minutes). It is not precomputable, not transferable between
validators, and not reusable.

Single-use bounds **reuse** of a nonce; it does not bound the **number** of
nonces, and those are different quantities. A validator that answers every
issuance request from one address hands that address as many concurrent slots
as it asks for, and Part 1 then costs an attacker one round trip rather than an
address. Two further requirements therefore close the volume, and without them
the rest of this section does not hold:

- **issuance is counted against the step-1 per-source rate limit**, on the same
  terms as the failed step 9 already is, so a nonce request is not a free
  operation that bypasses the limit the ordering exists to apply;
- **a validator declares a cap `k` on the un-consumed, unexpired nonces
  outstanding for one source**, and refuses issuance with `rate_limited` above
  it, never by queueing.

**"Source" means the observed remote address, normatively, and never the libp2p
Peer ID.** Both are available at that point of the exchange and the distinction
used to be harmless: before [ADR-015] a Peer ID was derived from the identity
key, so a fresh one cost an enrollment. After [ADR-015] a Peer ID costs one
`keygen` — it is unenrolled, unlimited and rotatable by design — so a limit
anchored to it is a limit an attacker resets for free, and that applies equally
to `k`, to the step-1 per-source rate limit, and to the count of failed step-9
evaluations. The cost argument two paragraphs below is only true under this
reading: an attacker pays a distinct **reachable address** for every `k`
concurrent slots. The per-key constraint stated in the validation order — at
most one in-flight step-9 evaluation per public key — is anchored to the
enrolling key and is unaffected either way.

With those in place the honest requester pays one round trip, and an attacker
pays a distinct **reachable** address for every `k` concurrent slots it wants
to hold — `k` being the validator's declared cap, not one, and not unbounded.
That address cost is the part of the attack that does not scale with CPU, and
it is proportional to the concurrency the attacker wants rather than equal to
it. `k` is local operational policy, like the other bounds in this section:
what is normative is that it exists, is declared, and fails closed.

**Part 2 — a constant-verification puzzle.** With the nonce, the validator
issues `admission_difficulty_bits`. The requester searches `admission_solution`
until `admission_tag` — defined in the
[hash preimage registry](README.md#hash-preimage-registry) — has that many
leading zero bits, most significant bit first. Verification is **one SHA-256**.
The primitive is deliberately SHA-256 and not Argon2id, and this must not be
"corrected" later: a memory-hard function costs the verifier what it costs the
producer, which is exactly the property that makes it useless as a shield, and
the hardware advantage that disqualifies SHA-256 for anti-Sybil work is harmless
when the defender's cost is constant.

**The difficulty is adaptive, and this is not a detail.** A fixed difficulty
large enough to blunt a GPU is not affordable on the devices this network exists
for. Sizing it against an attacker at ~10¹⁰ H/s and a validator capacity of tens
of evaluations per second lands near 2^28 attempts, which the declared reference
device would spend tens of seconds on — more than the enrollment proof of work it
is meant to protect, and paid once **per validator** the requester must reach for
quorum. Validators therefore MUST set `admission_difficulty_bits` as a function
of observed saturation of the step-9 stage:

- **zero** while the memory-hard queue is below its declared threshold, so that
  ordinary onboarding pays nothing beyond the round trip of Part 1;
- rising only under saturation, and only as far as a declared maximum;
- that maximum MUST NOT exceed the difficulty whose expected solution time on
  the **declared reference device** exceeds the time that same device spends on
  the enrollment proof of work itself. A shield that costs the honest phone more
  than the thing it shields has replaced the attack, not stopped it.

**Declared limit — availability of enrollment is not a protocol guarantee.**
This is stated with the same plainness as the Sybil limits below, because it is
the same kind of honesty. Under sustained attack an honest requester pays a real
puzzle, per validator, and slow devices are the ones that suffer; the shield
converts a permanent, cost-free shutdown into a degradation whose cost the
attacker also pays and cannot amortize across validators. It does not make
enrollment always available. The bounds and the difficulty schedule are local
operational policy and are **not** signed network parameters, so a deployment's
onboarding availability depends on choices no certificate attests. The
cryptographic guarantees of this document do not extend to it.

Conformance fixture `ADM-1`: a burst of validly signed enrollment requests
carrying no proof of work MUST leave bounded **both** the validator's enrollment
memory **and** the admission latency of concurrent honest requests. A test that
measures only memory does not exercise this section.

### Declared limits of this mechanism

The proof cannot be copied to another identity, chain, parameter set, or recent
finalized window because all are hashed. Creating N key pairs is cheap, but
registering them is not free: every distinct key requires a fresh independent
proof, so expected cost grows linearly to `N × 2^difficulty_bits` evaluations.
Parallel hardware can pay that cost faster; proof of work is a cost, not proof
of personhood. Admission rate limits and validator monitoring are defense in
depth and MUST NOT be counted as the cryptographic Sybil guarantee.

Three further limits are stated plainly, because a security specification that
omits them is dishonest rather than merely incomplete:

1. **The protocol does not distinguish `N` emulated nodes on one host from `N`
   real devices.** No field, proof, or challenge in Coblox v0 attests that a key
   corresponds to a distinct physical machine or person. The availability
   challenge proves that *a key is online*, not that *a device exists*: a single
   process holding `N` keys answers for all of them at negligible marginal cost.
   Hardware attestation is excluded in v0 by [ADR-007]. This is a deliberate,
   permanent property of v0 and MUST NOT be described as a temporary gap.
2. **A one-time cost cannot price a perpetual flow.** Enrollment is paid once
   while existence income accrues every epoch, so any entry proof is amortized in
   finite time regardless of its difficulty. Argon2id raises the floor by about
   two orders of magnitude over SHA-256 — an advantage that holds only because
   the cost floor is a validity rule and not a recommendation, and that a
   governed parameter set could otherwise have removed entirely — and it does not
   change this.
3. **Sybil containment in Coblox is therefore economic, not cryptographic.** It
   rests on the per-epoch existence fund being capped and shared rather than paid
   per node — specified in
   [ledger.md](ledger.md#existence-income-is-a-share-of-a-capped-fund) — and on
   validator eligibility being anchored to demonstrated storage and compute
   work, specified in
   [ledger.md](ledger.md#eligibility-demonstrated-storage-and-compute-never-availability).
   That anchoring makes eligibility **expensive to fake, not impossible to
   fake**, and the earlier wording here said "cannot be faked without spending
   real resources", which was stronger than the protocol delivers. The price of
   faking it is the enrollment of at least `validator_eligibility_min_issuers`
   colluding identities per fabricated candidate, plus the beacon grinding that
   [ledger.md](ledger.md#challenge-evidence) already quantifies; the eligibility
   section states that residual with its origin. The network is robust against
   forgery — balances, signatures, double spending
   — while not being Sybil-resistant by cryptographic means, and those two
   claims must be stated together.

## Enrollment certificate

An accepted request produces a certificate committed to the ledger. Domain for
each validator signature: `coblox-enrollment-certificate-v0`.

```text
EnrollmentCertificate = {
  "schema_version": "0.1",
  "network_id": string,
  "node_id": string,
  "public_key": base64url(32 bytes),
  "enrollment_request_hash": sha256-string,
  "issued_at_ms": u64-string,
  "valid_from_height": u64-string,
  "validator_set_hash": sha256-string,
  "signatures": [{"validator_id": string, "signature": base64url(64 bytes)}]
}
```

Signatures are sorted by `validator_id`, unique, and cover the object with
`signatures` removed using the global chain-bound procedure. The certificate is
valid exactly when `signed_power * 3 > total_power * 2`, evaluated with checked
`u128` intermediates. The complete validator set is resolved by hash as defined in
[ledger.md](ledger.md#validator-set-continuity).

Canonical serialized example:

```json
{"enrollment_request_hash":"sha256:44dc2df246a89f42d9a9da10f621c86f5141b597b1a6f08cc78b5e61a8388eb1","issued_at_ms":"1787654405000","network_id":"coblox-devnet-0","node_id":"cblx176fmuouuc5v2xyqqxgef5uwrdqt53yqazdlxwcfl6a63bxarnuyq","public_key":"L_o1qZ06PPuxe7fB3FVhsYqNzKTfONxhPqhZw36xM2s","schema_version":"0.1","signatures":[{"signature":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA","validator_id":"val-001"}],"valid_from_height":"42","validator_set_hash":"sha256:1df0a6454faaa5985b7f98c48d3c60d2ed62d5b3b24fe8e97d3dca1dd36f1120"}
```

## Transport key attestation

To bind an ephemeral or rotated transport key to an enrolled identity without
publishing the link on the ledger ([ADR-015]), a node produces a signed
`TransportKeyAttestation` and presents it in-session during peer connection
setup.

Domain: `coblox-transport-key-attestation-v0`.

```text
TransportKeyAttestation = {
  "schema_version": "0.1",
  "network_id": string,
  "node_id": string,
  "transport_public_key": base64url(32 bytes),
  "created_at_ms": u64-string,
  "expires_at_ms": u64-string,
  "signature": base64url(64 bytes)
}
```

Canonical serialized example:

```json
{"created_at_ms":"1787654400000","expires_at_ms":"1787654460000","network_id":"coblox-devnet-0","node_id":"cblx176fmuouuc5v2xyqqxgef5uwrdqt53yqazdlxwcfl6a63bxarnuyq","schema_version":"0.1","signature":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA","transport_public_key":"n0lDnp2wlbxBEe0l01eV2DG8VaBH9LHX9q7jd3u0EiA"}
```

The `transport_public_key` of this fixture is **deliberately not** the identity
fixture key `L_o1qZ06PPuxe7fB3FVhsYqNzKTfONxhPqhZw36xM2s` that the enrollment
request and the enrollment certificate of this document carry. The two keys are
the same key in no conformant deployment, and a fixture that spelled them the
same would teach, inside the normative document, the one configuration that
annuls [ADR-015].

The signature is generated by the node's identity key covering the global
chain-bound procedure over `coblox-transport-key-attestation-v0` and JCS of the
attestation with `signature` removed.

### Mandatory rejection rules

A receiver MUST reject an attestation, and disconnect the peer presenting it,
when any of the following holds. Each rule is stated as a rejection because it
is one, and each is decidable by the receiver alone from the certificate, the
attestation, the authenticated connection, and the active signed
`consensus_parameters` document.

1. `transport_public_key` **equals the enrolled identity public key** of the
   certificate. See §[Key hierarchy](#key-hierarchy) for why this is a validity
   rule.
2. `transport_public_key` is not the key authenticated by the Noise or QUIC
   handshake of this connection.
3. `expires_at_ms < created_at_ms`.
4. `expires_at_ms - created_at_ms > max_transport_attestation_validity_ms`, read
   from the active signed `consensus_parameters` document.
5. `now_ms > expires_at_ms`, or
   `created_at_ms > now_ms + max_transport_attestation_future_skew_ms`, both
   read as defined in §[Bounded validity in time](#bounded-validity-in-time).
6. `network_id` is not the local network, or `node_id` is not the `node_id`
   derived from the certificate's public key, or the signature does not verify
   under that public key with domain `coblox-transport-key-attestation-v0`.

### Bounded validity in time

The attestation validity is bounded by `created_at_ms` and `expires_at_ms`.
Using timestamps rather than block heights is a deliberate architectural
choice:

1. **Decoupled from ledger sync:** Transport connections are established at the
   network layer to synchronize ledger data. Anchoring attestation validity to
   chain heights would create a circular dependency where a node cannot establish
   a transport connection to fetch blocks without already knowing the current
   finalized height.
2. **Ephemeral key lifecycle:** Transport keys are rotatable session credentials.
   If a transport private key is compromised or leaked, its attestation expires
   automatically without requiring an on-chain `RevokeIdentity` transaction, which
   would destroy the node's permanent identity, balance, and reputation.

   **This is a transfer of risk and not a net gain, and the trade is stated
   here rather than left to be inferred.** For the whole validity window, a
   holder of the transport private key *is* the node towards every peer that
   opens a direct connection: it completes the handshake, replays the
   attestation — which is neither secret nor bound to a recipient — and is
   accepted as the victim. What it cannot do is the solid half of the design
   and is equally part of the trade: application objects stay signed by the
   **identity** key, so it can forge no `SignedEnvelope` and produce no valid
   `subject_signature` for a `challenge_request`. What it can do is occupy the
   victim's place in direct connections and stay silent, letting challenges
   expire into `failed` or `late` evidence at the victim's economic expense.
   Before [ADR-015] the same result required the identity key: total
   compromise, but **revocable**. There is no early invalidation of an
   attestation already in circulation — no epoch counter, no serial number, no
   list — so the only bound on the exposure is the length of the window, which
   is why rule 3 is a rule and not an example. The threat model carries the
   scenario as TM-37.
3. **Bounded exposure window:** Nodes MUST reject attestations where
   `expires_at_ms < created_at_ms`, and MUST reject attestations where
   `expires_at_ms - created_at_ms` exceeds
   **`max_transport_attestation_validity_ms`** of the active signed
   `consensus_parameters` document
   ([README.md](README.md#signed-protocol-documents)). The cap is a signed
   network parameter and not local policy, for the reason
   [ADR-010] states generally: a bound whose value each operator picks is a
   preference, and the property here — that a leaked transport key stops being
   usable on its own — depends on the magnitude, not merely on the ordering of
   the two timestamps. This is the same closure `max_envelope_validity_ms`
   already gives the wire envelope, which carries the same pair of fields.
4. **Declared clock tolerance, in one direction only.** Reason 1 says the node
   that most needs an attestation to verify is the one that cannot yet reach
   the ledger — freshly installed, long offline — and that node is also the one
   whose clock is least trustworthy. A bare comparison against the local clock
   would therefore isolate it permanently: a clock a few seconds slow rejects
   **every** freshly issued attestation, and because an attestation is required
   on every stream except enrollment, the node loses `ledger-sync`, which is
   the only source from which it could correct its notion of time. Receivers
   MUST therefore accept an attestation whose `created_at_ms` lies at most
   `max_transport_attestation_future_skew_ms` ahead of the local clock, on the
   model of the `max_future_skew_ms` the enrollment window already uses. No
   tolerance is granted past `expires_at_ms`: the two directions are not
   symmetric, because slack there extends the exposure window that rule 3
   exists to bound. As for the wire envelope, a detected clock rollback fails
   closed for protected protocols
   ([wire.md](wire.md#signed-envelope)).

   **Declared limit.** A receiver whose clock is far *behind* accepts
   attestations that expired hours ago, and no certificate attests a clock.
   This is stated with the same plainness as the declared limit on the
   availability of enrollment: the guarantees of this section are relative to
   the receiver's clock, and a deployment whose clocks are unmanaged gets a
   weaker exposure bound than the parameter names.

### Anti-reuse property

An eavesdropper or malicious peer that intercepts a valid `TransportKeyAttestation`
**cannot reuse it**.

The argument is structural: presenting a `TransportKeyAttestation` authorizes
only the specific `transport_public_key` named in the attestation. Before
exchanging application protocols, the remote peer must complete the Noise or
QUIC transport handshake, which cryptographically proves possession of the
private key corresponding to `transport_public_key`. An attacker possessing only
the intercepted attestation lacks the private transport key and will fail the
underlying transport handshake.

**The scope of that argument, stated so it is not read wider than it is.** It
covers the third party that intercepts an attestation, which is the case the
network exposes routinely. It does **not** cover an attacker that holds the
transport private key itself, because there the handshake proves exactly what
the attacker has; that case is the risk transfer written in
§[Bounded validity in time](#bounded-validity-in-time) point 2 and is bounded
only by the length of the window.

## Authentication on a connection

Noise or QUIC authenticates the libp2p transport key. Before a peer may publish
Coblox gossip or open protected streams, the receiver MUST obtain its
certificate, obtain its `TransportKeyAttestation`, verify both against a
finalized ledger state and local clock, and confirm that:

- the certificate's public key derives the certificate's and attestation's `node_id`;
- `attestation.transport_public_key` **differs from the certificate's public
  key**, under the rule of §[Key hierarchy](#key-hierarchy);
- `attestation.transport_public_key` derives the authenticated libp2p Peer ID of
  the connection under the canonical derivation rule;
- the attestation signature verifies under the certificate's identity public key
  with domain `coblox-transport-key-attestation-v0`;
- `created_at_ms <= expires_at_ms`,
  `expires_at_ms - created_at_ms <= max_transport_attestation_validity_ms`, and
  `now_ms <= expires_at_ms` with
  `created_at_ms <= now_ms + max_transport_attestation_future_skew_ms`;
- `valid_from_height` of the certificate is finalized and no revocation exists at
  the receiver's finalized height;
- `network_id` in both the certificate and the attestation equals the local network.

**Mandatory validity rule:** A peer presenting a transport key lacking a valid,
verified attestation MUST be rejected and disconnected. Unenrolled peers or peers
without valid attestations MAY use only the enrollment stream and native libp2p
connectivity protocols, subject to rate limits. The complete list of rejection
conditions is
§[Mandatory rejection rules](#mandatory-rejection-rules).

**Revocation applies to live connections, not only to new ones.** When the
finalized revocation set changes, a receiver MUST re-evaluate the established
connections it holds against it and close those whose peer is now revoked. The
check above is specified at connection setup, and setup is not enough on its
own: without this rule a session opened one block before a revocation is
finalized outlives it for as long as the peer keeps it open. The gap predates
[ADR-015] and is written here because that decision moves the whole
verification into the session and so makes it load-bearing.

## Revocation and key replacement

A `RevokeIdentity` governance transaction names the node ID, reason code,
effective height, and replacement node ID if any. It requires a validator quorum
certificate. Revocation is not retroactive: historical signatures remain valid
at heights before the effective height.

If the revoked node holds a seat in the active validator set, revocation alone
would leave its voting power intact until some later set transition that no rule
requires. It therefore **forces** a set transition. A full node sees the
`revoke_identity` transaction and enforces that rule completely. A light client
sees no transactions, so its position is narrower and is stated here exactly as
the ledger states it: it observes *a* transition if one happens, and can check
that the transition only removes members, but it **cannot establish that a
transition was due**. For the part that is covered it relies on the
`revoked_validators` list carried by its weak subjectivity checkpoint, which
closes the gap only for revocations known when that checkpoint was issued. The
binding rule, that closure and its declared limit are in
[ledger.md](ledger.md#revocation-forces-a-validator-set-transition).

Key replacement is a fresh enrollment
and proof of work; it never inherits the old balance or nonce automatically.
Account recovery and any authorized balance migration require a future explicit
ledger transaction and are not present in v0.

## Failure handling

Expired parameter sets, stale timestamps, malformed base64url, non-canonical
JSON, invalid proof, invalid signature, duplicate enrollment, and revoked keys
are hard failures. Responders return only a stable error code over the wire and
MUST NOT reveal key-store or validator-internal details.

## DRAFT: launch difficulty policy

Open alternatives are (a) a fixed `difficulty_bits` and cost profile for an
entire protocol epoch, simple and predictable, or (b) bounded adjustment at
epoch boundaries using observed enrollment rate, more adaptive but manipulable.
The launch values of `memory_kib`, `iterations`, and `lanes` are open **only
above the cost floor**, and must be chosen together with `difficulty_bits`,
against the declared reference device. Governance may raise the floor; it cannot
lower it. AGENT-007 owns the security recommendation and the Project
Lead/AGENT-002 own the signed parameter governance. The admission shield's
saturation threshold and maximum difficulty are per-deployment operational
values, not governance values, and are open in the same sense.

The algorithm, the salt and password construction, the verification rules, the
mandatory validation order with its resource bounds, the cost floor and its
status as a validity rule, the admission shield and its adaptive-difficulty
requirement, the 2–6 difficulty safety bounds, the per-identity linear cost, and
the declared limits above are **not** draft.
