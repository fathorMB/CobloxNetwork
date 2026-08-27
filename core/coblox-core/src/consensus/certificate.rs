//! The `QuorumCertificate` a finalized block carries, and its verifier.
//!
//! # What is new here and what is not
//!
//! The **object** is not new. `ledger.md#what-validators-sign` has published its
//! schema and its four rules since the first protocol spec, and [ADR-018] makes
//! the premise that nothing published changes. This module is the first Rust
//! representation of that object and the first verifier of it; the schema, the
//! field order, the signature domain and the quorum predicate are taken from the
//! document and from [`crate::quorum`], never restated.
//!
//! What [ADR-018] adds is a **reading**: the signatures in a certificate are
//! *precommits*, the second of two phases. That changes nothing about the bytes,
//! and it is why this file lives in a consensus module rather than next to
//! [`crate::block`].
//!
//! # The four rules, verbatim from `ledger.md`
//!
//! > Signatures are unique and sorted by validator ID. Their summed voting power
//! > MUST satisfy [the strict quorum predicate](#quorum-predicate). An empty or
//! > duplicate signature entry invalidates the certificate. Aggregating Ed25519
//! > signatures is not defined in v0.
//!
//! [`QuorumCertificate::verify`] implements the first three and the fourth by
//! construction: there is no aggregate branch to take.

use crate::block::BlockHeader;
use crate::error::{ConsensusError, Error, JsonError, Result};
use crate::hash::{ChainId, Digest32, Domain};
use crate::json::{Json, JsonObject};
use crate::quorum::quorum;
use crate::registry::block_vote_preimage;
use crate::validator_set::ValidatorSet;
use crate::verifier::verify_in_context;
use crate::{SignatureVerifier, encoding};

/// One `(validator_id, signature)` entry of a certificate.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CertificateSignature {
    /// The signing member's `validator_id`.
    pub validator_id: String,
    /// The 64-byte Ed25519 signature over the precommit preimage.
    pub signature: [u8; 64],
}

/// The field names of a certificate signature entry.
const SIGNATURE_FIELDS: [&str; 2] = ["signature", "validator_id"];

/// The field names of `QuorumCertificate`.
const CERTIFICATE_FIELDS: [&str; 5] = [
    "block_id",
    "height",
    "round",
    "signatures",
    "validator_set_hash",
];

impl CertificateSignature {
    /// The canonical object this entry serializes to.
    pub fn to_json(&self) -> Result<JsonObject> {
        JsonObject::builder()
            .bytes("signature", &self.signature)
            .str("validator_id", &self.validator_id)
            .build()
    }

    /// Reads an entry from a canonical object, rejecting unknown fields.
    pub fn from_json(object: &JsonObject) -> Result<Self> {
        object.reject_unknown_fields(&SIGNATURE_FIELDS)?;
        let signature = encoding::base64url_decode_fixed::<64>(
            object.string("signature")?,
            "certificate signature",
        )?;
        Ok(Self {
            validator_id: object.string("validator_id")?.to_owned(),
            signature,
        })
    }
}

/// A `QuorumCertificate`: the proof that a block was finalized.
///
/// `round` is the round in which the precommits were gathered, which is **not**
/// necessarily `header.round`. See [`crate::consensus`] §*Two rounds, and why
/// they are allowed to differ* for the reason, and for why the header's cannot
/// be rewritten to agree.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct QuorumCertificate {
    /// The height the certificate finalizes.
    pub height: u64,
    /// The round in which the precommits were gathered.
    pub round: u64,
    /// The finalized block's ID.
    pub block_id: Digest32,
    /// The hash of the set whose members signed.
    pub validator_set_hash: Digest32,
    /// The precommit signatures, unique and sorted by validator ID.
    pub signatures: Vec<CertificateSignature>,
}

impl QuorumCertificate {
    /// The canonical object this certificate serializes to.
    pub fn to_json(&self) -> Result<JsonObject> {
        let mut entries = Vec::with_capacity(self.signatures.len());
        for signature in &self.signatures {
            entries.push(Json::Object(signature.to_json()?));
        }
        JsonObject::builder()
            .digest("block_id", &self.block_id)
            .uint("height", self.height)
            .uint("round", self.round)
            .array("signatures", entries)
            .digest("validator_set_hash", &self.validator_set_hash)
            .build()
    }

    /// Reads a certificate from a canonical object, rejecting unknown fields.
    pub fn from_json(object: &JsonObject) -> Result<Self> {
        object.reject_unknown_fields(&CERTIFICATE_FIELDS)?;
        let mut signatures = Vec::new();
        for entry in object.array("signatures")? {
            let Json::Object(entry) = entry else {
                return Err(JsonError::NotAnObject.into());
            };
            signatures.push(CertificateSignature::from_json(entry)?);
        }
        Ok(Self {
            height: object.uint("height")?,
            round: object.uint("round")?,
            block_id: object.digest("block_id")?,
            validator_set_hash: object.digest("validator_set_hash")?,
            signatures,
        })
    }

    /// Verifies the certificate against `set` under `chain_id`.
    ///
    /// The order is chosen so that the cheapest rejections happen first and no
    /// curve arithmetic is spent on a structurally invalid certificate:
    ///
    /// 1. the certificate names `set` — `validator_set_hash` equals
    ///    [`ValidatorSet::hash`]. Without this first, the remaining checks would
    ///    be against a set of the verifier's choosing rather than the one the
    ///    certificate claims, and the quorum in step 5 would be a quorum of the
    ///    wrong total power;
    /// 2. `signatures` is non-empty and **strictly** sorted by `validator_id`,
    ///    which is the document's "unique and sorted" and its "an empty or
    ///    duplicate signature entry invalidates the certificate" in one
    ///    comparison;
    /// 3. every `validator_id` is a member of `set`;
    /// 4. every signature verifies, under [`Domain::SIG_BLOCK_VOTE`] and
    ///    `chain_id`, over the preimage of `(height, round, block_id)`;
    /// 5. the signed power satisfies [`crate::quorum::quorum`] against the set's
    ///    total power.
    ///
    /// Step 4 goes through [`verify_in_context`] rather than through
    /// [`SignatureVerifier::verify`], so a preimage built for another domain or
    /// another chain is rejected before the signature is considered. This is the
    /// first consensus caller in the crate, and the convention
    /// [`verify_in_context`] documents as having no enforcement is followed here
    /// rather than restated.
    pub fn verify<V: SignatureVerifier + ?Sized>(
        &self,
        chain_id: &ChainId,
        set: &ValidatorSet,
        verifier: &V,
    ) -> Result<()> {
        let set_hash = set.hash()?;
        if self.validator_set_hash != set_hash {
            return Err(ConsensusError::CertificateNamesAnotherSet {
                expected: set_hash,
                actual: self.validator_set_hash,
            }
            .into());
        }
        if self.signatures.is_empty() {
            return Err(ConsensusError::CertificateEmpty.into());
        }
        for pair in self.signatures.windows(2) {
            if pair[0].validator_id >= pair[1].validator_id {
                return Err(ConsensusError::CertificateNotSortedOrUnique.into());
            }
        }
        let preimage = block_vote_preimage(chain_id, self.height, self.round, &self.block_id);
        let mut signed_power: u64 = 0;
        for entry in &self.signatures {
            let member = set
                .validators
                .iter()
                .find(|candidate| candidate.validator_id == entry.validator_id)
                .ok_or_else(|| ConsensusError::NotAMember {
                    validator_id: entry.validator_id.clone(),
                })?;
            if !verify_in_context(
                verifier,
                Domain::SIG_BLOCK_VOTE,
                chain_id,
                &member.consensus_public_key,
                &preimage,
                &entry.signature,
            ) {
                return Err(ConsensusError::InvalidSignature {
                    validator_id: entry.validator_id.clone(),
                }
                .into());
            }
            signed_power = signed_power
                .checked_add(member.voting_power)
                .ok_or(Error::Arithmetic("certificate signed power overflows u64"))?;
        }
        let total_power = set.total_voting_power()?;
        if !quorum(signed_power, total_power)? {
            return Err(ConsensusError::BelowQuorum {
                signed_power,
                total_power,
            }
            .into());
        }
        Ok(())
    }
}

/// A finalized block: the published `Block` of `ledger.md#block-format`.
///
/// ```text
/// Block = {
///   "header":BlockHeader,
///   "transactions":[Transaction],
///   "quorum_certificate":QuorumCertificate
/// }
/// ```
///
/// It lives here rather than beside [`BlockHeader`] because of the property
/// [ADR-018] draws its whole architecture from: **a block carries its own
/// certificate**, so a `Block` is never a proposal and can only be produced at
/// the moment consensus finishes. The proposal, which is the object a block
/// format module might have been expected to hold, is
/// [`super::messages::BlockProposal`] and carries no certificate at all.
///
/// Nothing in [`crate::block`] changed to add this. That is checkable in the
/// diff and is `GATE-NOTHING-PUBLISHED-CHANGED`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FinalizedBlock {
    /// The finalized header.
    pub header: BlockHeader,
    /// The transactions, in canonical execution order.
    pub transactions: Vec<JsonObject>,
    /// The certificate that finalized it.
    pub quorum_certificate: QuorumCertificate,
}

/// The field names of `Block`.
const BLOCK_FIELDS: [&str; 3] = ["header", "quorum_certificate", "transactions"];

impl FinalizedBlock {
    /// `block_id`, recomputed from the header.
    pub fn block_id(&self, chain_id: &ChainId) -> Result<Digest32> {
        self.header.block_id(chain_id)
    }

    /// The canonical `Block` object.
    pub fn to_json(&self) -> Result<JsonObject> {
        JsonObject::builder()
            .object("header", self.header.to_json()?)
            .object("quorum_certificate", self.quorum_certificate.to_json()?)
            .array(
                "transactions",
                self.transactions
                    .iter()
                    .cloned()
                    .map(Json::Object)
                    .collect(),
            )
            .build()
    }

    /// Reads a `Block`, rejecting unknown fields.
    pub fn from_json(object: &JsonObject) -> Result<Self> {
        object.reject_unknown_fields(&BLOCK_FIELDS)?;
        let mut transactions = Vec::new();
        for entry in object.array("transactions")? {
            let Json::Object(entry) = entry else {
                return Err(JsonError::NotAnObject.into());
            };
            transactions.push(entry.clone());
        }
        Ok(Self {
            header: BlockHeader::from_json(object.object("header")?)?,
            transactions,
            quorum_certificate: QuorumCertificate::from_json(object.object("quorum_certificate")?)?,
        })
    }

    /// Verifies that the certificate finalizes **this** block under `set`.
    ///
    /// Two checks the certificate alone cannot make, then
    /// [`QuorumCertificate::verify`]:
    ///
    /// * `quorum_certificate.block_id` is the ID of the header carried here. A
    ///   certificate is a set of signatures over an ID, so a block paired with a
    ///   valid certificate for a *different* ID is the substitution this check
    ///   exists for, and it costs one hash;
    /// * `quorum_certificate.height` is the header's height.
    ///
    /// The certificate's `round` is deliberately **not** compared with the
    /// header's. See [`QuorumCertificate`].
    pub fn verify<V: SignatureVerifier + ?Sized>(
        &self,
        chain_id: &ChainId,
        set: &ValidatorSet,
        verifier: &V,
    ) -> Result<()> {
        let block_id = self.block_id(chain_id)?;
        if self.quorum_certificate.block_id != block_id {
            return Err(ConsensusError::CertificateForAnotherBlock {
                expected: block_id,
                actual: self.quorum_certificate.block_id,
            }
            .into());
        }
        if self.quorum_certificate.height != self.header.height {
            return Err(ConsensusError::CertificateHeightMismatch {
                header: self.header.height,
                certificate: self.quorum_certificate.height,
            }
            .into());
        }
        self.quorum_certificate.verify(chain_id, set, verifier)
    }
}
