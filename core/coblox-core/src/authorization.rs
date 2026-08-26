//! The single-key transaction authorization rule of
//! `ledger.md#what-enrolled-unrevoked-means-and-as-of-which-height`.
//!
//! Four authorization structures of `ledger.md` are satisfied by one key that
//! MUST derive an **enrolled, unrevoked** node ID: `FundAppAuthorization`,
//! `SubscriptionBurnAuthorization`, `ChallengeCommitmentAuthorization` and
//! `ValidatorCandidacyAuthorization`. The qualification is the same in all four
//! and it is one predicate here for that reason: the defect this module closes
//! ([DEBT-022]) was one of the four saying something different from the other
//! three, and a rule written once cannot drift into an asymmetry again.
//!
//! **The height is an argument, and that is the whole point.** The predicate is
//! evaluated as of the height of the block that includes the transaction, never
//! as of the verifier's own head. It takes no clock, no view, and no quorum
//! certificate; it reads `effective_height` out of the finalized
//! `revoke_identity` body, which every verifier reads from the same bytes.
//! Anchoring instead to *when the revocation became final* would need a
//! quantity the chain does not record — no block carries a `QuorumCertificate`
//! — so two verifiers holding different certificates would return opposite
//! verdicts on the same block.

use crate::error::{AuthorizationError, Error, Result};
use crate::hash::NodeId;

/// A finalized enrollment certificate, reduced to what the qualification reads.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnrollmentRecord {
    /// The identity the certificate enrolls.
    pub node_id: String,
    /// The height from which the certificate is in force.
    pub valid_from_height: u64,
}

/// A finalized `revoke_identity`, reduced to what the qualification reads.
///
/// The height at which the transaction was *included* is deliberately absent:
/// the predicate does not read it, and a field nobody reads is a field a later
/// reader mistakes for one that matters.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RevocationRecord {
    /// The identity the revocation names.
    pub node_id: String,
    /// `effective_height` from the `revoke_identity` body.
    pub effective_height: u64,
}

/// Whether `node_id` is *enrolled, unrevoked* as of `including_height`.
///
/// `including_height` is the height of the block that includes the transaction
/// being authorized. `enrollments` and `revocations` are the finalized records
/// the replaying verifier holds from the ancestors of that block; this crate
/// does not replay transactions, so they enter as facts, exactly as
/// [`crate::election::CandidateFacts`] does.
///
/// # Errors
///
/// [`AuthorizationError::NotEnrolled`] when no finalized enrollment certificate
/// names `node_id` with `valid_from_height <= including_height`, and
/// [`AuthorizationError::Revoked`] when a finalized `revoke_identity` names it
/// with `effective_height <= including_height`.
pub fn enrolled_unrevoked(
    node_id: &str,
    including_height: u64,
    enrollments: &[EnrollmentRecord],
    revocations: &[RevocationRecord],
) -> Result<()> {
    // The comparison is `<=`: `valid_from_height` is the first height at which
    // the certificate authorizes, not the last at which it does not. It mirrors
    // the revocation comparison below, and both boundaries are exercised —
    // [REVIEW-033] RF-004 found this one stated and unpinned.
    let enrolled = enrollments
        .iter()
        .any(|record| record.node_id == node_id && record.valid_from_height <= including_height);
    if !enrolled {
        return Err(AuthorizationError::NotEnrolled {
            node_id: node_id.to_owned(),
            height: including_height,
        }
        .into());
    }
    // The comparison is `<=`: `effective_height` is the first height at which
    // the revocation bites, not the last at which it does not.
    if let Some(record) = revocations
        .iter()
        .filter(|record| record.node_id == node_id)
        .find(|record| record.effective_height <= including_height)
    {
        return Err(AuthorizationError::Revoked {
            node_id: node_id.to_owned(),
            height: including_height,
            effective_height: record.effective_height,
        }
        .into());
    }
    Ok(())
}

/// The complete single-key authorization rule: the key derives the node ID, and
/// that node ID is enrolled and unrevoked as of the including height.
///
/// The two halves are one function because the document states them as one
/// sentence, and because a caller that checked only the derivation would have
/// implemented exactly the defect of [DEBT-022].
///
/// # Errors
///
/// [`AuthorizationError::KeyDoesNotDerive`] when the public key does not derive
/// `node_id`, and otherwise whatever [`enrolled_unrevoked`] returns.
pub fn authorize_single_key(
    public_key: &[u8; 32],
    node_id: &str,
    including_height: u64,
    enrollments: &[EnrollmentRecord],
    revocations: &[RevocationRecord],
) -> Result<()> {
    if NodeId::derive(public_key).as_str() != node_id {
        return Err(Error::Authorization(AuthorizationError::KeyDoesNotDerive {
            node_id: node_id.to_owned(),
        }));
    }
    enrolled_unrevoked(node_id, including_height, enrollments, revocations)
}
