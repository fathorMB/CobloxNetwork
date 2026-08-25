//! The chain's clocks: the measured cadence, and the reward-epoch derivation.
//!
//! Two rules live here and they answer the same question from opposite sides.
//!
//! **The proposition this module is built on**, established by AGENT-007
//! evaluating [DEBT-013] and written into [ADR-013] part 3: *no validity rule
//! internal to the chain can constrain real time, because every clock the chain
//! carries is written by the validators.* `timestamp_ms` is written by the
//! validators. The median-of-eleven rule imposes monotonicity, not a step. A
//! rule on the distance between consecutive `timestamp_ms` values would oblige
//! validators to **write** close timestamps, not to **produce** close blocks,
//! and is explicitly rejected by [ADR-013]. **It is not reintroduced here by any
//! door: `timestamp_ms` is not an input to any function in this module.**
//!
//! What v0 does have is one clock the validators do not write: the **weak
//! subjectivity checkpoint**, signed by a release key that belongs to no
//! validator, carrying `height` and `issued_at_ms`
//! (`README.md#weak-subjectivity-checkpoint`). Two points on that clock measure
//! the real cadence.
//!
//! **What this buys, in the words that fit it.** Nothing here *prevents* a
//! validator set from slowing block production down: the set still decides how
//! fast it produces blocks, and no rule in this crate changes that. What it
//! does is make the rate **measurable and declared** rather than invisible. For
//! a defect whose severity is entirely in its invisibility that is the part
//! that counts, and "closed" would say more than the code says.
//!
//! **The second rule, and why it belongs in the same module.** `reward_epoch`
//! had no derivation at all ([DEBT-019]): a conforming quorum could increment
//! it at every block and multiply real issuance without violating anything, and
//! the floor `reward_epoch_ms_min` that [SPEC-009] introduced bounds the
//! duration a **signed document declares**, not the speed at which the index
//! advances. [`check_mint_reward_epoch`] derives the index from `height`, which
//! is the one chain quantity a validator cannot write freely: `height` is
//! `previous + 1`, and that is re-checkable by anyone from headers alone,
//! forever. The residue is then exactly the residue of [DEBT-013] — how many
//! real milliseconds a block takes — and that residue is what the first rule
//! measures. The two rules are the same closure applied twice.

use crate::error::{CadenceError, Error, Result};
use crate::hash::ChainId;
use crate::params::CadenceBand;

/// The outcome of a cadence measurement.
///
/// The two out-of-band variants are named for the **rate of block production**,
/// not for the numeric value of the ratio: `FasterThanBand` means blocks are
/// arriving more often than `min_ms_per_block` allows, which is the numerically
/// *smaller* millisecond-per-block figure. The naming follows the quantity the
/// reader cares about rather than the arithmetic, because a reader who has to
/// remember which way the ratio points will eventually remember wrong.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CadenceVerdict {
    /// The observed rate lies inside the genesis band.
    WithinBand {
        /// Blocks produced across the measured interval.
        blocks: u64,
        /// Real milliseconds the interval spans.
        elapsed_ms: u64,
        /// `elapsed_ms / blocks`, truncated. A diagnostic, never the comparison:
        /// the band comparison is exact and does not divide.
        observed_ms_per_block: u64,
    },
    /// Blocks are being produced **faster** than `min_ms_per_block` permits.
    FasterThanBand {
        /// Blocks produced across the measured interval.
        blocks: u64,
        /// Real milliseconds the interval spans.
        elapsed_ms: u64,
        /// `elapsed_ms / blocks`, truncated.
        observed_ms_per_block: u64,
    },
    /// Blocks are being produced **more slowly** than `max_ms_per_block`
    /// permits.
    SlowerThanBand {
        /// Blocks produced across the measured interval.
        blocks: u64,
        /// Real milliseconds the interval spans.
        elapsed_ms: u64,
        /// `elapsed_ms / blocks`, truncated.
        observed_ms_per_block: u64,
    },
    /// The interval carried fewer than `min_measured_blocks` blocks, so the
    /// measurement is not made.
    ///
    /// This is not a pass. A ratio over three blocks is noise, and a guard that
    /// reports a verdict on noise teaches its caller to disbelieve it.
    Inconclusive {
        /// Blocks produced across the measured interval.
        blocks: u64,
        /// The genesis minimum this measurement did not reach.
        min_measured_blocks: u64,
    },
}

impl CadenceVerdict {
    /// Whether the measurement produced an in-band verdict.
    #[must_use]
    pub const fn is_within_band(&self) -> bool {
        matches!(self, Self::WithinBand { .. })
    }

    /// Whether the measurement was made at all.
    #[must_use]
    pub const fn is_conclusive(&self) -> bool {
        !matches!(self, Self::Inconclusive { .. })
    }
}

/// The core measurement, shared by both entry points.
///
/// Deliberately private: every caller must go through one of the two named
/// functions, because *which two clocks were used* is the whole of the
/// argument and a general-purpose `(blocks, ms)` entry point would let a future
/// caller feed it `timestamp_ms`.
fn measure(blocks: u64, elapsed_ms: u64, band: &CadenceBand) -> CadenceVerdict {
    if blocks < band.min_measured_blocks {
        return CadenceVerdict::Inconclusive {
            blocks,
            min_measured_blocks: band.min_measured_blocks,
        };
    }
    // The comparison is exact in `u128`: dividing first would let a chain sit
    // just outside the band and be reported inside it by the truncation. The
    // truncated figure is carried only as a diagnostic.
    let elapsed = u128::from(elapsed_ms);
    let observed_ms_per_block = elapsed_ms / blocks;
    if elapsed < u128::from(blocks) * u128::from(band.min_ms_per_block) {
        return CadenceVerdict::FasterThanBand {
            blocks,
            elapsed_ms,
            observed_ms_per_block,
        };
    }
    if elapsed > u128::from(blocks) * u128::from(band.max_ms_per_block) {
        return CadenceVerdict::SlowerThanBand {
            blocks,
            elapsed_ms,
            observed_ms_per_block,
        };
    }
    CadenceVerdict::WithinBand {
        blocks,
        elapsed_ms,
        observed_ms_per_block,
    }
}

/// The measurement a **light client** runs, from the checkpoint it already
/// holds and the header it has authenticated.
///
/// The interval runs from `checkpoint_issued_at_ms` — signed by the release
/// key, which belongs to no validator — to `now_ms`, the client's own clock,
/// which is the same clock step 1 of the light-client algorithm already uses
/// for checkpoint freshness. The block count runs from the checkpoint's
/// `height` to the height of the header the client is treating as the finalized
/// tip.
///
/// **Neither endpoint is a chain clock**, and that is the point of the
/// construction rather than a detail of it. `timestamp_ms` is not a parameter
/// of this function and must not become one.
///
/// The band is validated against `chain_id` as this function's first act, for
/// the reason the crate documentation gives for the two bounds objects: a
/// degenerate trust anchor does not fail, it silently disables the rule it
/// carries. A `min_measured_blocks` of zero would pronounce on a single block
/// and a `min_ms_per_block` of zero would admit any rate at all, and neither
/// would look like an error at the call site.
///
/// **Declared limit, and it is the reason [`check_cadence_light_client`] is
/// asymmetric.** A client that has not caught up to the tip counts fewer blocks
/// than the chain produced, so its measurement is biased *downwards* and only
/// downwards. A `SlowerThanBand` verdict from this vantage point is therefore
/// not soundly attributable to the chain; a `FasterThanBand` verdict is,
/// because sync lag cannot manufacture blocks.
pub fn measure_cadence_from_checkpoint(
    chain_id: &ChainId,
    checkpoint_height: u64,
    checkpoint_issued_at_ms: u64,
    tip_height: u64,
    now_ms: u64,
    band: &CadenceBand,
) -> Result<CadenceVerdict> {
    band.validate(chain_id)?;
    let blocks = tip_height
        .checked_sub(checkpoint_height)
        .ok_or(CadenceError::HeightRegression)?;
    let elapsed_ms = now_ms
        .checked_sub(checkpoint_issued_at_ms)
        .ok_or(CadenceError::ClockRegression)?;
    Ok(measure(blocks, elapsed_ms, band))
}

/// The measurement the **checkpoint release procedure** runs, over two
/// consecutive checkpoints of the same chain.
///
/// Both endpoints are signed by the release key, so this form has no sync lag
/// and no client clock in it. It is the form entitled to fail closed in both
/// directions, and [`check_cadence_release`] is that wrapper.
pub fn measure_cadence_between_checkpoints(
    chain_id: &ChainId,
    earlier_height: u64,
    earlier_issued_at_ms: u64,
    later_height: u64,
    later_issued_at_ms: u64,
    band: &CadenceBand,
) -> Result<CadenceVerdict> {
    band.validate(chain_id)?;
    let blocks = later_height
        .checked_sub(earlier_height)
        .ok_or(CadenceError::HeightRegression)?;
    let elapsed_ms = later_issued_at_ms
        .checked_sub(earlier_issued_at_ms)
        .ok_or(CadenceError::ClockRegression)?;
    Ok(measure(blocks, elapsed_ms, band))
}

/// The light client's normative behaviour on a measurement.
///
/// Fails closed on `FasterThanBand` and returns the verdict otherwise, so a
/// `SlowerThanBand` reading reaches the caller as a **reported observation**
/// rather than a rejection. The asymmetry is argued at
/// [`measure_cadence_from_checkpoint`]: from a client's vantage point one
/// direction is soundly attributable to the chain and the other is not, and
/// rejecting on a reading that the client's own sync lag produces would be a
/// guard that cries wolf — which [ADR-012] precision 3 records as the way a
/// guard stops being run at all.
pub fn check_cadence_light_client(verdict: CadenceVerdict) -> Result<CadenceVerdict> {
    match verdict {
        CadenceVerdict::FasterThanBand {
            blocks,
            elapsed_ms,
            observed_ms_per_block,
        } => Err(CadenceError::FasterThanBand {
            blocks,
            elapsed_ms,
            observed_ms_per_block,
        }
        .into()),
        other => Ok(other),
    }
}

/// The release procedure's normative behaviour on a measurement.
///
/// Fails closed in **both** directions: a process that publishes checkpoints
/// for a chain running outside its declared band would be signing the external
/// clock onto a chain the band says is not running, which is the one thing the
/// external clock exists not to do. `Inconclusive` also fails closed here,
/// because a release process that cannot measure has the option of waiting and
/// a light client does not.
pub fn check_cadence_release(verdict: CadenceVerdict) -> Result<CadenceVerdict> {
    match verdict {
        CadenceVerdict::WithinBand { .. } => Ok(verdict),
        CadenceVerdict::FasterThanBand {
            blocks,
            elapsed_ms,
            observed_ms_per_block,
        } => Err(CadenceError::FasterThanBand {
            blocks,
            elapsed_ms,
            observed_ms_per_block,
        }
        .into()),
        CadenceVerdict::SlowerThanBand {
            blocks,
            elapsed_ms,
            observed_ms_per_block,
        } => Err(CadenceError::SlowerThanBand {
            blocks,
            elapsed_ms,
            observed_ms_per_block,
        }
        .into()),
        CadenceVerdict::Inconclusive {
            blocks,
            min_measured_blocks,
        } => Err(CadenceError::Inconclusive {
            blocks,
            min_measured_blocks,
        }
        .into()),
    }
}

/// `reward_epoch_blocks`: how many blocks one reward-epoch index spans.
///
/// `ceil(reward_epoch_ms / block_interval_ms)`, where `reward_epoch_ms` comes
/// from the `reward_policy` document a mint names through its own `policy_hash`
/// and `block_interval_ms` is the genesis constant of
/// `README.md#genesis-constants`.
///
/// The ceiling rather than the floor, and it is not a rounding preference: the
/// quantity is a **floor under how much chain must pass before an epoch may be
/// settled**, so rounding it down would widen the permission. Rounding a bound
/// in the direction that loosens it is how a limit becomes decorative.
pub fn reward_epoch_blocks(reward_epoch_ms: u64, block_interval_ms: u64) -> Result<u64> {
    if block_interval_ms == 0 {
        return Err(CadenceError::DegenerateInterval.into());
    }
    if reward_epoch_ms == 0 {
        return Err(CadenceError::DegenerateEpoch.into());
    }
    Ok(reward_epoch_ms.div_ceil(block_interval_ms))
}

/// The settlement floor on `reward_epoch`: the derivation [DEBT-019] asked for.
///
/// A `mint` naming `reward_epoch` `e` is valid only in a finalized block at
/// height `h` with
///
/// ```text
/// (e + 1) * reward_epoch_blocks  <=  h
/// ```
///
/// **What this bounds, stated as narrowly as it is true.** Cumulative existence
/// emission through height `h` is at most `floor(h / reward_epoch_blocks) * F`,
/// because epoch `e` is unmintable before its floor and at most `F` is mintable
/// per epoch (`ledger.md#existence-income-is-a-share-of-a-capped-fund`). That
/// is a bound **per block**, not per real millisecond. It is not a real-time
/// bound and must not be described as one: how many real milliseconds a block
/// takes is [DEBT-013]'s residue, and it is measured by
/// [`measure_cadence_from_checkpoint`], not constrained by anything here.
///
/// **The consequence that is new, and that the debt did not anticipate.**
/// Before this rule the direction of danger for block production was slowdown
/// only, and acceleration merely shortened everything denominated in blocks.
/// Once the emission index is paced by height, acceleration multiplies real
/// issuance. The cadence band is therefore **two-sided**, and its fast side is
/// not symmetry for its own sake: it is the side that carries this rule's
/// real-time meaning.
pub fn check_mint_reward_epoch(
    reward_epoch: u64,
    height: u64,
    reward_epoch_ms: u64,
    block_interval_ms: u64,
) -> Result<()> {
    let blocks = reward_epoch_blocks(reward_epoch_ms, block_interval_ms)?;
    let floor = u128::from(reward_epoch)
        .checked_add(1)
        .and_then(|epochs| epochs.checked_mul(u128::from(blocks)))
        .ok_or(Error::Arithmetic(
            "reward epoch settlement floor overflowed",
        ))?;
    if floor > u128::from(height) {
        return Err(CadenceError::RewardEpochAhead {
            reward_epoch,
            height,
        }
        .into());
    }
    Ok(())
}

/// The highest `reward_epoch` the settlement floor already permits at `height`.
///
/// `floor(height / reward_epoch_blocks) - 1`, and `None` when no epoch is
/// settleable yet.
pub fn settleable_reward_epoch(
    height: u64,
    reward_epoch_ms: u64,
    block_interval_ms: u64,
) -> Result<Option<u64>> {
    let blocks = reward_epoch_blocks(reward_epoch_ms, block_interval_ms)?;
    Ok((height / blocks).checked_sub(1))
}

/// The observable for the **opposite** direction: an index that does not
/// advance.
///
/// A quorum that simply stops minting freezes existence income without breaking
/// any rule, and no validity rule internal to the chain can compel a quorum to
/// act — which is the same proposition as [ADR-013] part 3, met from the other
/// side. The closure therefore has the same shape as the cadence measure: the
/// lag is made **computable** rather than prevented.
///
/// Returns how many epoch indices the chain is behind: the number of epochs
/// whose settlement floor has passed and which have not been settled. Zero
/// means the chain is current.
pub fn reward_epoch_lag(
    highest_settled: Option<u64>,
    height: u64,
    reward_epoch_ms: u64,
    block_interval_ms: u64,
) -> Result<u64> {
    let settleable = settleable_reward_epoch(height, reward_epoch_ms, block_interval_ms)?;
    Ok(match (settleable, highest_settled) {
        (None, _) => 0,
        (Some(available), None) => available.saturating_add(1),
        (Some(available), Some(settled)) => available.saturating_sub(settled),
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::hash::ChainId;

    /// Test inputs, not launch values. The genesis band is an operator decision
    /// listed in `README.md#draft-governance-selected-launch-parameters`; these
    /// numbers exist to exercise the arithmetic and are chosen wide enough that
    /// no reader mistakes them for a recommendation.
    fn band() -> CadenceBand {
        CadenceBand {
            network_id: "coblox-testnet-cadence".to_owned(),
            chain_id: ChainId::from_digest(crate::hash::Digest32::repeated(0x11)),
            block_interval_ms: 5_000,
            min_ms_per_block: 2_500,
            max_ms_per_block: 10_000,
            min_measured_blocks: 100,
        }
    }

    fn chain() -> ChainId {
        ChainId::from_digest(crate::hash::Digest32::repeated(0x11))
    }

    #[test]
    fn a_chain_inside_the_band_is_within_band() {
        let verdict =
            measure_cadence_from_checkpoint(&chain(), 1_000, 0, 2_000, 5_000_000, &band()).unwrap();
        assert!(verdict.is_within_band(), "{verdict:?}");
        assert!(check_cadence_light_client(verdict).is_ok());
        assert!(check_cadence_release(verdict).is_ok());
    }

    #[test]
    fn a_chain_outside_the_band_is_reported_on_both_sides() {
        let fast =
            measure_cadence_from_checkpoint(&chain(), 1_000, 0, 2_000, 1_000_000, &band()).unwrap();
        assert!(
            matches!(fast, CadenceVerdict::FasterThanBand { .. }),
            "{fast:?}"
        );
        let slow = measure_cadence_from_checkpoint(&chain(), 1_000, 0, 2_000, 40_000_000, &band())
            .unwrap();
        assert!(
            matches!(slow, CadenceVerdict::SlowerThanBand { .. }),
            "{slow:?}"
        );
    }

    #[test]
    fn too_short_an_interval_is_inconclusive_and_never_a_pass() {
        let verdict =
            measure_cadence_from_checkpoint(&chain(), 1_000, 0, 1_010, 50_000, &band()).unwrap();
        assert!(!verdict.is_conclusive());
        assert!(!verdict.is_within_band());
        assert!(check_cadence_release(verdict).is_err());
    }

    #[test]
    fn a_regressed_clock_or_height_is_an_error_not_a_verdict() {
        assert!(
            measure_cadence_from_checkpoint(&chain(), 2_000, 0, 1_000, 5_000, &band()).is_err()
        );
        assert!(
            measure_cadence_from_checkpoint(&chain(), 1_000, 9_000, 2_000, 5_000, &band()).is_err()
        );
    }

    #[test]
    fn the_band_comparison_does_not_divide_first() {
        // 100 blocks in 249 999 ms is 2 499.99 ms per block: outside the band,
        // but `249_999 / 100 == 2_499` only by truncation and a dividing
        // implementation that compared `2_500 <= 2_499` would agree by accident
        // while disagreeing on the neighbouring case below.
        let b = band();
        assert!(matches!(
            measure(100, 249_999, &b),
            CadenceVerdict::FasterThanBand { .. }
        ));
        assert!(matches!(
            measure(100, 250_000, &b),
            CadenceVerdict::WithinBand { .. }
        ));
        // 100 blocks in 1 000 099 ms truncates to 10 000 ms per block, which a
        // dividing implementation would call in-band; it is not.
        assert!(matches!(
            measure(100, 1_000_099, &b),
            CadenceVerdict::SlowerThanBand { .. }
        ));
    }

    #[test]
    fn reward_epoch_blocks_rounds_towards_the_tighter_bound() {
        assert_eq!(reward_epoch_blocks(86_400_000, 5_000).unwrap(), 17_280);
        // 5 001 ms of epoch is more than one block of chain, not exactly one.
        assert_eq!(reward_epoch_blocks(5_001, 5_000).unwrap(), 2);
        assert!(reward_epoch_blocks(0, 5_000).is_err());
        assert!(reward_epoch_blocks(86_400_000, 0).is_err());
    }

    #[test]
    fn the_settlement_floor_rejects_an_index_that_ran_ahead() {
        // Epoch 17 needs height 18 * 17 280 = 311 040.
        assert!(check_mint_reward_epoch(17, 311_040, 86_400_000, 5_000).is_ok());
        assert!(check_mint_reward_epoch(17, 311_039, 86_400_000, 5_000).is_err());
        // The [DEBT-019] manoeuvre in full: one index per block.
        assert!(check_mint_reward_epoch(42, 42, 86_400_000, 5_000).is_err());
    }

    #[test]
    fn the_lag_names_an_index_that_is_not_advancing() {
        // Height 3 456 000 is 200 epochs of chain, so epochs 0..=198 are
        // settleable.
        assert_eq!(
            settleable_reward_epoch(3_456_000, 86_400_000, 5_000).unwrap(),
            Some(199)
        );
        assert_eq!(
            reward_epoch_lag(Some(199), 3_456_000, 86_400_000, 5_000).unwrap(),
            0
        );
        assert_eq!(
            reward_epoch_lag(Some(3), 3_456_000, 86_400_000, 5_000).unwrap(),
            196
        );
        assert_eq!(
            reward_epoch_lag(None, 3_456_000, 86_400_000, 5_000).unwrap(),
            200
        );
        // Before the first floor there is nothing to be behind on.
        assert_eq!(
            reward_epoch_lag(None, 17_279, 86_400_000, 5_000).unwrap(),
            0
        );
    }
}
