"""The parameter combination this study recommends, and the assumptions it rests on.

Every value here is an *output* of the scenarios in ``scenarios.py``. Changing
one and re-running is the intended way to disagree with it: the constraint block
is checked mechanically, so a combination that does not hold together fails
loudly rather than quietly.

Scale anchor, declared because nothing external pins it: the credit is a
measure with no peg ([ADR-009]), so the absolute size of the emission is a free
choice. It is fixed here by the one quantity governance does **not** control —
the work channel — set at 90 000 cr per one-day epoch for a reference network of
10 000 present nodes of which one in five serves storage or compute. The
existence fund then follows from the chosen ``alpha``, which is the correct
causal direction: ``alpha`` is observed, ``F`` is the knob.
"""

from __future__ import annotations

from .params import ConsensusParameters, ElectionBounds, ParameterSet, RewardPolicy


BLOCK_INTERVAL_SECONDS = 5  # assumption; see the report's "Assumptions" section

CONSENSUS = ConsensusParameters(
    election_epoch_blocks=120_960,  # 7 days at 5 s per block
    candidacy_close_blocks=17_280,  # 1 day before the boundary
    election_entropy_blocks=720,  # 1 hour
    validator_min_set_size=18,
    validator_target_set_size=27,
    validator_max_set_size=45,  # see the governance-reach section: V <= 36 for ever
    validator_churn_cap_seats=3,
    validator_max_consecutive_terms=9,
    validator_cooldown_epochs=2,
    validator_min_capture_epochs=3,
)

REWARD = RewardPolicy(
    reward_epoch_ms=86_400_000,  # 1 day
    existence_fund_microtokens_per_epoch=300_000_000,  # 300 cr/epoch at launch (~200 nodes expected, ADR-011)
    availability_microtokens_per_unit=0,  # see the report: a non-zero value breaks ADR-007 (a)
    storage_units_per_contribution_unit=1_073_741_824,  # 1 unit per GiB-epoch proven
    compute_units_per_contribution_unit=1_000_000,  # 1 unit per million fuel re-executed
    validator_eligibility_threshold_units=512,
    validator_eligibility_window_epochs=28,
    validator_eligibility_min_issuers=3,
    publisher_reward_cap_numerator=1,
    publisher_reward_cap_denominator=2,
)

BOUNDS = ElectionBounds(
    election_epoch_blocks_max=241_920,  # 14 days: at most a doubling of the boundary period
    validator_max_consecutive_terms_max=12,
    validator_max_set_size_max=81,
    validator_min_set_size_min=18,
    validator_min_capture_epochs_min=3,
    election_parameter_change_numerator=5,
    election_parameter_change_denominator=4,
    election_parameter_min_activation_gap_blocks=120_960,  # one full election epoch
)

RECOMMENDED = ParameterSet(
    name="coblox-v0-genesis-candidate",
    consensus=CONSENSUS,
    reward=REWARD,
    bounds=BOUNDS,
)

# --- the two quantities [ADR-007] left open --------------------------------

ALPHA_TARGET = 0.15
ALPHA_SURVEILLANCE_BAND = (0.10, 0.20)
X_DECLARED = 0.20  # ADR-007 metric (c): equal to the upper edge of the band

# --- reference regime used throughout the report ---------------------------

REFERENCE_PRESENT_NODES = 10_000
REFERENCE_CONTRIBUTOR_FRACTION = 0.20
REFERENCE_WORK_CHANNEL_MICROTOKENS = 90_000_000_000  # 90 000 cr per epoch
