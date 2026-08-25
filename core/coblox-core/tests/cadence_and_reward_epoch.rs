//! The two clocks, exercised as guards rather than as calculations.
//!
//! A measure that has never been seen to fire is a calculation, not a guard.
//! Every rule this file covers is therefore tested from the failing side first:
//! the chain that is out of band, the mint whose index ran ahead, the trust
//! anchor that would have disabled the measurement it carries.
//!
//! The last test is of a different kind and is the reason this file exists in
//! the tests directory rather than only in the module. [ADR-013] rejects a
//! validity rule on the distance between consecutive `timestamp_ms` values —
//! not as an oversight, but because `timestamp_ms` is written by the same
//! validators whose cadence it would purport to constrain. That rejection is
//! prose today and prose is not a guard, so
//! [`the_cadence_module_never_reads_a_chain_written_clock`] makes it one: it
//! reads the module's own source and fails if `timestamp_ms` ever becomes an
//! input to it.

use coblox_core::cadence::{
    CadenceVerdict, check_cadence_light_client, check_cadence_release, check_mint_reward_epoch,
    measure_cadence_between_checkpoints, measure_cadence_from_checkpoint, reward_epoch_blocks,
    reward_epoch_lag, settleable_reward_epoch,
};
use coblox_core::error::{CadenceError, Error, ParameterError};
use coblox_core::hash::{ChainId, Digest32};
use coblox_core::params::CadenceBand;

/// Test inputs, not launch values.
///
/// The genesis band is an operator decision listed in
/// `README.md#draft-governance-selected-launch-parameters`. These numbers exist
/// to exercise the arithmetic. The declared interval is the one value that is
/// real — 5 000 ms, `README.md#genesis-constants` — because the relational rule
/// of the band is stated against it.
fn band() -> CadenceBand {
    CadenceBand {
        network_id: "coblox-testnet-cadence".to_owned(),
        chain_id: ChainId::from_digest(Digest32::repeated(0x11)),
        block_interval_ms: 5_000,
        min_ms_per_block: 2_500,
        max_ms_per_block: 10_000,
        min_measured_blocks: 100,
    }
}

fn chain() -> ChainId {
    ChainId::from_digest(Digest32::repeated(0x11))
}

// --- the measurement --------------------------------------------------------

#[test]
fn a_chain_running_at_the_declared_interval_is_inside_its_band() {
    // 1 000 blocks in 5 000 000 ms is exactly 5 000 ms per block.
    let verdict =
        measure_cadence_from_checkpoint(&chain(), 4_000, 1_000_000, 5_000, 6_000_000, &band())
            .unwrap();
    assert!(
        matches!(verdict, CadenceVerdict::WithinBand { .. }),
        "{verdict:?}"
    );
    assert!(check_cadence_light_client(verdict).is_ok());
    assert!(check_cadence_release(verdict).is_ok());
}

#[test]
fn a_chain_faster_than_its_band_fails_closed_for_both_parties() {
    // 1 000 blocks in 1 000 000 ms is 1 000 ms per block, four times the
    // declared interval. Under the settlement floor of `reward_epoch` that is
    // four times the intended real issuance rate.
    let verdict =
        measure_cadence_from_checkpoint(&chain(), 4_000, 1_000_000, 5_000, 2_000_000, &band())
            .unwrap();
    assert!(
        matches!(
            verdict,
            CadenceVerdict::FasterThanBand {
                observed_ms_per_block: 1_000,
                ..
            }
        ),
        "{verdict:?}"
    );
    assert!(matches!(
        check_cadence_light_client(verdict),
        Err(Error::Cadence(CadenceError::FasterThanBand { .. }))
    ));
    assert!(matches!(
        check_cadence_release(verdict),
        Err(Error::Cadence(CadenceError::FasterThanBand { .. }))
    ));
}

#[test]
fn a_chain_slower_than_its_band_is_reported_to_a_client_and_refused_by_the_release() {
    // 1 000 blocks in 40 000 000 ms is 40 000 ms per block: eight times the
    // declared interval, so every term and every revocation delay lasts eight
    // times as long in real time as the tuning of [SPEC-007] intends.
    let verdict =
        measure_cadence_from_checkpoint(&chain(), 4_000, 1_000_000, 5_000, 41_000_000, &band())
            .unwrap();
    assert!(
        matches!(
            verdict,
            CadenceVerdict::SlowerThanBand {
                observed_ms_per_block: 40_000,
                ..
            }
        ),
        "{verdict:?}"
    );

    // The asymmetry, and it is the substance of the step rather than a detail:
    // a client that has not caught up counts fewer blocks than the chain
    // produced, so this reading is not soundly attributable to the chain from
    // its vantage point. It is reported, not rejected.
    let reported = check_cadence_light_client(verdict).unwrap();
    assert!(matches!(reported, CadenceVerdict::SlowerThanBand { .. }));

    // The release process has two of its own signed checkpoints and no sync
    // lag, so it is the party entitled to fail closed on this side.
    assert!(matches!(
        check_cadence_release(verdict),
        Err(Error::Cadence(CadenceError::SlowerThanBand { .. }))
    ));
}

#[test]
fn the_release_measure_uses_two_external_points_and_agrees_with_the_client_measure() {
    let b = band();
    let between =
        measure_cadence_between_checkpoints(&chain(), 4_000, 1_000_000, 5_000, 41_000_000, &b)
            .unwrap();
    let from =
        measure_cadence_from_checkpoint(&chain(), 4_000, 1_000_000, 5_000, 41_000_000, &b).unwrap();
    assert_eq!(between, from);
    assert!(matches!(
        check_cadence_release(between),
        Err(Error::Cadence(CadenceError::SlowerThanBand { .. }))
    ));
}

#[test]
fn an_interval_too_short_to_measure_is_not_a_pass() {
    let verdict =
        measure_cadence_from_checkpoint(&chain(), 4_000, 1_000_000, 4_050, 1_250_000, &band())
            .unwrap();
    assert!(matches!(
        verdict,
        CadenceVerdict::Inconclusive {
            blocks: 50,
            min_measured_blocks: 100
        }
    ));
    assert!(!verdict.is_within_band());
    // The client may proceed on an unmeasurable interval; the release process,
    // which can wait, may not.
    assert!(check_cadence_light_client(verdict).is_ok());
    assert!(matches!(
        check_cadence_release(verdict),
        Err(Error::Cadence(CadenceError::Inconclusive { .. }))
    ));
}

#[test]
fn a_client_whose_clock_or_tip_regresses_gets_an_error_and_not_a_verdict() {
    let b = band();
    assert!(matches!(
        measure_cadence_from_checkpoint(&chain(), 5_000, 1_000_000, 4_000, 2_000_000, &b),
        Err(Error::Cadence(CadenceError::HeightRegression))
    ));
    assert!(matches!(
        measure_cadence_from_checkpoint(&chain(), 4_000, 3_000_000, 5_000, 2_000_000, &b),
        Err(Error::Cadence(CadenceError::ClockRegression))
    ));
}

// --- the trust anchor -------------------------------------------------------

#[test]
fn a_band_that_would_disable_its_own_measurement_is_rejected() {
    let good = band();
    good.validate(&chain()).unwrap();

    let cases: [(&str, CadenceBand); 5] = [
        (
            "another chain's band",
            CadenceBand {
                chain_id: ChainId::from_digest(Digest32::repeated(0x22)),
                ..band()
            },
        ),
        (
            "a zero declared interval",
            CadenceBand {
                block_interval_ms: 0,
                ..band()
            },
        ),
        (
            "a zero fast side, which admits any rate at all",
            CadenceBand {
                min_ms_per_block: 0,
                ..band()
            },
        ),
        (
            "an inverted band",
            CadenceBand {
                min_ms_per_block: 20_000,
                max_ms_per_block: 10_000,
                ..band()
            },
        ),
        (
            "a zero noise floor, which pronounces on a single block",
            CadenceBand {
                min_measured_blocks: 0,
                ..band()
            },
        ),
    ];
    for (label, candidate) in cases {
        assert!(
            matches!(
                candidate.validate(&chain()),
                Err(Error::Parameter(
                    ParameterError::Bounds { .. } | ParameterError::ChainIdMismatch
                ))
            ),
            "{label} was accepted"
        );
    }

    // The relational rule: a band that excludes the interval the protocol
    // declares would put every conformant chain permanently out of band.
    let excludes_the_declared_interval = CadenceBand {
        min_ms_per_block: 6_000,
        max_ms_per_block: 10_000,
        ..band()
    };
    assert!(excludes_the_declared_interval.validate(&chain()).is_err());
}

#[test]
fn a_degenerate_band_is_refused_by_the_measurement_and_not_only_by_validate() {
    // [REVIEW-017] RF-001 on the reward side: `RewardBounds::validate` existed
    // and was called from nowhere, so a distribution carrying a degenerate
    // anchor disabled the rule it was supposed to carry *without an error*.
    // The same mistake is available here and is closed the same way — the
    // measurement validates the anchor as its first act, so a caller cannot
    // reach the arithmetic with a band that admits everything.
    let admits_every_rate = CadenceBand {
        min_ms_per_block: 0,
        ..band()
    };
    assert!(matches!(
        measure_cadence_from_checkpoint(
            &chain(),
            4_000,
            1_000_000,
            5_000,
            2_000_000,
            &admits_every_rate
        ),
        Err(Error::Parameter(ParameterError::Bounds { .. }))
    ));

    let pronounces_on_one_block = CadenceBand {
        min_measured_blocks: 0,
        ..band()
    };
    assert!(
        measure_cadence_between_checkpoints(
            &chain(),
            4_000,
            1_000_000,
            5_000,
            2_000_000,
            &pronounces_on_one_block
        )
        .is_err()
    );

    // And a band belonging to another chain never measures this one.
    let other_chain = CadenceBand {
        chain_id: ChainId::from_digest(Digest32::repeated(0x22)),
        ..band()
    };
    assert!(matches!(
        measure_cadence_from_checkpoint(&chain(), 4_000, 1_000_000, 5_000, 6_000_000, &other_chain),
        Err(Error::Parameter(ParameterError::ChainIdMismatch))
    ));
}

// --- the reward-epoch derivation -------------------------------------------

#[test]
fn an_index_that_advances_too_fast_is_invalid() {
    // A one-day epoch is 17 280 blocks of five seconds.
    assert_eq!(reward_epoch_blocks(86_400_000, 5_000).unwrap(), 17_280);

    // The [DEBT-019] manoeuvre in full: a conforming quorum incrementing
    // `reward_epoch` once per block. Every such mint is now invalid.
    for height in [1_u64, 2, 41, 42, 17_279] {
        assert!(
            matches!(
                check_mint_reward_epoch(height, height, 86_400_000, 5_000),
                Err(Error::Cadence(CadenceError::RewardEpochAhead { .. }))
            ),
            "an index of {height} at height {height} was accepted"
        );
    }

    // The boundary, from both sides: epoch 17 needs height 18 * 17 280.
    assert!(check_mint_reward_epoch(17, 311_039, 86_400_000, 5_000).is_err());
    check_mint_reward_epoch(17, 311_040, 86_400_000, 5_000).unwrap();
    // And settling late is permitted, because no rule can say how late is late.
    check_mint_reward_epoch(17, 9_999_999, 86_400_000, 5_000).unwrap();
}

#[test]
fn shortening_the_declared_epoch_shortens_the_floor_and_the_reward_bounds_floor_holds_it() {
    // The point [REVIEW-014] made one level up, now one level down: the
    // settlement floor is denominated in `reward_epoch_ms`, so the genesis
    // `reward_epoch_ms_min` is what stops the floor from collapsing.
    assert_eq!(reward_epoch_blocks(86_400_000, 5_000).unwrap(), 17_280);
    assert_eq!(reward_epoch_blocks(86_400, 5_000).unwrap(), 18);
    // At a thousandth of the declared epoch, epoch 17 becomes settleable a
    // thousand times earlier — which is exactly the multiplication of real
    // issuance that `reward_epoch_ms_min` exists to bound.
    check_mint_reward_epoch(17, 324, 86_400, 5_000).unwrap();
    assert!(check_mint_reward_epoch(17, 323, 86_400, 5_000).is_err());
}

#[test]
fn an_index_that_does_not_advance_is_computable_and_not_rejectable() {
    // Height 3 456 000 is 200 epochs of chain: epochs 0 through 199 have passed
    // their floor.
    assert_eq!(
        settleable_reward_epoch(3_456_000, 86_400_000, 5_000).unwrap(),
        Some(199)
    );

    // A chain that is current has no lag.
    assert_eq!(
        reward_epoch_lag(Some(199), 3_456_000, 86_400_000, 5_000).unwrap(),
        0
    );

    // A quorum that stopped minting after epoch 3 breaks no rule — nothing in
    // the protocol can compel it to mint, which is the same proposition as
    // [ADR-013] part 3 met from the other side. What the derivation buys is
    // that the freeze is a number anyone can recompute from headers.
    assert_eq!(
        reward_epoch_lag(Some(3), 3_456_000, 86_400_000, 5_000).unwrap(),
        196
    );

    // A quorum that never minted at all.
    assert_eq!(
        reward_epoch_lag(None, 3_456_000, 86_400_000, 5_000).unwrap(),
        200
    );

    // And before the first floor there is nothing to be behind on, so the
    // observable does not raise an alarm on a chain that has just started.
    assert_eq!(
        reward_epoch_lag(None, 17_279, 86_400_000, 5_000).unwrap(),
        0
    );
}

#[test]
fn a_degenerate_denominator_is_an_error_and_never_a_silent_zero() {
    assert!(matches!(
        reward_epoch_blocks(86_400_000, 0),
        Err(Error::Cadence(CadenceError::DegenerateInterval))
    ));
    assert!(matches!(
        reward_epoch_blocks(0, 5_000),
        Err(Error::Cadence(CadenceError::DegenerateEpoch))
    ));
}

// --- the rejected remedy ----------------------------------------------------

/// The guard that keeps [ADR-013]'s rejection from decaying into prose.
///
/// A rule on the distance between consecutive `timestamp_ms` values is the
/// remedy that looks obvious and does not work: `timestamp_ms` is written by
/// the same validators whose production rate it would purport to bound, so such
/// a rule obliges them to *write* a cadence rather than to *produce* one. Both
/// measurements in `cadence` therefore take their endpoints from outside the
/// chain, and this test fails if a future change makes the module read the
/// chain's own clock — including through the light-client entry point, which is
/// where it would be easiest to reach for a `BlockHeader` field that is already
/// in the caller's hand.
#[test]
fn the_cadence_module_never_reads_a_chain_written_clock() {
    let source = include_str!("../src/cadence.rs");
    let code: String = source
        .lines()
        .filter(|line| !line.trim_start().starts_with("//"))
        .collect::<Vec<_>>()
        .join("\n");
    assert!(
        !code.contains("timestamp_ms"),
        "`timestamp_ms` appeared in executable code of src/cadence.rs. It is \
         written by the validators whose cadence this module measures; using it \
         would be [ADR-013]'s rejected rule reintroduced through the \
         measurement instead of through a validity rule."
    );
    // The prohibition is only meaningful if the module is still the one that
    // measures, so pin the two entry points by name as well.
    assert!(code.contains("pub fn measure_cadence_from_checkpoint"));
    assert!(code.contains("pub fn measure_cadence_between_checkpoints"));
}
