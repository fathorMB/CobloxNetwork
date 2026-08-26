//! Fixture `AUTH-0` of
//! `ledger.md#what-enrolled-unrevoked-means-and-as-of-which-height`.
//!
//! **The two rows that matter are the ones the two readings disagree on.**
//! Under [ADR-017], *enrolled, unrevoked* on the transaction authorization path
//! bites at the height of the block that *included* the revocation (`included_height`),
//! while `effective_height` governs the validator set transition path.
//! Under the previous reading where spending was permitted until `effective_height` 50,
//! rows 21 and 49 were valid; under [ADR-017], where the revocation is included in
//! block 20, both rows are invalid and return `AuthorizationError::Revoked`.
//!
//! The fixture also varies the one quantity the revoked rows hold constant —
//! whether a revocation for the key exists at all — because rows about one
//! revoked node would pass an implementation that rejected every key.
//!
//! **Both clauses of the definition have their boundary pinned.** Clause 1
//! has row `5` (`h = valid_from_height`); clause 2 has row `20`
//! (`h = included_height`), which is the only height separating `<=` from `<`:
//! the two predicates differ exactly where `20 == h`. Row `21` does not
//! separate them, because `20 <= 21` and `20 < 21` are both true.
//! A clause stated with an inclusive comparison and exercised only away from the boundary is a
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
/// The height of the block that included the `revoke_identity` transaction of `AUTH-0`.
const INCLUDED_HEIGHT: u64 = 20;
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
/// It is included in the block at height 20.
fn revocations() -> Vec<RevocationRecord> {
    vec![RevocationRecord {
        node_id: REVOKED.to_owned(),
        included_height: INCLUDED_HEIGHT,
    }]
}

fn qualification_at(node_id: &str, height: u64) -> Result<(), Error> {
    enrolled_unrevoked(node_id, height, &enrollments(), &revocations())
}

#[test]
fn the_revocation_does_not_bite_below_its_inclusion_height() {
    assert!(qualification_at(REVOKED, 19).is_ok());
}

/// The boundary of clause 2, which [REVIEW-039] RF-001 found unpinned.
///
/// `included_height` is the first height at which the revocation bites, not the
/// last at which it does not: the comparison is `<=`. This is the only height
/// at which `included_height <= h` and `included_height < h` disagree, so it is
/// the only case that can hold clause 2 to the inclusive reading.
#[test]
fn the_revocation_bites_exactly_at_its_inclusion_height() {
    let Err(Error::Authorization(AuthorizationError::Revoked {
        node_id,
        height,
        included_height,
    })) = qualification_at(REVOKED, INCLUDED_HEIGHT)
    else {
        panic!("expected revocation to bite at its own inclusion height 20");
    };
    assert_eq!(node_id, REVOKED);
    assert_eq!(height, INCLUDED_HEIGHT);
    assert_eq!(included_height, INCLUDED_HEIGHT);
}

/// The first of the two rows the readings disagree on.
///
/// The `revoke_identity` is included at height 20, so under [ADR-017] this burn
/// is invalid; under the older reading it was valid because `effective_height` was 50.
#[test]
fn a_revocation_included_at_20_bites_at_21() {
    let Err(Error::Authorization(AuthorizationError::Revoked {
        node_id,
        height,
        included_height,
    })) = qualification_at(REVOKED, 21)
    else {
        panic!("expected revocation to bite at height 21");
    };
    assert_eq!(node_id, REVOKED);
    assert_eq!(height, 21);
    assert_eq!(included_height, INCLUDED_HEIGHT);
}

/// The second divergent row, at height 49.
#[test]
fn a_revocation_included_at_20_bites_at_49() {
    let Err(Error::Authorization(AuthorizationError::Revoked {
        node_id,
        height,
        included_height,
    })) = qualification_at(REVOKED, 49)
    else {
        panic!("expected revocation to bite at height 49");
    };
    assert_eq!(node_id, REVOKED);
    assert_eq!(height, 49);
    assert_eq!(included_height, INCLUDED_HEIGHT);
}

#[test]
fn the_revocation_bites_at_50() {
    assert!(matches!(
        qualification_at(REVOKED, 50),
        Err(Error::Authorization(AuthorizationError::Revoked { .. }))
    ));
}

#[test]
fn the_revocation_keeps_biting_above_50() {
    assert!(matches!(
        qualification_at(REVOKED, 51),
        Err(Error::Authorization(AuthorizationError::Revoked { .. }))
    ));
}

/// The row that varies the quantity the other rows hold constant.
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
/// not the last at which it does not: the comparison is `<=`.
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
        included_height: 20,
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
