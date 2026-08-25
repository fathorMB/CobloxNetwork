//! The hash preimage registry of `docs/protocol/README.md#hash-preimage-registry`.
//!
//! One function per registry entry, in the order the registry lists them. Each
//! reconstructs its preimage from typed inputs, so a conformance suite compares
//! all 32 digest bytes rather than a presentation string, as the registry
//! requires.
//!
//! Objects enter these functions as [`JsonObject`], which can only be
//! serialized canonically, so "the JSON object is validated and JCS-serialized
//! before hashing" holds by construction rather than by discipline.

use crate::error::{ElectionError, Result};
use crate::hash::{AccountKey, ChainId, Digest32, Domain, PreimageWriter};
use crate::json::JsonObject;

/// `enrollment_request_hash`.
#[must_use]
pub fn enrollment_request_hash(chain_id: &ChainId, request: &JsonObject) -> Digest32 {
    PreimageWriter::new(Domain::ENROLLMENT_REQUEST_HASH)
        .chain(chain_id)
        .jcs(request)
        .finish()
}

/// The four governed document kinds, each with its own hash domain.
///
/// "The hash domain MUST match `document_kind`" — so the domain is selected
/// from this enum rather than passed independently, and
/// [`protocol_document_hash`] additionally checks that the document says what
/// the caller says it says.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DocumentKind {
    /// `parameter_set_hash`.
    EnrollmentParameters,
    /// `policy_hash`.
    RewardPolicy,
    /// `hosting_rate_card_hash`.
    HostingRateCard,
    /// `consensus_parameters_hash`.
    ConsensusParameters,
}

impl DocumentKind {
    /// The `document_kind` string this kind carries.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::EnrollmentParameters => "enrollment_parameters",
            Self::RewardPolicy => "reward_policy",
            Self::HostingRateCard => "hosting_rate_card",
            Self::ConsensusParameters => "consensus_parameters",
        }
    }

    /// The hash domain bound to this kind.
    #[must_use]
    pub const fn domain(self) -> Domain {
        match self {
            Self::EnrollmentParameters => Domain::ENROLLMENT_PARAMETER_SET,
            Self::RewardPolicy => Domain::REWARD_POLICY,
            Self::HostingRateCard => Domain::HOSTING_RATE_CARD,
            Self::ConsensusParameters => Domain::CONSENSUS_PARAMETERS,
        }
    }
}

/// The governed-document hash for `kind` over an `UnsignedProtocolDocument`.
///
/// Rejects a document whose `document_kind` field does not equal `kind`, which
/// is what makes the domain/kind agreement a checked fact rather than a
/// convention at the call site.
pub fn protocol_document_hash(
    kind: DocumentKind,
    chain_id: &ChainId,
    document: &JsonObject,
) -> Result<Digest32> {
    let declared = document.string("document_kind")?;
    if declared != kind.as_str() {
        return Err(crate::error::JsonError::Field("document_kind".to_owned()).into());
    }
    Ok(PreimageWriter::new(kind.domain())
        .chain(chain_id)
        .jcs(document)
        .finish())
}

/// `object_id = H("coblox-storage-object-v0\0" || u64be(len) || bytes)`.
///
/// Note the absence of a chain binding: a storage object is content-addressed
/// and the same bytes have the same identifier on every chain.
pub fn object_id(object_bytes: &[u8]) -> Result<Digest32> {
    let length = u64::try_from(object_bytes.len())
        .map_err(|_| crate::error::Error::Arithmetic("object length"))?;
    Ok(PreimageWriter::new(Domain::STORAGE_OBJECT)
        .u64be(length)
        .raw(object_bytes)
        .finish())
}

/// `input_hash = H("coblox-compute-input-v0\0" || u64be(len) || bytes)`.
pub fn input_hash(input_bytes: &[u8]) -> Result<Digest32> {
    let length = u64::try_from(input_bytes.len())
        .map_err(|_| crate::error::Error::Arithmetic("input length"))?;
    Ok(PreimageWriter::new(Domain::COMPUTE_INPUT)
        .u64be(length)
        .raw(input_bytes)
        .finish())
}

/// `request_hash`, over the challenge request without `challenge_id` and
/// without `issuer_signature`. `challenge_id` MUST equal this value.
#[must_use]
pub fn challenge_request_hash(
    chain_id: &ChainId,
    request_without_id_or_signature: &JsonObject,
) -> Digest32 {
    PreimageWriter::new(Domain::CHALLENGE_REQUEST_HASH)
        .chain(chain_id)
        .jcs(request_without_id_or_signature)
        .finish()
}

/// `response_hash`, over the challenge response without `subject_signature`.
#[must_use]
pub fn challenge_response_hash(
    chain_id: &ChainId,
    response_without_signature: &JsonObject,
) -> Digest32 {
    PreimageWriter::new(Domain::CHALLENGE_RESPONSE_HASH)
        .chain(chain_id)
        .jcs(response_without_signature)
        .finish()
}

/// `issuer_commitment`.
pub fn issuer_commitment(
    chain_id: &ChainId,
    issuer_node_id: &str,
    commitment_epoch: u64,
    issuer_secret: &[u8; 32],
) -> Result<Digest32> {
    Ok(PreimageWriter::new(Domain::CHALLENGE_ISSUER_COMMITMENT)
        .chain(chain_id)
        .length_prefixed_utf8(issuer_node_id)?
        .u64be(commitment_epoch)
        .raw(issuer_secret)
        .finish())
}

/// `challenge_randomness`.
pub fn challenge_randomness(
    chain_id: &ChainId,
    beacon_height: u64,
    beacon_block_id: &Digest32,
    issuer_commitment: &Digest32,
    issuer_secret: &[u8; 32],
    subject_node_id: &str,
) -> Result<Digest32> {
    Ok(PreimageWriter::new(Domain::CHALLENGE_RANDOMNESS)
        .chain(chain_id)
        .u64be(beacon_height)
        .raw32(beacon_block_id)
        .raw32(issuer_commitment)
        .raw(issuer_secret)
        .length_prefixed_utf8(subject_node_id)?
        .finish())
}

/// `election_entropy` over the entropy window, in ascending height order.
///
/// The declared `election_entropy_blocks` enters the preimage *and* fixes the
/// number of block IDs, so a window of the wrong length is rejected here rather
/// than hashed into a plausible seed.
pub fn election_entropy(
    chain_id: &ChainId,
    election_epoch: u64,
    election_entropy_blocks: u64,
    entropy_block_ids: &[Digest32],
) -> Result<Digest32> {
    if u64::try_from(entropy_block_ids.len())
        .map_err(|_| crate::error::Error::Arithmetic("entropy window"))?
        != election_entropy_blocks
    {
        return Err(ElectionError::EntropyWindow {
            expected: election_entropy_blocks,
            actual: entropy_block_ids.len(),
        }
        .into());
    }
    let mut writer = PreimageWriter::new(Domain::ELECTION_ENTROPY)
        .chain(chain_id)
        .u64be(election_epoch)
        .u64be(election_entropy_blocks);
    for block_id in entropy_block_ids {
        writer = writer.raw32(block_id);
    }
    Ok(writer.finish())
}

/// `election_seed`.
///
/// The seed depends on the entropy window and on nothing else: `candidate_root`
/// and `candidate_count` are bound by validity and are deliberately not in the
/// preimage. This signature is the enforcement of that: there is no parameter
/// through which they could be mixed in.
#[must_use]
pub fn election_seed(chain_id: &ChainId, election_epoch: u64, entropy: &Digest32) -> Digest32 {
    PreimageWriter::new(Domain::ELECTION_SEED)
        .chain(chain_id)
        .u64be(election_epoch)
        .raw32(entropy)
        .finish()
}

/// `election_ticket`.
#[must_use]
pub fn election_ticket(chain_id: &ChainId, seed: &Digest32, account_key: &AccountKey) -> Digest32 {
    PreimageWriter::new(Domain::ELECTION_TICKET)
        .chain(chain_id)
        .raw32(seed)
        .raw(account_key.as_bytes())
        .finish()
}

/// `enrollment_pow_salt`: the **first 16 bytes** of its digest.
#[must_use]
pub fn enrollment_pow_salt(
    chain_id: &ChainId,
    public_key: &[u8; 32],
    recent_block_id: &Digest32,
) -> [u8; 16] {
    let digest = PreimageWriter::new(Domain::ENROLLMENT_POW_SALT)
        .chain(chain_id)
        .raw(public_key)
        .raw32(recent_block_id)
        .finish();
    let mut salt = [0u8; 16];
    salt.copy_from_slice(&digest.as_bytes()[..16]);
    salt
}

/// `admission_tag`, the enrollment admission shield's constant-verification
/// puzzle output.
#[must_use]
pub fn admission_tag(
    chain_id: &ChainId,
    admission_nonce: &[u8; 16],
    public_key: &[u8; 32],
    admission_solution: u64,
) -> Digest32 {
    PreimageWriter::new(Domain::ENROLLMENT_ADMISSION)
        .chain(chain_id)
        .raw(admission_nonce)
        .raw(public_key)
        .u64be(admission_solution)
        .finish()
}

/// `weak_subjectivity_checkpoint_hash`.
#[must_use]
pub fn weak_subjectivity_checkpoint_hash(chain_id: &ChainId, unsigned: &JsonObject) -> Digest32 {
    PreimageWriter::new(Domain::WEAK_SUBJECTIVITY_CHECKPOINT)
        .chain(chain_id)
        .jcs(unsigned)
        .finish()
}

/// `validator_set_hash = H("coblox-validator-set-v0\0" || JCS(ValidatorSet))`.
///
/// This preimage carries **no** chain binding, and the reason is now written
/// where the formula is: `ledger.md#validator-set-continuity` and
/// `README.md#hash-preimage-registry`. The bytes of a `ValidatorSet` bind it to
/// its chain three times already — `election_seed` and every `election_ticket`
/// are derived through `chain_id_32`, and every `key_binding_signature` is taken
/// over the global chain-bound signature procedure — so adding `chain_id` here
/// would restate a binding that is present and would change every published
/// value that depends on this hash.
///
/// It is repeated here because every neighbouring formula does carry a chain
/// binding and the omission looks like an oversight until it is checked; an
/// exception without its reason is read as a mistake, and the reader who
/// "fixes" it pays for a migration that buys nothing.
#[must_use]
pub fn validator_set_hash(set: &JsonObject) -> Digest32 {
    PreimageWriter::new(Domain::VALIDATOR_SET).jcs(set).finish()
}

/// `tx_id`, over the transaction with `authorization` removed.
#[must_use]
pub fn tx_id(chain_id: &ChainId, unsigned_transaction: &JsonObject) -> Digest32 {
    PreimageWriter::new(Domain::TX_ID)
        .chain(chain_id)
        .jcs(unsigned_transaction)
        .finish()
}

/// `block_id`, over the block header.
#[must_use]
pub fn block_id(chain_id: &ChainId, header: &JsonObject) -> Digest32 {
    PreimageWriter::new(Domain::BLOCK_ID)
        .chain(chain_id)
        .jcs(header)
        .finish()
}

/// The wire envelope `message_id`, over the envelope without `message_id` and
/// without `signature`.
#[must_use]
pub fn message_id(chain_id: &ChainId, envelope_without_id_or_signature: &JsonObject) -> Digest32 {
    PreimageWriter::new(Domain::MESSAGE_ID)
        .chain(chain_id)
        .jcs(envelope_without_id_or_signature)
        .finish()
}

/// The Kademlia DHT namespace key.
#[must_use]
pub fn dht_namespace_key(genesis_block_id: &Digest32) -> Digest32 {
    PreimageWriter::new(Domain::DHT)
        .raw32(genesis_block_id)
        .finish()
}

/// An assembled signature preimage.
///
/// Under protocol rules, every consensus signature signs a chain-bound,
/// domain-separated preimage produced by [`signing_preimage`] (or one of its
/// specialized wrappers like [`block_vote_preimage`] or
/// [`transport_key_attestation_signing_preimage`]).
///
/// This type enforces at compile time that callers pass a complete preimage
/// rather than a `Digest32` or an arbitrary byte slice to signature verifiers.
///
/// # Raw byte construction (non-consensus)
///
/// Upstream conformance test suites (such as `ed25519-speccheck`) verify raw
/// non-Coblox messages. To support testing and verification tooling without
/// weakening consensus paths, `SigningPreimage::from_raw_bytes_non_consensus`
/// permits constructing an instance from arbitrary bytes. It MUST NOT be used
/// on consensus-critical paths.
///
/// That rule is held by a compilation boundary and not only by this sentence.
/// The constructor exists only under the non-default `conformance-testing`
/// feature, which nothing but this crate's own dev-dependency on itself enables
/// (`core/coblox-core/Cargo.toml`, which also states the limit of the guarantee),
/// so it is not compiled into a production build of `coblox-node`, `coblox-ffi`
/// or the desktop shell. A second, textual check lives in
/// `sim/tools/non_consensus_containment.py` and runs in CI: it fails if the
/// constructor is named anywhere outside this file and
/// `core/coblox-core/tests/`. [REVIEW-023] RF-001.
///
/// The wrapped `Vec<u8>` is private, not `pub(crate)`: no module of this crate
/// can build a preimage around the constructors below, nor mutate one that has
/// already been built. [REVIEW-023] RF-003.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SigningPreimage(Vec<u8>);

impl SigningPreimage {
    /// Returns the underlying preimage bytes as a slice.
    #[must_use]
    pub fn as_bytes(&self) -> &[u8] {
        &self.0
    }

    /// Explicitly creates a `SigningPreimage` from raw bytes for non-consensus
    /// conformance tests and test vectors (e.g. `ed25519-speccheck`).
    ///
    /// # Warning
    ///
    /// This constructor MUST NOT be used on consensus paths. Consensus-critical
    /// signatures must always construct preimages via [`signing_preimage`]:
    /// bytes taken straight from the wire carry no `domain || 0x00 || chain_id`
    /// prefix, so a signature verified over them is bound to neither the domain
    /// nor the chain.
    ///
    /// It is behind the non-default `conformance-testing` feature so that the
    /// rule above is enforced by the compiler for every dependant crate rather
    /// than by review alone. See `core/coblox-core/Cargo.toml` for what that
    /// boundary does and does not guarantee.
    #[cfg(feature = "conformance-testing")]
    #[must_use]
    pub fn from_raw_bytes_non_consensus(bytes: &[u8]) -> Self {
        Self(bytes.to_vec())
    }
}

/// The global chain-bound signature preimage.
///
/// "Every Coblox signature input is the ASCII domain shown by the schema, one
/// zero byte, `raw_32_bytes(chain_id)`, then the described bytes." The returned
/// value is the **message**, not a digest: Ed25519 hashes it internally, and
/// pre-hashing it would be a different signature scheme.
#[must_use]
pub fn signing_preimage(domain: Domain, chain_id: &ChainId, payload: &[u8]) -> SigningPreimage {
    let domain_bytes = domain.as_str().as_bytes();
    let mut out = Vec::with_capacity(domain_bytes.len() + 1 + 32 + payload.len());
    out.extend_from_slice(domain_bytes);
    out.push(0);
    out.extend_from_slice(chain_id.as_digest().as_bytes());
    out.extend_from_slice(payload);
    SigningPreimage(out)
}

/// The exact bytes a finality vote signs.
#[must_use]
pub fn block_vote_preimage(
    chain_id: &ChainId,
    height: u64,
    round: u64,
    block_id: &Digest32,
) -> SigningPreimage {
    let mut payload = Vec::with_capacity(8 + 8 + 32);
    payload.extend_from_slice(&height.to_be_bytes());
    payload.extend_from_slice(&round.to_be_bytes());
    payload.extend_from_slice(block_id.as_bytes());
    signing_preimage(Domain::SIG_BLOCK_VOTE, chain_id, &payload)
}

/// The exact bytes a transport key attestation signs.
#[must_use]
pub fn transport_key_attestation_signing_preimage(
    chain_id: &ChainId,
    unsigned_attestation: &JsonObject,
) -> SigningPreimage {
    signing_preimage(
        Domain::SIG_TRANSPORT_KEY_ATTESTATION,
        chain_id,
        &unsigned_attestation.to_jcs(),
    )
}
