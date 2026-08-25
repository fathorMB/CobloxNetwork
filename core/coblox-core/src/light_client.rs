//! The light client's normative checks on validator-set composition.
//!
//! `ledger.md#what-a-light-client-can-establish-about-set-composition` states
//! the perimeter as **two closed lists**, and says why: "This document has
//! already had to correct one overstated safety claim [...] and the correction
//! stands as its standard: a wrong safety statement is worse than a missing
//! one."
//!
//! Both lists are part of the specification. This module implements the first
//! and represents the second as data — [`CANNOT_ESTABLISH`] — rather than as
//! prose that a future reader might mistake for an unimplemented backlog. There
//! is deliberately **no function in this crate** that claims any item of the
//! second list: not a "candidate set completeness" check, not a "the fills were
//! the lowest tickets" check, not a "this contraction was honest attrition"
//! check. Each of those would be an assumption the protocol does not authorize,
//! dressed as a guarantee.
//!
//! The claim this module is entitled to make, quoted in full:
//!
//! > Within the election parameter limits fixed at genesis, a light client
//! > establishes that the active set is **of lawful shape and in lawful
//! > rotation** — bounded terms, bounded entry, floored contraction, no
//! > off-schedule change, no member seated beyond its term — and does not
//! > establish that it is the set the eligibility rule should have produced.

use crate::block::{BlockHeader, TransitionOccasion, check_successor_commitment};
use crate::error::{Error, ParameterError, Result, SetError};
use crate::hash::{AccountKey, ChainId, Digest32};
use crate::json::JsonObject;
use crate::merkle::TaggedTree;
use crate::params::{
    ActiveRewardPolicyDocument, ConsensusParameters, ElectionBounds, RewardBounds, RewardPolicy,
    ValidatedConsensusParameters, ValidatedRewardPolicy,
};
use crate::registry::{self, DocumentKind};
use crate::validator_set::ValidatorSet;

/// The eight composition facts a light client **cannot** establish, verbatim
/// from `ledger.md`.
///
/// This constant exists so that the second closed list is as reviewable as the
/// first, and so that a future contributor adding a check can see, in the same
/// module, which checks are forbidden rather than merely absent.
pub const CANNOT_ESTABLISH: [&str; 8] = [
    "(a) that candidate_root contains every node that was genuinely eligible",
    "(b) that every committed candidate actually met the contribution threshold",
    "(c) that the fills are the lowest-ticket members of the pool",
    "(d) that the seed was not ground, which no verifier of any kind can establish",
    "(e) cooldown, beyond the boundaries it observed itself",
    "(f) that an off-boundary transition was due",
    "(g) that a lawful contraction is not a capture by attrition",
    "(h) that no candidacy was excluded by never being finalized",
];

/// Step 1 of the light-client algorithm: checkpoint freshness.
///
/// "The value a client uses at step 1 is **the one in the signed checkpoint**,
/// never one learned from a peer."
///
/// The signature check that must precede this is not here: verifying it needs a
/// consensus-critical Ed25519 verifier, which this crate deliberately does not
/// ship. See the crate documentation.
pub fn checkpoint_is_fresh(
    now_ms: u64,
    issued_at_ms: u64,
    max_weak_subjectivity_age_ms_from_checkpoint: u64,
) -> Result<bool> {
    let age = now_ms
        .checked_sub(issued_at_ms)
        .ok_or(Error::Arithmetic("checkpoint issued in the future"))?;
    Ok(age <= max_weak_subjectivity_age_ms_from_checkpoint)
}

/// The resolved parameter circularity: once the client holds an authenticated
/// header it must check that the chain and the checkpoint agree about the trust
/// window, and fail closed if they do not.
pub fn checkpoint_agrees_with_chain(
    max_weak_subjectivity_age_ms_from_checkpoint: u64,
    parameters: &ValidatedConsensusParameters,
) -> bool {
    parameters.get().max_weak_subjectivity_age_ms == max_weak_subjectivity_age_ms_from_checkpoint
}

/// Step 3: the non-regression rule, as persisted state.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TrustedTip {
    /// The highest trusted height, and the block ID seen at it.
    pub highest: Option<(u64, Digest32)>,
}

impl TrustedTip {
    /// Accepts a height/ID pair, or rejects it as a regression or a fork.
    ///
    /// "Reject a checkpoint, response, or restart state below it, and reject a
    /// different block ID at the same height."
    pub fn accept(&mut self, height: u64, block_id: Digest32) -> Result<()> {
        if let Some((trusted_height, trusted_id)) = self.highest {
            if height < trusted_height {
                return Err(Error::Arithmetic("light client height regression"));
            }
            if height == trusted_height && block_id != trusted_id {
                return Err(Error::Arithmetic("light client fork at trusted height"));
            }
        }
        self.highest = Some((height, block_id));
        Ok(())
    }
}

/// Step 5, first half: obtain and authenticate the election parameters.
///
/// The order and the absence of any fallback are normative. "A missing,
/// unverifiable, hash-mismatched, or out-of-bounds parameter document fails
/// closed [...] It MUST NOT proceed with defaults, with values from an earlier
/// document, or with values supplied by a peer."
///
/// The quorum-signature verification of the document is the caller's step and
/// is not folded in here, for the same reason as
/// [`checkpoint_is_fresh`]: this crate ships no signature verifier.
pub fn authenticate_consensus_parameters(
    chain_id: &ChainId,
    trusted_header: &BlockHeader,
    unsigned_document: &JsonObject,
    bounds: &ElectionBounds,
) -> Result<ValidatedConsensusParameters> {
    bounds.validate(chain_id)?;
    let recomputed = registry::protocol_document_hash(
        DocumentKind::ConsensusParameters,
        chain_id,
        unsigned_document,
    )?;
    if recomputed != trusted_header.consensus_parameters_hash {
        return Err(ParameterError::Constraint {
            rule: "consensus_parameters_hash MUST equal the hash of the trusted header",
        }
        .into());
    }
    let declared_chain = unsigned_document.digest("chain_id")?;
    if declared_chain != *chain_id.as_digest() {
        return Err(ParameterError::ChainIdMismatch.into());
    }
    let parameters = ConsensusParameters::from_body(unsigned_document.object("body")?)?;
    parameters.validate(
        bounds,
        unsigned_document.uint("activation_height")?,
        unsigned_document.uint("sequence")?,
        None,
    )
}

/// The reward-side twin of [`authenticate_consensus_parameters`]: obtain and
/// authenticate a `reward_policy` document.
///
/// The election side has shipped its composed entry point since the beginning,
/// and the reward side had none: [REVIEW-017] RF-001 observed that
/// [`RewardBounds::validate`] was not called from anywhere in the crate, so a
/// distribution carrying a degenerate bounds object — a
/// `reward_parameter_change_denominator` of zero, an activation gap of zero, a
/// `reward_epoch_ms_min` of zero — passed every check the reward path ran and
/// disabled the rate limit *without an error*. This function is the point where
/// the trust anchor is checked before it is trusted.
///
/// The binding differs from the election side in one way that is the
/// specification's and not a shortcut. `consensus_parameters_hash` is a field of
/// the block header, so the twin reads it from the trusted header. A
/// `reward_policy` is instead referenced by the `policy_hash` carried in the
/// signed `mint` transactions that apply it (`ledger.md#mint`), so the expected
/// digest is the caller's input: it comes from the signed object that names the
/// policy, never from the document being authenticated.
///
/// As with the twin, the quorum-signature verification of the document is the
/// caller's step and is not folded in here: this crate ships no signature
/// verifier.
pub fn authenticate_reward_policy(
    chain_id: &ChainId,
    expected_policy_hash: &Digest32,
    unsigned_document: &JsonObject,
    bounds: &RewardBounds,
    active: Option<&ActiveRewardPolicyDocument>,
) -> Result<ValidatedRewardPolicy> {
    bounds.validate(chain_id)?;
    let recomputed =
        registry::protocol_document_hash(DocumentKind::RewardPolicy, chain_id, unsigned_document)?;
    if recomputed != *expected_policy_hash {
        return Err(ParameterError::Constraint {
            rule: "policy_hash MUST equal the hash of the reward_policy document",
        }
        .into());
    }
    let declared_chain = unsigned_document.digest("chain_id")?;
    if declared_chain != *chain_id.as_digest() {
        return Err(ParameterError::ChainIdMismatch.into());
    }
    let policy = RewardPolicy::from_body(unsigned_document.object("body")?)?;
    policy.validate(
        bounds,
        unsigned_document.uint("activation_height")?,
        unsigned_document.uint("sequence")?,
        active,
    )
}

/// Step 5, second half: checks 1 to 10 over a transition the client accepts.
///
/// The numbering is the specification's. Check 11, candidate membership, needs
/// a proof the client was given and is [`candidate_membership`].
pub fn check_transition(
    chain_id: &ChainId,
    parameters: &ValidatedConsensusParameters,
    previous: &ValidatorSet,
    new: &ValidatorSet,
    revoked_validators: &[(String, u64)],
) -> Result<()> {
    // 3, 4 — uniform power, validator_id == node_id, size bounds, term limit.
    new.check_elected_shape(parameters)?;
    // 2, 6, 7 — activation height, election_epoch, the three counts, the
    // rotation cap, the seed derivation from the committed entropy IDs.
    new.check_election_record(chain_id, parameters)?;
    new.check_previous_hash(previous)?;
    // 5 — seated_since_epoch / term_expiry_epoch consistency.
    new.check_stamps_against_previous(previous, parameters)?;
    // 9 — the contraction floor.
    new.check_contraction_floor(previous)?;
    // Step 4's revocation application, with the data the checkpoint gives.
    new.check_against_revocations(revoked_validators)?;
    Ok(())
}

/// Check 1: the set changed only where it was permitted to.
pub fn check_no_off_schedule_change(
    header: &BlockHeader,
    occasion: TransitionOccasion,
) -> Result<()> {
    check_successor_commitment(header, occasion)
}

/// Check 8: the composition drift of the set at a boundary.
///
/// Returns `(entered, left)` as node-ID lists. This is a *quantity the client
/// can compute*, not a verdict: `ledger.md` is explicit that a lawful
/// contraction and a capture by attrition are indistinguishable — item `(g)` of
/// [`CANNOT_ESTABLISH`] — so this function reports the drift and judges nothing.
#[must_use]
pub fn composition_drift(
    previous: &ValidatorSet,
    new: &ValidatorSet,
) -> (Vec<String>, Vec<String>) {
    let entered = new
        .validators
        .iter()
        .filter(|entry| previous.find(&entry.node_id).is_none())
        .map(|entry| entry.node_id.clone())
        .collect();
    let left = previous
        .validators
        .iter()
        .filter(|entry| new.find(&entry.node_id).is_none())
        .map(|entry| entry.node_id.clone())
        .collect();
    (entered, left)
}

/// Check 11: given a membership proof against `candidate_root`, that a seated
/// member was in the committed candidate set.
///
/// This establishes membership and **nothing about completeness**: item `(a)`
/// of [`CANNOT_ESTABLISH`] remains out of reach with or without this proof.
#[must_use]
pub fn candidate_membership(
    candidate_root: &Digest32,
    election_epoch: u64,
    account_key: &AccountKey,
    leaf_index: usize,
    siblings: &[Digest32],
) -> bool {
    let leaf = crate::merkle::candidate_leaf(election_epoch, account_key);
    TaggedTree::CANDIDATES.verify_inclusion(candidate_root, &leaf, leaf_index, siblings)
}

/// Rules 1 and 2 of the revocation rule as a light client applies them to a
/// header: reject any block at height `>= effective_height` whose active set
/// contains a revoked `node_id`.
pub fn check_header_against_revocations(
    header: &BlockHeader,
    active_set: &ValidatorSet,
    revoked_validators: &[(String, u64)],
) -> Result<()> {
    for (node_id, effective_height) in revoked_validators {
        if header.height >= *effective_height && active_set.find(node_id).is_some() {
            return Err(SetError::Revocation {
                node_id: node_id.clone(),
            }
            .into());
        }
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_cannot_establish_list_is_the_specification_list() {
        assert_eq!(CANNOT_ESTABLISH.len(), 8);
        for (index, label) in ["(a)", "(b)", "(c)", "(d)", "(e)", "(f)", "(g)", "(h)"]
            .into_iter()
            .enumerate()
        {
            assert!(CANNOT_ESTABLISH[index].starts_with(label));
        }
    }

    #[test]
    fn non_regression_rejects_a_lower_height_and_a_fork() {
        let mut tip = TrustedTip::default();
        tip.accept(10, Digest32::repeated(1)).unwrap();
        assert!(tip.accept(9, Digest32::repeated(1)).is_err());
        assert!(tip.accept(10, Digest32::repeated(2)).is_err());
        assert!(tip.accept(11, Digest32::repeated(3)).is_ok());
    }
}
