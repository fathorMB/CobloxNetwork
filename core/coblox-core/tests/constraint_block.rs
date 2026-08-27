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
    ActiveConsensusDocument, ActiveRewardPolicyDocument, ConsensusParameters, ElectionBounds,
    EnrollmentParameters, RewardBounds, RewardPolicy, existence_income_share,
};

use common::{
    Pd0Kind, consensus_parameters_pd0, permissive_bounds, permissive_reward_bounds,
    protocol_document_pd0, reward_policy_pd0, zero_chain_id,
};

fn validate(parameters: &ConsensusParameters) -> Result<(), Error> {
    parameters
        .validate(&permissive_bounds(), 1, 1, None)
        .map(|_| ())
}

fn validate_reward(policy: &RewardPolicy) -> Result<(), Error> {
    policy
        .validate(&permissive_reward_bounds(), 1, 1, None)
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
        validator_min_set_size: 12,
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
        (
            "3 * validator_min_set_size >= 2 * V",
            ConsensusParameters {
                validator_min_set_size: 7, // 3 * 7 = 21 < 2 * 12 = 24
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

/// The floor under `revocation_effective_grace_blocks` is a **genesis** bound,
/// so a `consensus_parameters` document below it is rejected on acceptance.
///
/// [REVIEW-042] RF-001, and the correction [ADR-017] took on 2026-08-27. Before
/// it, `G >= 1` lived in `check_relations` — among the rules a governed document
/// satisfies by itself — while only the ceiling was anchored in
/// `ElectionBounds`. A sitting quorum could therefore publish `G = 1`, and from
/// that height a `key_compromise` had to be signed by a quorum and included
/// inside a **two-block** window predicted before the signing round began.
///
/// The two cases below are the two halves of the closure: the document is held
/// to a floor it does not carry, and the floor itself is held to the genesis
/// relation that gives it a magnitude.
#[test]
fn the_grace_floor_is_taken_from_genesis_and_not_from_the_document() {
    let chain = zero_chain_id();

    // A distribution that demands a window of at least eight blocks: `G >= 7`.
    let bounds = ElectionBounds {
        revocation_effective_grace_blocks_min: 7,
        ..permissive_bounds()
    };
    bounds.validate(&chain).expect("7 + 1 >= 1");

    // `PD-0` carries `G = 1`, which satisfies every relational rule and is
    // exactly the value the review measured. Under these bounds it is refused.
    assert_eq!(
        consensus_parameters_pd0()
            .validate(&bounds, 1, 1, None)
            .unwrap_err(),
        Error::Parameter(ParameterError::Constraint {
            rule: "revocation_effective_grace_blocks >= revocation_effective_grace_blocks_min",
        })
    );

    // At the floor it is accepted, and one below it is not: the boundary is
    // `G == G_min` and it is pinned rather than approached.
    let at_floor = ConsensusParameters {
        revocation_effective_grace_blocks: 7,
        max_planned_revocation_delay_blocks: 64,
        ..consensus_parameters_pd0()
    };
    at_floor
        .validate(&bounds, 1, 1, None)
        .expect("G exactly at the genesis floor is accepted");
    assert!(
        ConsensusParameters {
            revocation_effective_grace_blocks: 6,
            ..at_floor
        }
        .validate(&bounds, 1, 1, None)
        .is_err()
    );

    // The floor is a relation between genesis constants and not a number: the
    // `key_compromise` window is `G + 1` blocks wide and must span at least one
    // full rotation of the minimum set. A distribution whose floor is too small
    // for its own minimum set size is refused — by the bounds object, and again
    // by the document path, which does not go through `ElectionBounds::validate`.
    let too_small = ElectionBounds {
        revocation_effective_grace_blocks_min: 7,
        validator_min_set_size_min: 9, // 7 + 1 = 8 < 9
        ..permissive_bounds()
    };
    let expected = Error::Parameter(ParameterError::Bounds {
        rule: "revocation_effective_grace_blocks_min + 1 >= validator_min_set_size_min",
    });
    assert_eq!(too_small.validate(&chain).unwrap_err(), expected);
    assert_eq!(
        at_floor.validate(&too_small, 1, 1, None).unwrap_err(),
        expected
    );

    // Exactly at the relation the same distribution is admissible.
    let exact = ElectionBounds {
        validator_min_set_size_min: 8, // 7 + 1 = 8
        ..too_small
    };
    exact.validate(&chain).expect("8 >= 8");
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
    // validator_min_set_size moves from 8 to 11 (11*2 <= 8*3 holds and 3*11 = 33 >= 32).
    let inside = ConsensusParameters {
        validator_target_set_size: 16,
        validator_max_set_size: 16,
        validator_min_set_size: 11,
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

    // A move outside the ratio: 12 -> 24 and 8 -> 16 exceeds 3/2.
    let outside = ConsensusParameters {
        validator_target_set_size: 24,
        validator_max_set_size: 24,
        validator_min_set_size: 16,
        validator_churn_cap_seats: 7,
        ..consensus_parameters_pd0()
    };
    assert_eq!(
        outside
            .validate(&bounds, 1_100, 8, Some(&active))
            .unwrap_err(),
        Error::Parameter(ParameterError::ChangeRatio {
            parameter: "validator_min_set_size"
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
    // The cap is a method on the *validated* policy, so this fixture reaches it
    // only by passing acceptance first — which is the point of [REVIEW-017]
    // RF-001: no reward is computed against a policy nobody validated.
    let bounds = RewardBounds {
        publisher_reward_cap_numerator_max: 3,
        ..permissive_reward_bounds()
    };
    let policy = RewardPolicy {
        publisher_reward_cap_numerator: 3,
        publisher_reward_cap_denominator: 7,
        ..reward_policy_pd0()
    }
    .validate(&bounds, 1, 1, None)
    .expect("a valid reward policy");
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
    let base = reward_policy_pd0();
    validate_reward(&base).expect("the reward PD-0 values are admissible");
    let parsed = RewardPolicy::from_body(
        protocol_document_pd0(Pd0Kind::Reward)
            .object("body")
            .unwrap(),
    )
    .unwrap();
    assert_eq!(parsed, base);

    // Rule 1: availability tariff must be 0
    assert!(
        validate_reward(&RewardPolicy {
            availability_microtokens_per_unit: 1,
            ..base
        })
        .is_err()
    );
    assert!(
        validate_reward(&RewardPolicy {
            availability_microtokens_per_unit: 1000,
            ..base
        })
        .is_err()
    );

    // Creator cap denominator positive and kn < kd
    for bad in [
        RewardPolicy {
            publisher_reward_cap_denominator: 0,
            ..base
        },
        RewardPolicy {
            publisher_reward_cap_numerator: 2,
            publisher_reward_cap_denominator: 2,
            ..base
        },
        RewardPolicy {
            storage_units_per_contribution_unit: 0,
            ..base
        },
        RewardPolicy {
            compute_units_per_contribution_unit: 0,
            ..base
        },
        RewardPolicy {
            validator_eligibility_window_epochs: 0,
            ..base
        },
        RewardPolicy {
            validator_eligibility_min_issuers: 1,
            ..base
        },
    ] {
        assert!(validate_reward(&bad).is_err());
    }
}

/// Boundary conformance fixtures for `3 * validator_min_set_size >= 2 * V`,
/// from `ledger.md` §`3 * validator_min_set_size >= 2 * V`.
#[test]
fn the_consensus_parameters_min_set_relational_rule_fixtures() {
    let base = consensus_parameters_pd0();
    let cases = [
        (12u64, 8u64, true, "24 >= 24, exact floor (PD-0 fixture)"),
        (12, 7, false, "21 < 24, below 2/3 floor"),
        (12, 1, false, "3 < 24, below 2/3 floor"),
        (
            27,
            18,
            true,
            "54 >= 54, exact equality for recommended values",
        ),
        (27, 17, false, "51 < 54, below 2/3 floor"),
        (36, 24, true, "72 >= 72, exact floor at V = 36"),
        (36, 18, false, "54 < 72, 50% ratio rejected on acceptance"),
    ];

    for (v, min_set, expected_valid, reason) in cases {
        let params = ConsensusParameters {
            validator_target_set_size: v,
            validator_max_set_size: v,
            validator_min_set_size: min_set,
            // T = v/3, c = 3, m = 1 satisfies ceil(V/T) <= c and 3*c < V
            validator_churn_cap_seats: 3,
            validator_max_consecutive_terms: v / 3,
            validator_min_capture_epochs: 1,
            ..base
        };
        let res = validate(&params);
        assert_eq!(
            res.is_ok(),
            expected_valid,
            "V={v} min_set={min_set} ({reason}): expected valid={expected_valid}, got={res:?}"
        );
    }
}

/// The `RewardBounds` object is validated against the configured chain.
#[test]
fn the_reward_bounds_object_is_validated_against_the_configured_chain() {
    let chain = zero_chain_id();
    permissive_reward_bounds()
        .validate(&chain)
        .expect("valid reward bounds");

    let wrong_chain = RewardBounds {
        chain_id: coblox_core::hash::ChainId::from_digest(coblox_core::hash::Digest32::repeated(1)),
        ..permissive_reward_bounds()
    };
    assert_eq!(
        wrong_chain.validate(&chain).unwrap_err(),
        Error::Parameter(ParameterError::ChainIdMismatch)
    );

    for bad in [
        RewardBounds {
            reward_parameter_change_denominator: 0,
            ..permissive_reward_bounds()
        },
        RewardBounds {
            reward_parameter_change_numerator: 2,
            reward_parameter_change_denominator: 2,
            ..permissive_reward_bounds()
        },
        RewardBounds {
            reward_parameter_min_activation_gap_blocks: 0,
            ..permissive_reward_bounds()
        },
        RewardBounds {
            reward_epoch_ms_min: 0,
            ..permissive_reward_bounds()
        },
        RewardBounds {
            reward_epoch_ms_min: 100,
            reward_epoch_ms_max: 99,
            ..permissive_reward_bounds()
        },
    ] {
        assert!(bad.validate(&chain).is_err());
    }
}

/// Exact mirroring of the published boundary tables from `README.md` §*Reward bounds*
/// and `sim/tools/reward_rules.py`.
#[test]
#[allow(clippy::too_many_lines)]
fn the_reward_policy_boundary_fixtures_mirroring_reward_rules_py() {
    let bounds = RewardBounds {
        network_id: "fixture".to_owned(),
        chain_id: zero_chain_id(),
        existence_fund_microtokens_per_epoch_max: 15_882_352_941,
        reward_epoch_ms_min: 3_600_000,   // one hour
        reward_epoch_ms_max: 604_800_000, // one week
        publisher_reward_cap_numerator_max: 1,
        publisher_reward_cap_denominator_min: 2,
        validator_eligibility_threshold_units_min: 512,
        validator_eligibility_window_epochs_max: 90,
        validator_eligibility_min_issuers_min: 2,
        storage_units_per_contribution_unit_max: 1_073_741_824,
        compute_units_per_contribution_unit_max: 1_000_000,
        storage_microtokens_per_byte_epoch_min: 1,
        compute_microtokens_per_million_fuel_min: 1,
        reward_parameter_change_numerator: 5,
        reward_parameter_change_denominator: 4,
        reward_parameter_min_activation_gap_blocks: 120_960,
    };
    bounds.validate(&zero_chain_id()).unwrap();

    let base = RewardPolicy {
        reward_epoch_ms: 86_400_000,
        existence_fund_microtokens_per_epoch: 300_000_000,
        availability_microtokens_per_unit: 0,
        storage_microtokens_per_byte_epoch: 1,
        compute_microtokens_per_million_fuel: 1,
        publisher_microtokens_per_active_subscriber: 1,
        publisher_reward_cap_numerator: 1,
        publisher_reward_cap_denominator: 2,
        storage_units_per_contribution_unit: 1_073_741_824,
        compute_units_per_contribution_unit: 1_000_000,
        validator_eligibility_threshold_units: 512,
        validator_eligibility_window_epochs: 28,
        validator_eligibility_min_issuers: 3,
    };

    // The 22 test cases of CASES in reward_rules.py
    let cases: [(&str, RewardPolicy, bool); 22] = [
        ("availability tariff 0", base, true),
        (
            "availability tariff 1",
            RewardPolicy {
                availability_microtokens_per_unit: 1,
                ..base
            },
            false,
        ),
        (
            "availability tariff 1000",
            RewardPolicy {
                availability_microtokens_per_unit: 1000,
                ..base
            },
            false,
        ),
        ("creator cap 1/2", base, true),
        (
            "creator cap 2/2",
            RewardPolicy {
                publisher_reward_cap_numerator: 2,
                publisher_reward_cap_denominator: 2,
                ..base
            },
            false,
        ),
        (
            "creator cap 1/0",
            RewardPolicy {
                publisher_reward_cap_denominator: 0,
                ..base
            },
            false,
        ),
        (
            "F exactly at the ceiling",
            RewardPolicy {
                existence_fund_microtokens_per_epoch: bounds
                    .existence_fund_microtokens_per_epoch_max,
                ..base
            },
            true,
        ),
        (
            "F one above the ceiling",
            RewardPolicy {
                existence_fund_microtokens_per_epoch: bounds
                    .existence_fund_microtokens_per_epoch_max
                    + 1,
                ..base
            },
            false,
        ),
        (
            "epoch exactly at the floor",
            RewardPolicy {
                reward_epoch_ms: bounds.reward_epoch_ms_min,
                ..base
            },
            true,
        ),
        (
            "epoch one below the floor",
            RewardPolicy {
                reward_epoch_ms: bounds.reward_epoch_ms_min - 1,
                ..base
            },
            false,
        ),
        (
            "epoch of 86 400 ms (the x1000 attack)",
            RewardPolicy {
                reward_epoch_ms: 86_400,
                ..base
            },
            false,
        ),
        (
            "epoch one above the ceiling",
            RewardPolicy {
                reward_epoch_ms: bounds.reward_epoch_ms_max + 1,
                ..base
            },
            false,
        ),
        ("storage divisor at the ceiling", base, true),
        (
            "storage divisor x 10^6",
            RewardPolicy {
                storage_units_per_contribution_unit: 1_073_741_824 * 1_000_000,
                ..base
            },
            false,
        ),
        (
            "compute divisor above the ceiling",
            RewardPolicy {
                compute_units_per_contribution_unit: 1_000_001,
                ..base
            },
            false,
        ),
        (
            "window at the ceiling",
            RewardPolicy {
                validator_eligibility_window_epochs: bounds.validator_eligibility_window_epochs_max,
                ..base
            },
            true,
        ),
        (
            "window of 3000 epochs",
            RewardPolicy {
                validator_eligibility_window_epochs: 3000,
                ..base
            },
            false,
        ),
        (
            "storage tariff at the floor",
            RewardPolicy {
                storage_microtokens_per_byte_epoch: 1,
                ..base
            },
            true,
        ),
        (
            "storage tariff zero",
            RewardPolicy {
                storage_microtokens_per_byte_epoch: 0,
                ..base
            },
            false,
        ),
        (
            "compute tariff zero",
            RewardPolicy {
                compute_microtokens_per_million_fuel: 0,
                ..base
            },
            false,
        ),
        (
            "threshold at the floor",
            RewardPolicy {
                validator_eligibility_threshold_units: 512,
                ..base
            },
            true,
        ),
        (
            "threshold below the floor",
            RewardPolicy {
                validator_eligibility_threshold_units: 511,
                ..base
            },
            false,
        ),
    ];

    for (name, policy, expected) in cases {
        let res = policy.validate(&bounds, 1, 1, None);
        assert_eq!(
            res.is_ok(),
            expected,
            "case '{name}': expected valid={expected}, got={res:?}"
        );
    }

    // The 5 test cases of RATE_CASES in reward_rules.py
    let active = ActiveRewardPolicyDocument {
        sequence: 1,
        activation_height: 0,
        policy: base,
    };

    let rate_cases: [(&str, RewardPolicy, u64, bool); 5] = [
        (
            "F at exactly 5/4",
            RewardPolicy {
                existence_fund_microtokens_per_epoch: 375_000_000,
                ..base
            },
            120_960,
            true,
        ),
        (
            "F one above 5/4",
            RewardPolicy {
                existence_fund_microtokens_per_epoch: 375_000_001,
                ..base
            },
            120_960,
            false,
        ),
        (
            "epoch 86 400 000 -> 86 400 in one document",
            RewardPolicy {
                reward_epoch_ms: 86_400,
                ..base
            },
            120_960,
            false,
        ),
        ("activation exactly at the gap", base, 120_960, true),
        ("activation one block short", base, 120_959, false),
    ];

    for (name, policy, height, expected) in rate_cases {
        let res = policy.validate(&bounds, height, 2, Some(&active));
        assert_eq!(
            res.is_ok(),
            expected,
            "rate case '{name}': expected valid={expected}, got={res:?}"
        );
    }
}

/// GATE-DIRECTION: For every new limit, tests show acceptance at the exact limit
/// and rejection in the direction of danger (above ceiling, below floor).
#[test]
#[allow(clippy::too_many_lines)]
fn the_direction_of_danger_for_all_economic_limits() {
    let bounds = RewardBounds {
        network_id: "fixture".to_owned(),
        chain_id: zero_chain_id(),
        existence_fund_microtokens_per_epoch_max: 10_000,
        reward_epoch_ms_min: 1_000,
        reward_epoch_ms_max: 50_000,
        publisher_reward_cap_numerator_max: 3,
        publisher_reward_cap_denominator_min: 5,
        validator_eligibility_threshold_units_min: 100,
        validator_eligibility_window_epochs_max: 30,
        validator_eligibility_min_issuers_min: 3,
        storage_units_per_contribution_unit_max: 1_000,
        compute_units_per_contribution_unit_max: 500,
        storage_microtokens_per_byte_epoch_min: 10,
        compute_microtokens_per_million_fuel_min: 20,
        reward_parameter_change_numerator: 5,
        reward_parameter_change_denominator: 4,
        reward_parameter_min_activation_gap_blocks: 100,
    };

    let base = RewardPolicy {
        reward_epoch_ms: 10_000,
        existence_fund_microtokens_per_epoch: 5_000,
        availability_microtokens_per_unit: 0,
        storage_microtokens_per_byte_epoch: 50,
        compute_microtokens_per_million_fuel: 50,
        publisher_microtokens_per_active_subscriber: 1,
        publisher_reward_cap_numerator: 1,
        publisher_reward_cap_denominator: 10,
        storage_units_per_contribution_unit: 500,
        compute_units_per_contribution_unit: 200,
        validator_eligibility_threshold_units: 200,
        validator_eligibility_window_epochs: 15,
        validator_eligibility_min_issuers: 4,
    };

    // 1. existence_fund_microtokens_per_epoch (ceiling)
    assert!(
        RewardPolicy {
            existence_fund_microtokens_per_epoch: 10_000,
            ..base
        }
        .validate(&bounds, 1, 1, None)
        .is_ok()
    );
    assert!(
        RewardPolicy {
            existence_fund_microtokens_per_epoch: 10_001,
            ..base
        }
        .validate(&bounds, 1, 1, None)
        .is_err()
    );

    // 2. reward_epoch_ms (floor)
    assert!(
        RewardPolicy {
            reward_epoch_ms: 1_000,
            ..base
        }
        .validate(&bounds, 1, 1, None)
        .is_ok()
    );
    assert!(
        RewardPolicy {
            reward_epoch_ms: 999,
            ..base
        }
        .validate(&bounds, 1, 1, None)
        .is_err()
    );

    // 3. reward_epoch_ms (ceiling)
    assert!(
        RewardPolicy {
            reward_epoch_ms: 50_000,
            ..base
        }
        .validate(&bounds, 1, 1, None)
        .is_ok()
    );
    assert!(
        RewardPolicy {
            reward_epoch_ms: 50_001,
            ..base
        }
        .validate(&bounds, 1, 1, None)
        .is_err()
    );

    // 4. publisher_reward_cap_numerator (ceiling)
    assert!(
        RewardPolicy {
            publisher_reward_cap_numerator: 3,
            publisher_reward_cap_denominator: 10,
            ..base
        }
        .validate(&bounds, 1, 1, None)
        .is_ok()
    );
    assert!(
        RewardPolicy {
            publisher_reward_cap_numerator: 4,
            publisher_reward_cap_denominator: 10,
            ..base
        }
        .validate(&bounds, 1, 1, None)
        .is_err()
    );

    // 5. publisher_reward_cap_denominator (floor)
    assert!(
        RewardPolicy {
            publisher_reward_cap_numerator: 1,
            publisher_reward_cap_denominator: 5,
            ..base
        }
        .validate(&bounds, 1, 1, None)
        .is_ok()
    );
    assert!(
        RewardPolicy {
            publisher_reward_cap_numerator: 1,
            publisher_reward_cap_denominator: 4,
            ..base
        }
        .validate(&bounds, 1, 1, None)
        .is_err()
    );

    // 6. validator_eligibility_threshold_units (floor)
    assert!(
        RewardPolicy {
            validator_eligibility_threshold_units: 100,
            ..base
        }
        .validate(&bounds, 1, 1, None)
        .is_ok()
    );
    assert!(
        RewardPolicy {
            validator_eligibility_threshold_units: 99,
            ..base
        }
        .validate(&bounds, 1, 1, None)
        .is_err()
    );

    // 7. validator_eligibility_window_epochs (ceiling)
    assert!(
        RewardPolicy {
            validator_eligibility_window_epochs: 30,
            ..base
        }
        .validate(&bounds, 1, 1, None)
        .is_ok()
    );
    assert!(
        RewardPolicy {
            validator_eligibility_window_epochs: 31,
            ..base
        }
        .validate(&bounds, 1, 1, None)
        .is_err()
    );

    // 8. validator_eligibility_min_issuers (floor)
    assert!(
        RewardPolicy {
            validator_eligibility_min_issuers: 3,
            ..base
        }
        .validate(&bounds, 1, 1, None)
        .is_ok()
    );
    assert!(
        RewardPolicy {
            validator_eligibility_min_issuers: 2,
            ..base
        }
        .validate(&bounds, 1, 1, None)
        .is_err()
    );

    // 9. storage_units_per_contribution_unit (ceiling)
    assert!(
        RewardPolicy {
            storage_units_per_contribution_unit: 1_000,
            ..base
        }
        .validate(&bounds, 1, 1, None)
        .is_ok()
    );
    assert!(
        RewardPolicy {
            storage_units_per_contribution_unit: 1_001,
            ..base
        }
        .validate(&bounds, 1, 1, None)
        .is_err()
    );

    // 10. compute_units_per_contribution_unit (ceiling)
    assert!(
        RewardPolicy {
            compute_units_per_contribution_unit: 500,
            ..base
        }
        .validate(&bounds, 1, 1, None)
        .is_ok()
    );
    assert!(
        RewardPolicy {
            compute_units_per_contribution_unit: 501,
            ..base
        }
        .validate(&bounds, 1, 1, None)
        .is_err()
    );

    // 11. storage_microtokens_per_byte_epoch (floor)
    assert!(
        RewardPolicy {
            storage_microtokens_per_byte_epoch: 10,
            ..base
        }
        .validate(&bounds, 1, 1, None)
        .is_ok()
    );
    assert!(
        RewardPolicy {
            storage_microtokens_per_byte_epoch: 9,
            ..base
        }
        .validate(&bounds, 1, 1, None)
        .is_err()
    );

    // 12. compute_microtokens_per_million_fuel (floor)
    assert!(
        RewardPolicy {
            compute_microtokens_per_million_fuel: 20,
            ..base
        }
        .validate(&bounds, 1, 1, None)
        .is_ok()
    );
    assert!(
        RewardPolicy {
            compute_microtokens_per_million_fuel: 19,
            ..base
        }
        .validate(&bounds, 1, 1, None)
        .is_err()
    );

    // 13. 3 * validator_min_set_size >= 2 * V (relational floor)
    let cbase = consensus_parameters_pd0();
    let c_valid = ConsensusParameters {
        validator_target_set_size: 27,
        validator_max_set_size: 27,
        validator_min_set_size: 18, // 3 * 18 = 54 == 2 * 27 = 54
        validator_churn_cap_seats: 3,
        validator_max_consecutive_terms: 9,
        validator_min_capture_epochs: 1,
        ..cbase
    };
    assert!(validate(&c_valid).is_ok());
    let c_invalid = ConsensusParameters {
        validator_min_set_size: 17, // 3 * 17 = 51 < 2 * 27 = 54
        ..c_valid
    };
    assert!(validate(&c_invalid).is_err());
}

/// The election-side twin of the finding below: the rate-of-change rule on the
/// ten `ELECTION_PARAMETERS` binds **downward** as well as upward.
///
/// Found by the Lead while reproducing [REVIEW-017] RF-002 — a mutation
/// anchored on the wrong loop, which then showed that deleting `old_bounded`
/// from `ConsensusParameters::check_against_active` left the whole suite green.
/// Same defect, same shape, older code: it comes from [SPEC-006] and [SPEC-008]
/// and not from this spec's delivery.
/// `the_rules_that_compare_against_the_active_document` exercises the ratio only
/// upward (12 -> 24), plus the gap, the term-limit ratchet and the sequence; no
/// case anywhere moved a parameter down past the ratio.
///
/// Three things about this side are worth stating rather than working around.
///
/// **The trap the reward side fell into is real here too.** A downward move can
/// be intercepted by the relational block *before* the ratio is evaluated —
/// `validate` runs `check_relations` first — exactly as the one published row
/// naming a descent was intercepted by a magnitude floor. A case written on a
/// base whose relations are tight in the direction under test passes, looks
/// like directional coverage, and constrains nothing. Each base below is
/// therefore slack enough that the relational block still holds at the boundary
/// *and* one step past it, every rejection asserts `ChangeRatio` **naming the
/// parameter that moved**, and the trap itself is exhibited at the end of this
/// test instead of being assumed absent.
///
/// **`validator_min_set_size` and `validator_target_set_size` cannot be swept
/// in both directions from one base.** `3 * validator_min_set_size >= 2 * V`
/// couples them: descending `min_set` needs `min_set >= (5/6) V`, ascending it
/// needs `min_set <= (4/5) V`, and the two intervals do not intersect. The same
/// coupling, mirrored, applies to `V`. So two bases appear below, differing only
/// in `min_set`, and which base serves which direction is dictated by that rule.
/// That is a property of the constraint block, not a shortcoming of the test.
///
/// **`validator_max_consecutive_terms` has no lawful downward document at all.**
/// Reported rather than forced, per the Lead's instruction: "on a live chain the
/// term limit never decreases", so every decrease is refused — and *which rule*
/// refuses it depends on how far it moves. That distinction is asserted below.
#[test]
#[allow(clippy::too_many_lines)]
fn the_election_rate_of_change_binds_downward_on_every_parameter() {
    type Setter = fn(ConsensusParameters, u64) -> ConsensusParameters;

    let bounds = ElectionBounds {
        network_id: "fixture".to_owned(),
        chain_id: zero_chain_id(),
        min_revocation_effective_delay_blocks_max: u64::MAX,
        revocation_effective_grace_blocks_max: u64::MAX,
        revocation_effective_grace_blocks_min: 1,
        max_planned_revocation_delay_blocks_max: u64::MAX,
        election_epoch_blocks_max: u64::MAX,
        validator_max_consecutive_terms_max: u64::MAX,
        validator_max_set_size_max: u64::MAX,
        validator_min_set_size_min: 1,
        validator_min_capture_epochs_min: 1,
        election_parameter_change_numerator: 5,
        election_parameter_change_denominator: 4,
        election_parameter_min_activation_gap_blocks: 1,
    };
    bounds.validate(&zero_chain_id()).unwrap();

    // Wide on every magnitude and slack on every relation, so that a rejection
    // in this test can only come from the ratio. Every swept value divides by
    // both 4 and 5, so each boundary is an exact integer.
    let base = ConsensusParameters {
        // The three revocation-delay parameters joined `ELECTION_PARAMETERS` on
        // [REVIEW-042] RF-003, so they are swept here like the rest. `PD-0`
        // carries them at 1, 1 and 2, which divide by neither 4 nor 5, so the
        // sweep base sets them to values that do — with `P` slack enough that
        // `P >= F + G` still holds one step past every boundary in both
        // directions, which is the trap this test's doc comment describes.
        min_revocation_effective_delay_blocks: 10_000,
        revocation_effective_grace_blocks: 10_000,
        max_planned_revocation_delay_blocks: 40_000,
        election_epoch_blocks: 10_000,
        candidacy_close_blocks: 5_000,
        election_entropy_blocks: 500,
        validator_min_set_size: 20_000,
        validator_target_set_size: 20_000,
        validator_max_set_size: 80_000,
        validator_churn_cap_seats: 100,
        validator_max_consecutive_terms: 1_000,
        validator_cooldown_epochs: 20,
        validator_min_capture_epochs: 20,
        ..consensus_parameters_pd0()
    };
    base.validate(&bounds, 1, 1, None)
        .expect("the sweep base must itself be admissible");

    // The same base with `min_set` at 0.7 V instead of at V. It serves the two
    // directions the first base cannot reach: ascending `min_set` and
    // descending `V`.
    let slack = ConsensusParameters {
        validator_min_set_size: 14_000,
        ..base
    };
    slack
        .validate(&bounds, 1, 1, None)
        .expect("the second base must itself be admissible");

    let active_of = |parameters: ConsensusParameters| ActiveConsensusDocument {
        sequence: 1,
        activation_height: 1,
        parameters,
    };

    // One row per parameter per direction. `down` selects which boundary is
    // exercised; the base is chosen per row, for the reason in the doc comment.
    let sweep: [(&str, ConsensusParameters, u64, Setter, bool); 24] = [
        (
            "min_revocation_effective_delay_blocks",
            base,
            10_000,
            |p, v| ConsensusParameters {
                min_revocation_effective_delay_blocks: v,
                ..p
            },
            true,
        ),
        (
            "min_revocation_effective_delay_blocks",
            base,
            10_000,
            |p, v| ConsensusParameters {
                min_revocation_effective_delay_blocks: v,
                ..p
            },
            false,
        ),
        (
            "revocation_effective_grace_blocks",
            base,
            10_000,
            |p, v| ConsensusParameters {
                revocation_effective_grace_blocks: v,
                ..p
            },
            true,
        ),
        (
            "revocation_effective_grace_blocks",
            base,
            10_000,
            |p, v| ConsensusParameters {
                revocation_effective_grace_blocks: v,
                ..p
            },
            false,
        ),
        (
            "max_planned_revocation_delay_blocks",
            base,
            40_000,
            |p, v| ConsensusParameters {
                max_planned_revocation_delay_blocks: v,
                ..p
            },
            true,
        ),
        (
            "max_planned_revocation_delay_blocks",
            base,
            40_000,
            |p, v| ConsensusParameters {
                max_planned_revocation_delay_blocks: v,
                ..p
            },
            false,
        ),
        (
            "election_epoch_blocks",
            base,
            10_000,
            |p, v| ConsensusParameters {
                election_epoch_blocks: v,
                ..p
            },
            true,
        ),
        (
            "election_epoch_blocks",
            base,
            10_000,
            |p, v| ConsensusParameters {
                election_epoch_blocks: v,
                ..p
            },
            false,
        ),
        (
            "candidacy_close_blocks",
            base,
            5_000,
            |p, v| ConsensusParameters {
                candidacy_close_blocks: v,
                ..p
            },
            true,
        ),
        (
            "candidacy_close_blocks",
            base,
            5_000,
            |p, v| ConsensusParameters {
                candidacy_close_blocks: v,
                ..p
            },
            false,
        ),
        (
            "election_entropy_blocks",
            base,
            500,
            |p, v| ConsensusParameters {
                election_entropy_blocks: v,
                ..p
            },
            true,
        ),
        (
            "election_entropy_blocks",
            base,
            500,
            |p, v| ConsensusParameters {
                election_entropy_blocks: v,
                ..p
            },
            false,
        ),
        (
            "validator_min_set_size",
            base,
            20_000,
            |p, v| ConsensusParameters {
                validator_min_set_size: v,
                ..p
            },
            true,
        ),
        (
            "validator_min_set_size",
            slack,
            14_000,
            |p, v| ConsensusParameters {
                validator_min_set_size: v,
                ..p
            },
            false,
        ),
        (
            "validator_target_set_size",
            slack,
            20_000,
            |p, v| ConsensusParameters {
                validator_target_set_size: v,
                ..p
            },
            true,
        ),
        (
            "validator_target_set_size",
            base,
            20_000,
            |p, v| ConsensusParameters {
                validator_target_set_size: v,
                ..p
            },
            false,
        ),
        (
            "validator_max_set_size",
            base,
            80_000,
            |p, v| ConsensusParameters {
                validator_max_set_size: v,
                ..p
            },
            true,
        ),
        (
            "validator_max_set_size",
            base,
            80_000,
            |p, v| ConsensusParameters {
                validator_max_set_size: v,
                ..p
            },
            false,
        ),
        (
            "validator_churn_cap_seats",
            base,
            100,
            |p, v| ConsensusParameters {
                validator_churn_cap_seats: v,
                ..p
            },
            true,
        ),
        (
            "validator_churn_cap_seats",
            base,
            100,
            |p, v| ConsensusParameters {
                validator_churn_cap_seats: v,
                ..p
            },
            false,
        ),
        (
            "validator_cooldown_epochs",
            base,
            20,
            |p, v| ConsensusParameters {
                validator_cooldown_epochs: v,
                ..p
            },
            true,
        ),
        (
            "validator_cooldown_epochs",
            base,
            20,
            |p, v| ConsensusParameters {
                validator_cooldown_epochs: v,
                ..p
            },
            false,
        ),
        (
            "validator_min_capture_epochs",
            base,
            20,
            |p, v| ConsensusParameters {
                validator_min_capture_epochs: v,
                ..p
            },
            true,
        ),
        (
            "validator_min_capture_epochs",
            base,
            20,
            |p, v| ConsensusParameters {
                validator_min_capture_epochs: v,
                ..p
            },
            false,
        ),
    ];

    let num = bounds.election_parameter_change_numerator;
    let den = bounds.election_parameter_change_denominator;

    for (name, from, old, set, down) in sweep {
        assert_eq!(old % num, 0, "{name}: the sweep base must divide exactly");
        assert_eq!(old % den, 0, "{name}: the sweep base must divide exactly");
        let active = active_of(from);
        let way = if down { "downward" } else { "upward" };
        // `old * 4 / 5` going down, `old * 5 / 4` going up.
        let boundary = if down {
            old / num * den
        } else {
            old / den * num
        };
        let past = if down { boundary - 1 } else { boundary + 1 };

        set(from, boundary)
            .validate(&bounds, 2, 2, Some(&active))
            .unwrap_or_else(|e| panic!("{name} {way}: the boundary must be accepted: {e:?}"));
        assert_eq!(
            set(from, past)
                .validate(&bounds, 2, 2, Some(&active))
                .unwrap_err(),
            Error::Parameter(ParameterError::ChangeRatio { parameter: name }),
            "{name} {way}: one step past the boundary must be refused by the ratio"
        );
    }

    // The tenth parameter, reported rather than forced. The ratio loop runs
    // before the term-limit ratchet, so at the downward boundary the ratio is
    // satisfied and the ratchet is what refuses,
    let active = active_of(base);
    assert_eq!(
        ConsensusParameters {
            validator_max_consecutive_terms: 800, // 1_000 * 4 / 5
            ..base
        }
        .validate(&bounds, 2, 2, Some(&active))
        .unwrap_err(),
        Error::Parameter(ParameterError::TermLimitDecreased)
    );
    // and one step further down the ratio refuses first, naming the parameter.
    assert_eq!(
        ConsensusParameters {
            validator_max_consecutive_terms: 799,
            ..base
        }
        .validate(&bounds, 2, 2, Some(&active))
        .unwrap_err(),
        Error::Parameter(ParameterError::ChangeRatio {
            parameter: "validator_max_consecutive_terms",
        })
    );
    // Upward it behaves like the other nine.
    ConsensusParameters {
        validator_max_consecutive_terms: 1_250,
        ..base
    }
    .validate(&bounds, 2, 2, Some(&active))
    .expect("raising the term limit inside the ratio is permitted");
    assert_eq!(
        ConsensusParameters {
            validator_max_consecutive_terms: 1_251,
            ..base
        }
        .validate(&bounds, 2, 2, Some(&active))
        .unwrap_err(),
        Error::Parameter(ParameterError::ChangeRatio {
            parameter: "validator_max_consecutive_terms",
        })
    );

    // The trap, exhibited instead of assumed away. The descent of
    // `validator_target_set_size` that the sweep proves is caught by the ratio
    // on `slack` is caught here, on `base` where `min_set == V`, by the
    // relational block one layer earlier — and the ratio is never reached.
    assert_eq!(
        ConsensusParameters {
            validator_target_set_size: 15_999,
            ..base
        }
        .validate(&bounds, 2, 2, Some(&active))
        .unwrap_err(),
        Error::Parameter(ParameterError::Constraint {
            rule: "0 < validator_min_set_size <= V <= validator_max_set_size",
        })
    );
}

/// [REVIEW-017] RF-002: the rate-of-change rule binds in **both** directions,
/// on **every** parameter it governs.
///
/// The rule is two inequalities — `new * den <= old * num` bounds the rise and
/// `old * den <= new * num` bounds the fall — and before this test only the
/// first was exercised anywhere: deleting the second left the whole Rust suite
/// and the whole Python oracle green. The missing half is the dangerous one for
/// the two work tariffs and for the eligibility threshold, which `README.md`
/// motivates with "the dangerous direction here is **downward**".
///
/// The cases here are **not transcribed from the published tables**. They are
/// derived from the rule's own structure: a sweep over the parameters the rule
/// governs, each moved to the exact boundary the ratio permits and then one
/// step past it, in each direction. That derivation is deliberately different
/// from the Python oracle's, which *searches* for the flip point instead of
/// computing it — two oracles written from the same table are independent in
/// implementation but not in which cases exist, which is how this hole survived
/// two of them.
///
/// The bounds below are wide on every magnitude on purpose: a rejection here
/// must come from the ratio and never from a floor or a ceiling, which is the
/// defect the published `reward_epoch_ms` row had. Every rejection therefore
/// asserts `ChangeRatio` naming the parameter that moved, not merely
/// `is_err()`.
#[test]
#[allow(clippy::too_many_lines)]
fn the_rate_of_change_binds_in_both_directions_on_every_parameter() {
    // `V` divides by both 4 and 5, so both boundaries are exact integers and no
    // rounding hides a case. The creator-share denominator is larger so that
    // `kn < kd` still holds at every point of its own sweep.
    const V: u64 = 5_000;
    const KD: u64 = 100_000;
    type Setter = fn(RewardPolicy, u64) -> RewardPolicy;

    let bounds = RewardBounds {
        network_id: "fixture".to_owned(),
        chain_id: zero_chain_id(),
        existence_fund_microtokens_per_epoch_max: u64::MAX,
        reward_epoch_ms_min: 1,
        reward_epoch_ms_max: u64::MAX,
        publisher_reward_cap_numerator_max: u64::MAX,
        publisher_reward_cap_denominator_min: 1,
        validator_eligibility_threshold_units_min: 1,
        validator_eligibility_window_epochs_max: u64::MAX,
        validator_eligibility_min_issuers_min: 2,
        storage_units_per_contribution_unit_max: u64::MAX,
        compute_units_per_contribution_unit_max: u64::MAX,
        storage_microtokens_per_byte_epoch_min: 1,
        compute_microtokens_per_million_fuel_min: 1,
        reward_parameter_change_numerator: 5,
        reward_parameter_change_denominator: 4,
        reward_parameter_min_activation_gap_blocks: 1,
    };
    bounds.validate(&zero_chain_id()).unwrap();

    let base = RewardPolicy {
        reward_epoch_ms: V,
        existence_fund_microtokens_per_epoch: V,
        // Pinned at zero by [ADR-010], so the ratio can never bind on it: 0 -> 0
        // satisfies both inequalities, and any other value is rejected by
        // `check_internal` first. Excluded from the sweep for that reason and
        // not by oversight.
        availability_microtokens_per_unit: 0,
        storage_microtokens_per_byte_epoch: V,
        compute_microtokens_per_million_fuel: V,
        publisher_microtokens_per_active_subscriber: V,
        publisher_reward_cap_numerator: V,
        publisher_reward_cap_denominator: KD,
        storage_units_per_contribution_unit: V,
        compute_units_per_contribution_unit: V,
        validator_eligibility_threshold_units: V,
        validator_eligibility_window_epochs: V,
        validator_eligibility_min_issuers: V,
    };
    base.validate(&bounds, 1, 1, None)
        .expect("the sweep base must itself be admissible");

    let active = ActiveRewardPolicyDocument {
        sequence: 1,
        activation_height: 1,
        policy: base,
    };

    let sweep: [(&str, u64, Setter); 12] = [
        ("reward_epoch_ms", V, |p, v| RewardPolicy {
            reward_epoch_ms: v,
            ..p
        }),
        ("existence_fund_microtokens_per_epoch", V, |p, v| {
            RewardPolicy {
                existence_fund_microtokens_per_epoch: v,
                ..p
            }
        }),
        ("storage_microtokens_per_byte_epoch", V, |p, v| {
            RewardPolicy {
                storage_microtokens_per_byte_epoch: v,
                ..p
            }
        }),
        ("compute_microtokens_per_million_fuel", V, |p, v| {
            RewardPolicy {
                compute_microtokens_per_million_fuel: v,
                ..p
            }
        }),
        ("publisher_microtokens_per_active_subscriber", V, |p, v| {
            RewardPolicy {
                publisher_microtokens_per_active_subscriber: v,
                ..p
            }
        }),
        ("publisher_reward_cap_numerator", V, |p, v| RewardPolicy {
            publisher_reward_cap_numerator: v,
            ..p
        }),
        ("publisher_reward_cap_denominator", KD, |p, v| {
            RewardPolicy {
                publisher_reward_cap_denominator: v,
                ..p
            }
        }),
        ("storage_units_per_contribution_unit", V, |p, v| {
            RewardPolicy {
                storage_units_per_contribution_unit: v,
                ..p
            }
        }),
        ("compute_units_per_contribution_unit", V, |p, v| {
            RewardPolicy {
                compute_units_per_contribution_unit: v,
                ..p
            }
        }),
        ("validator_eligibility_threshold_units", V, |p, v| {
            RewardPolicy {
                validator_eligibility_threshold_units: v,
                ..p
            }
        }),
        ("validator_eligibility_window_epochs", V, |p, v| {
            RewardPolicy {
                validator_eligibility_window_epochs: v,
                ..p
            }
        }),
        ("validator_eligibility_min_issuers", V, |p, v| {
            RewardPolicy {
                validator_eligibility_min_issuers: v,
                ..p
            }
        }),
    ];

    let num = bounds.reward_parameter_change_numerator;
    let den = bounds.reward_parameter_change_denominator;
    let accept = |policy: &RewardPolicy| policy.validate(&bounds, 2, 2, Some(&active));

    for (name, old, set) in sweep {
        assert_eq!(old % num, 0, "{name}: the sweep base must divide exactly");
        assert_eq!(old % den, 0, "{name}: the sweep base must divide exactly");
        let highest = old / den * num; // old * 5 / 4
        let lowest = old / num * den; // old * 4 / 5

        // Upward: the boundary is admissible, one step past it is not.
        accept(&set(base, highest))
            .unwrap_or_else(|e| panic!("{name}: the upward boundary must be accepted: {e:?}"));
        assert_eq!(
            accept(&set(base, highest + 1)).unwrap_err(),
            Error::Parameter(ParameterError::ChangeRatio { parameter: name }),
            "{name}: one above the upward boundary must be refused by the ratio"
        );

        // Downward: the half that no case exercised before [REVIEW-017].
        accept(&set(base, lowest))
            .unwrap_or_else(|e| panic!("{name}: the downward boundary must be accepted: {e:?}"));
        assert_eq!(
            accept(&set(base, lowest - 1)).unwrap_err(),
            Error::Parameter(ParameterError::ChangeRatio { parameter: name }),
            "{name}: one below the downward boundary must be refused by the ratio"
        );
    }

    // The concrete attack [REVIEW-017] RF-002 describes: the storage tariff cut
    // to a tenth in one document. `W` collapses, the surveilled ratio
    // `F / (F + W)` climbs toward one without `F` moving a microtoken, and
    // every magnitude floor is respected because the floor sits far below the
    // current value.
    assert_eq!(
        accept(&RewardPolicy {
            storage_microtokens_per_byte_epoch: V / 10,
            ..base
        })
        .unwrap_err(),
        Error::Parameter(ParameterError::ChangeRatio {
            parameter: "storage_microtokens_per_byte_epoch",
        })
    );

    // An unchanged document is inside the ratio, including on the availability
    // tariff the sweep excludes.
    accept(&base).expect("an unchanged document is inside the ratio");
}

/// Checked arithmetic overflow returns `Error::Arithmetic` rather than truncating or panicking.
#[test]
fn the_arithmetic_overflow_rejection_for_economic_rules() {
    let bounds = permissive_reward_bounds();
    let base = reward_policy_pd0();

    // 1. Activation gap checked_add overflow in reward policy
    let max_height_active = ActiveRewardPolicyDocument {
        sequence: 1,
        activation_height: u64::MAX,
        policy: base,
    };
    assert_eq!(
        base.validate(&bounds, u64::MAX, 2, Some(&max_height_active))
            .unwrap_err(),
        Error::Arithmetic("activation gap")
    );

    // 2. Activation gap checked_add overflow in consensus parameters
    let c_bounds = permissive_bounds();
    let c_base = consensus_parameters_pd0();
    let c_max_height_active = ActiveConsensusDocument {
        sequence: 1,
        activation_height: u64::MAX,
        parameters: c_base,
    };
    assert_eq!(
        c_base
            .validate(&c_bounds, u64::MAX, 2, Some(&c_max_height_active))
            .unwrap_err(),
        Error::Arithmetic("activation gap")
    );
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

/// `min_revocation_effective_delay_blocks >= 1`: a delay floor of zero is rejected.
#[test]
fn revocation_delay_floor_of_zero_is_rejected() {
    let parameters = ConsensusParameters {
        min_revocation_effective_delay_blocks: 0,
        ..consensus_parameters_pd0()
    };
    assert_eq!(
        validate(&parameters).unwrap_err(),
        Error::Parameter(ParameterError::Constraint {
            rule: "min_revocation_effective_delay_blocks >= 1"
        })
    );
}

/// `revocation_effective_grace_blocks >= 1`: grace period of zero is rejected.
#[test]
fn revocation_grace_blocks_of_zero_is_rejected() {
    let parameters = ConsensusParameters {
        revocation_effective_grace_blocks: 0,
        ..consensus_parameters_pd0()
    };
    assert_eq!(
        validate(&parameters).unwrap_err(),
        Error::Parameter(ParameterError::Constraint {
            rule: "revocation_effective_grace_blocks >= 1"
        })
    );
}

/// `max_planned_revocation_delay_blocks >= min_revocation_effective_delay_blocks + revocation_effective_grace_blocks`
#[test]
fn max_planned_revocation_delay_below_floor_is_rejected() {
    let parameters = ConsensusParameters {
        min_revocation_effective_delay_blocks: 5,
        revocation_effective_grace_blocks: 5,
        max_planned_revocation_delay_blocks: 9, // < 5 + 5 = 10
        ..consensus_parameters_pd0()
    };
    assert_eq!(
        validate(&parameters).unwrap_err(),
        Error::Parameter(ParameterError::Constraint {
            rule: "max_planned_revocation_delay_blocks >= min_revocation_effective_delay_blocks + revocation_effective_grace_blocks"
        })
    );

    // Exactly equal is accepted.
    let valid_parameters = ConsensusParameters {
        min_revocation_effective_delay_blocks: 5,
        revocation_effective_grace_blocks: 5,
        max_planned_revocation_delay_blocks: 10,
        ..consensus_parameters_pd0()
    };
    assert!(validate(&valid_parameters).is_ok());
}

/// Magnitude bounds on revocation parameters from `ElectionBounds`.
#[test]
fn revocation_delay_magnitude_bounds_are_enforced() {
    let bounds = ElectionBounds {
        min_revocation_effective_delay_blocks_max: 10,
        revocation_effective_grace_blocks_max: 10,
        max_planned_revocation_delay_blocks_max: 50,
        ..permissive_bounds()
    };

    // 1. Exceeding min_revocation_effective_delay_blocks_max
    let p1 = ConsensusParameters {
        min_revocation_effective_delay_blocks: 11,
        revocation_effective_grace_blocks: 5,
        max_planned_revocation_delay_blocks: 20,
        ..consensus_parameters_pd0()
    };
    assert_eq!(
        p1.validate(&bounds, 1, 1, None).unwrap_err(),
        Error::Parameter(ParameterError::Constraint {
            rule: "min_revocation_effective_delay_blocks <= min_revocation_effective_delay_blocks_max"
        })
    );

    // 2. Exceeding revocation_effective_grace_blocks_max
    let p2 = ConsensusParameters {
        min_revocation_effective_delay_blocks: 5,
        revocation_effective_grace_blocks: 11,
        max_planned_revocation_delay_blocks: 20,
        ..consensus_parameters_pd0()
    };
    assert_eq!(
        p2.validate(&bounds, 1, 1, None).unwrap_err(),
        Error::Parameter(ParameterError::Constraint {
            rule: "revocation_effective_grace_blocks <= revocation_effective_grace_blocks_max"
        })
    );

    // 3. Exceeding max_planned_revocation_delay_blocks_max
    let p3 = ConsensusParameters {
        min_revocation_effective_delay_blocks: 5,
        revocation_effective_grace_blocks: 5,
        max_planned_revocation_delay_blocks: 51,
        ..consensus_parameters_pd0()
    };
    assert_eq!(
        p3.validate(&bounds, 1, 1, None).unwrap_err(),
        Error::Parameter(ParameterError::Constraint {
            rule: "max_planned_revocation_delay_blocks <= max_planned_revocation_delay_blocks_max"
        })
    );
}
