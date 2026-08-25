//! The single quorum predicate.
//!
//! `ledger.md#quorum-predicate`: "Every v0 quorum (block finality, transaction
//! authorization, enrollment certificate, validator-set document, and protocol
//! document) uses exactly `signed_power * 3 > total_power * 2`."
//!
//! There is deliberately one function here and no variants. The predicate is
//! strict, is not a rounded fraction, and is not a validator count; a second
//! spelling of it somewhere in a codebase is how those three become true by
//! accident.

use crate::error::{Error, Result};

/// `quorum(signed_power, total_power) := signed_power * 3 > total_power * 2`.
///
/// Both multiplications use checked `u128`. A zero total power rejects: there
/// is no quorum over an empty set, and `0 > 0` would otherwise silently be
/// false in a way that reads like a policy decision rather than a rule.
pub fn quorum(signed_power: u64, total_power: u64) -> Result<bool> {
    if total_power == 0 {
        return Err(Error::Arithmetic("quorum over zero total power"));
    }
    let signed = u128::from(signed_power)
        .checked_mul(3)
        .ok_or(Error::Arithmetic("quorum signed_power * 3"))?;
    let total = u128::from(total_power)
        .checked_mul(2)
        .ok_or(Error::Arithmetic("quorum total_power * 2"))?;
    Ok(signed > total)
}

/// The same strict ratio applied to seat counts rather than to voting power.
///
/// `ledger.md`'s contraction floor is `3 * member_count(new) > 2 *
/// member_count(old)`, and the specification says the shape is deliberate: "the
/// same strict `signed * 3 > total * 2` used for every quorum in v0, applied to
/// seats instead of power, so a reviewer reading it recognizes the arithmetic
/// and its boundary cases without new fixtures of its own." It therefore calls
/// the same function.
pub fn contraction_floor(new_member_count: u64, old_member_count: u64) -> Result<bool> {
    quorum(new_member_count, old_member_count)
}

/// Sums voting power, rejecting a zero-power entry and any `u64` overflow.
pub fn total_voting_power(powers: impl IntoIterator<Item = u64>) -> Result<u64> {
    let mut total: u64 = 0;
    for power in powers {
        if power == 0 {
            return Err(Error::Arithmetic("validator with zero voting power"));
        }
        total = total
            .checked_add(power)
            .ok_or(Error::Arithmetic("summed voting power overflows u64"))?;
    }
    Ok(total)
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The boundary fixtures published with the predicate.
    #[test]
    fn published_boundary_fixtures() {
        assert!(!quorum(66, 100).unwrap());
        assert!(quorum(67, 100).unwrap());
        assert!(!quorum(67, 101).unwrap());
        assert!(quorum(68, 101).unwrap());
        assert!(!quorum(68, 102).unwrap());
        assert!(quorum(69, 102).unwrap());
    }

    #[test]
    fn the_predicate_is_strict_and_rejects_zero_total_power() {
        // Exactly two thirds is not a quorum.
        assert!(!quorum(2, 3).unwrap());
        assert!(quorum(3, 3).unwrap());
        assert!(quorum(u64::MAX, u64::MAX).unwrap());
        assert!(quorum(0, 0).is_err());
    }
}
