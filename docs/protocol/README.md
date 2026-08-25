# Coblox protocol v0

Status: **versioned implementation contract** for protocol family `coblox/0`.

This directory defines the interoperable formats used by Coblox node identity,
peer-to-peer messages, the federated ledger, light clients, and WASM app
packages. An implementation MUST NOT infer fields or defaults that are not
defined here.

## Documents

- [Identity and enrollment](identity.md): node keys, identifiers, signatures,
  certificates, revocation, and the one-time anti-Sybil proof of work.
- [P2P wire protocol](wire.md): libp2p transports, discovery, NAT traversal,
  signed envelopes, gossip topics, challenge messages, and ledger sync.
- [Ledger](ledger.md): blocks, quorum certificates, transactions, validator-set
  commitments, validator election and rotation, state tree, and light-client
  balance proofs.
- [App manifest and package](app-manifest.md): WASM capabilities, limits,
  pricing, publisher signatures, and the deterministic `.cobloxapp` container.

## Normative language

The words MUST, MUST NOT, REQUIRED, SHOULD, SHOULD NOT, and MAY are interpreted
as in RFC 2119 and RFC 8174. Unless a section is explicitly headed `DRAFT`, it
is normative for v0.

## Common representation

Coblox application objects use UTF-8 JSON in the I-JSON subset (RFC 7493),
canonicalized with the JSON Canonicalization Scheme (JCS, RFC 8785).

The following restrictions are part of the protocol:

1. Object keys are lower `snake_case` ASCII and duplicate keys are invalid.
2. Integers whose schema type is `u64` are JSON strings containing the shortest
   unsigned base-10 form (`"0"`, never `"00"`). This avoids host-language
   integer precision differences.
3. Fixed byte strings use unpadded RFC 4648 base64url. Hashes use
   `sha256:` followed by exactly 64 lowercase hexadecimal digits.
4. Timestamps are `u64` Unix milliseconds represented as decimal strings.
5. Floating-point values, `null`, and Unicode normalization by the decoder are
   forbidden. Text is compared by Unicode scalar value after UTF-8 validation.
6. Unknown fields are rejected unless they occur inside an explicitly defined
   `extensions` object. v0 defines no consensus-relevant extensions.
7. Parsers MUST enforce the size limits before allocating from untrusted input.

The bytes that are hashed or signed are always the JCS bytes, without a byte
order mark or trailing newline. Pretty-printed examples in these documents are
display-only; each example also includes a one-line canonical serialization.

**Choice rationale.** Protocol Buffers were rejected for Coblox objects because
deterministic serialization is not a cross-implementation canonicality
guarantee and unknown-field handling complicates signing. Deterministic CBOR
was rejected for v0 because third-party app tooling and operator inspection are
material requirements. Native libp2p protocols continue to use their specified
Protobuf formats; only Coblox application payloads use JCS.

## Identifiers and cryptographic conventions

- Hash: SHA-256.
- Identity and validator signatures: Ed25519 (RFC 8032), 32-byte public keys and
  64-byte signatures.
- Enrollment proof of work: Argon2id (RFC 9106), version `0x13`. SHA-256 is
  **not** the anti-Sybil proof-of-work primitive in v0; it remains the hash for
  identifiers, commitments, and Merkle trees, and it is the primitive of the
  enrollment admission shield of
  [identity.md](identity.md#validation-order-and-its-reason). The two uses are
  not in tension and the distinction is load-bearing: an anti-Sybil cost must be
  expensive to *produce* and is ruined by a hardware advantage, whereas a
  denial-of-service shield must be cheap to *verify* and tolerates one. SHA-256
  is the wrong primitive for the first and the right one for the second.
- `chain_id`: `sha256:` plus
  `SHA-256("coblox-chain-id-v0\0" || u32be(len(network_id_utf8)) ||
  network_id_utf8 || raw_32_bytes(genesis_block_id))`.
- Domain separation: every Coblox signature input is the ASCII domain shown by
  the schema, one zero byte, `raw_32_bytes(chain_id)`, then the described bytes.
  The `chain_id` is supplied by the trust anchor and need not be repeated in
  every wire object.
- `node_id`: `cblx1` plus lowercase base32 without padding of
  `SHA-256("coblox-node-id-v0\0" || ed25519_public_key)`.
- `tx_id`: `sha256:` plus SHA-256 of
  `"coblox-tx-id-v0\0" || raw_32_bytes(chain_id) ||
  JCS(unsigned_transaction)`.
- `block_id`: `sha256:` plus SHA-256 of
  `"coblox-block-id-v0\0" || raw_32_bytes(chain_id) || JCS(block_header)`.
- Token values are non-negative `u64` microtokens. One displayed token is
  1,000,000 microtokens; consensus never uses decimal fractions.

Cryptographic comparisons MUST be constant-time where the implementation's
language permits it. Decoders MUST reject non-canonical encodings before
signature verification, so a logical object has one signing representation.
The uniqueness of `network_id` is an operational convention, not a replay
control; `chain_id` is the cryptographic chain binding.

### Consensus-critical Ed25519 verification

All implementations MUST apply one identical ZIP-215-derived rule. Given
32-byte encodings `A_enc` and `R_enc`, scalar bytes `S_enc`, message `M`, base
point `B`, subgroup order `L`, and `k = SHA-512(R_enc || A_enc || M) mod L`:

1. decode `A_enc` and `R_enc` as points `A` and `R` on the complete Ed25519
   twisted Edwards curve; non-canonical y-coordinate encodings are accepted and
   reduced modulo `2^255-19` as required by ZIP-215;
2. interpret `S_enc` as little-endian and require `0 <= S < L`;
3. require `[8]A != identity` so an identity/validator key has no small order;
4. accept if and only if `[8][S]B = [8]R + [8][k]A`.

The cofactorless equation `[S]B = R + [k]A` MUST NOT be used. Implementations
MUST NOT substitute `ed25519-dalek::verify_strict`, legacy-compatibility modes,
or a library default whose edge-case acceptance has not been shown equivalent
to these four rules. The hash for `k` uses the original encodings, not
re-encoded points.

Conformance uses vectors 0–11 from `novifinancial/ed25519-speccheck`. With the
additional small-order public-key rejection above, expected outcomes are:

| Vector | 0 | 1 | 2 | 3 | 4 | 5 | 6 | 7 | 8 | 9 | 10 | 11 |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| Coblox v0 | reject | reject | accept | accept | accept | accept | reject | reject | accept | accept | reject | reject |

Every signature verifier used for enrollment, envelopes, transactions,
certificates, validator bindings, votes, challenge evidence, and app manifests
MUST pass this table before it can participate in a Coblox network.

## Hash preimage registry

`H` below is SHA-256. `raw_32_bytes` strips the `sha256:` presentation prefix.
Unless a formula explicitly says otherwise, the JSON object is validated and
JCS-serialized before hashing.

```text
enrollment_request_hash = H("coblox-enrollment-request-hash-v0\0"
                            || chain_id_32 || JCS(EnrollmentRequest))
parameter_set_hash      = H("coblox-enrollment-parameter-set-v0\0"
                            || chain_id_32 || JCS(UnsignedProtocolDocument))
policy_hash             = H("coblox-reward-policy-v0\0"
                            || chain_id_32 || JCS(UnsignedProtocolDocument))
hosting_rate_card_hash  = H("coblox-hosting-rate-card-v0\0"
                            || chain_id_32 || JCS(UnsignedProtocolDocument))
consensus_parameters_hash = H("coblox-consensus-parameters-v0\0"
                              || chain_id_32 || JCS(UnsignedProtocolDocument))
object_id               = H("coblox-storage-object-v0\0"
                            || u64be(object_length) || object_bytes)
input_hash              = H("coblox-compute-input-v0\0"
                            || u64be(input_length) || input_bytes)
request_hash            = H("coblox-challenge-request-hash-v0\0"
                            || chain_id_32 || JCS(ChallengeRequestWithoutIdOrSignature))
response_hash           = H("coblox-challenge-response-hash-v0\0"
                            || chain_id_32 || JCS(ChallengeResponseWithoutSignature))
issuer_commitment       = H("coblox-challenge-issuer-commitment-v0\0"
                            || chain_id_32 || u32be(len(issuer_node_id_utf8))
                            || issuer_node_id_utf8 || u64be(commitment_epoch)
                            || issuer_secret_32)
challenge_randomness    = H("coblox-challenge-randomness-v0\0"
                            || chain_id_32 || u64be(beacon_height)
                            || raw_32_bytes(beacon_block_id)
                            || raw_32_bytes(issuer_commitment) || issuer_secret_32
                            || u32be(len(subject_node_id_utf8)) || subject_node_id_utf8)
election_entropy        = H("coblox-election-entropy-v0\0"
                            || chain_id_32 || u64be(election_epoch)
                            || u64be(election_entropy_blocks)
                            || raw_32_bytes(block_id[first]) || ...
                            || raw_32_bytes(block_id[last]))
election_seed           = H("coblox-election-seed-v0\0"
                            || chain_id_32 || u64be(election_epoch)
                            || raw_32_bytes(election_entropy))
election_ticket         = H("coblox-election-ticket-v0\0"
                            || chain_id_32 || raw_32_bytes(election_seed)
                            || account_key_32)
enrollment_pow_salt     = first 16 bytes of
                          H("coblox-enrollment-pow-salt-v0\0"
                            || chain_id_32 || public_key_32
                            || raw_32_bytes(recent_block_id))
admission_tag           = H("coblox-enrollment-admission-v0\0"
                            || chain_id_32 || admission_nonce_16
                            || public_key_32 || u64be(admission_solution))
weak_subjectivity_checkpoint_hash =
                          H("coblox-weak-subjectivity-checkpoint-v0\0"
                            || chain_id_32
                            || JCS(UnsignedWeakSubjectivityCheckpoint))
```

`ChallengeRequestWithoutIdOrSignature` is the challenge request with both
`challenge_id` and `issuer_signature` removed; `challenge_id` MUST equal
`request_hash`. The evidence `request_hash` and
`response_hash` MUST equal the formulas above. Lengths are byte lengths, not
Unicode character counts.

Hash provenance is normative. Enrollment requests are retained through their
certificate hash and served with the enrollment record. Governed documents are
served by hash as described below. A storage `object_id` is computed at upload
over the complete immutable bytes; providers advertise and serve those exact
bytes by content address, and validators MUST retrieve and rehash them before
issuing storage work. `input_hash` is recomputed from the exact `input` bytes
embedded in `ComputeAssignment`. Finalized challenge-evidence transactions
embed the complete request and response (except a missing response for
`no_response`), so any verifier can recompute their hashes from ledger sync.
Missing source bytes fail verification; a hash asserted without its normative
source is never sufficient.

### Hash conformance fixtures

Fixture `HASH-0` uses 32 zero bytes for `chain_id`; byte fixtures use
`00 01 02`. `ER-0` is the exact enrollment-request schema with all timestamps
and recent height set to `"1"`, nonce `"0"`, network `"fixture"`,
node `"cblx176fmuouuc5v2xyqqxgef5uwrdqt53yqazdlxwcfl6a63bxarnuyq"`, the Peer ID/public key fixture from
[identity.md](identity.md#canonical-libp2p-peer-id), algorithm
`"argon2id-leading-zero-bits-v0"`, `difficulty_bits:"4"`, the RFC 9106 second
recommended cost profile (`memory_kib:"65536"`, `iterations:"3"`, `lanes:"4"`),
parameter hash `11` repeated 32 bytes, recent block hash `22` repeated 32 bytes,
and a 64-zero-byte base64url signature. Each `PD-0` has common fields
`schema_version:"0.1"`, `network_id:"fixture"`, zero `chain_id`,
`sequence:"1"`, and `activation_height:"1"`; it uses its matching
`document_kind` and required body, with every numeric value `"1"` except the
enrollment body's algorithm/difficulty/cost values listed above with
`tag_length_bytes:"32"`, and the reward body's
`publisher_reward_cap_denominator:"2"` (the cap must be strictly below one, so
`"1"/"1"` would not be a structurally valid fixture) and
`validator_eligibility_min_issuers:"2"` (a single-issuer score does not qualify,
so `"1"` would not be structurally valid either). The consensus body needs a
second group of exceptions for the same reason — the election constraint block of
[ledger.md](ledger.md#rotation-the-cap-and-the-floor) rejects an all-`"1"`
document — so it uses `election_entropy_blocks:"2"`,
`candidacy_close_blocks:"3"`, `election_epoch_blocks:"4"`,
`validator_target_set_size:"12"`, `validator_max_set_size:"12"`,
`validator_churn_cap_seats:"3"` and `validator_max_consecutive_terms:"4"`, with
`validator_min_set_size`, `validator_cooldown_epochs` and
`validator_min_capture_epochs` at `"1"`. Two facts about those numbers are worth
stating, because a fixture teaches a shape whether or not it means to.

The block requires `ceil(V/T) <= c < V/3`, which is **unsatisfiable for `T <= 3`
at any `V`**, so a fixture at `T:"3"` would encode a state no conformant network
can reach; `V:"3"` is impossible for the same kind of reason, since `3c < 3`
cannot hold for any `c >= 1`. Both are proved by exhausting the parameter space
rather than argued.

This fixture also takes `c > 1` on purpose. With `c:"1"` a cohort is a single
seat, so the entry cap is never exercised, and neither is the interaction between
the cap, the term stagger and the contraction floor that the constraints exist to
keep consistent. `V:"12"`, `T:"4"`, `c:"3"` satisfies that and is convenient; it
is **not** claimed to be the smallest such instance, and no minimality is claimed
for it. Smaller ones with `c > 1` exist — `V:"7"`, `T:"4"`, `c:"2"`, `m:"1"` is
one. A superlative in a normative document has to be proved or not written, and
this one buys nothing that would justify proving it. `CMT-0` is the issuer
commitment for issuer `cblx1issuerfixture`, `commitment_epoch` 1, and an issuer
secret of `44` repeated 32 bytes. `RND-0` is the challenge randomness derived
from `CMT-0`, beacon height 1, beacon block ID `55` repeated 32 bytes, and
subject `cblx1fixture`. `REQ-0` is an availability request without ID or issuer
signature for subject `cblx1fixture`, issued by `cblx1issuerfixture` at 1,
deadline 2, `randomness` equal to `RND-0`, `issuer_commitment` equal to `CMT-0`,
the `RND-0` randomness source with `commitment_epoch:"1"`, and
`response_bytes:"1"`. `RESP-0` is an unsigned response at time 2, challenge
hash `33` repeated 32 bytes, and one zero response byte. `ADM-0` uses zero
`chain_id`, `admission_nonce` `88` repeated 16 bytes (`iIiIiIiIiIiIiIiIiIiIiA`
as unpadded base64url), the identity fixture public key of
[identity.md](identity.md#node-identifier), and `admission_solution` `"0"`.
`ELEC-0` is the epoch-3 election of the worked example in
[ledger.md](ledger.md#worked-example-of-the-derivation): `election_epoch` 3,
`election_entropy_blocks` 3, and entropy block IDs `aa`, `bb` and `cc` each
repeated 32 bytes in that order. The seed derives from those alone —
`candidate_root` and `candidate_count` are bound by validity and are deliberately
not in the preimage, for the reason given in
[ledger.md](ledger.md#the-second-lever-the-pool-itself-and-what-is-honestly-claimable-about-it).
Its `election_ticket` row uses the account key `05` repeated 32 bytes.
`WSC-0` is the unsigned
weak subjectivity checkpoint of [Trust anchors](#trust-anchors) with
`schema_version:"0.1"`, `network_id:"fixture"`, zero `chain_id`, `height:"1"`,
`block_id` `66` repeated 32 bytes, `timestamp_ms:"1"`, `issued_at_ms:"1"`,
`validator_set_hash` `77` repeated 32 bytes,
`max_weak_subjectivity_age_ms:"1"`, an empty `revoked_validators` array, and
the corresponding empty `revocation_root` `H(0x33)`. These definitions are
exact after JCS; no omitted/default fields are implied.

| Hash | Fixture | Expected value |
| --- | --- | --- |
| `enrollment_request_hash` | `ER-0` | `sha256:cb1245f681d732aba57064face8872cd2104a185916ff1f0ac2d2e0651e7fb7f` |
| `parameter_set_hash` | enrollment `PD-0` | `sha256:a2553f36f496d30a7773b9f6424c3ffd5ef22e3f8620bf0cca88a9bcdccd4f63` |
| `policy_hash` | reward `PD-0` | `sha256:fbc7493ae6da64e92d935f35ecb9c2703c005df960e18e7cb609606838132f0d` |
| `hosting_rate_card_hash` | hosting `PD-0` | `sha256:9b10204164f4197fb368f0f6ad6c186ae7af1a85b7b6383eeac412a10b8b3ae8` |
| `consensus_parameters_hash` | consensus `PD-0` | `sha256:840dd6a980a6350b4879c60f8581466165125408a62839d67468c32ca3f0c33f` |
| `object_id` | bytes `00 01 02` | `sha256:fa67b77e3e686a4b3a2022fbe81edecd3e70a43a98d7e5aee2b76fdbdbe8a78c` |
| `input_hash` | bytes `00 01 02` | `sha256:66810b0847d6694ce6ac99a10db2f7339b89b10d3ed7817f6d27af832a6462c9` |
| `issuer_commitment` | `CMT-0` | `sha256:19556b209c36de1940340bd3ada4a4c821fe70cde0fd3906af2b71f31445e4d5` |
| `challenge_randomness` | `RND-0` | `sha256:8cebe4ad890bd41e8c37b87ad976ad92b8ef35aa3284c441d86691cfdaad88d7` |
| `request_hash` | `REQ-0` | `sha256:8beb98273d89ed31dd62803506e6739fc83ccf3bbca9c20d1028b998fa033360` |
| `response_hash` | `RESP-0` | `sha256:cb7b622e8c2530b8da824765ccdd58cc29b116824bc8ad527fde2f262647df41` |
| `admission_tag` | `ADM-0` | `sha256:457915b8cd8816c5fe76651bdda0578983f8e393c7e4fe0b24376ca0bca22628` |
| `election_entropy` | `ELEC-0` | `sha256:29a63c10926e75ed418c6f3c7670b0edfeb0b021868d1342b788327f53767e42` |
| `election_seed` | `ELEC-0` | `sha256:9e2aa2621f957279e4bdf1c4ccea5629ce429892ac3f9af10c9456a3c78dad85` |
| `election_ticket` | `ELEC-0` | `sha256:a10e8ec4a79c2defa40f869c68c2a1570bbb4a5b12b597a28d94294bf7582f21` |
| `weak_subjectivity_checkpoint_hash` | `WSC-0` | `sha256:2bc543a3f8e4df60735e6431a6c1fb7293ed53047e98fe2e5bc1a879f200c71e` |

`challenge_randomness` is carried on the wire as the unpadded base64url of those
32 bytes, which for `RND-0` is `jOvkrYkL1B6MN7h62XatkrjvNaoyhMRB2GaRz9qtiNc`.

Conformance suites MUST reconstruct every preimage from these definitions and
compare all 32 digest bytes; checking only presentation strings is insufficient.

**Parameter fixtures are not free choices.** A conformance suite that builds a
`consensus_parameters` document picks values that the constraint block of
[ledger.md](ledger.md#rotation-the-cap-and-the-floor) may forbid outright, and
some forbidden combinations look entirely ordinary — `validator_max_consecutive_terms`
of 3 is the clearest example, and it is inadmissible at **every** set size. A test
case built on one of those is not testing a different value: it is asserting
behaviour for a state no conformant network can be in, and it will either pass
vacuously or fail for reasons the implementation cannot fix. Suites MUST
therefore validate their own parameter fixtures against the constraint block
before using them, and a case that fails validation is removed rather than
adjusted.

### Signed protocol documents

The four governed hashes above commit to this common unsigned shape:

```text
UnsignedProtocolDocument = {
  "schema_version":"0.1",
  "document_kind":"enrollment_parameters"|"reward_policy"|
                  "hosting_rate_card"|"consensus_parameters",
  "network_id":string,
  "chain_id":sha256-string,
  "sequence":u64-string,
  "activation_height":u64-string,
  "body":object
}
SignedProtocolDocument = UnsignedProtocolDocument + {
  "signatures":[ValidatorSignature]
}
```

The hash domain MUST match `document_kind`. Signatures use domain
`coblox-protocol-document-v0` and cover the raw document hash. They satisfy the
single quorum predicate in [ledger.md](ledger.md#quorum-predicate). Sequence is
strictly increasing per kind; activation cannot be retroactive.

Required bodies are:

```text
EnrollmentParametersBody = {
  "pow_algorithm":string, "difficulty_bits":u64-string,
  "memory_kib":u64-string, "iterations":u64-string, "lanes":u64-string,
  "tag_length_bytes":u64-string,
  "max_request_age_ms":u64-string, "max_future_skew_ms":u64-string,
  "recent_block_window":u64-string
}
RewardPolicyBody = {
  "reward_epoch_ms":u64-string,
  "existence_fund_microtokens_per_epoch":u64-string,
  "availability_microtokens_per_unit":u64-string,
  "storage_microtokens_per_byte_epoch":u64-string,
  "compute_microtokens_per_million_fuel":u64-string,
  "publisher_microtokens_per_active_subscriber":u64-string,
  "publisher_reward_cap_numerator":u64-string,
  "publisher_reward_cap_denominator":u64-string,
  "storage_units_per_contribution_unit":u64-string,
  "compute_units_per_contribution_unit":u64-string,
  "validator_eligibility_threshold_units":u64-string,
  "validator_eligibility_window_epochs":u64-string,
  "validator_eligibility_min_issuers":u64-string
}
HostingRateCardBody = {
  "billing_epoch_ms":u64-string, "minimum_billable_epochs":u64-string,
  "microtokens_per_replica_epoch":u64-string,
  "microtokens_per_gib_epoch":u64-string,
  "microtokens_per_million_fuel":u64-string
}
ConsensusParametersBody = {
  "max_clock_drift_ms":u64-string,
  "max_envelope_validity_ms":u64-string,
  "replay_cache_entries_per_peer":u64-string,
  "replay_cache_entries_global":u64-string,
  "max_weak_subjectivity_age_ms":u64-string,
  "max_current_balance_age_ms":u64-string,
  "app_suspension_notice_epochs":u64-string,
  "min_revocation_effective_delay_blocks":u64-string,
  "election_epoch_blocks":u64-string,
  "candidacy_close_blocks":u64-string,
  "election_entropy_blocks":u64-string,
  "validator_min_set_size":u64-string,
  "validator_target_set_size":u64-string,
  "validator_max_set_size":u64-string,
  "validator_churn_cap_seats":u64-string,
  "validator_max_consecutive_terms":u64-string,
  "validator_cooldown_epochs":u64-string,
  "validator_min_capture_epochs":u64-string
}
```

`pow_algorithm` MUST be `argon2id-leading-zero-bits-v0` in v0.

#### The enrollment cost floor is a validity rule, not a recommendation

The parameter ranges of [RFC 9106](https://www.rfc-editor.org/rfc/rfc9106.html)
— parallelism `p` "from 1 to 2^(24)-1", memory "from 8\*p to 2^(32)-1"
kibibytes, passes "from 1 to 2^(32)-1" — are the **domain of the function**, not
a secure configuration. Taking them for a security floor would leave
`memory_kib` legally as low as 8 KiB at `lanes: 1`, which is roughly a
factor 8,000 below the profile [ADR-007] assumed and a *larger* attacker
advantage than the SHA-256 ratio that motivated that decision. A governed
parameter set could then revoke the memory-hard floor while remaining fully
conformant and leaving no on-chain trace.

v0 therefore enforces a cost floor **when the document is accepted**, using the
same mechanism as the creator-share cap of
[ledger.md](ledger.md#creator-share-cap-a-validity-rule-not-a-policy-note). An
`enrollment_parameters` document is **invalid**, not merely unwise, unless all
of the following hold:

```text
lanes in 1..=16                                    // domain, narrowed
tag_length_bytes == 32
memory_kib      >= 65536                           // memory-hardness floor
iterations      >= 1                               // domain
memory_kib * iterations >= 196608                  // cost-area floor, checked u128
```

`memory_kib` and `iterations` are **security parameters, not performance
parameters**, and this document says so explicitly so that future governance
knows what it is touching. The two constraints are separate on purpose:

- the `memory_kib >= 65536` floor preserves *memory-hardness itself*. Trading
  memory for passes — say 8 KiB with 24,576 iterations — reaches the same
  cost-area on paper while making the function compute-bound and perfectly
  GPU-friendly, which is precisely the property [ADR-007] rejected SHA-256 for;
- the `memory_kib * iterations >= 196608` floor fixes the *amount of work*, in
  KiB-passes, at no less than the RFC's second recommended profile
  (`m = 2^16`, `t = 3`).

Expressing the second constraint as an area rather than as `iterations >= 3` is
deliberate. A literal `iterations >= 3` rule would reject the RFC's **first**
recommended profile — "a uniformly safe option", `t = 1`, `p = 4`,
`m = 2^21` (2 GiB) — which is the *stronger* of the two. The area form admits
both RFC recommendations and rejects everything weaker than either.

Boundary conformance fixtures, which a suite MUST exercise:

| `memory_kib` | `iterations` | Verdict | Reason |
| --- | --- | --- | --- |
| `"65536"` | `"3"` | valid | RFC second recommended profile; exactly at the floor |
| `"65535"` | `"3"` | **invalid** | below the memory-hardness floor |
| `"65536"` | `"2"` | **invalid** | area `131072 < 196608` |
| `"2097152"` | `"1"` | valid | RFC first recommended profile, 2 GiB |
| `"8"` | `"1"` | **invalid** | RFC domain minimum at `lanes: 1`; not a security floor |

A network MUST NOT lower these minima by governance. Raising them is permitted
and is the intended adjustment path. Because the floor is stated against the
**declared reference device** of
[identity.md](identity.md#one-time-anti-sybil-proof-of-work), any proposal to
change it requires re-declaring and re-publishing that device and its measured
onboarding time; a cost floor without the device it was measured on is not a
bound.

`publisher_reward_cap_denominator` MUST be non-zero and
`publisher_reward_cap_numerator` MUST be strictly smaller than it. That
strictness is what makes the subscription cycle of
[ledger.md](ledger.md#mint-existence-income-work-compensation-and-publisher-reward)
lossy rather than merely unprofitable at the margin; a governance document
with `numerator >= denominator` is invalid, not merely unwise.

Reward arithmetic uses checked `u128` intermediates, integer multiplication by
the eligible units, and rejects a result above `u64::MAX`; no floating point or
implicit rounding exists. Hosting charges use the same rule, with each partial
GiB or billing epoch rounded upward before multiplication. Numeric launch values
remain governance-selected as described in the DRAFT section.

Genesis distributions contain the initial four signed documents. Full nodes
MUST retain every historical version referenced by a finalized object and serve
it by hash through `ledger_status_response`; a verifier MUST fetch, hash,
quorum-verify, and check activation before using it. This makes mint and hosting
calculation independently auditable rather than a validator-only assertion.

## Protocol versioning

The protocol family is `0`; object schemas carry `schema_version: "0.1"` and
libp2p protocol IDs carry `/0.1.0`. The rules are:

- patch releases clarify behavior without changing accepted bytes;
- a minor release may add an optional field only through a negotiated protocol
  ID and never changes consensus validation of existing objects;
- a major release may break formats and uses a different protocol family;
- peers negotiate libp2p stream versions with multistream-select and MUST close
  a stream with `unsupported_version` when no common Coblox version exists;
- ledger rules are selected by `protocol_version` in the block header. Nodes
  MUST NOT apply rules from an unrecognized version.

## Security and resource limits

Before decoding, nodes enforce: 64 KiB envelope, 16 KiB challenge request or
response, 8 MiB ledger-sync response, 2 MiB block, 64 KiB manifest, and the
module/package limits declared in [app-manifest.md](app-manifest.md). A sender
that repeatedly violates canonical encoding, signature, replay, or size rules
SHOULD be disconnected and locally rate-limited. Invalid objects are never
gossiped onward.

## Trust anchors

A signed network distribution MUST ship the network ID, genesis block ID,
derived chain ID, genesis validator set, initial protocol documents, the election
bounds below, and a weak subjectivity checkpoint. A fresh client MUST refuse
genesis-only synchronization when the checkpoint is missing, invalid, or stale;
it requires a newer distribution obtained through an authenticated release
channel. These values are trust anchors, not discoverable security facts. Network
peers cannot replace that external trust.

### Election bounds

The election rule of
[ledger.md](ledger.md#validator-election-and-rotation) is defined by parameters
that live in the `consensus_parameters` document — and that document is signed by
a validator quorum, which is to say by the sitting validator set. Constraining
those parameters only against **each other** would therefore leave the invariant
switchable off by the very set it constrains: an epoch length or a term limit
large enough that no boundary and no expiry ever arrives satisfies every
relational constraint while freezing the set for ever, on a chain that stays
valid at every height. This is the same failure the enrollment cost floor was
made a validity rule to prevent, and it is answered the same way, one level
further out.

`ElectionBounds` is therefore **configuration, not chain state**. It ships inside
the signed distribution and in no other channel, cannot be changed by any on-chain
document, and MUST NOT be learned from a peer, a header, or a protocol document.
Changing it is a new authenticated release and a chain-level decision, exactly
like rotating a trust key.

```text
ElectionBounds = {
  "schema_version":"0.1",
  "network_id":string,
  "chain_id":sha256-string,
  "election_epoch_blocks_max":u64-string,
  "validator_max_consecutive_terms_max":u64-string,
  "validator_max_set_size_max":u64-string,
  "validator_min_set_size_min":u64-string,
  "validator_min_capture_epochs_min":u64-string,
  "election_parameter_change_numerator":u64-string,
  "election_parameter_change_denominator":u64-string,
  "election_parameter_min_activation_gap_blocks":u64-string
}
```

`chain_id` MUST equal the client's configured chain ID,
`election_parameter_change_numerator` MUST exceed
`election_parameter_change_denominator`, which MUST be positive, and
`election_parameter_min_activation_gap_blocks` MUST be positive. The gap is what
makes the change ratio a limit **per unit of chain** rather than per document:
`sequence` need only increase, so without it a quorum publishes a document per
block and walks a parameter to its genesis ceiling in as many blocks as the ratio
needs steps. The ceiling still holds either way; the gap is what leaves anyone
time to notice the walk. A
`consensus_parameters` document whose election parameters fall outside these
bounds, or which moves any of them by more than the permitted ratio against the
currently active document, is **rejected on acceptance** — the full constraint
block is in
[ledger.md](ledger.md#magnitudes-not-only-relations-the-bounds-are-fixed-at-genesis).
A light client applies the same bounds at step 5 of the light-client algorithm
and fails closed rather than proceeding with unbounded values.

Values are a genesis decision of the network operator rather than a simulator
output, and are deliberately not fixed in this document. What is fixed is that
they exist, that they are outside on-chain governance, and that a network which
ships no `ElectionBounds` is not a conformant Coblox network.

**Declared limit.** A client's bounds are only as trustworthy as the release
channel that delivered them, which is the same footing as the trust key below. A
client running an older distribution enforces older bounds: narrower than the
network's, and it fails closed on chains the network considers valid; wider, and
it is more permissive than the network, which will not produce the sets it would
have wrongly accepted. Neither direction lets an attacker widen the bounds a
given client enforces, and that is the property being claimed — not that the
bounds are unforgeable in general.

### Weak subjectivity checkpoint

The checkpoint is the light client's only anchor against the long-range attack,
so it is specified here as a normative object rather than described in prose.
This schema is the single definition; [ledger.md](ledger.md#light-client-balance-verification)
consumes it and does not restate its fields.

```text
UnsignedWeakSubjectivityCheckpoint = {
  "schema_version":"0.1",
  "network_id":string,
  "chain_id":sha256-string,
  "height":u64-string,
  "block_id":sha256-string,
  "timestamp_ms":u64-string,
  "issued_at_ms":u64-string,
  "validator_set_hash":sha256-string,
  "max_weak_subjectivity_age_ms":u64-string,
  "revoked_validators":[{"node_id":string,"effective_height":u64-string}],
  "revocation_root":sha256-string
}
WeakSubjectivityCheckpoint = UnsignedWeakSubjectivityCheckpoint + {
  "trust_key":base64url(32 bytes),
  "signature":base64url(64 bytes)
}
```

`height`/`block_id`/`validator_set_hash` describe a **finalized** block and its
active set; `timestamp_ms` is that block header's timestamp and `issued_at_ms`
is when the checkpoint itself was produced. The two are distinct and both are
required: age is measured on `issued_at_ms`, chain position on `timestamp_ms`.

The signature uses domain `coblox-weak-subjectivity-signature-v0` and covers
`raw_32_bytes(weak_subjectivity_checkpoint_hash)` from the preimage registry,
through the global chain-bound procedure. A checkpoint whose `chain_id` does not
equal the client's configured chain ID, or whose `trust_key` the client does not
hold, is rejected; a client MUST NOT learn a trust key from a checkpoint, from a
peer, or from any network source.

**Revocation commitment.** `revoked_validators` lists every identity revoked by
a finalized `revoke_identity` as of `height` that held a seat in any validator
set active at or after its own `effective_height`. Entries are unique and sorted
bytewise by `node_id`. The commitment mirrors the subscription tree of
[ledger.md](ledger.md#mint-existence-income-work-compensation-and-publisher-reward):

```text
revocation_leaf  = H(0x30 || u32be(len(node_id_utf8)) || node_id_utf8
                          || u64be(effective_height))
revocation_node  = H(0x31 || left_32 || right_32)
revocation_empty = H(0x32)
```

The tree preserves sorted order and pads to a power of two with
`revocation_empty`; an empty list uses `H(0x33)` as `revocation_root`, which is
`sha256:4e07408562bedb8b60ce05c1decfe3ad16b72230967de01f640b7e4729b49fce`.
Fixture `REVL-0`, the leaf for `cblx1revokedfixture` at `effective_height` 50,
is `sha256:7fb1f4024627c413cbf70b49a390b6d31778e667e86042864c4bed107cd52497`,
which is also the single-entry root. The list is carried in full because
validator revocations are rare and the checkpoint is an out-of-band release
artifact, not a wire object; the root exists so that a future proof-served form
does not change the signed bytes.

**Why the light client needs this.** It is the closure of the revocation rule
of [ledger.md](ledger.md#revocation-forces-a-validator-set-transition) for
clients that never see transactions. Without it, an attacker holding the leaked
consensus keys of a revoked super-majority can continue an
hash-continuous chain whose every `key_binding_signature` still verifies, and a
light client following it passes every other check. The rule is stated with the
verification algorithm at
[ledger.md](ledger.md#light-client-balance-verification).

### The network-release trust key

The trust key is an Ed25519 key held by the network's release process, not by
any validator, and not by a node. Its provenance and lifecycle are normative:

- the 32-byte public key ships **inside** the signed network distribution and in
  no other channel. It is configuration, not a discoverable fact;
- a distribution MAY carry more than one trust key. A client accepts a
  checkpoint signed by any key it currently holds, which is what makes rotation
  possible without a flag day;
- rotation is performed by publishing a new distribution that contains both the
  outgoing and the incoming key, and then a later distribution that contains
  only the incoming key. A client that skips the overlapping release cannot
  verify newer checkpoints and MUST fail closed, reporting that it needs a newer
  distribution — it MUST NOT accept an unknown key on the strength of a peer
  claim, a self-signed successor, or a checkpoint that carries its own key;
- compromise recovery is out-of-band by construction: it is a new authenticated
  release, because a compromised signer can otherwise sign whatever supersession
  message the protocol would define. v0 states this rather than implying that a
  network mechanism exists;
- **declared limit.** A client whose distribution is older than the compromise
  will accept checkpoints from the compromised key until it updates. The
  containment is the release channel's own authentication and the non-regression
  rule of the light-client algorithm, not the protocol.

**Resolving the parameter circularity.** `max_weak_subjectivity_age_ms` is a
consensus parameter that a client would have to read from the chain before it is
entitled to trust the chain. The checkpoint therefore carries its own copy, and
the value a client uses at step 1 of the light-client algorithm is **the one in
the signed checkpoint**, never one learned from a peer. Once the client has an
authenticated header it MUST check that the two agree and fail closed if they do
not; a mismatch means the distribution and the chain disagree about the trust
window.

## DRAFT: governance-selected launch parameters

The algorithms and parameter names are fixed in v0, but their launch values are
not economic facts and remain open:

- enrollment `difficulty_bits` and the Argon2id cost profile: benchmark-derived
  fixed values vs epoch values bounded by governance. Both must be chosen
  together, because with a memory-hard primitive the cost of one evaluation and
  the expected number of evaluations are independent knobs;
- the per-epoch existence fund, work reward curves, hosting prices, and
  subscription minimums: simulator output vs conservative bootstrap values;
- the validator election parameters — epoch length, candidacy close, entropy
  window, set sizes, churn cap, term limit, cooldown, declared capture horizon,
  the eligibility threshold with its window, and the minimum number of distinct
  issuers behind a contribution score. The **algorithm** is no longer open: it is
  specified in [ledger.md](ledger.md#validator-election-and-rotation). Nor are
  the relations among these values open, nor their magnitudes — a
  consensus-parameters document that violates the constraint block of
  [ledger.md](ledger.md#rotation-the-cap-and-the-floor), or that leaves the
  [election bounds](#election-bounds) of the genesis trust anchor, is rejected on
  acceptance. The simulator therefore chooses inside a feasible region that the
  chain's own governance cannot widen.

The Project Lead owns the economic choices with AGENT-002; AGENT-007 owns the
security review of enrollment bounds. Until signed network parameters select
values, a deployment is a development network and MUST NOT identify itself as
Coblox mainnet.

## Reference sources

- [libp2p protocol specifications](https://github.com/libp2p/specs)
- [Circuit Relay v2](https://github.com/libp2p/specs/blob/master/relay/circuit-v2.md)
- [DCUtR](https://github.com/libp2p/specs/blob/master/relay/DCUtR.md)
- [AutoNAT v1](https://github.com/libp2p/specs/blob/master/autonat/autonat-v1.md)
- [libp2p hole-punching model](https://github.com/libp2p/specs/blob/master/connections/hole-punching.md)
- [JSON Canonicalization Scheme, RFC 8785](https://www.rfc-editor.org/rfc/rfc8785)
- [Ed25519, RFC 8032](https://www.rfc-editor.org/rfc/rfc8032)
- [ZIP-215 Ed25519 validation](https://zips.z.cash/zip-0215)
- [Argon2 memory-hard function, RFC 9106](https://www.rfc-editor.org/rfc/rfc9106.html)
- [libp2p Peer IDs and keys](https://github.com/libp2p/specs/blob/master/peer-ids/peer-ids.md)
- [CometBFT light-client trust model](https://github.com/cometbft/cometbft/blob/main/spec/light-client/README.md)
