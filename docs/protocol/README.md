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

### Genesis constants

A genesis constant is fixed when a chain is created and cannot be changed by a
signed governance document. Changing one requires a new genesis, which is the
point: these are the values that give meaning to the governed parameters, and a
denominator the governance can move is not a denominator.

| Constant | Value | Governed? |
| --- | --- | --- |
| `block_interval_seconds` | `5` | no — genesis only ([ADR-013]) |

**`block_interval_seconds = 5` is declared, not enforced.** Every quantity the
protocol denominates in blocks acquires its real-time meaning from this value:
`election_epoch_blocks` of 120,960 is seven days only because a block is five
seconds. But **no v0 validity rule imposes the cadence.** The only temporal
constraints on a block are that its `timestamp_ms` exceed the median of the
previous eleven finalized blocks and not run ahead of the receiver's clock by
more than the active maximum drift — monotonicity and a ceiling, not a step.
See [ledger.md](ledger.md#block-format) for the consequence: the active
validator set determines the real-time duration of its own epochs, and
therefore of its own incumbency, without breaking any rule. It is stated here
because the alternative — publishing a cadence and letting a reader assume it
is checked — is the failure this specification is written to avoid.

**No rule of this protocol prevents that, and none can.** Every clock the chain
carries is written by the validators, so a validity rule that compared one of
them to another would oblige a set to *write* a cadence, not to *produce* one —
which is why a rule on the distance between consecutive `timestamp_ms` values is
**rejected** rather than merely absent ([ADR-013]). What v0 does instead is make
the real rate **measurable**, using the one clock no validator writes: the
`issued_at_ms` of a weak subjectivity checkpoint. The tolerance is the
[cadence band](#cadence-band) of the genesis trust anchor, and the measurement
is step 4b of the
[light-client algorithm](ledger.md#light-client-balance-verification). The
distinction is the whole of what is claimed: the slowdown is not prevented, it
is made visible and given a declared threshold.

It is not a governed parameter for the same reason the reward epoch has a
floor: an interval a sitting quorum could shorten or lengthen would be a
denominator underneath every limit expressed in blocks. It does not appear in
`ConsensusParametersBody` and no signed document carries it.

Cryptographic comparisons MUST be constant-time where the implementation's
language permits it. Decoders MUST reject non-canonical encodings before
signature verification, so a logical object has one signing representation.
The uniqueness of `network_id` is an operational convention, not a replay
control; `chain_id` is the cryptographic chain binding.

### Genesis derivation and the placeholder chain ID

`chain_id` is derived from `genesis_block_id`; `genesis_block_id` is the
`block_id` of the height-0 header; and the `block_id` preimage carries
`chain_id_32`. The derivation is circular at genesis, and until this section
existed no rule said how the circle is broken. Two implementations given the
same genesis material could therefore derive two different `chain_id` values,
and since almost every preimage of this protocol binds `chain_id`, they would
agree on nothing afterwards.

**The rule.** The **genesis placeholder chain ID** is 32 zero bytes. A value
that is an input to `genesis_block_id`, and any signature taken over such a
value, is computed with the placeholder in place of `chain_id_32` and in place
of any `chain_id` field. Every other value of the chain is computed with the
derived `chain_id`.

The boundary of that sentence is mechanical rather than a matter of taste: *is
this value an input to `genesis_block_id`, directly or through a hash the
height-0 header carries?* For v0 it enumerates as follows, and the enumeration
is normative.

Computed with the placeholder:

- the `block_id` preimage of the height-0 header;
- the `consensus_parameters` document that header names through
  `consensus_parameters_hash` — its `chain_id` field, its
  `consensus_parameters_hash` preimage, and the `coblox-protocol-document-v0`
  signatures over that hash;
- every `key_binding_signature` of the genesis validator set, and of any set the
  header names through `next_validator_set_hash`, because `validator_set_hash`
  is a header field and those signatures are bytes of the set;
- the finality votes of the height-0 quorum certificate, if the block carries
  one.

**The height-0 block MUST carry no transactions**, so its `transactions_root` is
the empty-block root `H(0x03)`. That is a rule and not a fixture convention, and
it is what closes the enumeration below. `transactions_root` **is** a header
field, and some transaction bodies name a signed protocol document by hash — a
`burn` carries `pricing_hash`, which is a `hosting_rate_card_hash`. A height-0
transaction naming one would make that document an input to `genesis_block_id`,
which puts it on the placeholder side under the test above and on the derived
side under the enumeration below: two conformant implementations, two chain IDs,
which is the whole defect this section closes. Two of the transaction kinds were
already impossible at height 0 — a `mint` cannot satisfy the settlement floor
`(e + 1) * reward_epoch_blocks <= h` at `h = 0`, and a `validator_candidacy`
needs an entropy window of earlier blocks — but `burn` and the two challenge
kinds were not, and a rule that holds for three kinds out of five is not a rule.
Genesis balances are declared by the distribution through `state_root`, so
nothing needs a genesis transaction. [REVIEW-029] RF-005.

Computed with the derived `chain_id`:

- the other three signed protocol documents of the genesis distribution. No
  header field names them, and no height-0 transaction can name them because
  there are none, so nothing requires them to exist before `chain_id` does and a
  genesis distribution MUST issue them after it;
- `ElectionBounds`, `RewardBounds` and `CadenceBand`, which are configuration of
  the distribution and enter no preimage the header carries;
- the weak subjectivity checkpoint of the distribution, **including one issued
  at height 0**. A checkpoint is issued after the chain exists and is never
  genesis material, so the `chain_id` a client compares against its own
  configuration is always the derived one.

**One `network_id`, and which one.** The `network_id` hashed into `chain_id` is
the `network_id` field of the height-0 header, byte for byte after UTF-8
validation. Every object of the chain that carries a `network_id` — headers,
transactions, signed protocol documents, `ElectionBounds`, `RewardBounds`,
`CadenceBand`, wire envelopes — MUST carry that same byte string. This is
written because the formula above names `network_id_utf8` without saying which
of the several fields spelled `network_id` it means, and two implementations
that resolved that differently would derive two chain IDs from one genesis
distribution — the same defect as the circularity, arriving through a second
door. Text is compared by Unicode scalar value and is never normalized (see
[Common representation](#common-representation)), so two spellings that display
identically are two different networks and not one network spelled two ways.

**What the placeholder does not buy.** The placeholder is the same 32 zero bytes
on every network, so inside the genesis window the `chain_id_32` prefix
separates **domains and not chains**. A genesis signature is therefore
replayable onto another chain exactly when its **signed payload** is
byte-identical there — the condition is on the payload alone, and nothing about
the rest of the genesis material enters it.

An earlier draft of this paragraph stated the condition as *two networks whose
genesis material is identical and which differ only in `network_id`*. That is
**false in both directions** and the correction is recorded rather than quietly
made, because this is the paragraph that declares a security residual. It is
false because it cannot happen: `network_id` is a field of the height-0 header,
so two networks differing in it have different genesis material and different
`genesis_block_id` — `GEN-0` and `GEN-1` below differ in exactly that one field
and their genesis block IDs are `sha256:1334f536…` and `sha256:6b625392…`. And
it is false because the real condition is **wider**: one signed payload
coinciding is enough, and no coincidence of the surrounding material is needed.

**So the requirement is on every payload, and it is enumerated.** Each of the
twelve signature domains either can never be genesis material, or carries
network-distinguishing bytes inside its own signed payload:

- `coblox-block-vote-v0` signs a `block_id`, over a header carrying `network_id`;
- `coblox-protocol-document-v0` signs a document hash, over a document carrying
  `network_id`;
- `coblox-consensus-key-binding-v0` signs a JCS object that **now carries
  `network_id`**; it was the only one of the twelve whose payload carried
  neither `network_id` nor anything derived from it, and the count is by
  enumerating all twelve and not by impression ([REVIEW-029] RF-002);
- `coblox-ledger-transaction-v0`, `coblox-challenge-request-v0`,
  `coblox-challenge-response-v0` and `coblox-challenge-evidence-v0` sign values
  reached through a transaction, and the height-0 block carries none;
- `coblox-enrollment-request-v0`, `coblox-enrollment-certificate-v0`,
  `coblox-transport-key-attestation-v0` and `coblox-wire-envelope-v0` sign
  objects that carry `network_id` as a field, and none of them is named by the
  height-0 header;
- `coblox-weak-subjectivity-signature-v0` is never genesis material, per the
  enumeration above.

**The residual, stated for what it is.** What a genesis payload can bind to is
the network **name**, and the uniqueness of `network_id` is an operational
convention rather than a replay control. So two chains that share a
`network_id` share every genesis-window payload, and a `key_binding_signature`
from a genesis set is evidence of the network it was made for and not of the
chain. **Nothing available before `genesis_block_id` exists could do better**,
which is what makes this a ceiling and not an omission: every candidate binding
would have to be either the chain ID, which is what is being derived, or another
operator-chosen name. From height 1 onward the derived `chain_id` is in every
preimage and the binding is cryptographic. The genesis set is in any case a
trust anchor obtained through an authenticated release channel
([Trust anchors](#trust-anchors)).

A placeholder derived from the network — `H("coblox-chain-id-v0\0" ||
u32be(len(network_id_utf8)) || network_id_utf8 || 32 zero bytes)`, say — is
**not** adopted, and the reason is re-stated here against the perimeter above
rather than against the narrower one it was first written against. It would bind
each genesis payload to the network name; the enumeration shows every genesis
payload now carries the network name in its own bytes, so it buys **the same
ceiling twice**. Against that it adds a second spelling of *there is no such
value yet* inside one object, next to `previous_block_id`, and the cost of the
second spelling is an implementation that derives the placeholder correctly for
the header and incorrectly for the set. A binding that already exists is not
worth a second way to get it wrong.

### Consensus-critical Ed25519 verification

All implementations MUST apply one identical ZIP-215-derived rule. Given
32-byte encodings `A_enc` and `R_enc`, scalar bytes `S_enc`, message `M`, base
point `B`, subgroup order `L`, and `k = SHA-512(R_enc || A_enc || M) mod L`:

1. decode `A_enc` and `R_enc` as points `A` and `R` on the complete Ed25519
   twisted Edwards curve, following ZIP-215 in **both** of the places where it
   departs from RFC 8032 §5.1.3. Bit 255 of an encoding is the sign of `x` and
   the low 255 bits are `y`:
   a. a `y` whose masked value is `>= 2^255-19` is **not** rejected: it is
      reduced modulo `2^255-19` and the reduced value is decoded. RFC 8032
      §5.1.3 step 2 rejects such an encoding; Coblox does not. No upstream
      `speccheck` vector exercises this clause, which is why the Coblox
      extension vectors below exist and why conformance requires them;
   b. `x = 0` with the sign bit set to 1 is **not** rejected: the conditional
      negation is applied unconditionally, so the encoding decodes to the
      order-2 point `(0, -1)`. RFC 8032 §5.1.3 step 3 rejects this case; Coblox
      does not. Upstream vectors 8–11 exercise this clause, and only this one;
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
| Coblox v0 | reject | reject | accept | accept | accept | accept | reject | reject | reject | accept | reject | reject |

Each rejection above is produced by exactly one of the rules: `[8]A = identity`
on vectors 0, 1, 10 and 11; `S >= L` on vectors 6 and 7; a failed cofactored
equation on vector 8.

**Vectors 8 and 9 are a pair, and they are the reason the paragraph above ends
with "the original encodings, not re-encoded points".** Both carry the same
`R_enc` = `ec ff … ff ff`, which is clause 1b and not clause 1a: its masked
`y` is `2^255 - 20 = p - 1`, a perfectly **canonical** value, and what is
irregular about the encoding is the sign bit set on a point whose `x` is `0`, so
it decodes to the order-2 point `(0, -1)`. Nothing about these two vectors is
reduced modulo `2^255-19`.

They differ only in which `R` their signer digested when computing `k`: vector 8
was crafted with `k` over the **canonically re-encoded** `R` — the same point
with the sign bit cleared — and vector 9 with `k` over the raw `R_enc`. A
verifier that re-encodes points before hashing accepts 8 and rejects 9; a
verifier that hashes the original encodings, as this specification requires,
rejects 8 and accepts 9. **Vector 8 therefore verifies only if an implementation
hashes a re-encoded `R`, so `reject` is its only conformant outcome** — an
`accept` there would contradict the rule stated above it.

> The row previously published `accept` at vector 8. It was compiled by hand and
> never executed; the error was found by [REVIEW-018] during [SPEC-012], which
> delivered the first implementation of this rule, and is corrected here.
>
> The paragraph above previously described the same `R_enc` as a "non-canonical
> `y`" and its counterpart as a "reduced `R`". Both were wrong about the
> mechanism while right about the outcome: `p - 1` is canonical, and what vector
> 8 digests is a re-encoding, not a reduction. [REVIEW-019] found it while
> establishing that clause 1a had no vector at all.

#### Coblox extension vectors

The twelve upstream vectors exercise clause 1b and never clause 1a: no `y` in
any of their twenty-four point encodings is `>= 2^255-19`. An implementation
that reduces `y` as clause 1a requires and one that rejects it as RFC 8032 does
therefore return the **same verdict on all twelve**, and diverge on inputs any
key holder can construct in constant time. Conformance to this section requires
both tables, and an implementation that passes only the twelve has demonstrated
nothing about half of rule 1.

The seven vectors below are versioned at
`core/coblox-core/tests/fixtures/ed25519_coblox_extension.json`, generated by
`sim/tools/ed25519_coblox_extension_vectors.py`, and their expected outcomes are
derived from rules 1–4 above rather than observed from any implementation:

| Vector | 0 | 1 | 2 | 3 | 4 | 5 | 6 |
| --- | --- | --- | --- | --- | --- | --- | --- |
| Coblox v0 | accept | accept | accept | accept | reject | reject | reject |

Vectors 0–3 carry an `R_enc` whose masked `y` is `>= 2^255-19` and whose reduced
point satisfies `[8]R = identity` (`y_raw = p+1` decodes to the identity,
`y_raw = p` to a point of order 4, each with both sign bits). For such an `R`
the cofactored equation collapses to `[8][S]B = [8][k]A`, which the holder of
`a` solves with `S = k·a mod L`; the signature is valid under this rule and
invalid under a decoder that rejects `y >= p`. **These four are not a sample:
they are the complete set of divergent inputs constructible without a discrete
logarithm**, because a non-canonical `A_enc`, or a non-canonical `R_enc` of
large order, would require the discrete logarithm of the reduced point.

Vectors 4–6 carry the remaining non-canonical shapes — `A_enc` reducing to a
point of order 4 (rejected by rule 3), `A_enc` and `R_enc` reducing to points of
large order (rejected by rule 4) — and are rejected under both decoders. They
record decoding coverage, not divergence, and are published as such.

Every signature verifier used for enrollment, envelopes, transactions,
certificates, validator bindings, votes, challenge evidence, and app manifests
MUST pass **both** tables before it can participate in a Coblox network.

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

**`validator_set_hash` does not bind `chain_id`, and the omission is
deliberate.** Its formula is in
[ledger.md](ledger.md#validator-set-continuity) rather than in the list above,
and every preimage in the list above carries `chain_id_32`. The exception is
written here because an undeclared exception reads as an oversight, and a reader
who assumed an oversight would close it and change every published value that
depends on it for no reason.

The claim is stated with its class, because the unqualified version is false and
was checked rather than assumed — twice, since the first correction was itself
still too wide. Six other **domain-separated** preimages of this protocol also
omit `chain_id`, and each omits it for a reason of its own: `chain_id` itself,
which cannot bind its own output; `node_id` and the `account_key` derivations,
which are keys rather than references to a chain object; `object_id` and
`input_hash`, which are content addresses and are **required** to be
chain-independent so the same bytes have one name everywhere; and
`dht_namespace_key`, which binds the genesis block ID, the chain ID's own input.
What `validator_set_hash` is the exception to is the narrower class it belongs
to: **a domain-separated preimage over a chain-specific consensus object that
other consensus objects reference by hash.** Every other member of that class —
`tx_id`, `block_id`, `consensus_parameters_hash`, `policy_hash`,
`parameter_set_hash`, `hosting_rate_card_hash`,
`weak_subjectivity_checkpoint_hash`, `enrollment_request_hash`, `request_hash`,
`response_hash`, `issuer_commitment`, `challenge_randomness`,
`election_entropy`, `election_seed`, `election_ticket`, `admission_tag`,
`enrollment_pow_salt` and `message_id` — carries `chain_id_32`.

**`domain-separated` is doing work in that sentence and is not a flourish.** The
tagged-tree preimages are outside the class and would falsify it if they were
not: `node_leaf` (`0x10`), `app_leaf` (`0x13`), `subscription_leaf` (`0x20`),
`eligible_leaf` (`0x24`), `revocation_leaf` (`0x30`), `candidate_leaf` (`0x40`)
and their interior nodes are all preimages over chain-specific consensus objects
referenced by hash — through `state_root`, `eligible_set_root`,
`revocation_root` — and none of them carries `chain_id`. They are separated by
**tag byte** rather than by domain string, and their exemption is the general
one below: a leaf is reachable only through the root that names it, and that
root is carried by an object already bound to its chain.

**The reason `validator_set_hash` needs no binding is that reason, and it is
placed first because it is the one without an exception.** Every object that
names a validator set by hash is itself chain-bound, on each of the three
surfaces separately:

- **quorum certificates** — the signatures they carry are taken over
  `"coblox-block-vote-v0\0" || chain_id_32 || …`
  ([ledger.md](ledger.md#what-validators-sign)), so a certificate replayed onto
  another chain fails signature verification before its `validator_set_hash` is
  consulted at all;
- **weak subjectivity checkpoints** — the checkpoint preimage carries
  `chain_id_32`, and a client rejects a checkpoint whose `chain_id` is not the
  one it is configured with. The `validator_set_hash` inside it is a field of an
  object that is already bound;
- **set transitions** — `next_validator_set_hash` is a field of a `BlockHeader`,
  and `block_id` carries `chain_id_32`. A transition is never observed outside
  an authenticated header.

A second, independent binding exists in the set's **own bytes**, and it is
recorded here as corroboration rather than as the argument: `election.election_seed`
and each `election_ticket` are derived through `chain_id_32`
([ledger.md](ledger.md#the-derivation)), and every `key_binding_signature` is
taken over the global chain-bound signature procedure, so two chains cannot
share a `validator_set_hash` without those coinciding too.

**Why it is corroboration and not the argument.** On the **genesis** set — the
only set without an `election` record — two of those three bindings do not
exist, and the remaining one, `key_binding_signature`, is taken over the
**placeholder** chain ID and not over the derived one, because the set's bytes
are an input to `genesis_block_id`
([Genesis derivation](#genesis-derivation-and-the-placeholder-chain-id)). What
it does bind on the genesis set is the `network_id` its object carries, which is
the network name and not the chain. The
bytes argument is therefore complete on every set except the one where it would
have to stand alone. The three surfaces above cover the genesis set without
depending on that derivation, which is why they are stated first.

Adding `chain_id_32` to this preimage would restate a binding that is already
there, at the cost of recomputing every published value that depends on it.

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
node `"cblx176fmuouuc5v2xyqqxgef5uwrdqt53yqazdlxwcfl6a63bxarnuyq"`, the public key fixture from
[identity.md](identity.md#node-identifier), algorithm
`"argon2id-leading-zero-bits-v0"`, `difficulty_bits:"4"`, the RFC 9106 second
recommended cost profile (`memory_kib:"65536"`, `iterations:"3"`, `lanes:"4"`),
parameter hash `11` repeated 32 bytes, recent block hash `22` repeated 32 bytes,
and a 64-zero-byte base64url signature. Each `PD-0` has common fields
`schema_version:"0.1"`, `network_id:"fixture"`, zero `chain_id`,
`sequence:"1"`, and `activation_height:"1"`; it uses its matching
`document_kind` and required body, with every numeric value `"1"` except the
enrollment body's algorithm/difficulty/cost values listed above with
`tag_length_bytes:"32"`, and the reward body's
`availability_microtokens_per_unit:"0"` (a positive tariff is **rejected on
acceptance** under [ADR-010], so `"1"` would encode a document no conformant
network can activate),
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
`validator_cooldown_epochs` and `validator_min_capture_epochs` at `"1"`, and
`validator_min_set_size:"8"`. The minimum set size cannot be `"1"` because the
consensus constraint block requires `3 * validator_min_set_size >= 2 * V`
([ADR-010]): for `V = 12`, `3 * 8 = 24 >= 2 * 12 = 24`, which is the exact
floor. Two facts about those numbers are worth
stating, because a fixture teaches a shape whether or not it means to.

The block requires `ceil(V/T) <= c < V/3`, which is **unsatisfiable for `T <= 3`
at any `V`**, so a fixture at `T:"3"` would encode a state no conformant network
can reach; `V:"3"` is impossible for the same kind of reason, since `3c < 3`
cannot hold for any `c >= 1`. Both are proved by exhausting the parameter space
rather than argued.

This fixture also takes `c > 1` on purpose. With `c:"1"` a cohort is a single
seat, so the entry cap is never exercised, and neither is the interaction between
the cap, the term stagger and the contraction floor that the constraints exist to
keep consistent. `V:"12"`, `T:"4"`, `c:"3"`, `min_set:"8"` satisfies that and is
convenient; it is **not** claimed to be the smallest such instance, and no
minimality is claimed for it. Smaller ones with `c > 1` exist. A superlative in
a normative document has to be proved or not written, and this one buys nothing
that would justify proving it. `CMT-0` is the issuer
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
the corresponding empty `revocation_root` `H(0x33)`. `APP-0` is an **app**
account in state `suspended`, for `app_id` `99` repeated 32 bytes, with
`balance_microtokens` 1, `account_nonce` 1 and `suspension_effective_epoch` 1;
its `lifecycle_u8` is therefore `0x03` under the encoding of
[ledger.md](ledger.md#lifecycle_u8-and-why-zero-is-not-active). It is
deliberately **not** `active`: a fixture in the state whose byte an
implementation would guess correctly proves nothing about the encoding, which
is the gap this fixture exists to close. Its `account_key` row is the app-side
derivation, `H("coblox-account-key-v0\0" || 0x01 || app_id_32)`. These
definitions are exact after JCS; no omitted/default fields are implied.

`GEN-0` is the genesis material of the network `genesis-fixture`, and it is what
exercises
[Genesis derivation and the placeholder chain ID](#genesis-derivation-and-the-placeholder-chain-id).
It is deliberately **not** on the `fixture` network of the rows above, and the
reason is worth a sentence because the two look composable and are not.
`HASH-0` fixes `chain_id` to 32 zero bytes **by declaration**, for rows that
need some chain ID and do not care which; the genesis placeholder is 32 zero
bytes **by rule**. The values coincide and the meanings do not, and reading the
registry rows as one chain's genesis would make `WSC-0` inadmissible — a weak
subjectivity checkpoint is never genesis material and always carries the derived
`chain_id`.

`GEN-0` has three parts. Its **genesis `consensus_parameters` document** has
`schema_version:"0.1"`, `document_kind:"consensus_parameters"`,
`network_id:"genesis-fixture"`, `chain_id` the 32 zero bytes of the placeholder,
`sequence:"1"`, `activation_height:"0"`, and the body of the consensus `PD-0`
unchanged. Unchanged on purpose: that body is already published as one the
election constraint block accepts, and a second admissible body invented here
would be a second parameter set in the documents that nothing checks.

Its **genesis header** has `schema_version:"0.1"`, `protocol_version:"0.1"`,
`network_id:"genesis-fixture"`, `height:"0"`, `round:"0"`, `timestamp_ms:"1"`,
`previous_block_id` 32 zero bytes, `transactions_root` `H(0x03)` — the
empty-block root of [ledger.md](ledger.md#hashing-primitives), since `GEN-0`
carries no transactions — `state_root` `ee` repeated 32 bytes,
`validator_set_hash` and `next_validator_set_hash` both `dd` repeated 32 bytes,
and `consensus_parameters_hash` the hash of the document above. The two set
hashes are declared literals rather than the hash of a published `ValidatorSet`,
because publishing a `ValidatorSet` here would publish a genesis cohort, whose
size, stagger and term limits the election constraint block governs, and a
cohort that satisfied all of them is a larger artifact than this fixture needs.
**The consequence is that no published value exercises the `key_binding_signature`
clause of the rule**, which is stated here rather than left for a reader to
notice. It is no longer normative on the strength of its text alone — the clause
is now expressed where the bytes are built, and a conformance test asserts that a
genesis binding is taken under the placeholder and moves with `network_id`
([REVIEW-029] RF-004) — but no *published* value asserts it, and a suite that
reads only this table will not check it.

Its **`chain_id`** is derived from `network_id` `genesis-fixture` — 15 bytes, so
the length prefix is `u32be(15)` — and the `genesis_block_id` above. `DHT-0` is
the Kademlia namespace key of that same genesis block ID, and it is published
here because it had no fixture for want of a genesis block ID and now has one.

`GEN-1` is `GEN-0` with one field changed: `network_id` is `genesis-fixture-b`,
seventeen bytes instead of fifteen. It changes no other value and introduces no
shape `GEN-0` does not already show, and it is published for one reason, stated
because a second fixture that looked redundant would be deleted by the next
reader who tidied. **A derivation fixture that fixes one network name never
exercises `u32be(len(network_id_utf8))`, and never shows that a name enters the
height-0 header as well as the `chain_id` preimage.** With `GEN-1` both are
exercised: every derived value of `GEN-1` differs from its `GEN-0` counterpart,
and the two differ in the length of the one field that changed. Its
`consensus_parameters` document, its header and its `dht_namespace_key` are
those of `GEN-0` with that substitution and nothing else.


| Hash | Fixture | Expected value |
| --- | --- | --- |
| `enrollment_request_hash` | `ER-0` | `sha256:52118f65908736ec7fd837a4d6c1b8c2b3ba28e2f0127cea6e282b311e401e58` |
| `parameter_set_hash` | enrollment `PD-0` | `sha256:a2553f36f496d30a7773b9f6424c3ffd5ef22e3f8620bf0cca88a9bcdccd4f63` |
| `policy_hash` | reward `PD-0` | `sha256:89da35fbb8f0ba3c9ebffc0e3c5987045a005aaa7414356ef16a978a92025c48` |
| `hosting_rate_card_hash` | hosting `PD-0` | `sha256:9b10204164f4197fb368f0f6ad6c186ae7af1a85b7b6383eeac412a10b8b3ae8` |
| `consensus_parameters_hash` | consensus `PD-0` | `sha256:87dc1d92edcd94d5efe3837af9157a4bda604dbd7a658f509bd6fb864f86ada5` |
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
| `account_key` (app) | `APP-0` | `sha256:a881e2e0907aa86b225aaa2a2e1898afda1ce4733bd6d9cb390475ded4737e9d` |
| `app_leaf` | `APP-0` | `sha256:2eac8b0a7955a70543eddf975843fb8e4ddf377daef08b61c7b8cde469515697` |
| `empty_transactions_root` | `GEN-0` | `sha256:084fed08b978af4d7d196a7446a86b58009e636b611db16211b65a9aadff29c5` |
| `consensus_parameters_hash` (genesis) | `GEN-0` document | `sha256:bec637279b6dceb786a0758c8a48de508d6d08bff5878c0b71f844e48da0f275` |
| `block_id` (genesis) | `GEN-0` header | `sha256:1334f5368141f78f23528624bf91973cb4cdf316c1e3452cb0e5470ff7145f92` |
| `chain_id` | `GEN-0` | `sha256:3004d71cffe8ea2cc07b254abcc65494c112c13b20a305910476860b6cc62847` |
| `dht_namespace_key` | `DHT-0` | `sha256:80c13c86cb480fe927e4aafe885b687d5fd2900a2d53e46de0460ee48f943b26` |
| `consensus_parameters_hash` (genesis, `GEN-1`) | `GEN-1` document | `sha256:6ba582b42339763c4b79e7a41ff7d75f6283800a5a4b4d97176f318cb5f63c0d` |
| `block_id` (genesis, `GEN-1`) | `GEN-1` header | `sha256:6b62539240dcbc9aedf3e47e32edef91d302cf0687865dad8904326d8f49c53d` |
| `chain_id` (`GEN-1`) | `GEN-1` | `sha256:172fd2e8bbdffefecc8952c1e0b97b69275af0de9bc637c6735a09b872d5e033` |
| `dht_namespace_key` (`GEN-1`) | `GEN-1` | `sha256:e8ceaa4c9095078ae2347bb111484ed532e5c494e49341aba2f5b57312d72c7b` |

`challenge_randomness` is carried on the wire as the unpadded base64url of those
32 bytes, which for `RND-0` is `jOvkrYkL1B6MN7h62XatkrjvNaoyhMRB2GaRz9qtiNc`.

Conformance suites MUST reconstruct every preimage from these definitions and
compare all 32 digest bytes; checking only presentation strings is insufficient.

#### Inline examples are not conformance oracles

The one-line `json` blocks throughout these documents exist to fix **canonical
form** — key order, string escaping, integer spelling, base64url padding, hash
presentation. Their `sha256:` values are illustrative placeholders and are
**not** claimed to be the digests of anything. The same placeholder appears as a
`parameter_set_hash` in one example and a `policy_hash` in another, which no
real chain could produce, because the examples are about shape and not about
provenance. **The fixture table above is the only oracle in these documents.** A
suite that treats an inline example's hash field as an expected value is testing
a value nobody computed.

Two obligations survive that, and both are checked by
`sim/tools/published_artifacts.py`:

- an inline example MUST still satisfy every equality the specification states
  between its own fields — `challenge_id` equals `request_hash` is the one such
  equality in v0, and the challenge-evidence example of
  [ledger.md](ledger.md#challenge-evidence) violated it until 2026-08-25;
- no `sha256:` literal may exist in these documents without being classified, in
  the published-artifact manifest, as either a registry value or a placeholder.
  An unclassified one is a value a reader can mistake for an expectation.

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
  "max_transport_attestation_validity_ms":u64-string,
  "max_transport_attestation_future_skew_ms":u64-string,
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
`m = 2^21` (2 GiB) — which is the *stronger* of the two.

**What the two floors actually guarantee**, stated as the property the rules
hold rather than as the property one would like them to hold:

1. no admitted configuration has less than 64 MiB of memory, so no admitted
   configuration is compute-bound;
2. no admitted configuration has a cost-area below 196,608 KiB-passes, which is
   the area of the RFC's second recommended profile;
3. both RFC recommended profiles are admitted.

That is **not** the same as "everything weaker than either recommendation is
rejected", which an earlier version of this section claimed and which is
broader than the rules impose. The area form admits a band that matches neither
recommendation: `iterations = 1` with `196608 <= memory_kib < 2097152` — 192 MiB
at a single pass, up to just under 2 GiB — satisfies both floors. That band is
admitted **on purpose and with its cost named**: it has the same KiB-passes as
the second recommendation and more memory than it, so it is not weaker by
either quantity the rules measure, but the RFC does not recommend it and this
document does not claim it is equivalent. Narrowing the rule to the two named
profiles was rejected: a rule that enumerates the current recommendations of
one RFC is a whitelist that ages the moment that RFC is revised, and this
document already carries four occurrences of a published value outliving the
rule that made it true.

Boundary conformance fixtures, which a suite MUST exercise:

| `memory_kib` | `iterations` | Verdict | Reason |
| --- | --- | --- | --- |
| `"65536"` | `"3"` | valid | RFC second recommended profile; exactly at the floor |
| `"65535"` | `"3"` | **invalid** | below the memory-hardness floor |
| `"65536"` | `"2"` | **invalid** | area `131072 < 196608` |
| `"2097152"` | `"1"` | valid | RFC first recommended profile, 2 GiB |
| `"196608"` | `"1"` | valid | the admitted band above: neither recommendation, both floors met |
| `"196607"` | `"1"` | **invalid** | area `196607 < 196608`; the low edge of that band |
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

#### The availability tariff is zero as a validity rule

A `reward_policy` document is **invalid in acceptance** if
`availability_microtokens_per_unit != 0`.

The reason is structural and load-bearing: `work_compensation` for
`availability` is the only channel that pays **per node without an aggregate
cap**. If this tariff were positive, an adversary operating a fleet of `N`
emulated identities would increase total network emission linearly with `N`,
destroying criterion (a) of [ADR-007] by construction rather than by
misconfiguration. Enforcing this tariff at zero on acceptance prevents any
sitting validator quorum from introducing uncapped per-node issuance through
routine governance ([ADR-010]). If availability is ever to be compensated, it
MUST be through a capped aggregate fund divided among eligible nodes, never
through an uncapped per-unit rate.

#### Cap proportional to eligible nodes is explicitly rejected

A fund cap proportional to the number of eligible nodes `E` (such as
`F = k * E`) is **explicitly rejected**. Such a rule would allow an adversary
running a fleet of emulated nodes to inflate `E`, raising the fund
proportionally and restoring the per-node payout under another name, directly
reopening criterion (a) of [ADR-007] ([ADR-011]).

Boundary conformance fixtures for reward policy acceptance:

| Parameter | Value | Verdict | Reason |
| --- | --- | --- | --- |
| `availability_microtokens_per_unit` | `"0"` | valid | availability has no per-unit rate; paid via capped fund |
| `availability_microtokens_per_unit` | `"1"` | **invalid** | positive rate creates uncapped per-node emission |
| `availability_microtokens_per_unit` | `"1000"` | **invalid** | positive rate rejected on acceptance |
| `publisher_reward_cap_numerator` / `_denominator` | `"1"` / `"2"` | valid | `kn < kd` strictly lossy |
| `publisher_reward_cap_numerator` / `_denominator` | `"2"` / `"2"` | **invalid** | `kn >= kd` not lossy |
| `publisher_reward_cap_numerator` / `_denominator` | `"1"` / `"0"` | **invalid** | division by zero |

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
derived chain ID, genesis validator set, initial protocol documents, the
election bounds, reward bounds and cadence band below, and a weak subjectivity
checkpoint. A fresh client MUST refuse
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

### Reward bounds

The anti-Sybil economic defense of [ADR-007] is parameterized by the
`reward_policy` document — which is signed by a sitting validator quorum.
Constraining those parameters only by internal relations would leave the
economic defense switchable off by the very quorum it constrains: an unbounded
existence fund `F` or an eroded validator eligibility threshold could be
enacted in a single valid document, turning a bounded loss into an unbounded
drain without leaving any distinguishing on-chain trace. This is the third
instance of the structural pattern previously resolved for the Argon2id cost
floor and for `ElectionBounds` ([ADR-010]), and it is resolved symmetrically.

`RewardBounds` is therefore **configuration, not chain state**. It ships inside
the signed network distribution and in no other channel, cannot be changed by
any on-chain document, and MUST NOT be learned from a peer, a header, or a
protocol document. Changing it is a new authenticated release and a chain-level
decision.

```text
RewardBounds = {
  "schema_version":"0.1",
  "network_id":string,
  "chain_id":sha256-string,
  "existence_fund_microtokens_per_epoch_max":u64-string,
  "reward_epoch_ms_min":u64-string,
  "reward_epoch_ms_max":u64-string,
  "publisher_reward_cap_numerator_max":u64-string,
  "publisher_reward_cap_denominator_min":u64-string,
  "validator_eligibility_threshold_units_min":u64-string,
  "validator_eligibility_window_epochs_max":u64-string,
  "validator_eligibility_min_issuers_min":u64-string,
  "storage_units_per_contribution_unit_max":u64-string,
  "compute_units_per_contribution_unit_max":u64-string,
  "storage_microtokens_per_byte_epoch_min":u64-string,
  "compute_microtokens_per_million_fuel_min":u64-string,
  "reward_parameter_change_numerator":u64-string,
  "reward_parameter_change_denominator":u64-string,
  "reward_parameter_min_activation_gap_blocks":u64-string
}
```

`chain_id` MUST equal the client's configured chain ID,
`reward_parameter_change_numerator` MUST exceed
`reward_parameter_change_denominator`, which MUST be positive, and
`reward_parameter_min_activation_gap_blocks` MUST be positive.
`reward_epoch_ms_min` MUST be positive and MUST NOT exceed `reward_epoch_ms_max`.

**The question each bound answers.** For every quantity in `RewardPolicyBody`
the question is not "does this need a limit?" but **"does a declared security
property depend on this quantity?"** — and the dependency is not only the
obvious one. A quantity that appears in the **denominator** of a bounded
quantity, or that **denominates the unit** a bounded quantity is expressed in,
carries the property just as much as the quantity that is named in the ADR. An
earlier version of this section bounded the quantities the ADR named and left
their denominators and their units governed; the bounds below were extended
after that was found ([ADR-010] applied to itself).

The three structural components of `RewardBounds` are defined with their
specific rationale:

1. **Magnitudes (genesis ceilings and floors):**
   - `existence_fund_microtokens_per_epoch_max`: Sets a hard genesis ceiling
     on the per-epoch existence fund `F`. Because the absolute amount diverted
     by an emulated fleet is `D = F · N/(N+H)` and does not contain network
     usage, `F` is the sole determinant of total loss; bounding `F` at genesis
     ensures that governance cannot inflate the fund beyond the declared
     maximum risk budget.
   - `publisher_reward_cap_numerator_max` and `publisher_reward_cap_denominator_min`:
     Bound the creator reward cap parameters so that `kn / kd` is strictly below 1,
     preserving the structurally lossy property of self-subscription cycles.
   - `validator_eligibility_threshold_units_min`: Sets a floor under the
     contribution score required to enter the validator candidate pool,
     preventing governance from reducing the entry barrier to unproven presence.
   - `validator_eligibility_min_issuers_min`: Sets a floor (at least 2) on the
     number of independent challenge issuers required to qualify for validator
     candidacy, preventing score fabrication by a single colluder.
   - `reward_epoch_ms_min` and `reward_epoch_ms_max`: **The epoch is the
     denominator of every cap in this object.** `existence_fund_microtokens_per_epoch_max`
     caps `F` *per epoch*, so real issuance per unit of wall-clock time is
     `F / reward_epoch_ms`; shortening the epoch from one day to 86 400 ms
     multiplies real issuance by one thousand without exceeding a single
     magnitude bound. The floor is the security-relevant direction, and the
     ceiling is there because an epoch stretched without limit freezes issuance
     entirely, which is no less a decision about the network than inflating it.
     `storage_microtokens_per_byte_epoch` is denominated per epoch for the same
     reason and inherits the same protection.
   - `storage_units_per_contribution_unit_max` and
     `compute_units_per_contribution_unit_max`: **The unit in which
     `validator_eligibility_threshold_units_min` is denominated.** The
     contribution score compared against that floor is computed by dividing
     measured physical work by these divisors, so multiplying one of them makes
     the floor satisfiable with proportionally less real work: the floor stays
     signed and respected to the letter and stops meaning anything. They are two
     **independent** factors, so changing one also reweights storage against
     compute and moves who is eligible without touching any bounded quantity. A
     floor denominated in a governed unit is not a floor.
   - `validator_eligibility_window_epochs_max`: Contribution units accumulate
     over the last `validator_eligibility_window_epochs` reward epochs, so a
     threshold expressed in units over an unbounded window drives the required
     **rate** of contribution toward zero. The bound needed is a **maximum**,
     which is the opposite direction to the one intuition suggests; the block's
     `>= 1` bounds the harmless end.
   - `storage_microtokens_per_byte_epoch_min` and
     `compute_microtokens_per_million_fuel_min`: **The denominator of the
     surveilled ratio.** The fraction of emission flowing through the existence
     channel is `F / (F + W)`, where `W` is what the work channels pay. Its
     numerator is now capped; without floors under these two tariffs a quorum
     may set them to zero, driving `W` to zero and the ratio to one on a mature
     network, with no on-chain trace distinguishing that document from routine
     downward tuning. The dangerous direction here is **downward**, which is why
     these are floors and not ceilings.

   **Assessment of remaining reward policy parameters:** For every other
   quantity in `RewardPolicyBody`, its relation to declared security properties
   is explicit:
   - `availability_microtokens_per_unit`: Governed by the strict validity rule
     `availability_microtokens_per_unit == 0` (rejected on acceptance if non-zero),
     hence requiring no separate magnitude bound.
   - `publisher_microtokens_per_active_subscriber`: Rate parameter governed by the
     mandatory creator share cap (`kn < kd`), which is itself bounded above.

2. **Rate of change ratio (`reward_parameter_change_numerator` / `reward_parameter_change_denominator`):**
   Bounds the maximum relative adjustment (e.g. 5/4, or 25% per document) between
   consecutive sequence versions of **every** `u64` quantity in
   `RewardPolicyBody`, without exception, against the currently active document:

   ```text
   x_new * den <= x_old * num   and   x_old * den <= x_new * num
   ```

   The scope is stated once, here, and is deliberately "every quantity" rather
   than "the bounded ones". An earlier version said "bounded reward parameters"
   in this paragraph and "any parameter" in the closing one; a textual ambiguity
   about which quantities the only residual defence covers is not a defence, and
   the wider reading is both simpler to verify and the safe one. A quantity fixed
   by a validity rule — `availability_microtokens_per_unit == 0` — is unaffected,
   because it may not change at all. This prevents a sitting quorum from jumping
   a parameter to its genesis ceiling, or its denominator to its floor, in a
   single step, converting parameter changes into an observable process.

3. **Minimum activation gap (`reward_parameter_min_activation_gap_blocks`):**
   Requires a minimum spacing in chain height between activations of consecutive
   `reward_policy` documents. This ensures the change rate limit is priced per
   unit of chain time, giving network participants sufficient blocks to observe,
   evaluate, and respond to governance changes.

A `reward_policy` document whose bounded parameters violate these limits, or
which adjusts any quantity of `RewardPolicyBody` faster than the permitted ratio
or closer than the activation gap against the active document, is **rejected on
acceptance**.

Boundary conformance fixtures for `RewardBounds` acceptance. `F_max` below is
`existence_fund_microtokens_per_epoch_max`, the ratio is 5/4, and the active
document is the one immediately preceding:

| Quantity | Situation | Verdict | Reason |
| --- | --- | --- | --- |
| `existence_fund_microtokens_per_epoch` | exactly `F_max` | valid | at the ceiling, not above it |
| `existence_fund_microtokens_per_epoch` | `F_max + 1` | **invalid** | magnitude bound exceeded |
| `existence_fund_microtokens_per_epoch` | `x_old * 5 / 4` exactly | valid | at the permitted ratio |
| `existence_fund_microtokens_per_epoch` | `x_old * 5 / 4 + 1` | **invalid** | rate of change exceeded |
| `reward_epoch_ms` | exactly `reward_epoch_ms_min` | valid | at the floor |
| `reward_epoch_ms` | `reward_epoch_ms_min - 1` | **invalid** | shortening the epoch inflates real issuance |
| `reward_epoch_ms` | `reward_epoch_ms_max + 1` | **invalid** | an unbounded epoch freezes issuance |
| `reward_epoch_ms` | `86 400 000 -> 86 400` in one document | **invalid** | rate of change exceeded by a factor of 1000 |
| `storage_units_per_contribution_unit` | exactly its `_max` | valid | at the ceiling |
| `storage_units_per_contribution_unit` | `_max + 1` | **invalid** | redenominating the eligibility unit |
| `compute_units_per_contribution_unit` | `_max + 1` | **invalid** | redenominating the eligibility unit |
| `validator_eligibility_window_epochs` | exactly its `_max` | valid | at the ceiling |
| `validator_eligibility_window_epochs` | `_max + 1` | **invalid** | drives the required contribution rate toward zero |
| `storage_microtokens_per_byte_epoch` | exactly its `_min` | valid | at the floor |
| `storage_microtokens_per_byte_epoch` | `0` | **invalid** | empties the denominator of the surveilled ratio |
| `compute_microtokens_per_million_fuel` | `0` | **invalid** | empties the denominator of the surveilled ratio |
| `validator_eligibility_threshold_units` | exactly its `_min` | valid | at the floor |
| `validator_eligibility_threshold_units` | `_min - 1` | **invalid** | lowers the candidate-pool entry barrier |
| `activation_height` | active `+ reward_parameter_min_activation_gap_blocks` | valid | at the gap |
| `activation_height` | active `+ gap - 1` | **invalid** | spacing not respected |

The table is normative in form and not in values: the magnitudes come from the
genesis distribution, and a conformance suite substitutes its own before using
it, exactly as it does for the consensus-parameters fixtures.

### Cadence band

`block_interval_seconds` is declared and not enforced, and
[genesis constants](#genesis-constants) says why no validity rule could enforce
it. The cadence band is the other half of that statement: the tolerance against
which the real production rate is **measured**, by the two parties that hold a
clock no validator wrote — a light client, and the process that releases
checkpoints.

`CadenceBand` is **configuration, not chain state**, for the same reason as the
two bounds objects above. It ships inside the signed network distribution and in
no other channel, cannot be changed by any on-chain document, and MUST NOT be
learned from a peer, a header, or a protocol document. A band a sitting quorum
could widen would be a tolerance underneath the only measurement this protocol
has of that quorum's own behaviour.

```text
CadenceBand = {
  "schema_version":"0.1",
  "network_id":string,
  "chain_id":sha256-string,
  "block_interval_ms":u64-string,
  "min_ms_per_block":u64-string,
  "max_ms_per_block":u64-string,
  "min_measured_blocks":u64-string,
  "max_external_clock_slack_ms":u64-string
}
```

`chain_id` MUST equal the client's configured chain ID; `block_interval_ms`,
`min_ms_per_block`, `min_measured_blocks` and `max_external_clock_slack_ms`
MUST be positive; `min_ms_per_block <= block_interval_ms <= max_ms_per_block`
MUST hold; and `max_external_clock_slack_ms` MUST be less than
`min_measured_blocks * block_interval_ms`.

The first relation is what makes the object mean its name: a band that excluded
the interval the protocol declares would put every conformant chain permanently
out of band, and a guard that fires on everything fires on nothing.
`min_measured_blocks` is a floor on the **numerator** — a ratio over a handful
of blocks is noise, and a measurement below it is **not made**, which is
reported as such and never as a pass.

**`max_external_clock_slack_ms` is the floor the denominator needs, and it is
there because for one revision this section had only the other one.** A light
client counts blocks from the checkpoint's `height` but counts time from its
`issued_at_ms`, and those are not the same instant: this document says two
sections below that `issued_at_ms` is when the checkpoint was *produced*, which
is after the height it names was finalized. Every block produced in between is
counted **without its time**, so the measured rate is faster than the real one
by an amount that has nothing to do with the chain. A client clock that is
behind, or a release clock that is ahead, shortens the same interval. The three
are indistinguishable inside the measurement and they add, which is why one
field bounds their sum rather than three fields bounding one term each.

The second relation couples it to the window: the tolerance must be smaller than
the real time an honest chain takes to produce the smallest measurable window,
`min_measured_blocks * block_interval_ms`. Above that, most of the blocks
counted would be blocks the tolerance exists to excuse, and a tolerance larger
than the measurement it qualifies is not a tolerance. A deployment whose release
latency trips this raises `min_measured_blocks` until the window dominates the
shortfall.

**A shortcut this protocol does not take.** The checkpoint also carries
`timestamp_ms`, and `issued_at_ms - timestamp_ms` looks like a free measurement
of the release latency, per checkpoint and exact. It MUST NOT be used for it.
`timestamp_ms` is written by the validators, so a client deriving its own
tolerance from it would let the measured party set the tolerance it is measured
against — [ADR-013] part 3 arriving through a door nobody was watching. The
slack is a genesis constant for exactly this reason.

**How the band differs from the two bounds objects, and it is not a detail.**
`ElectionBounds` and `RewardBounds` bound values that a signed document carries,
so a document outside them is rejected on acceptance. The cadence band bounds
**nothing any document carries**. It is applied to a measurement whose two
endpoints are outside the chain, and **no validity rule of this protocol
compares anything to it**. A chain running outside its band is not invalid; it
is *observably* outside its band, which is the strongest true statement
available and is the one made here.

**Where it is applied**, and the two applications are deliberately asymmetric:

- a **light client** measures from the checkpoint it holds to the header it has
  authenticated, at step 4b of the
  [light-client algorithm](ledger.md#light-client-balance-verification). It
  fails closed above the band, after `max_external_clock_slack_ms` has been
  granted to the measured interval, and **reports** below it;
- the **checkpoint release process** measures between two consecutive
  checkpoints it has itself signed, and MUST NOT issue a checkpoint for a chain
  whose measurement is outside the band **in either direction**, or whose
  interval is too short to measure. It grants **no** slack, because both of its
  endpoints are `issued_at_ms` values it produced itself: the release latency
  appears in both and cancels, and so does a constant offset in its own clock.
  It has neither sync lag nor a chain clock in its inputs, so it is the party
  entitled to fail closed both ways — and it can wait, which a light client
  asking for a balance cannot.

**Why the two directions are treated differently, stated in the form that
survives scrutiny.** Both ends of the client's measurement are biased, and they
push the ratio in **opposite** directions: the block count is short by sync lag,
and the elapsed time is short by release latency and clock error. Neither
verdict is therefore attributable to the chain on its own. What separates them
is what lies past the tolerance: nothing honest makes blocks appear, so a fast
reading beyond `max_external_clock_slack_ms` has no innocent explanation, while
a slow reading is indistinguishable from the client's own lag **at any
magnitude** and no tolerance would change that. The client fails closed where a
reading can be attributed and reports where it cannot.

**And the two directions do not cost an attacker the same.** Slowing production
down requires only a **blocking third**, which withholds the quorum. Speeding it
up requires a **quorum**, because every block carries a quorum certificate under
the [strict quorum predicate](ledger.md#quorum-predicate). The side a light
client fails closed on is the more expensive one; the cheaper one it only
reports. That is a consequence of where attribution is possible, not a judgement
that the cheaper side matters less.

`timestamp_ms` is not an input to either measurement, and MUST NOT become one.
Both endpoints of both measurements are external to the chain by construction:
the `issued_at_ms` of a checkpoint, signed by the release key, and the wall
clock of the party doing the measuring. Using the header's own timestamp would
be measuring the validators with the validators' own clock.

What is fixed by this document is that the band exists, that it lies outside
on-chain governance, that the declared interval lies inside it, and that a
network which ships no `CadenceBand` is not a conformant Coblox network. The
values themselves are a genesis decision of the network operator, taken on the
reasoning recorded in [ADR-016] and written here because a trust anchor whose
values live somewhere else is not one.

#### The genesis band

| Field | Value | Read against the declared interval |
| --- | --- | --- |
| `block_interval_ms` | `5000` | the [genesis constant](#genesis-constants) itself |
| `min_ms_per_block` | `2500` | `block_interval_ms / 2` |
| `max_ms_per_block` | `20000` | `4 * block_interval_ms` |
| `min_measured_blocks` | `720` | one hour of chain at the declared interval |
| `max_external_clock_slack_ms` | `600000` | ten minutes, and `600000 < 720 * 5000 = 3600000` |

These satisfy the rules stated above: every field is positive,
`2500 <= 5000 <= 20000`, and the tolerance is smaller than the smallest
measurable window.

**These five numbers do not limit the cadence, and reading them as a limit is
the misreading this section exists to prevent.** The chain's real production
rate is chosen by whoever produces the blocks;
[genesis constants](#genesis-constants) says why no rule of this protocol
reaches it, and this band does not become such a rule. What the band fixes is
the threshold past which the rate stops being unremarkable to the two parties
that hold an external clock. Each side therefore has a cost; the two sides do
not trade off against the same thing, so the costs are stated separately; and
each is a quantity rather than a quality, because a quality is what a reader
supplies for themselves when a document declines to.

**The slow side, `4 * block_interval_ms`, costs four.** An active validator set
can stretch its own epochs to **four times** their declared real-time length
before any measurement says so. The anti-capture guarantees of
[validator election and rotation](ledger.md#validator-election-and-rotation)
are true in **epochs**; their translation into days belongs to whoever produces
the epochs, and `4 ×` is the factor this band concedes them — nine epochs of a
maximum term are nine epochs either way, and up to four times as long in real
time. The side is where it is because narrower is worse rather than safer: a
band of `2 × block_interval_ms` calls a network out of band during an ordinary
partition, and the release process fails closed in **both** directions, so that
verdict would stop checkpoints from being issued during an event that is not an
attack — withdrawing the only external clock a light client has, for a reason
that is not a manoeuvre. A band of `20 ×` would let a set double its terms in
real time before anything said so, which is a guard that exists and says
nothing.

**The fast side, `block_interval_ms / 2`, objects at a doubling.** It admits a
real issuance rate up to **twice** the intended one and refuses beyond it, and
it is issuance rather than pace because `reward_epoch` is paced by height
([ledger.md](ledger.md#reward_epoch-is-derived-from-height)). This is the side a
light client **fails closed** on, and it is narrower than the
`block_interval_ms / 4` this document uses elsewhere to illustrate a wide fast
side, which would permit four times the intended real issuance rate before the
measurement objected. It is the side worth being severe on because it is the
only one where an attacker's gain is direct, and because nothing honest makes
blocks appear.

**`min_measured_blocks = 720` is the noise floor, and its cost is latency.** A
band measured over an hour sees an hour late, and inside the first 720 blocks
past a checkpoint the measurement is **not made** and is reported as not made.

**`max_external_clock_slack_ms = 600000` is a choice about the release latency
this deployment expects, and not a measurement of one.** It is the field sized
against the worst-case checkpoint release latency plus the worst-case clock
error tolerated at either end. A slack of
`L * min_ms_per_block / block_interval_ms` is exactly enough to absorb a release
latency of `L` on a chain running at the declared interval, and a slack of `L`
absorbs it with room; ten minutes is chosen against an expected worst case and
MUST be re-examined once a real release process exists to measure. Sizing it
**below** the release process's real latency is a declared choice to fail closed
on that process's own delay, and sizing it at or above
`min_measured_blocks * block_interval_ms` is refused, because the tolerance
would then exceed the window it qualifies — a deployment that needs more slack
raises `min_measured_blocks` instead. Its cost is the only cost a tolerance has:
a genuinely fast chain goes unreported until the measured window grows past it.

**Narrower is a release; wider is not available.** A new signed distribution can
narrow any of these values without touching genesis, so a band chosen wide
before a network exists is a postponement and not a concession. No on-chain
document can widen it, for the reason given at the head of this section.

**Declared limit.** The band's trustworthiness is the release channel's, exactly
as for `ElectionBounds` and the trust key. And the measurement is a measurement:
it says at what rate blocks arrived, never why. An honest network under a
partition and a cartel stretching its own incumbency produce the same reading,
and the protocol does not distinguish them. What the band removes is not the
manoeuvre; it is the manoeuvre's invisibility.

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

**Release procedure: a checkpoint is not issued for a chain outside its band.**
Before signing a checkpoint at `height` with `issued_at_ms`, the release process
MUST measure the cadence against the last checkpoint it signed for the same
chain — `(height - previous.height)` blocks over
`(issued_at_ms - previous.issued_at_ms)` real milliseconds — and MUST NOT issue
the checkpoint when that measurement falls outside the
[cadence band](#cadence-band) in either direction, or when it spans fewer than
`min_measured_blocks` blocks. The first checkpoint of a chain has no
predecessor and is exempt, which is stated rather than left to be inferred.

**The same procedure bounds its own latency, and the bound is the band's.** The
release process MUST NOT sign a checkpoint whose `issued_at_ms` is more than
`max_external_clock_slack_ms` after it observed the finality of the `height` it
names. This is the other half of that field: the client grants a tolerance, and
the procedure is what makes the tolerance an upper bound on something real
rather than a guess. A process that cannot meet it publishes a checkpoint on a
more recent height instead of a stale one on an old height.

**Declared limit.** If the release process violates that obligation, clients
past the tolerance report the chain as faster than its band and fail closed on
an honest chain. That is a fail-closed produced by the party holding the
external clock, and the protocol accepts it in that direction: a client cannot
tell a late checkpoint from a fast chain, and the alternative — deriving the
latency from the checkpoint's own `timestamp_ms` — would hand the choice of
tolerance to the validators. The containment is that the obligation is written,
that the two numbers are the same number, and that a chain's own operators can
observe their release latency directly.

This is a **procedure, not a validity rule**, and the distinction is the same
one the band itself rests on. Withholding a checkpoint does not stop a chain and
is not meant to: it withdraws the external clock from a chain that is not
running at the declared rate, so a client that fails closed on checkpoint
staleness fails closed on that chain too. The effect is real and it is indirect,
and calling it enforcement would overstate it.

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
