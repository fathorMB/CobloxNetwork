//! The election constraint block, the genesis bounds, and the two other
//! acceptance-time validity rules on governed documents.
//!
//! `ledger.md#magnitudes-not-only-relations-the-bounds-are-fixed-at-genesis`
//! lists the block; `README.md#the-enrollment-cost-floor-is-a-validity-rule-not-a-recommendation`
//! and `ledger.md#creator-share-cap-a-validity-rule-not-a-policy-note` are the
//! other two. All three are validity rules: a document that violates one is
//! **invalid**, not merely unwise, and validation is recoverable rather than a
//! panic because in production it arrives signed by a quorum.

mod common;

use coblox_core::error::{Error, ParameterError};
use coblox_core::params::{
    ActiveConsensusDocument, ConsensusParameters, ElectionBounds, EnrollmentParameters,
    RewardPolicyConstraints, existence_income_share,
};

use common::{Pd0Kind, consensus_parameters_pd0, permissive_bounds, protocol_document_pd0};

fn validate(parameters: &ConsensusParameters) -> Result<(), Error> {
    parameters
        .validate(&permissive_bounds(), 1, 1, None)
        .map(|_| ())
}

/// The suite's own obligation, discharged before any fixture is used:
/// "Suites MUST therefore validate their own parameter fixtures against the
/// constraint block before using them."
#[test]
fn the_pd0_consensus_fixture_satisfies_the_constraint_block() {
    validate(&consensus_parameters_pd0()).expect("the PD-0 consensus fixture must be admissible");
    // And the document it serializes to reads back as the same parameters.
    let document = protocol_document_pd0(Pd0Kind::Consensus);
    let parsed = ConsensusParameters::from_body(document.object("body").unwrap()).unwrap();
    assert_eq!(parsed, consensus_parameters_pd0());
}

/// "The block requires `ceil(V/T) <= c < V/3`, which is **unsatisfiable for
/// `T <= 3` at any `V`** [...] Both are proved by exhausting the parameter
/// space rather than argued."
///
/// This exhausts the space through the real validation path rather than through
/// a restatement of the inequality, so a bug in the implementation of either
/// rule shows up here.
#[test]
fn a_term_limit_of_three_or_fewer_is_rejected_at_every_set_size() {
    for target in 1u64..=64 {
        for terms in 1u64..=3 {
            for cap in 1u64..=target {
                let parameters = ConsensusParameters {
                    validator_target_set_size: target,
                    validator_max_set_size: target,
                    validator_max_consecutive_terms: terms,
                    validator_churn_cap_seats: cap,
                    ..consensus_parameters_pd0()
                };
                assert!(
                    validate(&parameters).is_err(),
                    "T={terms} was accepted at V={target}, c={cap}"
                );
            }
        }
    }
}

/// "`V:"3"` is impossible for the same kind of reason, since `3c < 3` cannot
/// hold for any `c >= 1`."
#[test]
fn a_target_set_size_of_three_is_rejected_at_every_term_limit() {
    for terms in 1u64..=64 {
        for cap in 1u64..=3 {
            let parameters = ConsensusParameters {
                validator_target_set_size: 3,
                validator_max_set_size: 3,
                validator_max_consecutive_terms: terms,
                validator_churn_cap_seats: cap,
                validator_cooldown_epochs: 1,
                ..consensus_parameters_pd0()
            };
            assert!(
                validate(&parameters).is_err(),
                "V=3 accepted at T={terms}, c={cap}"
            );
        }
    }
}

/// `T >= max(4, 3 * m)`, the joint satisfiability the block writes out.
#[test]
fn the_capture_horizon_is_bounded_by_the_term_limit() {
    // m = 2 needs T >= 6; T = 5 must be rejected however c and V are chosen.
    for target in 1u64..=64 {
        for cap in 1u64..=target {
            let parameters = ConsensusParameters {
                validator_target_set_size: target,
                validator_max_set_size: target,
                validator_max_consecutive_terms: 5,
                validator_churn_cap_seats: cap,
                validator_min_capture_epochs: 2,
                ..consensus_parameters_pd0()
            };
            assert!(
                validate(&parameters).is_err(),
                "T=5 accepted with m=2 at V={target}, c={cap}"
            );
        }
    }
    // And it is satisfiable at T = 6: V = 18, c = 3, m = 2 gives
    // ceil(18/6) = 3 <= 3, 3*3 = 9 < 18, 3*3*2 = 18 <= 18.
    let ok = ConsensusParameters {
        validator_target_set_size: 18,
        validator_max_set_size: 18,
        validator_max_consecutive_terms: 6,
        validator_churn_cap_seats: 3,
        validator_min_capture_epochs: 2,
        ..consensus_parameters_pd0()
    };
    validate(&ok).expect("T = 6 with m = 2 is admissible");
}

/// Each relational line of the block, violated one at a time.
#[test]
fn every_relational_constraint_is_enforced_individually() {
    let base = consensus_parameters_pd0();
    let cases: Vec<(&str, ConsensusParameters)> = vec![
        (
            "0 < validator_min_set_size",
            ConsensusParameters {
                validator_min_set_size: 0,
                ..base
            },
        ),
        (
            "validator_min_set_size <= V",
            ConsensusParameters {
                validator_min_set_size: 13,
                ..base
            },
        ),
        (
            "V <= validator_max_set_size",
            ConsensusParameters {
                validator_max_set_size: 11,
                ..base
            },
        ),
        (
            "election_entropy_blocks >= 2",
            ConsensusParameters {
                election_entropy_blocks: 1,
                ..base
            },
        ),
        (
            "candidacy_close_blocks > election_entropy_blocks",
            ConsensusParameters {
                candidacy_close_blocks: 2,
                ..base
            },
        ),
        (
            "election_epoch_blocks > candidacy_close_blocks",
            ConsensusParameters {
                election_epoch_blocks: 3,
                ..base
            },
        ),
        (
            "validator_cooldown_epochs >= 1",
            ConsensusParameters {
                validator_cooldown_epochs: 0,
                ..base
            },
        ),
        (
            "validator_cooldown_epochs <= T",
            ConsensusParameters {
                validator_cooldown_epochs: 5,
                ..base
            },
        ),
        (
            "ceil(V / T) <= c",
            ConsensusParameters {
                validator_churn_cap_seats: 2,
                ..base
            },
        ),
        (
            "3 * c < V",
            ConsensusParameters {
                validator_target_set_size: 9,
                validator_max_set_size: 9,
                validator_churn_cap_seats: 3,
                ..base
            },
        ),
        (
            "3 * c * m <= V",
            ConsensusParameters {
                validator_min_capture_epochs: 2,
                validator_max_consecutive_terms: 6,
                ..base
            },
        ),
    ];
    for (rule, parameters) in cases {
        let outcome = validate(&parameters);
        assert!(outcome.is_err(), "violating `{rule}` was accepted");
    }
}

/// The magnitude bounds come from the genesis anchor and never from the
/// document under evaluation.
#[test]
fn the_genesis_magnitude_bounds_are_enforced() {
    let bounds = ElectionBounds {
        election_epoch_blocks_max: 4,
        validator_max_consecutive_terms_max: 4,
        validator_max_set_size_max: 12,
        validator_min_set_size_min: 1,
        validator_min_capture_epochs_min: 1,
        ..permissive_bounds()
    };
    // The PD-0 fixture sits exactly on every one of those ceilings.
    consensus_parameters_pd0()
        .validate(&bounds, 1, 1, None)
        .expect("PD-0 is within these bounds");

    // The freeze attack the bounds exist to stop: an epoch length and a term
    // limit large enough that no boundary and no expiry ever arrives.
    let frozen = ConsensusParameters {
        election_epoch_blocks: 1 << 60,
        validator_max_consecutive_terms: 1 << 60,
        ..consensus_parameters_pd0()
    };
    assert!(frozen.validate(&bounds, 1, 1, None).is_err());
}

/// The change ratio, the activation gap, the monotonic term limit and the
/// strictly increasing sequence, each against the currently active document.
#[test]
fn the_rules_that_compare_against_the_active_document() {
    let bounds = permissive_bounds(); // ratio 3/2, gap 100
    let active = ActiveConsensusDocument {
        sequence: 7,
        activation_height: 1_000,
        parameters: consensus_parameters_pd0(),
    };

    // A move inside the ratio, spaced by the gap, is accepted. V and the
    // max set size move together from 12 to 16 (16*2 <= 12*3 holds exactly).
    let inside = ConsensusParameters {
        validator_target_set_size: 16,
        validator_max_set_size: 16,
        validator_churn_cap_seats: 4,
        ..consensus_parameters_pd0()
    };
    inside
        .validate(&bounds, 1_100, 8, Some(&active))
        .expect("a move inside the ratio, at the gap, is accepted");

    // The same move one block too early.
    assert_eq!(
        inside
            .validate(&bounds, 1_099, 8, Some(&active))
            .unwrap_err(),
        Error::Parameter(ParameterError::ActivationGap)
    );

    // A move outside the ratio: 12 -> 24 exceeds 3/2.
    let outside = ConsensusParameters {
        validator_target_set_size: 24,
        validator_max_set_size: 24,
        validator_churn_cap_seats: 7,
        ..consensus_parameters_pd0()
    };
    assert_eq!(
        outside
            .validate(&bounds, 1_100, 8, Some(&active))
            .unwrap_err(),
        Error::Parameter(ParameterError::ChangeRatio {
            parameter: "validator_target_set_size"
        })
    );

    // "On a live chain the term limit never decreases." T = 4 is the active
    // value; the block already forbids T <= 3 outright, so the decrease is
    // exercised from a higher active value.
    let active_long = ActiveConsensusDocument {
        parameters: ConsensusParameters {
            validator_max_consecutive_terms: 6,
            ..consensus_parameters_pd0()
        },
        ..active
    };
    let shortened = ConsensusParameters {
        validator_max_consecutive_terms: 5,
        ..consensus_parameters_pd0()
    };
    assert_eq!(
        shortened
            .validate(&bounds, 1_100, 8, Some(&active_long))
            .unwrap_err(),
        Error::Parameter(ParameterError::TermLimitDecreased)
    );
    // Raising it is unrestricted beyond the ceiling and the ratio.
    let lengthened = ConsensusParameters {
        validator_max_consecutive_terms: 8,
        ..consensus_parameters_pd0()
    };
    lengthened
        .validate(&bounds, 1_100, 8, Some(&active_long))
        .expect("raising the term limit is permitted");

    // `sequence` is strictly increasing per kind.
    assert_eq!(
        consensus_parameters_pd0()
            .validate(&bounds, 1_100, 7, Some(&active))
            .unwrap_err(),
        Error::Parameter(ParameterError::SequenceNotIncreasing)
    );
}

/// The `ElectionBounds` object is itself checked before it is trusted.
#[test]
fn the_election_bounds_object_is_validated_against_the_configured_chain() {
    let chain = common::zero_chain_id();
    permissive_bounds().validate(&chain).expect("valid bounds");

    let wrong_chain = ElectionBounds {
        chain_id: coblox_core::hash::ChainId::from_digest(coblox_core::hash::Digest32::repeated(1)),
        ..permissive_bounds()
    };
    assert_eq!(
        wrong_chain.validate(&chain).unwrap_err(),
        Error::Parameter(ParameterError::ChainIdMismatch)
    );

    for bad in [
        ElectionBounds {
            election_parameter_change_denominator: 0,
            ..permissive_bounds()
        },
        ElectionBounds {
            election_parameter_change_numerator: 2,
            election_parameter_change_denominator: 2,
            ..permissive_bounds()
        },
        ElectionBounds {
            election_parameter_min_activation_gap_blocks: 0,
            ..permissive_bounds()
        },
    ] {
        assert!(bad.validate(&chain).is_err());
    }
}

/// The boundary conformance fixtures of the enrollment cost floor, which the
/// specification says "a suite MUST exercise".
#[test]
fn the_enrollment_cost_floor_boundary_fixtures() {
    let base = EnrollmentParameters {
        pow_algorithm: EnrollmentParameters::POW_ALGORITHM.to_owned(),
        difficulty_bits: 4,
        memory_kib: 65_536,
        iterations: 3,
        lanes: 4,
        tag_length_bytes: 32,
        max_request_age_ms: 1,
        max_future_skew_ms: 1,
        recent_block_window: 1,
    };
    let cases = [
        // memory_kib, iterations, lanes, expected valid, reason
        (
            65_536u64,
            3u64,
            4u64,
            true,
            "RFC second recommended profile; exactly at the floor",
        ),
        (65_535, 3, 4, false, "below the memory-hardness floor"),
        (65_536, 2, 4, false, "area 131072 < 196608"),
        (
            2_097_152,
            1,
            4,
            true,
            "RFC first recommended profile, 2 GiB",
        ),
        (
            8,
            1,
            1,
            false,
            "RFC domain minimum at lanes: 1; not a security floor",
        ),
    ];
    for (memory_kib, iterations, lanes, expected, reason) in cases {
        let parameters = EnrollmentParameters {
            memory_kib,
            iterations,
            lanes,
            ..base.clone()
        };
        assert_eq!(
            parameters.validate().is_ok(),
            expected,
            "memory_kib={memory_kib} iterations={iterations}: {reason}"
        );
    }

    // The narrowed domain and the tag length.
    for lanes in [0u64, 17] {
        assert!(
            EnrollmentParameters {
                lanes,
                ..base.clone()
            }
            .validate()
            .is_err()
        );
    }
    assert!(
        EnrollmentParameters {
            tag_length_bytes: 16,
            ..base.clone()
        }
        .validate()
        .is_err()
    );
    // `difficulty_bits` is in the inclusive range 2-6 and that range is
    // normative in identity.md.
    for difficulty_bits in [0u64, 1, 7, 18, 40] {
        assert!(
            EnrollmentParameters {
                difficulty_bits,
                ..base.clone()
            }
            .validate()
            .is_err(),
            "difficulty_bits={difficulty_bits} was accepted"
        );
    }
    for difficulty_bits in 2u64..=6 {
        EnrollmentParameters {
            difficulty_bits,
            ..base.clone()
        }
        .validate()
        .expect("2-6 is the normative range");
    }
    // v0 fixes the algorithm.
    assert!(
        EnrollmentParameters {
            pow_algorithm: "sha256-leading-zero-bits-v0".to_owned(),
            ..base.clone()
        }
        .validate()
        .is_err()
    );
}

/// "with `kn/kd` and a counted burn sum `B`, a mint of exactly
/// `floor(kn * B / kd)` is valid and that value plus one is invalid."
#[test]
fn the_creator_share_cap_boundary() {
    let policy = RewardPolicyConstraints {
        publisher_reward_cap_numerator: 3,
        publisher_reward_cap_denominator: 7,
        storage_units_per_contribution_unit: 1,
        compute_units_per_contribution_unit: 1,
        validator_eligibility_window_epochs: 1,
        validator_eligibility_min_issuers: 2,
    };
    policy.validate().expect("a valid reward policy");
    for burn in [0u64, 1, 10, 1_000, 38_400_000] {
        let boundary = 3u128 * u128::from(burn) / 7;
        let boundary = u64::try_from(boundary).unwrap();
        assert!(
            policy.publisher_reward_within_cap(boundary, burn).unwrap(),
            "floor(kn*B/kd) must be valid at B={burn}"
        );
        assert!(
            !policy
                .publisher_reward_within_cap(boundary + 1, burn)
                .unwrap(),
            "floor(kn*B/kd)+1 must be invalid at B={burn}"
        );
    }
}

/// The reward-policy acceptance rules of the constraint block and of
/// `README.md#signed-protocol-documents`.
#[test]
fn the_reward_policy_acceptance_rules() {
    let base = RewardPolicyConstraints {
        publisher_reward_cap_numerator: 1,
        publisher_reward_cap_denominator: 2,
        storage_units_per_contribution_unit: 1,
        compute_units_per_contribution_unit: 1,
        validator_eligibility_window_epochs: 1,
        validator_eligibility_min_issuers: 2,
    };
    base.validate()
        .expect("the reward PD-0 values are admissible");
    let parsed = RewardPolicyConstraints::from_body(
        protocol_document_pd0(Pd0Kind::Reward)
            .object("body")
            .unwrap(),
    )
    .unwrap();
    assert_eq!(parsed, base);

    for bad in [
        RewardPolicyConstraints {
            publisher_reward_cap_denominator: 0,
            ..base
        },
        RewardPolicyConstraints {
            publisher_reward_cap_numerator: 2,
            publisher_reward_cap_denominator: 2,
            ..base
        },
        RewardPolicyConstraints {
            storage_units_per_contribution_unit: 0,
            ..base
        },
        RewardPolicyConstraints {
            compute_units_per_contribution_unit: 0,
            ..base
        },
        RewardPolicyConstraints {
            validator_eligibility_window_epochs: 0,
            ..base
        },
        RewardPolicyConstraints {
            validator_eligibility_min_issuers: 1,
            ..base
        },
    ] {
        assert!(bad.validate().is_err());
    }
}

/// "`amount_microtokens = F / E` // integer division, remainder discarded",
/// with `E > 0`.
#[test]
fn existence_income_is_an_exact_quotient_of_a_capped_fund() {
    let fund = 1_000_000u64;
    let eligible = 3u64;
    let share = existence_income_share(fund, eligible).unwrap();
    assert_eq!(share, 333_333);
    // The remainder is not minted and is not carried forward, so total
    // existence emission for the epoch is at most `F` by construction.
    assert!(share * eligible < fund);
    assert_eq!(existence_income_share(0, 1).unwrap(), 0);
    assert!(existence_income_share(1_000_000, 0).is_err());
}
