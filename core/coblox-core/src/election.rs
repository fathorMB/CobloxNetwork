//! The validator election derivation of `ledger.md#the-derivation`.
//!
//! "Every step below is a total function of finalized data. Two verifiers
//! holding the same finalized chain derive byte-identical sets, or the block at
//! the boundary is invalid."
//!
//! The derivation itself is here. Establishing *which* nodes are eligible is
//! not: it is a function of finalized transactions — candidacies, challenge
//! evidence, revocations, and the cooldown history — which this crate does not
//! replay. Those facts therefore enter as [`CandidateFacts`], and the
//! derivation checks the consistency invariants it can check on them rather
//! than trusting them silently.

use crate::error::{ElectionError, Error, Result};
use crate::hash::{AccountKey, ChainId, Digest32};
use crate::merkle;
use crate::params::ValidatedConsensusParameters;
use crate::quorum;
use crate::registry;
use crate::validator_set::{ElectionRecord, ValidatorEntry, ValidatorSet};

/// What a replaying full node establishes about one node for epoch `e`.
///
/// `eligible` is the conjunction of the five eligibility conditions of
/// `ledger.md#eligibility-demonstrated-storage-and-compute-never-availability`:
/// enrolled and unrevoked at `candidacy_close_height(e)`; a valid candidacy
/// finalized strictly below it; a contribution score at or above the threshold;
/// that score drawn from at least `validator_eligibility_min_issuers` distinct
/// issuers, none of them the subject; and not in cooldown.
///
/// It is one boolean rather than five because every one of the five is settled
/// by replaying finalized transactions, which is outside this crate, and
/// because splitting them here would suggest the derivation re-derives them.
/// It does not.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CandidateFacts {
    /// The enrolled identity.
    pub node_id: String,
    /// Its account key, the sort key of the candidate tree and the ticket
    /// preimage input.
    pub account_key: AccountKey,
    /// The consensus key declared by its epoch-`e` candidacy.
    pub consensus_public_key: [u8; 32],
    /// The binding signature carried by that candidacy, over
    /// `activation_height = election_boundary_height(e)`.
    pub key_binding_signature: [u8; 64],
    /// Whether all five eligibility conditions hold.
    pub eligible: bool,
}

/// The full result of a derivation, including the intermediate quantities a
/// conformance suite compares individually.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Derivation {
    /// The elected set, ready to be hashed and committed.
    pub set: ValidatorSet,
    /// `C`, the committed eligible set, sorted bytewise by account key.
    pub candidates: Vec<AccountKey>,
    /// `Nw = C \ R`, ordered by `(ticket ascending, account_key ascending)`.
    pub ranked_pool: Vec<(Digest32, AccountKey)>,
    /// `election_entropy`.
    pub entropy: Digest32,
    /// `election_seed`.
    pub seed: Digest32,
    /// `fills`, the number of seats actually filled.
    pub fills: u64,
}

/// `fills = min( max(0, validator_target_set_size - |R|), validator_churn_cap_seats, |Nw| )`.
#[must_use]
pub fn fill_count(target_set_size: u64, retained: u64, churn_cap: u64, pool_size: u64) -> u64 {
    target_set_size
        .saturating_sub(retained)
        .min(churn_cap)
        .min(pool_size)
}

/// Orders a fill pool by `(ticket ascending, account_key ascending)`.
///
/// "The second key makes the order **total**: equal tickets would require a
/// SHA-256 collision, but a derivation with an unspecified case is not
/// deterministic even when the case is unreachable."
///
/// Exposed separately from [`derive`] precisely so that the unreachable case
/// can be exercised: a test can hand this function two entries with an equal
/// ticket, which no seed ever will.
#[must_use]
pub fn rank_pool(mut pool: Vec<(Digest32, AccountKey)>) -> Vec<(Digest32, AccountKey)> {
    pool.sort_by(|left, right| {
        left.0
            .as_bytes()
            .cmp(right.0.as_bytes())
            .then_with(|| left.1.as_bytes().cmp(right.1.as_bytes()))
    });
    pool
}

/// Runs the derivation for epoch `e`.
///
/// Returns [`ElectionError::ContractionFloor`] or
/// [`ElectionError::BelowMinimumSetSize`] when no valid set exists for the
/// epoch. That is not an implementation failure: "If either fails, **no valid
/// set exists for epoch `e`** and the chain stalls at the boundary." Stalling
/// is the specified behaviour and the caller must not improvise around it.
// The seven numbered steps of `ledger.md#the-derivation` are kept in one
// function and in the specification's order, because a reviewer comparing code
// against the document needs them adjacent; splitting them into private helpers
// would trade that legibility against a line count.
#[allow(clippy::too_many_lines)]
pub fn derive(
    chain_id: &ChainId,
    parameters: &ValidatedConsensusParameters,
    election_epoch: u64,
    previous: &ValidatorSet,
    facts: &[CandidateFacts],
    entropy_block_ids: &[Digest32],
) -> Result<Derivation> {
    if election_epoch == 0 {
        return Err(ElectionError::NotAnElectionEpoch.into());
    }
    let params = parameters.get();
    let boundary = parameters.election_boundary_height(election_epoch)?;

    let fact_for = |node_id: &str| facts.iter().find(|fact| fact.node_id == node_id);

    // Step 1 — Retain.
    let mut retained: Vec<ValidatorEntry> = Vec::new();
    let mut departed: Vec<&str> = Vec::new();
    for entry in &previous.validators {
        let keeps_seat = fact_for(&entry.node_id).is_some_and(|fact| fact.eligible)
            && election_epoch < entry.term_expiry_epoch;
        if keeps_seat {
            let fact = fact_for(&entry.node_id).ok_or_else(|| {
                Error::Election(ElectionError::InconsistentCandidateSet {
                    node_id: entry.node_id.clone(),
                })
            })?;
            retained.push(ValidatorEntry {
                validator_id: entry.node_id.clone(),
                node_id: entry.node_id.clone(),
                // "takes its consensus key and binding from its epoch-e candidacy"
                consensus_public_key: fact.consensus_public_key,
                key_binding_signature: fact.key_binding_signature,
                // "keeps its seated_since_epoch and its term_expiry_epoch unchanged"
                seated_since_epoch: entry.seated_since_epoch,
                term_expiry_epoch: entry.term_expiry_epoch,
                voting_power: 1,
            });
        } else {
            departed.push(entry.node_id.as_str());
        }
    }

    // Step 2 — Commit the candidates. `C` contains the retained members too,
    // and never a member of `P` that failed step 1: "leaving a seat starts the
    // cooldown of eligibility condition 5 whatever the reason for leaving".
    let mut candidate_facts: Vec<&CandidateFacts> = Vec::new();
    for fact in facts.iter().filter(|fact| fact.eligible) {
        if departed.contains(&fact.node_id.as_str()) {
            return Err(ElectionError::InconsistentCandidateSet {
                node_id: fact.node_id.clone(),
            }
            .into());
        }
        candidate_facts.push(fact);
    }
    for entry in &retained {
        if !candidate_facts
            .iter()
            .any(|fact| fact.node_id == entry.node_id)
        {
            return Err(ElectionError::InconsistentCandidateSet {
                node_id: entry.node_id.clone(),
            }
            .into());
        }
    }

    let mut candidates: Vec<AccountKey> = candidate_facts
        .iter()
        .map(|fact| fact.account_key)
        .collect();
    candidates.sort_unstable_by(|left, right| left.as_bytes().cmp(right.as_bytes()));
    if candidates.windows(2).any(|pair| pair[0] == pair[1]) {
        return Err(ElectionError::DuplicateAccountKey.into());
    }
    let candidate_root = merkle::candidate_root(election_epoch, &candidates)?;
    let candidate_count =
        u64::try_from(candidates.len()).map_err(|_| Error::Arithmetic("candidate_count"))?;

    // Step 3 — Form the fill pool.
    let pool_facts: Vec<&CandidateFacts> = candidate_facts
        .iter()
        .copied()
        .filter(|fact| !retained.iter().any(|entry| entry.node_id == fact.node_id))
        .collect();

    // Step 4 — Derive the seed from the entropy window alone.
    let entropy = registry::election_entropy(
        chain_id,
        election_epoch,
        params.election_entropy_blocks,
        entropy_block_ids,
    )?;
    let seed = registry::election_seed(chain_id, election_epoch, &entropy);

    // Step 5 — Rank.
    let ranked_pool = rank_pool(
        pool_facts
            .iter()
            .map(|fact| {
                (
                    registry::election_ticket(chain_id, &seed, &fact.account_key),
                    fact.account_key,
                )
            })
            .collect(),
    );

    // Step 6 — Fill, under the cap.
    let retained_count =
        u64::try_from(retained.len()).map_err(|_| Error::Arithmetic("retained_count"))?;
    let pool_size = u64::try_from(ranked_pool.len()).map_err(|_| Error::Arithmetic("pool size"))?;
    let fills = fill_count(
        params.validator_target_set_size,
        retained_count,
        params.validator_churn_cap_seats,
        pool_size,
    );
    let term_expiry_epoch = election_epoch
        .checked_add(params.validator_max_consecutive_terms)
        .ok_or(Error::Arithmetic("term_expiry_epoch"))?;

    let mut members = retained;
    for (_, account_key) in ranked_pool
        .iter()
        .take(usize::try_from(fills).map_err(|_| Error::Arithmetic("fills"))?)
    {
        let fact = pool_facts
            .iter()
            .find(|fact| fact.account_key == *account_key)
            .ok_or(Error::Arithmetic("ranked pool entry without facts"))?;
        members.push(ValidatorEntry {
            validator_id: fact.node_id.clone(),
            node_id: fact.node_id.clone(),
            consensus_public_key: fact.consensus_public_key,
            key_binding_signature: fact.key_binding_signature,
            seated_since_epoch: election_epoch,
            term_expiry_epoch,
            voting_power: 1,
        });
    }

    // Step 7 — Assemble, then the two size conditions.
    members.sort_by(|left, right| left.validator_id.cmp(&right.validator_id));
    let member_count =
        u64::try_from(members.len()).map_err(|_| Error::Arithmetic("member_count"))?;
    let previous_count = previous.member_count()?;
    if !quorum::contraction_floor(member_count, previous_count)? {
        return Err(ElectionError::ContractionFloor {
            new: member_count,
            previous: previous_count,
        }
        .into());
    }
    if member_count < params.validator_min_set_size {
        return Err(ElectionError::BelowMinimumSetSize {
            new: member_count,
            minimum: params.validator_min_set_size,
        }
        .into());
    }

    let (entropy_first_height, _) = parameters.entropy_window(election_epoch)?;
    let set = ValidatorSet {
        schema_version: previous.schema_version.clone(),
        activation_height: boundary,
        election: Some(ElectionRecord {
            election_epoch,
            previous_validator_set_hash: previous.hash()?,
            candidate_root,
            candidate_count,
            entropy_first_height,
            entropy_block_ids: entropy_block_ids.to_vec(),
            election_seed: seed,
            retained_count,
            filled_count: fills,
            member_count,
        }),
        validators: members,
    };

    Ok(Derivation {
        set,
        candidates,
        ranked_pool,
        entropy,
        seed,
        fills,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_cap_and_the_short_pool_both_bind_the_fill_count() {
        // target 8, retained 2, cap 2, pool 3 -> the cap binds.
        assert_eq!(fill_count(8, 2, 2, 3), 2);
        // target 8, retained 2, cap 5, pool 1 -> the pool binds.
        assert_eq!(fill_count(8, 2, 5, 1), 1);
        // retained already above target -> max(0, ...) is zero, not a wrap.
        assert_eq!(fill_count(8, 9, 5, 3), 0);
    }

    /// The tie case is unreachable with real tickets — it needs a SHA-256
    /// collision — which is exactly why the ordering function is exposed and
    /// tested directly rather than through a seed.
    #[test]
    fn equal_tickets_are_broken_by_account_key_ascending() {
        let ticket = Digest32::repeated(0x99);
        let high = AccountKey::from_bytes([0xf0; 32]);
        let low = AccountKey::from_bytes([0x0f; 32]);
        let ranked = rank_pool(vec![(ticket, high), (ticket, low)]);
        assert_eq!(ranked[0].1, low);
        assert_eq!(ranked[1].1, high);
    }
}
