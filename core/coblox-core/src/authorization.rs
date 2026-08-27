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
//! certificate; under [ADR-017], it reads the height at which the
//! `revoke_identity` was included in the chain (`included_height`), which every
//! verifier reads identically from that block and its ancestors.
//! Anchoring instead to *when the revocation became final* would need a
//! quantity the chain does not record — no block carries a `QuorumCertificate`
//! — so two verifiers holding different certificates would return opposite
//! verdicts on the same block.
//!
//! **The predicate is evaluated at the granularity of a height, and never
//! consults intra-block ordering. That is the reason it is safe, and it is not
//! the reason the document used to give** ([REVIEW-042] RF-002). Its inputs are
//! `including_height` and two sets of records; nothing here can observe which
//! transaction of a block runs first. So the position of a `revoke_identity`
//! inside its own block moves no verdict, and neither does the raw-transaction-ID
//! ordering inside execution class 0: a revoker grinding `created_at_ms` to
//! reorder its own transaction against a target changes nothing this function
//! reads, which is why [DEBT-035] is not exploitable through this rule. An
//! implementation that instead evaluated the qualification *at the moment the
//! transaction executes* would make the verdict depend on that ordering. That
//! is a different predicate, and it is not this one.

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

/// A `revoke_identity` of the chain under validation, reduced to what the
/// qualification reads.
///
/// **The word *finalized* is deliberately not in this type's contract**
/// ([REVIEW-042] RF-002). A revocation included in the block being validated
/// shares that block's fate: if the block is not accepted neither exists, and
/// if it is accepted both do. Requiring the revocation to be final *while its
/// own block is being validated* is the verifier-dependent reading the module
/// header rejects.
///
/// **Comment retraction (ADR-017 / SPEC-022):** The previous version of this comment
/// stated that the height at which the `revoke_identity` transaction was included was
/// deliberately absent because the predicate did not read it (reading `effective_height`
/// instead). That rationale was invalidated by [ADR-017]: on the transaction authorization
/// / spending path, the predicate now reads the height of the block that *included*
/// the revocation (`included_height`), while `effective_height` is confined to the
/// validator set transition path.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RevocationRecord {
    /// The identity the revocation names.
    pub node_id: String,
    /// The height of the block that included the `revoke_identity` transaction.
    pub included_height: u64,
}

/// Whether `node_id` is *enrolled, unrevoked* as of `including_height`.
///
/// `including_height` is the height of the block that includes the transaction
/// being authorized. `enrollments` and `revocations` are the records the
/// replaying verifier holds from **that block and its ancestors** — the block
/// at `including_height` is inside the scope, which is what makes the boundary
/// row of `AUTH-0` (`h == included_height`) come out invalid. This crate does
/// not replay transactions, so they enter as facts, exactly as
/// [`crate::election::CandidateFacts`] does.
///
/// # Errors
///
/// [`AuthorizationError::NotEnrolled`] when no enrollment certificate in that
/// chain names `node_id` with `valid_from_height <= including_height`, and
/// [`AuthorizationError::Revoked`] when a `revoke_identity` in that chain names
/// it with `included_height <= including_height`. The reported
/// `included_height` is the **earliest** qualifying one, so the error does not
/// depend on the order of `revocations`.
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
    // The comparison is `<=`: `included_height` is the first height at which
    // the revocation bites on the transaction path, not the last at which it does not.
    //
    // **`min_by_key` and not `find`, and the difference is in the error and not
    // in the verdict** ([REVIEW-042] RF-007). Whether *some* qualifying record
    // exists does not depend on the order of the slice, but *which* one `find`
    // returns does: given `[20, 30]` it reported `20` and given `[30, 20]` it
    // reported `30`, for the same subject at the same height. `included_height`
    // is a field this delivery introduced, and the earliest qualifying
    // revocation is the only answer that is a fact about the chain rather than
    // about the caller's iteration order — an identity revoked twice is revoked
    // from the first of the two.
    if let Some(record) = revocations
        .iter()
        .filter(|record| record.node_id == node_id)
        .filter(|record| record.included_height <= including_height)
        .min_by_key(|record| record.included_height)
    {
        return Err(AuthorizationError::Revoked {
            node_id: node_id.to_owned(),
            height: including_height,
            included_height: record.included_height,
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
