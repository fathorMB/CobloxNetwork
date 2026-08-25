//! Governed parameters, the genesis bounds, and their validity rules.
//!
//! **Nothing in this module is a compiled constant.** The launch values of
//! every parameter below come from the economic simulator and the network
//! operator's genesis decision, and they reach this crate as configuration that
//! is validated against the constraint block of
//! `ledger.md#rotation-the-cap-and-the-floor`, the genesis
//! `ElectionBounds`, and the genesis `RewardBounds`. What *is* fixed here are
//! the constraints, which the specification states are "not parameters and are
//! fixed now".
//!
//! Validation is a recoverable [`crate::error::Error`], never a panic: in
//! production these values arrive inside a document a validator quorum signed,
//! and rejecting such a document is ordinary protocol operation.
//!
//! The type-level consequence is that the election derivation and the
//! light-client checks accept only [`ValidatedConsensusParameters`] and
//! [`ValidatedRewardPolicy`], which have no public constructors other than
//! [`ConsensusParameters::validate`] and [`RewardPolicy::validate`].

use crate::error::{Error, ParameterError, Result};
use crate::hash::ChainId;
use crate::json::JsonObject;

/// The election-parameter magnitudes fixed by the genesis trust anchor.
///
/// `ElectionBounds` is configuration, not chain state. It ships inside the
/// signed distribution and in no other channel, cannot be changed by any
/// on-chain document, and must not be learned from a peer, a header, or a
/// protocol document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElectionBounds {
    /// Network identifier of the distribution that carried these bounds.
    pub network_id: String,
    /// Chain the bounds belong to; must equal the client's configured chain.
    pub chain_id: ChainId,
    /// Ceiling on `election_epoch_blocks`.
    pub election_epoch_blocks_max: u64,
    /// Ceiling on `validator_max_consecutive_terms`.
    pub validator_max_consecutive_terms_max: u64,
    /// Ceiling on `validator_max_set_size`.
    pub validator_max_set_size_max: u64,
    /// Floor under `validator_min_set_size`.
    pub validator_min_set_size_min: u64,
    /// Floor under `validator_min_capture_epochs`.
    pub validator_min_capture_epochs_min: u64,
    /// Numerator of the per-change ratio; must exceed the denominator.
    pub election_parameter_change_numerator: u64,
    /// Denominator of the per-change ratio; must be positive.
    pub election_parameter_change_denominator: u64,
    /// Minimum chain-height spacing between consecutive election-parameter
    /// activations.
    pub election_parameter_min_activation_gap_blocks: u64,
}

impl ElectionBounds {
    /// Checks the bounds object itself against the client's configured chain.
    ///
    /// This is not a formality: bounds that fail here would let a quorum reach
    /// its genesis ceiling in a single document, which is the manoeuvre the
    /// ratio and the gap exist to convert into an observable process.
    pub fn validate(&self, configured_chain_id: &ChainId) -> Result<()> {
        if self.chain_id != *configured_chain_id {
            return Err(ParameterError::ChainIdMismatch.into());
        }
        if self.election_parameter_change_denominator == 0 {
            return Err(ParameterError::Bounds {
                rule: "election_parameter_change_denominator MUST be positive",
            }
            .into());
        }
        if self.election_parameter_change_numerator <= self.election_parameter_change_denominator {
            return Err(ParameterError::Bounds {
                rule: "election_parameter_change_numerator MUST exceed election_parameter_change_denominator",
            }
            .into());
        }
        if self.election_parameter_min_activation_gap_blocks == 0 {
            return Err(ParameterError::Bounds {
                rule: "election_parameter_min_activation_gap_blocks MUST be positive",
            }
            .into());
        }
        Ok(())
    }
}

/// The reward-policy magnitudes fixed by the genesis trust anchor.
///
/// `RewardBounds` is configuration, not chain state. It ships inside the
/// signed distribution and in no other channel, cannot be changed by any
/// on-chain document, and must not be learned from a peer, a header, or a
/// protocol document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RewardBounds {
    /// Network identifier of the distribution that carried these bounds.
    pub network_id: String,
    /// Chain the bounds belong to; must equal the client's configured chain.
    pub chain_id: ChainId,
    /// Hard genesis ceiling on the per-epoch existence fund `F`.
    pub existence_fund_microtokens_per_epoch_max: u64,
    /// Floor under `reward_epoch_ms`.
    pub reward_epoch_ms_min: u64,
    /// Ceiling on `reward_epoch_ms`.
    pub reward_epoch_ms_max: u64,
    /// Ceiling on `publisher_reward_cap_numerator`.
    pub publisher_reward_cap_numerator_max: u64,
    /// Floor under `publisher_reward_cap_denominator`.
    pub publisher_reward_cap_denominator_min: u64,
    /// Floor under `validator_eligibility_threshold_units`.
    pub validator_eligibility_threshold_units_min: u64,
    /// Ceiling on `validator_eligibility_window_epochs`.
    pub validator_eligibility_window_epochs_max: u64,
    /// Floor under `validator_eligibility_min_issuers`.
    pub validator_eligibility_min_issuers_min: u64,
    /// Ceiling on `storage_units_per_contribution_unit`.
    pub storage_units_per_contribution_unit_max: u64,
    /// Ceiling on `compute_units_per_contribution_unit`.
    pub compute_units_per_contribution_unit_max: u64,
    /// Floor under `storage_microtokens_per_byte_epoch`.
    pub storage_microtokens_per_byte_epoch_min: u64,
    /// Floor under `compute_microtokens_per_million_fuel`.
    pub compute_microtokens_per_million_fuel_min: u64,
    /// Numerator of the per-change ratio; must exceed the denominator.
    pub reward_parameter_change_numerator: u64,
    /// Denominator of the per-change ratio; must be positive.
    pub reward_parameter_change_denominator: u64,
    /// Minimum chain-height spacing between consecutive reward-parameter activations.
    pub reward_parameter_min_activation_gap_blocks: u64,
}

impl RewardBounds {
    /// Checks the bounds object itself against the client's configured chain.
    pub fn validate(&self, configured_chain_id: &ChainId) -> Result<()> {
        if self.chain_id != *configured_chain_id {
            return Err(ParameterError::ChainIdMismatch.into());
        }
        if self.reward_parameter_change_denominator == 0 {
            return Err(ParameterError::Bounds {
                rule: "reward_parameter_change_denominator MUST be positive",
            }
            .into());
        }
        if self.reward_parameter_change_numerator <= self.reward_parameter_change_denominator {
            return Err(ParameterError::Bounds {
                rule: "reward_parameter_change_numerator MUST exceed reward_parameter_change_denominator",
            }
            .into());
        }
        if self.reward_parameter_min_activation_gap_blocks == 0 {
            return Err(ParameterError::Bounds {
                rule: "reward_parameter_min_activation_gap_blocks MUST be positive",
            }
            .into());
        }
        if self.reward_epoch_ms_min == 0 {
            return Err(ParameterError::Bounds {
                rule: "reward_epoch_ms_min MUST be positive",
            }
            .into());
        }
        if self.reward_epoch_ms_min > self.reward_epoch_ms_max {
            return Err(ParameterError::Bounds {
                rule: "reward_epoch_ms_min MUST NOT exceed reward_epoch_ms_max",
            }
            .into());
        }
        Ok(())
    }
}

/// The `consensus_parameters` document body.
///
/// Field order follows `README.md#signed-protocol-documents`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConsensusParameters {
    /// Maximum accepted clock drift for a received proposal.
    pub max_clock_drift_ms: u64,
    /// Maximum `expires_at_ms - created_at_ms` of a wire envelope.
    pub max_envelope_validity_ms: u64,
    /// Replay cache cap per peer.
    pub replay_cache_entries_per_peer: u64,
    /// Global replay cache cap.
    pub replay_cache_entries_global: u64,
    /// Trust window for a weak subjectivity checkpoint.
    pub max_weak_subjectivity_age_ms: u64,
    /// Freshness bound for a served balance.
    pub max_current_balance_age_ms: u64,
    /// Notice epochs before an app suspension becomes effective.
    pub app_suspension_notice_epochs: u64,
    /// Minimum delay between proposing a revocation and its effective height.
    pub min_revocation_effective_delay_blocks: u64,
    /// `L`: blocks per election epoch.
    pub election_epoch_blocks: u64,
    /// Blocks before the boundary at which candidacies close.
    pub candidacy_close_blocks: u64,
    /// Blocks in the entropy window.
    pub election_entropy_blocks: u64,
    /// Lower bound on validator set size.
    pub validator_min_set_size: u64,
    /// `V`: the target validator set size.
    pub validator_target_set_size: u64,
    /// Upper bound on validator set size.
    pub validator_max_set_size: u64,
    /// `c`: seats that may be filled at one boundary.
    pub validator_churn_cap_seats: u64,
    /// `T`: consecutive-term limit.
    pub validator_max_consecutive_terms: u64,
    /// Epochs a departing member stays out of the pool.
    pub validator_cooldown_epochs: u64,
    /// `m`: boundaries a capture is declared to require.
    pub validator_min_capture_epochs: u64,
}

/// The ten election parameters the change ratio governs.
type ElectionParameterAccessor = (&'static str, fn(&ConsensusParameters) -> u64);

const ELECTION_PARAMETERS: [ElectionParameterAccessor; 10] = [
    ("election_epoch_blocks", |p| p.election_epoch_blocks),
    ("candidacy_close_blocks", |p| p.candidacy_close_blocks),
    ("election_entropy_blocks", |p| p.election_entropy_blocks),
    ("validator_min_set_size", |p| p.validator_min_set_size),
    ("validator_target_set_size", |p| p.validator_target_set_size),
    ("validator_max_set_size", |p| p.validator_max_set_size),
    ("validator_churn_cap_seats", |p| p.validator_churn_cap_seats),
    ("validator_max_consecutive_terms", |p| {
        p.validator_max_consecutive_terms
    }),
    ("validator_cooldown_epochs", |p| p.validator_cooldown_epochs),
    ("validator_min_capture_epochs", |p| {
        p.validator_min_capture_epochs
    }),
];

/// The currently active `consensus_parameters` document, for the rules that
/// compare a proposed document against it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ActiveConsensusDocument {
    /// `sequence` of the active document.
    pub sequence: u64,
    /// `activation_height` of the active document.
    pub activation_height: u64,
    /// Its parameters.
    pub parameters: ConsensusParameters,
}

impl ConsensusParameters {
    /// Reads a `ConsensusParametersBody` object.
    ///
    /// Every field is required and no unknown field is tolerated: "An
    /// implementation MUST NOT infer fields or defaults that are not defined
    /// here."
    pub fn from_body(body: &JsonObject) -> Result<Self> {
        body.reject_unknown_fields(&[
            "max_clock_drift_ms",
            "max_envelope_validity_ms",
            "replay_cache_entries_per_peer",
            "replay_cache_entries_global",
            "max_weak_subjectivity_age_ms",
            "max_current_balance_age_ms",
            "app_suspension_notice_epochs",
            "min_revocation_effective_delay_blocks",
            "election_epoch_blocks",
            "candidacy_close_blocks",
            "election_entropy_blocks",
            "validator_min_set_size",
            "validator_target_set_size",
            "validator_max_set_size",
            "validator_churn_cap_seats",
            "validator_max_consecutive_terms",
            "validator_cooldown_epochs",
            "validator_min_capture_epochs",
        ])?;
        Ok(Self {
            max_clock_drift_ms: body.uint("max_clock_drift_ms")?,
            max_envelope_validity_ms: body.uint("max_envelope_validity_ms")?,
            replay_cache_entries_per_peer: body.uint("replay_cache_entries_per_peer")?,
            replay_cache_entries_global: body.uint("replay_cache_entries_global")?,
            max_weak_subjectivity_age_ms: body.uint("max_weak_subjectivity_age_ms")?,
            max_current_balance_age_ms: body.uint("max_current_balance_age_ms")?,
            app_suspension_notice_epochs: body.uint("app_suspension_notice_epochs")?,
            min_revocation_effective_delay_blocks: body
                .uint("min_revocation_effective_delay_blocks")?,
            election_epoch_blocks: body.uint("election_epoch_blocks")?,
            candidacy_close_blocks: body.uint("candidacy_close_blocks")?,
            election_entropy_blocks: body.uint("election_entropy_blocks")?,
            validator_min_set_size: body.uint("validator_min_set_size")?,
            validator_target_set_size: body.uint("validator_target_set_size")?,
            validator_max_set_size: body.uint("validator_max_set_size")?,
            validator_churn_cap_seats: body.uint("validator_churn_cap_seats")?,
            validator_max_consecutive_terms: body.uint("validator_max_consecutive_terms")?,
            validator_cooldown_epochs: body.uint("validator_cooldown_epochs")?,
            validator_min_capture_epochs: body.uint("validator_min_capture_epochs")?,
        })
    }

    /// Applies the full election constraint block and the genesis magnitude
    /// bounds, plus — when `active` is supplied — the change ratio, the
    /// activation-gap spacing and the monotonic term limit.
    ///
    /// A document that fails any rule here is **invalid**, not merely unwise.
    pub fn validate(
        &self,
        bounds: &ElectionBounds,
        activation_height: u64,
        sequence: u64,
        active: Option<&ActiveConsensusDocument>,
    ) -> Result<ValidatedConsensusParameters> {
        self.check_relations()?;
        self.check_magnitudes(bounds)?;
        if let Some(active) = active {
            self.check_against_active(bounds, activation_height, sequence, active)?;
        }
        Ok(ValidatedConsensusParameters(*self))
    }

    /// The relational half of the constraint block.
    fn check_relations(&self) -> Result<()> {
        let v = self.validator_target_set_size;
        let t = self.validator_max_consecutive_terms;
        let c = self.validator_churn_cap_seats;
        let m = self.validator_min_capture_epochs;

        require(
            self.validator_min_set_size > 0
                && self.validator_min_set_size <= v
                && v <= self.validator_max_set_size,
            "0 < validator_min_set_size <= V <= validator_max_set_size",
        )?;
        require(
            self.election_entropy_blocks >= 2,
            "election_entropy_blocks >= 2",
        )?;
        require(
            self.candidacy_close_blocks > self.election_entropy_blocks,
            "candidacy_close_blocks > election_entropy_blocks",
        )?;
        require(
            self.election_epoch_blocks > self.candidacy_close_blocks,
            "election_epoch_blocks > candidacy_close_blocks",
        )?;
        require(t >= 1, "T >= 1")?;
        require(
            self.validator_cooldown_epochs >= 1,
            "validator_cooldown_epochs >= 1",
        )?;
        require(
            self.validator_cooldown_epochs <= t,
            "validator_cooldown_epochs <= T",
        )?;
        // `ceil(V / T) <= c`: the term floor must be satisfiable. `t >= 1` is
        // already established, so the division is safe.
        require(v.div_ceil(t) <= c, "ceil(V / T) <= c")?;
        let three_c = checked_mul_u128(3, u128::from(c), "3 * c")?;
        require(three_c < u128::from(v), "3 * c < V")?;
        let three_c_m = checked_mul_u128(three_c, u128::from(m), "3 * c * m")?;
        require(three_c_m <= u128::from(v), "3 * c * m <= V")?;
        let three_min_set = checked_mul_u128(
            3,
            u128::from(self.validator_min_set_size),
            "3 * validator_min_set_size",
        )?;
        let two_v = checked_mul_u128(2, u128::from(v), "2 * V")?;
        require(
            three_min_set >= two_v,
            "3 * validator_min_set_size >= 2 * V",
        )?;
        Ok(())
    }

    /// The magnitude half: taken from the genesis bounds and never from the
    /// document under evaluation.
    fn check_magnitudes(&self, bounds: &ElectionBounds) -> Result<()> {
        require(
            self.election_epoch_blocks <= bounds.election_epoch_blocks_max,
            "election_epoch_blocks <= election_epoch_blocks_max",
        )?;
        require(
            self.validator_max_consecutive_terms <= bounds.validator_max_consecutive_terms_max,
            "T <= validator_max_consecutive_terms_max",
        )?;
        require(
            self.validator_max_set_size <= bounds.validator_max_set_size_max,
            "validator_max_set_size <= validator_max_set_size_max",
        )?;
        require(
            self.validator_min_set_size >= bounds.validator_min_set_size_min,
            "validator_min_set_size >= validator_min_set_size_min",
        )?;
        require(
            self.validator_min_capture_epochs >= bounds.validator_min_capture_epochs_min,
            "m >= validator_min_capture_epochs_min",
        )?;
        Ok(())
    }

    fn check_against_active(
        &self,
        bounds: &ElectionBounds,
        activation_height: u64,
        sequence: u64,
        active: &ActiveConsensusDocument,
    ) -> Result<()> {
        if sequence <= active.sequence {
            return Err(ParameterError::SequenceNotIncreasing.into());
        }
        let numerator = u128::from(bounds.election_parameter_change_numerator);
        let denominator = u128::from(bounds.election_parameter_change_denominator);
        for (name, read) in ELECTION_PARAMETERS {
            let new = u128::from(read(self));
            let old = u128::from(read(&active.parameters));
            let new_bounded = checked_mul_u128(new, denominator, "change ratio")?
                <= checked_mul_u128(old, numerator, "change ratio")?;
            let old_bounded = checked_mul_u128(old, denominator, "change ratio")?
                <= checked_mul_u128(new, numerator, "change ratio")?;
            if !(new_bounded && old_bounded) {
                return Err(ParameterError::ChangeRatio { parameter: name }.into());
            }
        }
        let earliest = active
            .activation_height
            .checked_add(bounds.election_parameter_min_activation_gap_blocks)
            .ok_or(Error::Arithmetic("activation gap"))?;
        if activation_height < earliest {
            return Err(ParameterError::ActivationGap.into());
        }
        if self.validator_max_consecutive_terms < active.parameters.validator_max_consecutive_terms
        {
            return Err(ParameterError::TermLimitDecreased.into());
        }
        Ok(())
    }
}

/// Parameters that passed the constraint block.
///
/// The only way to obtain one is [`ConsensusParameters::validate`], so a
/// consumer of this type never has to re-check the block, and an unvalidated
/// parameter set cannot reach the election derivation by accident.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValidatedConsensusParameters(ConsensusParameters);

impl ValidatedConsensusParameters {
    /// Constructs a `ValidatedConsensusParameters` directly without checking
    /// the constraint block. For testing historic fixtures (like the worked example) only.
    #[doc(hidden)]
    #[must_use]
    pub const fn from_raw_for_testing(parameters: ConsensusParameters) -> Self {
        Self(parameters)
    }

    /// The underlying values.
    #[must_use]
    pub const fn get(&self) -> &ConsensusParameters {
        &self.0
    }

    /// `election_boundary_height(e) = e * L`.
    pub fn election_boundary_height(&self, election_epoch: u64) -> Result<u64> {
        election_epoch
            .checked_mul(self.0.election_epoch_blocks)
            .ok_or(Error::Arithmetic("election_boundary_height"))
    }

    /// `candidacy_close_height(e) = election_boundary_height(e) - candidacy_close_blocks`.
    pub fn candidacy_close_height(&self, election_epoch: u64) -> Result<u64> {
        self.election_boundary_height(election_epoch)?
            .checked_sub(self.0.candidacy_close_blocks)
            .ok_or(Error::Arithmetic("candidacy_close_height"))
    }

    /// `entropy_window(e)`, inclusive, as `(first_height, last_height)`.
    pub fn entropy_window(&self, election_epoch: u64) -> Result<(u64, u64)> {
        let boundary = self.election_boundary_height(election_epoch)?;
        let first = boundary
            .checked_sub(self.0.election_entropy_blocks)
            .ok_or(Error::Arithmetic("entropy_window"))?;
        let last = boundary
            .checked_sub(1)
            .ok_or(Error::Arithmetic("entropy_window"))?;
        Ok((first, last))
    }

    /// Whether `height` is an election boundary, i.e. `h = e * L` with `e >= 1`.
    #[must_use]
    pub fn is_election_boundary(&self, height: u64) -> bool {
        let length = self.0.election_epoch_blocks;
        length > 0 && height >= length && height.is_multiple_of(length)
    }

    /// The epoch whose boundary is `height`, when there is one.
    #[must_use]
    pub fn epoch_at_boundary(&self, height: u64) -> Option<u64> {
        if self.is_election_boundary(height) {
            Some(height / self.0.election_epoch_blocks)
        } else {
            None
        }
    }
}

/// The `reward_policy` document body.
///
/// Field order follows `README.md#signed-protocol-documents`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RewardPolicy {
    /// Duration of a reward epoch in milliseconds.
    pub reward_epoch_ms: u64,
    /// Hard per-epoch cap on existence income fund emission.
    pub existence_fund_microtokens_per_epoch: u64,
    /// Availability tariff per unit; MUST be zero in v0 ([ADR-010]).
    pub availability_microtokens_per_unit: u64,
    /// Storage work reward rate.
    pub storage_microtokens_per_byte_epoch: u64,
    /// Compute work reward rate.
    pub compute_microtokens_per_million_fuel: u64,
    /// Publisher reward rate per active subscriber.
    pub publisher_microtokens_per_active_subscriber: u64,
    /// Creator reward cap numerator (`kn`).
    pub publisher_reward_cap_numerator: u64,
    /// Creator reward cap denominator (`kd`).
    pub publisher_reward_cap_denominator: u64,
    /// Contribution score divisor for storage work.
    pub storage_units_per_contribution_unit: u64,
    /// Contribution score divisor for compute work.
    pub compute_units_per_contribution_unit: u64,
    /// Minimum contribution units required for validator pool eligibility.
    pub validator_eligibility_threshold_units: u64,
    /// Reward epochs across which contribution units accumulate.
    pub validator_eligibility_window_epochs: u64,
    /// Minimum number of distinct challenge issuers required.
    pub validator_eligibility_min_issuers: u64,
}

/// Backward compatibility alias for [`RewardPolicy`].
pub type RewardPolicyConstraints = RewardPolicy;

/// The thirteen reward parameters the change ratio governs.
type RewardParameterAccessor = (&'static str, fn(&RewardPolicy) -> u64);

const REWARD_PARAMETERS: [RewardParameterAccessor; 13] = [
    ("reward_epoch_ms", |p| p.reward_epoch_ms),
    ("existence_fund_microtokens_per_epoch", |p| {
        p.existence_fund_microtokens_per_epoch
    }),
    ("availability_microtokens_per_unit", |p| {
        p.availability_microtokens_per_unit
    }),
    ("storage_microtokens_per_byte_epoch", |p| {
        p.storage_microtokens_per_byte_epoch
    }),
    ("compute_microtokens_per_million_fuel", |p| {
        p.compute_microtokens_per_million_fuel
    }),
    ("publisher_microtokens_per_active_subscriber", |p| {
        p.publisher_microtokens_per_active_subscriber
    }),
    ("publisher_reward_cap_numerator", |p| {
        p.publisher_reward_cap_numerator
    }),
    ("publisher_reward_cap_denominator", |p| {
        p.publisher_reward_cap_denominator
    }),
    ("storage_units_per_contribution_unit", |p| {
        p.storage_units_per_contribution_unit
    }),
    ("compute_units_per_contribution_unit", |p| {
        p.compute_units_per_contribution_unit
    }),
    ("validator_eligibility_threshold_units", |p| {
        p.validator_eligibility_threshold_units
    }),
    ("validator_eligibility_window_epochs", |p| {
        p.validator_eligibility_window_epochs
    }),
    ("validator_eligibility_min_issuers", |p| {
        p.validator_eligibility_min_issuers
    }),
];

/// The currently active `reward_policy` document, for comparing a proposed
/// document against it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ActiveRewardPolicyDocument {
    /// `sequence` of the active document.
    pub sequence: u64,
    /// `activation_height` of the active document.
    pub activation_height: u64,
    /// Its parameters.
    pub policy: RewardPolicy,
}

/// Alias for [`ActiveRewardPolicyDocument`].
pub type ActiveRewardDocument = ActiveRewardPolicyDocument;

/// Reward policy that passed acceptance-time validation against `RewardBounds`.
///
/// The only way to obtain one is [`RewardPolicy::validate`], so a consumer
/// never has to re-check the rules, and an unvalidated policy cannot reach
/// reward calculation by accident.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ValidatedRewardPolicy(RewardPolicy);

impl ValidatedRewardPolicy {
    /// The underlying values.
    #[must_use]
    pub const fn get(&self) -> &RewardPolicy {
        &self.0
    }

    /// The creator-share cap: `amount * kd <= kn * counted_burn`.
    ///
    /// "Both products use checked `u128` intermediates; overflow rejects the
    /// block." Boundary: `floor(kn * B / kd)` is valid and that value plus one
    /// is invalid.
    ///
    /// This is a method on the *validated* policy, not on [`RewardPolicy`], for
    /// the reason the election derivation is a method on
    /// [`ValidatedConsensusParameters`]: a `kd` of zero or a `kn >= kd` would
    /// make the cap meaningless, and the type is what guarantees those were
    /// rejected before any reward is computed against it.
    pub fn publisher_reward_within_cap(
        &self,
        amount_microtokens: u64,
        counted_subscription_burn_microtokens: u64,
    ) -> Result<bool> {
        let left = checked_mul_u128(
            u128::from(amount_microtokens),
            u128::from(self.0.publisher_reward_cap_denominator),
            "creator-share cap",
        )?;
        let right = checked_mul_u128(
            u128::from(self.0.publisher_reward_cap_numerator),
            u128::from(counted_subscription_burn_microtokens),
            "creator-share cap",
        )?;
        Ok(left <= right)
    }
}

impl RewardPolicy {
    /// Reads a `RewardPolicyBody` object.
    ///
    /// Every field is required and no unknown field is tolerated.
    pub fn from_body(body: &JsonObject) -> Result<Self> {
        body.reject_unknown_fields(&[
            "reward_epoch_ms",
            "existence_fund_microtokens_per_epoch",
            "availability_microtokens_per_unit",
            "storage_microtokens_per_byte_epoch",
            "compute_microtokens_per_million_fuel",
            "publisher_microtokens_per_active_subscriber",
            "publisher_reward_cap_numerator",
            "publisher_reward_cap_denominator",
            "storage_units_per_contribution_unit",
            "compute_units_per_contribution_unit",
            "validator_eligibility_threshold_units",
            "validator_eligibility_window_epochs",
            "validator_eligibility_min_issuers",
        ])?;
        Ok(Self {
            reward_epoch_ms: body.uint("reward_epoch_ms")?,
            existence_fund_microtokens_per_epoch: body
                .uint("existence_fund_microtokens_per_epoch")?,
            availability_microtokens_per_unit: body.uint("availability_microtokens_per_unit")?,
            storage_microtokens_per_byte_epoch: body.uint("storage_microtokens_per_byte_epoch")?,
            compute_microtokens_per_million_fuel: body
                .uint("compute_microtokens_per_million_fuel")?,
            publisher_microtokens_per_active_subscriber: body
                .uint("publisher_microtokens_per_active_subscriber")?,
            publisher_reward_cap_numerator: body.uint("publisher_reward_cap_numerator")?,
            publisher_reward_cap_denominator: body.uint("publisher_reward_cap_denominator")?,
            storage_units_per_contribution_unit: body
                .uint("storage_units_per_contribution_unit")?,
            compute_units_per_contribution_unit: body
                .uint("compute_units_per_contribution_unit")?,
            validator_eligibility_threshold_units: body
                .uint("validator_eligibility_threshold_units")?,
            validator_eligibility_window_epochs: body
                .uint("validator_eligibility_window_epochs")?,
            validator_eligibility_min_issuers: body.uint("validator_eligibility_min_issuers")?,
        })
    }

    /// Applies internal validity rules and genesis magnitude bounds, plus
    /// change ratio and activation gap when `active` is supplied.
    pub fn validate(
        &self,
        bounds: &RewardBounds,
        activation_height: u64,
        sequence: u64,
        active: Option<&ActiveRewardPolicyDocument>,
    ) -> Result<ValidatedRewardPolicy> {
        self.check_internal()?;
        self.check_magnitudes(bounds)?;
        if let Some(active) = active {
            self.check_against_active(bounds, activation_height, sequence, active)?;
        }
        Ok(ValidatedRewardPolicy(*self))
    }

    /// The internal validity rules of `RewardPolicyBody` that do not depend on `RewardBounds`.
    pub fn check_internal(&self) -> Result<()> {
        if self.availability_microtokens_per_unit != 0 {
            return Err(ParameterError::RewardPolicy {
                rule: "availability_microtokens_per_unit == 0",
            }
            .into());
        }
        if self.publisher_reward_cap_denominator == 0 {
            return Err(ParameterError::RewardPolicy {
                rule: "publisher_reward_cap_denominator MUST be non-zero",
            }
            .into());
        }
        if self.publisher_reward_cap_numerator >= self.publisher_reward_cap_denominator {
            return Err(ParameterError::RewardPolicy {
                rule: "publisher_reward_cap_numerator MUST be strictly smaller than the denominator",
            }
            .into());
        }
        if self.storage_units_per_contribution_unit == 0 {
            return Err(ParameterError::RewardPolicy {
                rule: "storage_units_per_contribution_unit > 0",
            }
            .into());
        }
        if self.compute_units_per_contribution_unit == 0 {
            return Err(ParameterError::RewardPolicy {
                rule: "compute_units_per_contribution_unit > 0",
            }
            .into());
        }
        if self.validator_eligibility_window_epochs == 0 {
            return Err(ParameterError::RewardPolicy {
                rule: "validator_eligibility_window_epochs >= 1",
            }
            .into());
        }
        if self.validator_eligibility_min_issuers < 2 {
            return Err(ParameterError::RewardPolicy {
                rule: "validator_eligibility_min_issuers >= 2",
            }
            .into());
        }
        Ok(())
    }

    /// Magnitude bounds against genesis `RewardBounds`.
    pub fn check_magnitudes(&self, bounds: &RewardBounds) -> Result<()> {
        if self.existence_fund_microtokens_per_epoch
            > bounds.existence_fund_microtokens_per_epoch_max
        {
            return Err(ParameterError::RewardPolicy {
                rule: "existence_fund_microtokens_per_epoch <= existence_fund_microtokens_per_epoch_max",
            }
            .into());
        }
        if self.reward_epoch_ms < bounds.reward_epoch_ms_min {
            return Err(ParameterError::RewardPolicy {
                rule: "reward_epoch_ms >= reward_epoch_ms_min",
            }
            .into());
        }
        if self.reward_epoch_ms > bounds.reward_epoch_ms_max {
            return Err(ParameterError::RewardPolicy {
                rule: "reward_epoch_ms <= reward_epoch_ms_max",
            }
            .into());
        }
        if self.publisher_reward_cap_numerator > bounds.publisher_reward_cap_numerator_max {
            return Err(ParameterError::RewardPolicy {
                rule: "publisher_reward_cap_numerator <= publisher_reward_cap_numerator_max",
            }
            .into());
        }
        if self.publisher_reward_cap_denominator < bounds.publisher_reward_cap_denominator_min {
            return Err(ParameterError::RewardPolicy {
                rule: "publisher_reward_cap_denominator >= publisher_reward_cap_denominator_min",
            }
            .into());
        }
        if self.validator_eligibility_threshold_units
            < bounds.validator_eligibility_threshold_units_min
        {
            return Err(ParameterError::RewardPolicy {
                rule: "validator_eligibility_threshold_units >= validator_eligibility_threshold_units_min",
            }
            .into());
        }
        if self.validator_eligibility_window_epochs > bounds.validator_eligibility_window_epochs_max
        {
            return Err(ParameterError::RewardPolicy {
                rule: "validator_eligibility_window_epochs <= validator_eligibility_window_epochs_max",
            }
            .into());
        }
        if self.validator_eligibility_min_issuers < bounds.validator_eligibility_min_issuers_min {
            return Err(ParameterError::RewardPolicy {
                rule: "validator_eligibility_min_issuers >= validator_eligibility_min_issuers_min",
            }
            .into());
        }
        if self.storage_units_per_contribution_unit > bounds.storage_units_per_contribution_unit_max
        {
            return Err(ParameterError::RewardPolicy {
                rule: "storage_units_per_contribution_unit <= storage_units_per_contribution_unit_max",
            }
            .into());
        }
        if self.compute_units_per_contribution_unit > bounds.compute_units_per_contribution_unit_max
        {
            return Err(ParameterError::RewardPolicy {
                rule: "compute_units_per_contribution_unit <= compute_units_per_contribution_unit_max",
            }
            .into());
        }
        if self.storage_microtokens_per_byte_epoch < bounds.storage_microtokens_per_byte_epoch_min {
            return Err(ParameterError::RewardPolicy {
                rule: "storage_microtokens_per_byte_epoch >= storage_microtokens_per_byte_epoch_min",
            }
            .into());
        }
        if self.compute_microtokens_per_million_fuel
            < bounds.compute_microtokens_per_million_fuel_min
        {
            return Err(ParameterError::RewardPolicy {
                rule: "compute_microtokens_per_million_fuel >= compute_microtokens_per_million_fuel_min",
            }
            .into());
        }
        Ok(())
    }

    fn check_against_active(
        &self,
        bounds: &RewardBounds,
        activation_height: u64,
        sequence: u64,
        active: &ActiveRewardPolicyDocument,
    ) -> Result<()> {
        if sequence <= active.sequence {
            return Err(ParameterError::SequenceNotIncreasing.into());
        }
        // A degenerate ratio makes both inequalities below true for every pair,
        // which would disable the rate limit *silently* — the failure mode
        // [REVIEW-017] RF-001 exhibits. `RewardBounds::validate` already rejects
        // such an object, and `light_client::authenticate_reward_policy` calls
        // it; this guard is the second line, so that a caller reaching
        // `RewardPolicy::validate` directly cannot end up with a vacuous rule
        // instead of an error.
        if bounds.reward_parameter_change_denominator == 0
            || bounds.reward_parameter_change_numerator
                <= bounds.reward_parameter_change_denominator
        {
            return Err(ParameterError::Bounds {
                rule: "reward_parameter_change_numerator MUST exceed reward_parameter_change_denominator",
            }
            .into());
        }
        if bounds.reward_parameter_min_activation_gap_blocks == 0 {
            return Err(ParameterError::Bounds {
                rule: "reward_parameter_min_activation_gap_blocks MUST be positive",
            }
            .into());
        }
        let numerator = u128::from(bounds.reward_parameter_change_numerator);
        let denominator = u128::from(bounds.reward_parameter_change_denominator);
        for (name, read) in REWARD_PARAMETERS {
            let new = u128::from(read(self));
            let old = u128::from(read(&active.policy));
            let new_bounded = checked_mul_u128(new, denominator, "change ratio")?
                <= checked_mul_u128(old, numerator, "change ratio")?;
            let old_bounded = checked_mul_u128(old, denominator, "change ratio")?
                <= checked_mul_u128(new, numerator, "change ratio")?;
            if !(new_bounded && old_bounded) {
                return Err(ParameterError::ChangeRatio { parameter: name }.into());
            }
        }
        let earliest = active
            .activation_height
            .checked_add(bounds.reward_parameter_min_activation_gap_blocks)
            .ok_or(Error::Arithmetic("activation gap"))?;
        if activation_height < earliest {
            return Err(ParameterError::ActivationGap.into());
        }
        Ok(())
    }
}

/// The `enrollment_parameters` document body.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnrollmentParameters {
    /// Must be `argon2id-leading-zero-bits-v0` in v0.
    pub pow_algorithm: String,
    /// Leading zero bits required of the Argon2id tag.
    pub difficulty_bits: u64,
    /// Argon2id `m`.
    pub memory_kib: u64,
    /// Argon2id `t`.
    pub iterations: u64,
    /// Argon2id `p`.
    pub lanes: u64,
    /// Argon2id tag length.
    pub tag_length_bytes: u64,
    /// Enrollment request maximum age.
    pub max_request_age_ms: u64,
    /// Enrollment request maximum future skew.
    pub max_future_skew_ms: u64,
    /// Accepted `recent_block_height` lag.
    pub recent_block_window: u64,
}

impl EnrollmentParameters {
    /// The v0 proof-of-work algorithm identifier.
    pub const POW_ALGORITHM: &'static str = "argon2id-leading-zero-bits-v0";

    /// Reads an `EnrollmentParametersBody` object.
    pub fn from_body(body: &JsonObject) -> Result<Self> {
        body.reject_unknown_fields(&[
            "pow_algorithm",
            "difficulty_bits",
            "memory_kib",
            "iterations",
            "lanes",
            "tag_length_bytes",
            "max_request_age_ms",
            "max_future_skew_ms",
            "recent_block_window",
        ])?;
        Ok(Self {
            pow_algorithm: body.string("pow_algorithm")?.to_owned(),
            difficulty_bits: body.uint("difficulty_bits")?,
            memory_kib: body.uint("memory_kib")?,
            iterations: body.uint("iterations")?,
            lanes: body.uint("lanes")?,
            tag_length_bytes: body.uint("tag_length_bytes")?,
            max_request_age_ms: body.uint("max_request_age_ms")?,
            max_future_skew_ms: body.uint("max_future_skew_ms")?,
            recent_block_window: body.uint("recent_block_window")?,
        })
    }

    /// The enrollment cost floor, enforced when the document is accepted.
    ///
    /// The area form `memory_kib * iterations >= 196608` is deliberate: a
    /// literal `iterations >= 3` rule would reject RFC 9106's *first*
    /// recommended profile, which is the stronger of the two.
    pub fn validate(&self) -> Result<()> {
        if self.pow_algorithm != Self::POW_ALGORITHM {
            return Err(ParameterError::EnrollmentCostFloor {
                rule: "pow_algorithm MUST be argon2id-leading-zero-bits-v0",
            }
            .into());
        }
        if !(1..=16).contains(&self.lanes) {
            return Err(ParameterError::EnrollmentCostFloor {
                rule: "lanes in 1..=16",
            }
            .into());
        }
        if self.tag_length_bytes != 32 {
            return Err(ParameterError::EnrollmentCostFloor {
                rule: "tag_length_bytes == 32",
            }
            .into());
        }
        if self.memory_kib < 65_536 {
            return Err(ParameterError::EnrollmentCostFloor {
                rule: "memory_kib >= 65536",
            }
            .into());
        }
        if self.iterations < 1 {
            return Err(ParameterError::EnrollmentCostFloor {
                rule: "iterations >= 1",
            }
            .into());
        }
        let area = checked_mul_u128(
            u128::from(self.memory_kib),
            u128::from(self.iterations),
            "enrollment cost area",
        )?;
        if area < 196_608 {
            return Err(ParameterError::EnrollmentCostFloor {
                rule: "memory_kib * iterations >= 196608",
            }
            .into());
        }
        if !(2..=6).contains(&self.difficulty_bits) {
            return Err(ParameterError::EnrollmentCostFloor {
                rule: "difficulty_bits is in the inclusive range 2-6",
            }
            .into());
        }
        Ok(())
    }
}

/// Existence income: `amount = F / E` by integer division, with `E > 0`.
///
/// The remainder is not minted and is not carried forward.
pub fn existence_income_share(
    existence_fund_microtokens_per_epoch: u64,
    eligible_node_count: u64,
) -> Result<u64> {
    if eligible_node_count == 0 {
        return Err(Error::Arithmetic("existence income requires E > 0"));
    }
    Ok(existence_fund_microtokens_per_epoch / eligible_node_count)
}

fn require(condition: bool, rule: &'static str) -> Result<()> {
    if condition {
        Ok(())
    } else {
        Err(ParameterError::Constraint { rule }.into())
    }
}

fn checked_mul_u128(left: u128, right: u128, context: &'static str) -> Result<u128> {
    left.checked_mul(right).ok_or(Error::Arithmetic(context))
}

#[cfg(test)]
mod tests {
    /// `ceil(V / T) <= c < V / 3` requires `T >= 4`, and the specification says
    /// it is "proved by exhausting the parameter space rather than argued".
    /// This is that exhaustion, over every set size a test can afford.
    #[test]
    fn a_term_limit_of_three_or_fewer_is_unsatisfiable_at_every_set_size() {
        for v in 1u64..=512 {
            for t in 1u64..=3 {
                for c in 1u64..=v {
                    let satisfiable = v.div_ceil(t) <= c && 3 * c < v;
                    assert!(
                        !satisfiable,
                        "found a satisfiable instance at V={v} T={t} c={c}"
                    );
                }
            }
        }
    }

    /// The same exhaustion for `V = 3`: `3c < 3` cannot hold for any `c >= 1`.
    #[test]
    fn a_target_set_size_of_three_is_unsatisfiable_at_every_term_limit() {
        for t in 1u64..=512 {
            for c in 1u64..=3 {
                assert!(!(3u64.div_ceil(t) <= c && 3 * c < 3));
            }
        }
    }
}
