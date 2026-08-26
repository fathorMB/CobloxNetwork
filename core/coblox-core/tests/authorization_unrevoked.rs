//! Fixture `AUTH-0` of
//! `ledger.md#what-enrolled-unrevoked-means-and-as-of-which-height`.
//!
//! **The two rows that matter are the ones the two readings disagree on.**
//! Before that section existed, *enrolled, unrevoked* had two readings — a
//! revocation that is *finalized*, and a revocation that is *effective as of* a
//! height — and between them lies an interval the protocol keeps long on
//! purpose. A case in which a revoked-and-effective key is rejected is agreed
//! on by both readings: it would have been green while [DEBT-022] was open, so
//! it measures nothing. The two readings return opposite verdicts across the
//! whole interval `[20, 49]`; heights 21 and 49 are the sample this file takes
//! of it — the first interior height and the last — and they are why this file
//! exists.
//!
//! The fixture also varies the one quantity the divergent rows hold constant —
//! whether a revocation for the key exists at all — because rows about one
//! revoked node would pass an implementation that rejected every key.
//!
//! **Both clauses of the definition have their boundary pinned, and they did
//! not always.** Clause 2 has always had row `50`; clause 1 got its row only
//! after [REVIEW-033] RF-004 reintroduced `valid_from_height <` in place of
//! `<=` and watched all 176 tests of the workspace stay green. A clause stated
//! with an inclusive comparison and exercised only away from the boundary is a
//! clause whose boundary is a guess.

use coblox_core::authorization::{
    EnrollmentRecord, RevocationRecord, authorize_single_key, enrolled_unrevoked,
};
use coblox_core::error::{AuthorizationError, Error};
use coblox_core::hash::NodeId;

/// The revoked identity of `AUTH-0`, which is the `REVL-0` fixture identity.
const REVOKED: &str = "cblx1revokedfixture";
/// The comparison identity of `AUTH-0`: enrolled, and named by no revocation.
const NEVER_REVOKED: &str = "cblx1ci6q36gqm6u3spknxzr7p5r2y4xw7n25d5icm7rsoq7lq6ka";
/// `effective_height` of the `revoke_identity` of `AUTH-0`.
const EFFECTIVE_HEIGHT: u64 = 50;
/// `valid_from_height` of both enrollment certificates of `AUTH-0`.
const VALID_FROM_HEIGHT: u64 = 5;

fn enrollments() -> Vec<EnrollmentRecord> {
    vec![
        EnrollmentRecord {
            node_id: REVOKED.to_owned(),
            valid_from_height: VALID_FROM_HEIGHT,
        },
        EnrollmentRecord {
            node_id: NEVER_REVOKED.to_owned(),
            valid_from_height: VALID_FROM_HEIGHT,
        },
    ]
}

/// The single finalized `revoke_identity` of the fixture.
///
/// It is finalized in the block at height 20 and effective at 50, and the two
/// values are deliberately different: an implementation that compared the
/// including height against the height of inclusion rather than against
/// `effective_height` would pass every row if they were equal.
fn revocations() -> Vec<RevocationRecord> {
    vec![RevocationRecord {
        node_id: REVOKED.to_owned(),
        effective_height: EFFECTIVE_HEIGHT,
    }]
}

fn qualification_at(node_id: &str, height: u64) -> Result<(), Error> {
    enrolled_unrevoked(node_id, height, &enrollments(), &revocations())
}

#[test]
fn the_revocation_does_not_bite_below_its_effective_height() {
    assert!(qualification_at(REVOKED, 19).is_ok());
}

/// The first of the two rows the readings disagree on.
///
/// The `revoke_identity` is finalized at height 20, so under the *finalized*
/// reading this burn is invalid; under the definition in force it is valid,
/// because `effective_height` is 50. Two implementations splitting here split
/// on the validity of a block.
#[test]
fn a_finalized_but_not_yet_effective_revocation_still_authorizes_at_21() {
    assert!(qualification_at(REVOKED, 21).is_ok());
}

/// The second divergent row, at the last height before the revocation bites.
#[test]
fn a_finalized_but_not_yet_effective_revocation_still_authorizes_at_49() {
    assert!(qualification_at(REVOKED, 49).is_ok());
}

/// `effective_height` is the first height at which the revocation bites, not
/// the last at which it does not: the comparison is `<=` and this row pins it.
#[test]
fn the_revocation_bites_exactly_at_its_effective_height() {
    let Err(Error::Authorization(AuthorizationError::Revoked {
        node_id,
        height,
        effective_height,
    })) = qualification_at(REVOKED, EFFECTIVE_HEIGHT)
    else {
        panic!("expected the revocation to bite at its effective height");
    };
    assert_eq!(node_id, REVOKED);
    assert_eq!(height, EFFECTIVE_HEIGHT);
    assert_eq!(effective_height, EFFECTIVE_HEIGHT);
}

#[test]
fn the_revocation_keeps_biting_above_its_effective_height() {
    assert!(matches!(
        qualification_at(REVOKED, 51),
        Err(Error::Authorization(AuthorizationError::Revoked { .. }))
    ));
}

/// The row that varies the quantity the other five hold constant.
#[test]
fn a_key_no_revocation_names_is_authorized_at_the_same_height() {
    assert!(qualification_at(NEVER_REVOKED, 51).is_ok());
}

/// Clause 1 is not clause 2: an unenrolled identity is refused for its own
/// reason, and the two rejections are distinguishable.
#[test]
fn an_identity_no_certificate_names_is_not_enrolled_rather_than_revoked() {
    assert!(matches!(
        enrolled_unrevoked("cblx1strangerfixture", 51, &enrollments(), &revocations()),
        Err(Error::Authorization(AuthorizationError::NotEnrolled { .. }))
    ));
}

/// A certificate does not authorize below its own `valid_from_height`.
#[test]
fn enrollment_does_not_reach_below_its_valid_from_height() {
    assert!(matches!(
        qualification_at(REVOKED, VALID_FROM_HEIGHT - 1),
        Err(Error::Authorization(AuthorizationError::NotEnrolled { .. }))
    ));
}

/// The boundary of clause 1, which [REVIEW-033] RF-004 found unpinned.
///
/// `valid_from_height` is the first height at which the certificate authorizes,
/// not the last at which it does not: the comparison is `<=`. Until this case
/// existed the suite exercised `h = 4` and `h >= 19` and never `h = 5`, so
/// mutating clause 1 from `<=` to `<` left all 176 tests of the workspace
/// green — the same shape of defect this file exists to close, one height
/// further along. Clause 2 has had its boundary pinned since the first version
/// of this file; this is the row that gives clause 1 the same treatment.
#[test]
fn enrollment_authorizes_exactly_at_its_valid_from_height() {
    assert!(qualification_at(REVOKED, VALID_FROM_HEIGHT).is_ok());
}

/// The complete rule, as the four authorization structures state it: the key
/// derives the node ID **and** the node ID carries the qualification.
#[test]
fn the_complete_rule_checks_the_derivation_and_the_qualification() {
    let public_key = [7_u8; 32];
    let derived = NodeId::derive(&public_key);

    let enrollments = vec![EnrollmentRecord {
        node_id: derived.as_str().to_owned(),
        valid_from_height: 5,
    }];
    assert!(authorize_single_key(&public_key, derived.as_str(), 21, &enrollments, &[]).is_ok());

    assert!(matches!(
        authorize_single_key(&public_key, REVOKED, 21, &enrollments, &[]),
        Err(Error::Authorization(
            AuthorizationError::KeyDoesNotDerive { .. }
        ))
    ));

    let revocations = vec![RevocationRecord {
        node_id: derived.as_str().to_owned(),
        effective_height: 21,
    }];
    assert!(matches!(
        authorize_single_key(
            &public_key,
            derived.as_str(),
            21,
            &enrollments,
            &revocations
        ),
        Err(Error::Authorization(AuthorizationError::Revoked { .. }))
    ));
}
