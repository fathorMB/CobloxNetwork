"""Parameter sets and the constraint block of `docs/protocol/ledger.md`.

This module is the executable form of two things the protocol documents state in
prose:

* ``ConsensusParametersBody`` / ``RewardPolicyBody`` / ``ElectionBounds`` — the
  fields a conformant network must publish (``docs/protocol/README.md``);
* the **constraint block** of ``ledger.md`` §"Magnitudes, not only relations:
  the bounds are fixed at genesis", which a ``consensus_parameters`` document
  must satisfy to be accepted.

Nothing here changes a protocol rule. The constraint checker is a *reader* of
the document: every rule carries the exact text it transcribes, so a reviewer
can diff the two side by side.
"""

from __future__ import annotations

import math
from dataclasses import dataclass, field
from typing import Iterator


# --------------------------------------------------------------------------
# Parameter containers
# --------------------------------------------------------------------------


@dataclass(frozen=True)
class ElectionBounds:
    """Genesis trust-anchor magnitudes. Outside on-chain governance.

    ``docs/protocol/README.md`` §"Election bounds".
    """

    election_epoch_blocks_max: int
    validator_max_consecutive_terms_max: int
    validator_max_set_size_max: int
    validator_min_set_size_min: int
    validator_min_capture_epochs_min: int
    election_parameter_change_numerator: int
    election_parameter_change_denominator: int
    election_parameter_min_activation_gap_blocks: int


@dataclass(frozen=True)
class ConsensusParameters:
    """The election-relevant subset of ``ConsensusParametersBody``."""

    election_epoch_blocks: int
    candidacy_close_blocks: int
    election_entropy_blocks: int
    validator_min_set_size: int
    validator_target_set_size: int
    validator_max_set_size: int
    validator_churn_cap_seats: int
    validator_max_consecutive_terms: int
    validator_cooldown_epochs: int
    validator_min_capture_epochs: int

    # Short aliases used by the derivation, matching ledger.md notation.
    @property
    def V(self) -> int:  # noqa: N802 - protocol notation
        return self.validator_target_set_size

    @property
    def T(self) -> int:  # noqa: N802 - protocol notation
        return self.validator_max_consecutive_terms

    @property
    def c(self) -> int:
        return self.validator_churn_cap_seats

    @property
    def m(self) -> int:
        return self.validator_min_capture_epochs


@dataclass(frozen=True)
class RewardPolicy:
    """The eligibility-relevant and emission-relevant subset of
    ``RewardPolicyBody``."""

    reward_epoch_ms: int
    existence_fund_microtokens_per_epoch: int
    availability_microtokens_per_unit: int
    storage_units_per_contribution_unit: int
    compute_units_per_contribution_unit: int
    validator_eligibility_threshold_units: int
    validator_eligibility_window_epochs: int
    validator_eligibility_min_issuers: int
    publisher_reward_cap_numerator: int
    publisher_reward_cap_denominator: int


@dataclass(frozen=True)
class ParameterSet:
    """A full candidate parameter combination."""

    name: str
    consensus: ConsensusParameters
    reward: RewardPolicy
    bounds: ElectionBounds


# --------------------------------------------------------------------------
# Constraint block
# --------------------------------------------------------------------------


@dataclass(frozen=True)
class RuleResult:
    rule: str
    expression: str
    ok: bool

    def line(self) -> str:
        return f"[{'PASS' if self.ok else 'FAIL':4}] {self.rule:<52} {self.expression}"


def check_constraint_block(
    ps: ParameterSet,
    active: ConsensusParameters | None = None,
    active_activation_height: int | None = None,
    new_activation_height: int | None = None,
) -> list[RuleResult]:
    """Evaluate the ledger.md constraint block, one rule at a time.

    ``active`` is the currently active consensus-parameters document; when it is
    ``None`` the rate-of-change, spacing, and monotonic-``T`` rules are reported
    as vacuously satisfied for a genesis document, which is what they are.
    """

    p = ps.consensus
    r = ps.reward
    b = ps.bounds
    V, T, c, m = p.V, p.T, p.c, p.m
    out: list[RuleResult] = []

    def add(rule: str, expression: str, ok: bool) -> None:
        out.append(RuleResult(rule, expression, bool(ok)))

    # --- relational rules, ledger.md constraint block, in document order -----
    add(
        "0 < validator_min_set_size <= V <= validator_max_set_size",
        f"0 < {p.validator_min_set_size} <= {V} <= {p.validator_max_set_size}",
        0 < p.validator_min_set_size <= V <= p.validator_max_set_size,
    )
    add(
        "election_entropy_blocks >= 2",
        f"{p.election_entropy_blocks} >= 2",
        p.election_entropy_blocks >= 2,
    )
    add(
        "candidacy_close_blocks > election_entropy_blocks",
        f"{p.candidacy_close_blocks} > {p.election_entropy_blocks}",
        p.candidacy_close_blocks > p.election_entropy_blocks,
    )
    add(
        "election_epoch_blocks > candidacy_close_blocks",
        f"{p.election_epoch_blocks} > {p.candidacy_close_blocks}",
        p.election_epoch_blocks > p.candidacy_close_blocks,
    )
    add(
        "T >= 1 and validator_cooldown_epochs >= 1",
        f"{T} >= 1 and {p.validator_cooldown_epochs} >= 1",
        T >= 1 and p.validator_cooldown_epochs >= 1,
    )
    add(
        "validator_cooldown_epochs <= T",
        f"{p.validator_cooldown_epochs} <= {T}",
        p.validator_cooldown_epochs <= T,
    )
    add(
        "validator_eligibility_window_epochs >= 1",
        f"{r.validator_eligibility_window_epochs} >= 1",
        r.validator_eligibility_window_epochs >= 1,
    )
    add(
        "ceil(V / T) <= c",
        f"ceil({V}/{T}) = {math.ceil(V / T)} <= {c}",
        math.ceil(V / T) <= c,
    )
    add("3 * c < V", f"3*{c} = {3 * c} < {V}", 3 * c < V)
    add("3 * c * m <= V", f"3*{c}*{m} = {3 * c * m} <= {V}", 3 * c * m <= V)
    add(
        "storage_units_per_contribution_unit > 0",
        f"{r.storage_units_per_contribution_unit} > 0",
        r.storage_units_per_contribution_unit > 0,
    )
    add(
        "compute_units_per_contribution_unit > 0",
        f"{r.compute_units_per_contribution_unit} > 0",
        r.compute_units_per_contribution_unit > 0,
    )
    add(
        "validator_eligibility_min_issuers >= 2",
        f"{r.validator_eligibility_min_issuers} >= 2",
        r.validator_eligibility_min_issuers >= 2,
    )

    # --- magnitude bounds, from the genesis ElectionBounds ------------------
    add(
        "election_epoch_blocks <= election_epoch_blocks_max",
        f"{p.election_epoch_blocks} <= {b.election_epoch_blocks_max}",
        p.election_epoch_blocks <= b.election_epoch_blocks_max,
    )
    add(
        "T <= validator_max_consecutive_terms_max",
        f"{T} <= {b.validator_max_consecutive_terms_max}",
        T <= b.validator_max_consecutive_terms_max,
    )
    add(
        "validator_max_set_size <= validator_max_set_size_max",
        f"{p.validator_max_set_size} <= {b.validator_max_set_size_max}",
        p.validator_max_set_size <= b.validator_max_set_size_max,
    )
    add(
        "validator_min_set_size >= validator_min_set_size_min",
        f"{p.validator_min_set_size} >= {b.validator_min_set_size_min}",
        p.validator_min_set_size >= b.validator_min_set_size_min,
    )
    add(
        "m >= validator_min_capture_epochs_min",
        f"{m} >= {b.validator_min_capture_epochs_min}",
        m >= b.validator_min_capture_epochs_min,
    )

    # --- ElectionBounds internal validity (README.md §Election bounds) ------
    add(
        "change_numerator > change_denominator > 0",
        f"{b.election_parameter_change_numerator} > "
        f"{b.election_parameter_change_denominator} > 0",
        b.election_parameter_change_numerator > b.election_parameter_change_denominator > 0,
    )
    add(
        "election_parameter_min_activation_gap_blocks > 0",
        f"{b.election_parameter_min_activation_gap_blocks} > 0",
        b.election_parameter_min_activation_gap_blocks > 0,
    )

    # --- rate of change, spacing, direction ---------------------------------
    num = b.election_parameter_change_numerator
    den = b.election_parameter_change_denominator
    if active is None:
        add(
            "rate of change vs active document",
            "genesis document: no active document to compare against",
            True,
        )
        add(
            "activation_height spacing",
            "genesis document: no previous activation height",
            True,
        )
        add("T_new >= T_active", "genesis document: no active T", True)
    else:
        ok = True
        detail: list[str] = []
        for fname in _ELECTION_PARAMETER_FIELDS:
            x_new = getattr(p, fname)
            x_old = getattr(active, fname)
            this = (x_new * den <= x_old * num) and (x_old * den <= x_new * num)
            ok = ok and this
            if not this:
                detail.append(f"{fname}: {x_old} -> {x_new}")
        add(
            "rate of change vs active document",
            "all election parameters within "
            f"{num}/{den}" + ("" if ok else "; violated by " + ", ".join(detail)),
            ok,
        )
        gap_ok = (
            new_activation_height is not None
            and active_activation_height is not None
            and new_activation_height
            >= active_activation_height + b.election_parameter_min_activation_gap_blocks
        )
        add(
            "activation_height spacing",
            f"{new_activation_height} >= {active_activation_height} + "
            f"{b.election_parameter_min_activation_gap_blocks}",
            gap_ok,
        )
        add(
            "T_new >= T_active",
            f"{T} >= {active.T}",
            T >= active.T,
        )

    return out


_ELECTION_PARAMETER_FIELDS = (
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
)


def constraint_block_passes(results: list[RuleResult]) -> bool:
    return all(r.ok for r in results)


# --------------------------------------------------------------------------
# The two non-obvious couplings, verified rather than assumed
# --------------------------------------------------------------------------


def feasible_c_values(V: int, T: int, m: int) -> Iterator[int]:
    """Yield every ``c`` satisfying ``ceil(V/T) <= c < V/3`` and ``3*c*m <= V``."""

    lo = math.ceil(V / T)
    for c in range(max(1, lo), V):
        if 3 * c >= V:
            break
        if c < lo:
            continue
        if 3 * c * m <= V:
            yield c


def term_limit_satisfiable(T: int, m: int, v_max: int) -> tuple[bool, int | None]:
    """Return (satisfiable, smallest V) for a term limit ``T`` and horizon ``m``.

    Brute force over set sizes, which is how ``ledger.md`` says the impossibility
    of ``T <= 3`` should surface: when the parameters are chosen, not at the
    boundary where the chain would otherwise have stopped.
    """

    for V in range(1, v_max + 1):
        if any(True for _ in feasible_c_values(V, T, m)):
            return True, V
    return False, None
