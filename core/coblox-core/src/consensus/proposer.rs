//! Who proposes at `(height, round)`.
//!
//! # The rule, from [ADR-018] §3
//!
//! > Round-robin deterministico sul set di validatori attivo, indicizzato da
//! > `(height, round)` e pesato per potere di voto. Ogni nodo lo calcola dallo
//! > stesso `validator_set_hash` e ottiene lo stesso proponente senza scambiare
//! > messaggi.
//!
//! Four properties, and each one is a line of [`proposer_at`]:
//!
//! * **deterministic** — the function's only inputs are the set and the pair,
//!   and it reads no clock, no counter and no local state;
//! * **round-robin over the active set** — the ladder below walks the set in
//!   `validator_id` order, which `ValidatorSet::check_structure` already
//!   guarantees is total and unique, so the walk is canonical;
//! * **indexed by `(height, round)`** — and by nothing else. In particular not
//!   by any value a participant supplies. [ADR-018] gives the reason and names
//!   the precedent: [DEBT-035] established that a revoker can grind its own
//!   transaction ID, so an index a participant could choose would be that same
//!   grinding surface one level up, where the prize is the right to propose;
//! * **weighted by voting power** — the index runs over *power* and not over
//!   seats, so a member with power `w` out of `W` is the proposer for `w` of
//!   every `W` consecutive indices.
//!
//! # Why `(height + round)` and not `height * rounds + round`
//!
//! The index has to satisfy one liveness obligation, and it is the obligation
//! `GATE-LIVENESS-AFTER-SILENCE` exists to check: **consecutive rounds at the
//! same height must not repeat a proposer while an unvisited member remains**.
//! A mute proposer at round `r` has to be replaced at `r+1` by somebody else, or
//! the height waits for a node that has already declined to speak.
//!
//! `(height + round) mod total_power` gives exactly that: at a fixed height,
//! successive rounds step one unit along the power ladder, so they visit every
//! unit of power — and therefore every member — before returning. A product
//! form would be free to skip, and a hash of the pair would visit members in an
//! order that is uniform on average and repeats within a height. Uniformity is
//! not what this index is for: nothing here is secret, the pair is public before
//! the round begins, and there is nothing to hide from a grinder because there
//! is nothing a grinder can move.
//!
//! The arithmetic is `u128` throughout, so `height + round` cannot wrap: at
//! `u64::MAX` for both, a wrapping sum would map two distinct rounds to the same
//! proposer, which is the one thing the paragraph above forbids.

use crate::error::{Error, Result};
use crate::validator_set::{ValidatorEntry, ValidatorSet};

/// The proposer of `(height, round)` under the round-robin rule.
///
/// # Errors
///
/// Returns an error when the set is structurally invalid — empty, unsorted,
/// carrying a zero voting power, or with a power sum that overflows `u64`. Those
/// are [`ValidatorSet::check_structure`]'s rejections, and this function calls it
/// rather than assuming it: a caller holding a set it never checked would
/// otherwise get a proposer computed over a ladder with a hole in it.
pub fn proposer_at(set: &ValidatorSet, height: u64, round: u64) -> Result<&ValidatorEntry> {
    set.check_structure()?;
    let total_power = u128::from(set.total_voting_power()?);
    let index = (u128::from(height) + u128::from(round)) % total_power;
    let mut ladder: u128 = 0;
    for entry in &set.validators {
        ladder += u128::from(entry.voting_power);
        if index < ladder {
            return Ok(entry);
        }
    }
    // Unreachable while `total_voting_power` is the sum of the same powers this
    // loop adds: `index < total_power` and the ladder ends at `total_power`. It
    // is an error and not an `unreachable!`, because the invariant it depends on
    // lives in another module and a panic here would be a panic on a set that
    // arrived over the wire.
    Err(Error::Arithmetic("proposer ladder did not reach the index"))
}

/// Whether `validator_id` is the proposer of `(height, round)`.
pub fn is_proposer(
    set: &ValidatorSet,
    height: u64,
    round: u64,
    validator_id: &str,
) -> Result<bool> {
    Ok(proposer_at(set, height, round)?.validator_id == validator_id)
}
