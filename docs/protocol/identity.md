# Node identity and enrollment

This document defines the cryptographic identity lifecycle required before a
node can participate in authenticated Coblox protocols or receive ledger value.

## Key hierarchy

Each node has one Ed25519 identity key pair. The private key MUST be generated
with a cryptographically secure random generator, stored in the platform's
protected credential facility where available, never transmitted, and never
derived from a password. The public key is 32 bytes.

The same key is imported into libp2p. Consequently the libp2p Peer ID and the
Coblox `node_id` are independently derived identifiers for the same public key.
Implementations MUST verify both derivations when accepting an enrollment
certificate; a peer cannot substitute a transport identity after enrollment.

A validator MUST use a distinct Ed25519 consensus key. That key is subordinate
to, and bound by a proof of possession from, its enrolled identity key; it is
not a second enrolled identity. The binding and mandatory verification rules
are specified in [ledger.md](ledger.md#validator-set-continuity).

## Canonical libp2p Peer ID

Parsers MUST accept both legal libp2p textual forms (legacy base58btc
multihash and CIDv1 base32), decode them, and compare the resulting multihash.
Inside every Coblox signed object, however, `libp2p_peer_id` MUST be the legacy
base58btc multihash form; accepting a CID spelling and reserializing it before
signature verification is forbidden.

For an Ed25519 public key, the embedded libp2p `PublicKey` protobuf MUST be
deterministic: varints are minimal, fields appear in ascending tag order, all
required fields appear exactly once, and unknown/duplicate fields are rejected.
The identity multihash is computed from those exact canonical protobuf bytes.

Conformance fixture: public key
`L_o1qZ06PPuxe7fB3FVhsYqNzKTfONxhPqhZw36xM2s` has canonical protobuf hex
`080112202ffa35a99d3a3cfbb17bb7c1dc5561b18a8dcca4df38dc613ea859c37eb1336b`
and signed-object Peer ID
`12D3KooWD3eckifWpRn9wQpMG9R9hX3sD158z7EqHWmweQAJU5SA`. The equivalent CID
`bafzaajaiaejcal72gwuz2or47oyxxn6b3rkwdmmkrxgkjxzy3rqt5kczyn7lcm3l` parses to
the same multihash for connection comparison but MUST be rejected as a
non-canonical value in a signed Coblox object.

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
  "libp2p_peer_id": string,
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
{"created_at_ms":"1787654400000","libp2p_peer_id":"12D3KooWD3eckifWpRn9wQpMG9R9hX3sD158z7EqHWmweQAJU5SA","network_id":"coblox-devnet-0","node_id":"cblx176fmuouuc5v2xyqqxgef5uwrdqt53yqazdlxwcfl6a63bxarnuyq","pow":{"algorithm":"argon2id-leading-zero-bits-v0","difficulty_bits":"4","iterations":"3","lanes":"4","memory_kib":"65536","nonce":"11","parameter_set_hash":"sha256:7df04d03b60f62852f0d76c847d0181a2b17b43a50f987c0b9f814e70f064bcc","recent_block_height":"41","recent_block_id":"sha256:7e0694f564afa2d047db4eb58f4f2b3d322d71db808f6bbf5313ee2d2a4a95af"},"public_key":"L_o1qZ06PPuxe7fB3FVhsYqNzKTfONxhPqhZw36xM2s","schema_version":"0.1","signature":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"}
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
   and the public key derives both `node_id` and the canonical `libp2p_peer_id`;
5. neither `node_id` nor public key is already enrolled or revoked, and no
   pending request with the same key is being processed;
6. the referenced parameter set is active at the referenced height, and the
   echoed difficulty and cost profile exactly equal it;
7. `recent_block_id` is the locally finalized canonical block at
   `recent_block_height`, and that height is no more than the active parameter
   `recent_block_window` behind the validator's latest finalized height;
8. **last**, and only if every check above passed, the Argon2id tag is computed
   and MUST meet the target.

Ordering alone is necessary but **not sufficient**, and the specification says so
rather than assuming it: an attacker who holds a key can produce a request that
passes steps 1–7 and fails only at step 8, so each such request still costs a
validator one full evaluation. Validators therefore MUST additionally bound the
memory-hard stage itself:

- at most one in-flight step-8 evaluation per public key;
- a declared maximum number of concurrent step-8 evaluations, whose product with
  `memory_kib` is the enrollment subsystem's declared peak memory budget;
- a bounded admission queue that sheds with `rate_limited` when saturated,
  never by accepting unverified requests and never by unbounded queueing;
- a failed step 8 counted against the source connection for rate limiting.

These bounds are local operational policy, not consensus, and are deliberately
not signed network parameters: they govern a validator's own resources and must
be tunable per deployment. What is normative is that a bound exists, is
declared, and fails closed.

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
   two orders of magnitude over SHA-256; it does not change this.
3. **Sybil containment in Coblox is therefore economic, not cryptographic.** It
   rests on the per-epoch existence fund being capped and shared rather than
   paid per node, and on validator eligibility being anchored to storage and
   compute work that cannot be faked without spending real resources. Both are
   specified in [ledger.md](ledger.md#mint-existence-income-work-compensation-and-publisher-reward).
   The network is robust against forgery — balances, signatures, double spending
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
  "libp2p_peer_id": string,
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
{"enrollment_request_hash":"sha256:44dc2df246a89f42d9a9da10f621c86f5141b597b1a6f08cc78b5e61a8388eb1","issued_at_ms":"1787654405000","libp2p_peer_id":"12D3KooWD3eckifWpRn9wQpMG9R9hX3sD158z7EqHWmweQAJU5SA","network_id":"coblox-devnet-0","node_id":"cblx176fmuouuc5v2xyqqxgef5uwrdqt53yqazdlxwcfl6a63bxarnuyq","public_key":"L_o1qZ06PPuxe7fB3FVhsYqNzKTfONxhPqhZw36xM2s","schema_version":"0.1","signatures":[{"signature":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA","validator_id":"val-001"}],"valid_from_height":"42","validator_set_hash":"sha256:1df0a6454faaa5985b7f98c48d3c60d2ed62d5b3b24fe8e97d3dca1dd36f1120"}
```

## Authentication on a connection

Noise or QUIC authenticates the libp2p transport key. Before a peer may publish
Coblox gossip or open protected streams, the receiver MUST obtain its
certificate, verify it against a finalized ledger state, and confirm that:

- certificate public key derives the authenticated libp2p Peer ID;
- `valid_from_height` is finalized and no revocation exists at the receiver's
  finalized height;
- the certificate network ID equals the local network.

Unenrolled peers MAY use only the enrollment stream and native libp2p
connectivity protocols, subject to rate limits.

## Revocation and key replacement

A `RevokeIdentity` governance transaction names the node ID, reason code,
effective height, and replacement node ID if any. It requires a validator quorum
certificate. Revocation is not retroactive: historical signatures remain valid
at heights before the effective height.

If the revoked node holds a seat in the active validator set, revocation alone
would leave its voting power intact until some later set transition that no rule
requires. It therefore **forces** a set transition, and a light client can see
that transition rather than having to trust a claim about it: the binding rule
is in [ledger.md](ledger.md#revocation-forces-a-validator-set-transition).

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
The launch values of `memory_kib`, `iterations`, and `lanes` are equally open
and must be chosen together with `difficulty_bits`, against the declared
reference device. AGENT-007 owns the security recommendation and the Project
Lead/AGENT-002 own the signed parameter governance.

The algorithm, the salt and password construction, the verification rules, the
mandatory validation order with its resource bounds, the 2–6 difficulty safety
bounds, the per-identity linear cost, and the declared limits above are **not**
draft.
