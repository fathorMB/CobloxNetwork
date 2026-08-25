//! SHA-256 digests, domain separation, and the preimage writer.
//!
//! Domain separation is pervasive in the protocol and it is not decorative: a
//! preimage built with the wrong domain produces a plausible, wrong hash, which
//! is the worst defect to diagnose. The countermeasure here is that the only
//! way to obtain a [`PreimageWriter`] is to name a [`Domain`], the writer
//! emits `domain || 0x00` before anything else and cannot be told not to, and
//! every domain the protocol defines is an associated constant of [`Domain`]
//! rather than a string literal at a call site.

use sha2::{Digest, Sha256};

use crate::encoding::{hex_lower, hex_lower_decode};
use crate::error::{Error, Result};

/// A 32-byte SHA-256 digest.
///
/// The `sha256:` prefix is presentation. `raw` is what enters a preimage;
/// [`Digest32::to_prefixed`] is what appears in a JSON field.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Digest32([u8; 32]);

impl Digest32 {
    /// Wraps 32 raw bytes.
    #[must_use]
    pub const fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// The raw 32 bytes, as they enter a preimage.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// The presentation form: `sha256:` plus 64 lowercase hexadecimal digits.
    #[must_use]
    pub fn to_prefixed(&self) -> String {
        let mut out = String::with_capacity(71);
        out.push_str("sha256:");
        out.push_str(&hex_lower(&self.0));
        out
    }

    /// Parses `sha256:` plus exactly 64 lowercase hexadecimal digits.
    ///
    /// An uppercase spelling, a missing prefix, or any other length is a
    /// non-canonical encoding and is rejected.
    pub fn parse_prefixed(text: &str) -> Result<Self> {
        let hex = text.strip_prefix("sha256:").ok_or(Error::DigestString)?;
        Ok(Self(hex_lower_decode::<32>(hex)?))
    }

    /// Parses 64 lowercase hexadecimal digits without the `sha256:` prefix.
    ///
    /// The worked example and the Merkle sections of `ledger.md` quote bare
    /// hexadecimal; this is the accessor conformance fixtures use for them.
    pub fn parse_hex(text: &str) -> Result<Self> {
        Ok(Self(hex_lower_decode::<32>(text)?))
    }

    /// A digest whose bytes all equal `byte`.
    ///
    /// The protocol's conformance fixtures are written as "`aa` repeated 32
    /// bytes"; this is that constructor.
    #[must_use]
    pub const fn repeated(byte: u8) -> Self {
        Self([byte; 32])
    }
}

/// A hash-preimage domain: the ASCII string that separates one preimage family
/// from every other.
///
/// The zero terminator shown in the specification is written by
/// [`PreimageWriter`], not stored here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Domain(&'static str);

impl Domain {
    /// The domain text, without its zero terminator.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        self.0
    }

    // --- Identifiers (README.md#identifiers-and-cryptographic-conventions) ---
    /// `chain_id` derivation.
    pub const CHAIN_ID: Self = Self("coblox-chain-id-v0");
    /// `node_id` derivation.
    pub const NODE_ID: Self = Self("coblox-node-id-v0");
    /// `tx_id` derivation.
    pub const TX_ID: Self = Self("coblox-tx-id-v0");
    /// `block_id` derivation.
    pub const BLOCK_ID: Self = Self("coblox-block-id-v0");
    /// Sparse account tree key derivation.
    pub const ACCOUNT_KEY: Self = Self("coblox-account-key-v0");
    /// Kademlia DHT namespace key.
    pub const DHT: Self = Self("coblox-dht-v0");
    /// Wire envelope message ID.
    pub const MESSAGE_ID: Self = Self("coblox-message-id-v0");

    // --- Hash preimage registry (README.md#hash-preimage-registry) ---
    /// `enrollment_request_hash`.
    pub const ENROLLMENT_REQUEST_HASH: Self = Self("coblox-enrollment-request-hash-v0");
    /// `parameter_set_hash`.
    pub const ENROLLMENT_PARAMETER_SET: Self = Self("coblox-enrollment-parameter-set-v0");
    /// `policy_hash`.
    pub const REWARD_POLICY: Self = Self("coblox-reward-policy-v0");
    /// `hosting_rate_card_hash`.
    pub const HOSTING_RATE_CARD: Self = Self("coblox-hosting-rate-card-v0");
    /// `consensus_parameters_hash`.
    pub const CONSENSUS_PARAMETERS: Self = Self("coblox-consensus-parameters-v0");
    /// `object_id`.
    pub const STORAGE_OBJECT: Self = Self("coblox-storage-object-v0");
    /// `input_hash`.
    pub const COMPUTE_INPUT: Self = Self("coblox-compute-input-v0");
    /// `request_hash`.
    pub const CHALLENGE_REQUEST_HASH: Self = Self("coblox-challenge-request-hash-v0");
    /// `response_hash`.
    pub const CHALLENGE_RESPONSE_HASH: Self = Self("coblox-challenge-response-hash-v0");
    /// `issuer_commitment`.
    pub const CHALLENGE_ISSUER_COMMITMENT: Self = Self("coblox-challenge-issuer-commitment-v0");
    /// `challenge_randomness`.
    pub const CHALLENGE_RANDOMNESS: Self = Self("coblox-challenge-randomness-v0");
    /// `election_entropy`.
    pub const ELECTION_ENTROPY: Self = Self("coblox-election-entropy-v0");
    /// `election_seed`.
    pub const ELECTION_SEED: Self = Self("coblox-election-seed-v0");
    /// `election_ticket`.
    pub const ELECTION_TICKET: Self = Self("coblox-election-ticket-v0");
    /// `enrollment_pow_salt` (truncated to its first 16 bytes by the caller).
    pub const ENROLLMENT_POW_SALT: Self = Self("coblox-enrollment-pow-salt-v0");
    /// `admission_tag`.
    pub const ENROLLMENT_ADMISSION: Self = Self("coblox-enrollment-admission-v0");
    /// `weak_subjectivity_checkpoint_hash`.
    pub const WEAK_SUBJECTIVITY_CHECKPOINT: Self = Self("coblox-weak-subjectivity-checkpoint-v0");
    /// `validator_set_hash`. Note that this preimage carries **no** chain ID.
    pub const VALIDATOR_SET: Self = Self("coblox-validator-set-v0");
    /// Enrollment proof-of-work password prefix.
    pub const ENROLLMENT_POW: Self = Self("coblox-enrollment-pow-v0");

    // --- Signature domains ---
    /// Enrollment request signature.
    pub const SIG_ENROLLMENT_REQUEST: Self = Self("coblox-enrollment-request-v0");
    /// Enrollment certificate validator signature.
    pub const SIG_ENROLLMENT_CERTIFICATE: Self = Self("coblox-enrollment-certificate-v0");
    /// Signed protocol document validator signature.
    pub const SIG_PROTOCOL_DOCUMENT: Self = Self("coblox-protocol-document-v0");
    /// Ledger transaction authorization signature.
    pub const SIG_LEDGER_TRANSACTION: Self = Self("coblox-ledger-transaction-v0");
    /// Block finality vote.
    pub const SIG_BLOCK_VOTE: Self = Self("coblox-block-vote-v0");
    /// Validator consensus key binding proof of possession.
    pub const SIG_CONSENSUS_KEY_BINDING: Self = Self("coblox-consensus-key-binding-v0");
    /// Transport key attestation signature.
    pub const SIG_TRANSPORT_KEY_ATTESTATION: Self = Self("coblox-transport-key-attestation-v0");
    /// Challenge evidence auditor signature.
    pub const SIG_CHALLENGE_EVIDENCE: Self = Self("coblox-challenge-evidence-v0");
    /// Challenge request issuer signature.
    pub const SIG_CHALLENGE_REQUEST: Self = Self("coblox-challenge-request-v0");
    /// Challenge response subject signature.
    pub const SIG_CHALLENGE_RESPONSE: Self = Self("coblox-challenge-response-v0");
    /// Wire envelope signature.
    pub const SIG_WIRE_ENVELOPE: Self = Self("coblox-wire-envelope-v0");
    /// Weak subjectivity checkpoint trust-key signature.
    pub const SIG_WEAK_SUBJECTIVITY: Self = Self("coblox-weak-subjectivity-signature-v0");
}

/// Builds a domain-separated preimage and hashes it.
///
/// The domain and its zero terminator are written by the constructor, so a
/// preimage without domain separation cannot be built through this type.
#[derive(Debug, Clone)]
pub struct PreimageWriter {
    hasher: Sha256,
}

impl PreimageWriter {
    /// Starts a preimage with `domain || 0x00`.
    #[must_use]
    pub fn new(domain: Domain) -> Self {
        let mut hasher = Sha256::new();
        hasher.update(domain.as_str().as_bytes());
        hasher.update([0u8]);
        Self { hasher }
    }

    /// Appends `raw_32_bytes(chain_id)`.
    #[must_use]
    pub fn chain(mut self, chain_id: &ChainId) -> Self {
        self.hasher.update(chain_id.as_digest().as_bytes());
        self
    }

    /// Appends the raw 32 bytes of a digest.
    #[must_use]
    pub fn raw32(mut self, digest: &Digest32) -> Self {
        self.hasher.update(digest.as_bytes());
        self
    }

    /// Appends raw bytes.
    #[must_use]
    pub fn raw(mut self, bytes: &[u8]) -> Self {
        self.hasher.update(bytes);
        self
    }

    /// Appends `u64be(value)`.
    #[must_use]
    pub fn u64be(mut self, value: u64) -> Self {
        self.hasher.update(value.to_be_bytes());
        self
    }

    /// Appends `u32be(value)`.
    #[must_use]
    pub fn u32be(mut self, value: u32) -> Self {
        self.hasher.update(value.to_be_bytes());
        self
    }

    /// Appends `u32be(len(text_utf8)) || text_utf8`.
    ///
    /// "Lengths are byte lengths, not Unicode character counts."
    pub fn length_prefixed_utf8(mut self, text: &str) -> Result<Self> {
        let bytes = text.as_bytes();
        let length =
            u32::try_from(bytes.len()).map_err(|_| Error::Arithmetic("utf-8 length prefix"))?;
        self.hasher.update(length.to_be_bytes());
        self.hasher.update(bytes);
        Ok(self)
    }

    /// Appends the JCS bytes of an object.
    #[must_use]
    pub fn jcs(mut self, object: &crate::json::JsonObject) -> Self {
        self.hasher.update(object.to_jcs());
        self
    }

    /// Finishes the preimage and returns its digest.
    #[must_use]
    pub fn finish(self) -> Digest32 {
        Digest32(finalize_32(self.hasher))
    }
}

/// Hashes a single tag byte followed by `payload`.
///
/// This is the Merkle-tag family (`H(0x01 || left || right)` and friends),
/// which is deliberately *not* domain-string separated: the specification uses
/// one-byte tags there and an implementation must not "improve" on it.
#[must_use]
pub fn tagged_hash(tag: u8, payload: &[&[u8]]) -> Digest32 {
    let mut hasher = Sha256::new();
    hasher.update([tag]);
    for part in payload {
        hasher.update(part);
    }
    Digest32(finalize_32(hasher))
}

fn finalize_32(hasher: Sha256) -> [u8; 32] {
    let output = hasher.finalize();
    let mut bytes = [0u8; 32];
    bytes.copy_from_slice(output.as_slice());
    bytes
}

/// The chain binding every Coblox signature and most preimages carry.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ChainId(Digest32);

impl ChainId {
    /// Wraps an already-derived chain ID, as a client loads it from its signed
    /// network distribution.
    #[must_use]
    pub const fn from_digest(digest: Digest32) -> Self {
        Self(digest)
    }

    /// Derives `chain_id` from the network ID and genesis block ID.
    pub fn derive(network_id: &str, genesis_block_id: &Digest32) -> Result<Self> {
        Ok(Self(
            PreimageWriter::new(Domain::CHAIN_ID)
                .length_prefixed_utf8(network_id)?
                .raw32(genesis_block_id)
                .finish(),
        ))
    }

    /// The underlying digest.
    #[must_use]
    pub const fn as_digest(&self) -> &Digest32 {
        &self.0
    }
}

/// The `cblx1`-prefixed node identifier.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct NodeId(String);

impl NodeId {
    /// Derives `node_id` from a 32-byte Ed25519 identity public key.
    #[must_use]
    pub fn derive(public_key: &[u8; 32]) -> Self {
        let digest = PreimageWriter::new(Domain::NODE_ID)
            .raw(public_key)
            .finish();
        let mut text = String::with_capacity(57);
        text.push_str("cblx1");
        text.push_str(&crate::encoding::base32_lower_encode(digest.as_bytes()));
        Self(text)
    }

    /// Adopts an identifier that came from a protocol object.
    ///
    /// A node identifier is compared as an opaque UTF-8 string everywhere in
    /// the protocol; a verifier that holds the public key must additionally
    /// check the derivation with [`NodeId::derive`].
    #[must_use]
    pub fn from_string(text: String) -> Self {
        Self(text)
    }

    /// The identifier text.
    #[must_use]
    pub fn as_str(&self) -> &str {
        &self.0
    }
}

/// A 32-byte sparse-account-tree key.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct AccountKey([u8; 32]);

impl AccountKey {
    /// Wraps 32 raw bytes.
    #[must_use]
    pub const fn from_bytes(bytes: [u8; 32]) -> Self {
        Self(bytes)
    }

    /// The raw 32 bytes.
    #[must_use]
    pub const fn as_bytes(&self) -> &[u8; 32] {
        &self.0
    }

    /// `account_key = H("coblox-account-key-v0\0" || 0x00 || node_id_utf8)`.
    #[must_use]
    pub fn for_node(node_id: &NodeId) -> Self {
        Self(
            *PreimageWriter::new(Domain::ACCOUNT_KEY)
                .raw(&[0x00])
                .raw(node_id.as_str().as_bytes())
                .finish()
                .as_bytes(),
        )
    }

    /// `account_key = H("coblox-account-key-v0\0" || 0x01 || app_id_32)`.
    #[must_use]
    pub fn for_app(app_id: &Digest32) -> Self {
        Self(
            *PreimageWriter::new(Domain::ACCOUNT_KEY)
                .raw(&[0x01])
                .raw32(app_id)
                .finish()
                .as_bytes(),
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn digest_presentation_round_trips_and_rejects_uppercase() {
        let digest = Digest32::repeated(0xab);
        let text = digest.to_prefixed();
        assert_eq!(text.len(), 7 + 64);
        assert_eq!(Digest32::parse_prefixed(&text).unwrap(), digest);
        assert!(Digest32::parse_prefixed(&text.to_uppercase()).is_err());
        assert!(Digest32::parse_prefixed(&text[7..]).is_err());
    }

    #[test]
    fn the_domain_terminator_is_part_of_the_preimage() {
        // `H("d\0" || x)` and `H("d" || x)` must differ; the writer always
        // emits the terminator, so this is the guard against it being dropped.
        let with_terminator = PreimageWriter::new(Domain::ELECTION_SEED)
            .raw(b"x")
            .finish();
        let mut hasher = Sha256::new();
        hasher.update(Domain::ELECTION_SEED.as_str().as_bytes());
        hasher.update(b"x");
        let without = super::finalize_32(hasher);
        assert_ne!(with_terminator.as_bytes(), &without);
    }
}
