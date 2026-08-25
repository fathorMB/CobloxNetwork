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
  commitments, state tree, and light-client balance proofs.
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
- Enrollment proof of work: Argon2id (RFC 9106), version `0x13`. SHA-256 is not
  a proof-of-work primitive in v0; it remains the hash for identifiers,
  commitments, and Merkle trees.
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
enrollment_pow_salt     = first 16 bytes of
                          H("coblox-enrollment-pow-salt-v0\0"
                            || chain_id_32 || public_key_32
                            || raw_32_bytes(recent_block_id))
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
`"1"/"1"` would not be a structurally valid fixture). `CMT-0` is the issuer
commitment for issuer `cblx1issuerfixture`, `commitment_epoch` 1, and an issuer
secret of `44` repeated 32 bytes. `RND-0` is the challenge randomness derived
from `CMT-0`, beacon height 1, beacon block ID `55` repeated 32 bytes, and
subject `cblx1fixture`. `REQ-0` is an availability request without ID or issuer
signature for subject `cblx1fixture`, issued by `cblx1issuerfixture` at 1,
deadline 2, `randomness` equal to `RND-0`, `issuer_commitment` equal to `CMT-0`,
the `RND-0` randomness source with `commitment_epoch:"1"`, and
`response_bytes:"1"`. `RESP-0` is an unsigned response at time 2, challenge
hash `33` repeated 32 bytes, and one zero response byte. These definitions are
exact after JCS; no omitted/default fields are implied.

| Hash | Fixture | Expected value |
| --- | --- | --- |
| `enrollment_request_hash` | `ER-0` | `sha256:cb1245f681d732aba57064face8872cd2104a185916ff1f0ac2d2e0651e7fb7f` |
| `parameter_set_hash` | enrollment `PD-0` | `sha256:a2553f36f496d30a7773b9f6424c3ffd5ef22e3f8620bf0cca88a9bcdccd4f63` |
| `policy_hash` | reward `PD-0` | `sha256:1a4139ed0204a94efd654d324a859af913a351dea191a6c6839f8fddeee17075` |
| `hosting_rate_card_hash` | hosting `PD-0` | `sha256:9b10204164f4197fb368f0f6ad6c186ae7af1a85b7b6383eeac412a10b8b3ae8` |
| `consensus_parameters_hash` | consensus `PD-0` | `sha256:1a947f60bada6a4974ae55411f404216ffd4093ebf5add0ed34cb95ba20c6a92` |
| `object_id` | bytes `00 01 02` | `sha256:fa67b77e3e686a4b3a2022fbe81edecd3e70a43a98d7e5aee2b76fdbdbe8a78c` |
| `input_hash` | bytes `00 01 02` | `sha256:66810b0847d6694ce6ac99a10db2f7339b89b10d3ed7817f6d27af832a6462c9` |
| `issuer_commitment` | `CMT-0` | `sha256:19556b209c36de1940340bd3ada4a4c821fe70cde0fd3906af2b71f31445e4d5` |
| `challenge_randomness` | `RND-0` | `sha256:8cebe4ad890bd41e8c37b87ad976ad92b8ef35aa3284c441d86691cfdaad88d7` |
| `request_hash` | `REQ-0` | `sha256:8beb98273d89ed31dd62803506e6739fc83ccf3bbca9c20d1028b998fa033360` |
| `response_hash` | `RESP-0` | `sha256:cb7b622e8c2530b8da824765ccdd58cc29b116824bc8ad527fde2f262647df41` |

`challenge_randomness` is carried on the wire as the unpadded base64url of those
32 bytes, which for `RND-0` is `jOvkrYkL1B6MN7h62XatkrjvNaoyhMRB2GaRz9qtiNc`.

Conformance suites MUST reconstruct every preimage from these definitions and
compare all 32 digest bytes; checking only presentation strings is insufficient.

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
  "publisher_reward_cap_denominator":u64-string
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
  "min_revocation_effective_delay_blocks":u64-string
}
```

`pow_algorithm` MUST be `argon2id-leading-zero-bits-v0` in v0. Its cost
parameters obey the RFC 9106 limits: `lanes` in 1–16, `memory_kib` at least
`8 * lanes`, `iterations` at least 1, and `tag_length_bytes` exactly 32. A
document outside those ranges is invalid, so a governance change cannot
silently reduce the enrollment floor to zero cost.

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
derived chain ID, genesis validator set, initial protocol documents, and a weak
subjectivity checkpoint `(height, block_id, timestamp_ms, validator_set_hash)`.
The checkpoint MUST be finalized, no older than
`max_weak_subjectivity_age_ms`, and signed by the network-release trust key. A
fresh client MUST refuse genesis-only synchronization when the checkpoint is
missing, invalid, or stale; it requires a newer distribution obtained through
an authenticated release channel. These values are trust anchors, not
discoverable security facts. Network peers cannot replace that external trust.

## DRAFT: governance-selected launch parameters

The algorithms and parameter names are fixed in v0, but their launch values are
not economic facts and remain open:

- enrollment `difficulty_bits` and the Argon2id cost profile: benchmark-derived
  fixed values vs epoch values bounded by governance. Both must be chosen
  together, because with a memory-hard primitive the cost of one evaluation and
  the expected number of evaluations are independent knobs;
- the per-epoch existence fund, work reward curves, hosting prices, and
  subscription minimums: simulator output vs conservative bootstrap values;
- validator rotation and election: the algorithm is open, but eligibility is
  not. Eligibility MUST be anchored to finalized, hard-to-forge work evidence
  and MUST NOT rest on uptime or availability alone.

The Project Lead owns the economic choices with AGENT-002; AGENT-007 owns the
security review of enrollment bounds; the validator-election specification is
owned by the M-02 ledger specialist. Until signed network parameters select
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
