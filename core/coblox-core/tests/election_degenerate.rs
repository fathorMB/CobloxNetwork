//! The degenerate cases `ledger.md` enumerates, and the two joint
//! unsatisfiability halts the specification records.
//!
//! `ledger.md#degenerate-cases-and-what-the-protocol-does-instead-of-improvising`
//! is explicit that stalling is the specified outcome and that the tempting
//! repairs are refused: "An exception clause is worth exactly as much as the
//! difficulty of fabricating its trigger, and this one is free to fabricate."
//! Every stall below is therefore asserted as the *expected* result.

mod common;

use coblox_core::election::{self, CandidateFacts};
use coblox_core::error::{ElectionError, Error, SetError};
use coblox_core::hash::{AccountKey, Digest32};
use coblox_core::params::ValidatedConsensusParameters;
use coblox_core::validator_set::{ElectionRecord, ValidatorEntry, ValidatorSet};

use common::{consensus_parameters_of, consensus_parameters_pd0, zero_chain_id};

/// The PD-0 election parameters: `V = 12`, `T = 4`, `c = 3`, `m = 1`,
/// `L = 4`. Validated against the constraint block before use.
fn parameters() -> ValidatedConsensusParameters {
    consensus_parameters_of(&consensus_parameters_pd0())
}

fn node(index: u8) -> String {
    format!("cblx1node{index:02x}")
}

fn key(index: u8) -> AccountKey {
    AccountKey::from_bytes([index; 32])
}

fn entry(index: u8, seated_since_epoch: u64, term_expiry_epoch: u64) -> ValidatorEntry {
    ValidatorEntry {
        validator_id: node(index),
        node_id: node(index),
        consensus_public_key: [index; 32],
        key_binding_signature: [index; 64],
        seated_since_epoch,
        term_expiry_epoch,
        voting_power: 1,
    }
}

fn fact(index: u8, eligible: bool) -> CandidateFacts {
    CandidateFacts {
        node_id: node(index),
        account_key: key(index),
        consensus_public_key: [index; 32],
        key_binding_signature: [index; 64],
        eligible,
    }
}

fn set_of(validators: Vec<ValidatorEntry>, activation_height: u64, epoch: u64) -> ValidatorSet {
    let member_count = u64::try_from(validators.len()).unwrap();
    let retained = validators
        .iter()
        .filter(|entry| entry.seated_since_epoch < epoch)
        .count();
    ValidatorSet {
        schema_version: "0.1".to_owned(),
        activation_height,
        election: Some(ElectionRecord {
            election_epoch: epoch,
            previous_validator_set_hash: Digest32::repeated(0xee),
            candidate_root: Digest32::repeated(0xdd),
            candidate_count: member_count,
            entropy_first_height: activation_height.saturating_sub(2),
            entropy_block_ids: vec![Digest32::repeated(0x0a), Digest32::repeated(0x0b)],
            election_seed: Digest32::repeated(0xcc),
            retained_count: u64::try_from(retained).unwrap(),
            filled_count: member_count - u64::try_from(retained).unwrap(),
            member_count,
        }),
        validators,
    }
}

fn entropy() -> Vec<Digest32> {
    // `election_entropy_blocks` is 2 in the PD-0 fixture.
    vec![Digest32::repeated(0xaa), Digest32::repeated(0xbb)]
}

/// "Fewer eligible candidates than seats. `fills` is a minimum over three
/// quantities, one of which is `|Nw|`, so a short pool simply produces a
/// smaller set. Nothing is relaxed to fill it."
#[test]
fn a_short_pool_produces_a_smaller_set_and_relaxes_nothing() {
    let parameters = parameters();
    // Twelve incumbents, nine of them still eligible and in term, no newcomers.
    let previous = set_of((0..12).map(|i| entry(i, 4, 9)).collect(), 20, 5);
    let facts: Vec<CandidateFacts> = (0..12).map(|i| fact(i, i < 9)).collect();

    let derivation = election::derive(
        &zero_chain_id(),
        &parameters,
        6,
        &previous,
        &facts,
        &entropy(),
    )
    .expect("nine of twelve clears the contraction floor");
    assert_eq!(derivation.set.validators.len(), 9);
    assert_eq!(derivation.fills, 0);
    // The target set size is 12 and the pool was empty: nothing was relaxed to
    // reach it.
    assert_eq!(derivation.candidates.len(), 9);
}

/// "if it shrinks below `validator_min_set_size` **or past the contraction
/// floor** the chain stalls at the boundary."
#[test]
fn a_pool_too_short_to_clear_the_floor_stalls_the_chain() {
    let previous = set_of((0..12).map(|i| entry(i, 4, 9)).collect(), 20, 5);
    let facts: Vec<CandidateFacts> = (0..12).map(|i| fact(i, i < 8)).collect();
    let outcome = election::derive(
        &zero_chain_id(),
        &parameters(),
        6,
        &previous,
        &facts,
        &entropy(),
    );
    // 3 * 8 = 24 is not greater than 2 * 12 = 24: strictly, so it fails.
    assert_eq!(
        outcome.unwrap_err(),
        Error::Election(ElectionError::ContractionFloor {
            new: 8,
            previous: 12
        })
    );
}

/// The first of the two halts: **a synchronized genesis cohort**.
///
/// "If all of them carry the same expiry, they expire together. At that
/// boundary `R` is empty, so the new set is whatever the fill step can supply,
/// which is at most `c`; the contraction floor then demands `3c > 2V` [...]
/// while the capture constraint demands `3 * c * m <= V`. The interval is empty
/// for every `V`."
#[test]
fn a_synchronized_cohort_expiring_at_one_boundary_halts_the_chain() {
    let parameters = parameters();
    // Twelve seats all stamped with the same expiry, 4 (= T).
    let previous = set_of((0..12).map(|i| entry(i, 0, 4)).collect(), 0, 0);
    // A full pool of twelve fresh candidates is available; it does not help,
    // "because what limits the rebuild is the entry cap and not a shortage of
    // candidates".
    let facts: Vec<CandidateFacts> = (100..112).map(|i| fact(i, true)).collect();
    let outcome = election::derive(
        &zero_chain_id(),
        &parameters,
        4,
        &previous,
        &facts,
        &entropy(),
    );
    assert_eq!(
        outcome.unwrap_err(),
        Error::Election(ElectionError::ContractionFloor {
            new: 3,
            previous: 12
        })
    );
}

/// The rule that prevents that halt, checked where it is checkable: at the
/// trust anchor. "A genesis set violating either condition is not a valid trust
/// anchor and a client MUST refuse it."
#[test]
fn the_genesis_stagger_rule_refuses_a_synchronized_trust_anchor() {
    let parameters = parameters();
    let synchronized = ValidatorSet {
        schema_version: "0.1".to_owned(),
        activation_height: 0,
        election: None,
        validators: (0..12).map(|i| entry(i, 0, 4)).collect(),
    };
    assert!(synchronized.check_genesis_stagger(&parameters).is_err());

    // A staggered anchor: expiries in [1, T] with at most c = 3 sharing a value.
    let staggered = ValidatorSet {
        schema_version: "0.1".to_owned(),
        activation_height: 0,
        election: None,
        validators: (0..12).map(|i| entry(i, 0, u64::from(i / 3) + 1)).collect(),
    };
    staggered
        .check_genesis_stagger(&parameters)
        .expect("a staggered genesis anchor is valid");

    // Out of range in either direction.
    for expiry in [0u64, 5] {
        let bad = ValidatorSet {
            validators: (0..12)
                .map(|i| {
                    if i == 0 {
                        entry(i, 0, expiry)
                    } else {
                        entry(i, 0, u64::from(i / 3) + 1)
                    }
                })
                .collect(),
            ..staggered.clone()
        };
        assert!(bad.check_genesis_stagger(&parameters).is_err());
    }

    // An election record on the genesis set is itself a refusal.
    let anchored_with_record = set_of((0..12).map(|i| entry(i, 0, 1)).collect(), 0, 0);
    assert!(
        anchored_with_record
            .check_genesis_stagger(&parameters)
            .is_err()
    );
}

/// The second of the two halts: **a term limit walked downwards**.
///
/// "With `V = 12` and `c = 3`, walking `T` down one step per boundary from 12
/// to 4 [...] sends the seat filled at each of those boundaries to the same
/// expiry epoch [...] Nine of twelve seats then expire at boundary 12: `R` is
/// three, `fills` is capped at three, the new set is six against a previous
/// twelve, and `3 * 6 > 2 * 12` is false."
#[test]
fn a_term_limit_walked_downwards_collides_the_stamps_and_halts_the_chain() {
    let parameters = parameters();
    // Nine seats stamped 12 by the walk, three seats still in term.
    let mut validators: Vec<ValidatorEntry> =
        (0..9).map(|i| entry(i, u64::from(i) + 1, 12)).collect();
    validators.extend((9..12).map(|i| entry(i, 11, 15)));
    validators.sort_by(|left, right| left.validator_id.cmp(&right.validator_id));
    let previous = set_of(validators, 44, 11);

    // A full candidate pool, again: it does not save it.
    let mut facts: Vec<CandidateFacts> = (9..12).map(|i| fact(i, true)).collect();
    facts.extend((100..112).map(|i| fact(i, true)));

    let outcome = election::derive(
        &zero_chain_id(),
        &parameters,
        12,
        &previous,
        &facts,
        &entropy(),
    );
    assert_eq!(
        outcome.unwrap_err(),
        Error::Election(ElectionError::ContractionFloor {
            new: 6,
            previous: 12
        })
    );
    // The acceptance-time rule that prevents this state from ever arising is
    // the monotonic term limit, exercised in `constraint_block.rs`.
}

/// "**Revocation between two boundaries.** A revocation-forced transition
/// removes members and cannot admit any [...] At the next boundary the
/// derivation runs with the interim set as `P`, so the vacancies revocation
/// created are refilled **under the ordinary cap**, not in one step."
#[test]
fn a_removal_only_transition_removes_and_never_admits() {
    let previous = set_of((0..12).map(|i| entry(i, 4, 9)).collect(), 20, 5);
    let revocations: Vec<(String, u64)> = (9..12).map(|i| (node(i), 21u64)).collect();

    // Removing the three revoked members: 3 * 9 = 27 > 2 * 12 = 24.
    let mut interim = previous.clone();
    interim
        .validators
        .retain(|entry| !revocations.iter().any(|(id, _)| *id == entry.node_id));
    interim.activation_height = 22;
    if let Some(record) = interim.election.as_mut() {
        record.member_count = 9;
    }
    interim
        .check_removal_only_transition(&previous, &revocations)
        .expect("a lawful removal-only transition");

    // Removing five would breach the floor: 3 * 7 = 21 is not above 24.
    let mut too_far = previous.clone();
    too_far.validators.truncate(7);
    too_far.activation_height = 22;
    if let Some(record) = too_far.election.as_mut() {
        record.member_count = 7;
    }
    let wide_revocations: Vec<(String, u64)> = (7..12).map(|i| (node(i), 21u64)).collect();
    assert_eq!(
        too_far
            .check_removal_only_transition(&previous, &wide_revocations)
            .unwrap_err(),
        Error::ValidatorSet(SetError::ContractionFloor { new: 7, old: 12 })
    );

    // Admitting anyone is refused outright, whatever the arithmetic says.
    let mut laundered = interim.clone();
    laundered.validators.push(entry(200, 5, 9));
    laundered
        .validators
        .sort_by(|left, right| left.validator_id.cmp(&right.validator_id));
    if let Some(record) = laundered.election.as_mut() {
        record.member_count = 10;
    }
    assert!(
        laundered
            .check_removal_only_transition(&previous, &revocations)
            .is_err()
    );

    // A retained entry may re-issue only its key binding.
    let mut tampered = interim.clone();
    tampered.validators[0].key_binding_signature = [0x99; 64];
    tampered
        .check_removal_only_transition(&previous, &revocations)
        .expect("re-issuing key_binding_signature is permitted");
    tampered.validators[0].term_expiry_epoch += 1;
    assert!(
        tampered
            .check_removal_only_transition(&previous, &revocations)
            .is_err()
    );
}

/// Rules 1 and 2 of the revocation rule: a set containing a revoked identity at
/// or after the effective height is invalid, and the entry's power is never
/// reweighted — the set is simply rejected.
#[test]
fn a_set_containing_a_revoked_identity_is_rejected_rather_than_reweighted() {
    let set = set_of((0..12).map(|i| entry(i, 4, 9)).collect(), 30, 5);
    let revocations = vec![(node(3), 30u64)];
    assert_eq!(
        set.check_against_revocations(&revocations).unwrap_err(),
        Error::ValidatorSet(SetError::Revocation { node_id: node(3) })
    );
    // Below the effective height the same set is untouched.
    assert!(set.check_against_revocations(&[(node(3), 31)]).is_ok());
    // The power is not adjusted anywhere: the set still sums to twelve.
    assert_eq!(set.total_voting_power().unwrap(), 12);
}

/// "A revoked identity fails eligibility condition 1 permanently. A candidacy
/// finalized before the revocation does not survive it." A revoked incumbent
/// therefore leaves the set and is absent from `C` at the same boundary.
#[test]
fn a_revoked_incumbent_leaves_the_set_and_the_candidate_pool_together() {
    let previous = set_of((0..12).map(|i| entry(i, 4, 9)).collect(), 20, 5);
    // Node 11 is revoked: not eligible, and therefore not in C either.
    let facts: Vec<CandidateFacts> = (0..12).map(|i| fact(i, i != 11)).collect();
    let derivation = election::derive(
        &zero_chain_id(),
        &parameters(),
        6,
        &previous,
        &facts,
        &entropy(),
    )
    .expect("eleven of twelve clears the floor");
    assert!(derivation.set.find(&node(11)).is_none());
    assert!(!derivation.candidates.contains(&key(11)));
    assert_eq!(derivation.set.election.unwrap().candidate_count, 11);
}

/// A departed member offered back in `C` is a contradiction of the derivation,
/// not something to reconcile: "A member of `P` that failed step 1 is **not**
/// in `Nw` for this epoch, and is not in `C` either."
#[test]
fn a_departed_member_cannot_reappear_in_the_committed_candidate_set() {
    let previous = set_of((0..12).map(|i| entry(i, 4, 6)).collect(), 20, 5);
    // Node 0's term expires at 6, so it cannot be retained at e = 6 — yet the
    // caller claims it is eligible.
    let facts: Vec<CandidateFacts> = (0..12).map(|i| fact(i, true)).collect();
    let outcome = election::derive(
        &zero_chain_id(),
        &parameters(),
        6,
        &previous,
        &facts,
        &entropy(),
    );
    assert!(matches!(
        outcome.unwrap_err(),
        Error::Election(ElectionError::InconsistentCandidateSet { .. })
    ));
}

/// The derivation is a total function of its inputs: the same finalized data
/// yields byte-identical sets.
#[test]
fn the_derivation_is_deterministic_byte_for_byte() {
    let previous = set_of((0..12).map(|i| entry(i, 4, 9)).collect(), 20, 5);
    let mut facts: Vec<CandidateFacts> = (0..12).map(|i| fact(i, true)).collect();
    facts.extend((100..108).map(|i| fact(i, true)));

    let first = election::derive(
        &zero_chain_id(),
        &parameters(),
        6,
        &previous,
        &facts,
        &entropy(),
    )
    .unwrap();

    // The same facts, presented in a different order.
    let mut shuffled = facts.clone();
    shuffled.reverse();
    let second = election::derive(
        &zero_chain_id(),
        &parameters(),
        6,
        &previous,
        &shuffled,
        &entropy(),
    )
    .unwrap();

    assert_eq!(
        first.set.to_json().unwrap().to_jcs(),
        second.set.to_json().unwrap().to_jcs()
    );
    assert_eq!(first.set.hash().unwrap(), second.set.hash().unwrap());
    // The cap bound: twelve retained, target twelve, so nothing was filled.
    assert_eq!(first.fills, 0);
}

/// Epoch 0 is the genesis set and is never derived, and the entropy window must
/// hold exactly `election_entropy_blocks` identifiers.
#[test]
fn the_derivation_rejects_inputs_it_cannot_be_a_function_of() {
    let previous = set_of((0..12).map(|i| entry(i, 4, 9)).collect(), 20, 5);
    let facts: Vec<CandidateFacts> = (0..12).map(|i| fact(i, true)).collect();
    assert_eq!(
        election::derive(
            &zero_chain_id(),
            &parameters(),
            0,
            &previous,
            &facts,
            &entropy()
        )
        .unwrap_err(),
        Error::Election(ElectionError::NotAnElectionEpoch)
    );
    assert_eq!(
        election::derive(
            &zero_chain_id(),
            &parameters(),
            6,
            &previous,
            &facts,
            &[Digest32::repeated(0xaa)]
        )
        .unwrap_err(),
        Error::Election(ElectionError::EntropyWindow {
            expected: 2,
            actual: 1
        })
    );
}

/// The stamps: a retained entry keeps both values unchanged, a newly seated one
/// carries `e` and `e + T`, and a set that says otherwise is rejected.
#[test]
fn expiry_stamps_are_carried_for_the_retained_and_written_for_the_filled() {
    let parameters = parameters();
    let previous = set_of((0..9).map(|i| entry(i, 4, 9)).collect(), 20, 5);
    let mut facts: Vec<CandidateFacts> = (0..9).map(|i| fact(i, true)).collect();
    facts.extend((100..104).map(|i| fact(i, true)));

    let derivation = election::derive(
        &zero_chain_id(),
        &parameters,
        6,
        &previous,
        &facts,
        &entropy(),
    )
    .unwrap();
    assert_eq!(derivation.fills, 3); // the churn cap binds at c = 3
    for entry in &derivation.set.validators {
        if entry.seated_since_epoch == 6 {
            // e + T = 6 + 4
            assert_eq!(entry.term_expiry_epoch, 10);
        } else {
            assert_eq!((entry.seated_since_epoch, entry.term_expiry_epoch), (4, 9));
        }
    }
    derivation
        .set
        .check_stamps_against_previous(&previous, &parameters)
        .expect("the derived stamps are consistent");

    // A retained member whose stamp was extended is rejected.
    let mut tampered = derivation.set.clone();
    tampered.validators[0].term_expiry_epoch += 1;
    assert!(
        tampered
            .check_stamps_against_previous(&previous, &parameters)
            .is_err()
    );
}
