"""The experiments. Each returns plain data; ``__main__`` renders the transcript."""

from __future__ import annotations

import math
from dataclasses import dataclass, replace
from fractions import Fraction

from . import recommended as R
from .election import (
    CandidateNode,
    ChainStalled,
    ElectionState,
    Seat,
    ValidatorSet,
    boundaries_to_admission_capture,
    boundaries_to_attrition_capture,
    derive_boundary,
    entropy_ids,
    genesis_set,
    minimum_pool_for_sustained_set,
)
from .emission import (
    MICROTOKENS_PER_CREDIT,
    NodePopulation,
    ReputationMargin,
    WorkChannel,
    fund_for_target_alpha,
    run_epoch,
)
from .params import (
    ConsensusParameters,
    ParameterSet,
    check_constraint_block,
    constraint_block_passes,
    feasible_c_values,
    legal_next_intervals,
    max_reachable_target_set_size,
    term_limit_satisfiable,
)
from .population import (
    ContributorProfile,
    build_adversary_candidates,
    build_contributors,
)

CHAIN_ID = b"\x11" * 32
SEED = "SPEC-007/coblox-economic-simulator/v1"

# The entropy window is `election_entropy_blocks` long on chain (720 blocks).
# The derivation is a function of the *set* of block IDs, not of their number,
# so the model samples a fixed-size window; grinding resamples its last entry,
# which is the only entry a single proposer controls.
ENTROPY_WINDOW_SAMPLE = 8

# Grinding attempts per boundary in AT-10 configuration 1. The residual excess
# grows as sqrt(2 ln G), so the difference between 256 and 2^20 attempts is a
# factor of 2.8 on a quantity the churn cap bounds at c regardless. The pools
# are scaled down for the same reason: AT-10 measures a ratio N/H, not a size.
GRINDING_ATTEMPTS = 128


# --------------------------------------------------------------------------
# S1 — model validation against the arithmetic the Lead verified independently
# --------------------------------------------------------------------------


@dataclass(frozen=True)
class ValidationPoint:
    alpha_requested: float
    observed_alpha: float
    captured_pct: float
    expected_pct: float


def s1_model_validation() -> list[ValidationPoint]:
    """[ADR-007]: N = 10 000 emulated against H = 1 000 honest.

    alpha = 1   -> 90,9 % of emission captured
    alpha = 0,1 ->  9,1 % of emission captured
    """

    N, H = 10_000, 1_000
    out: list[ValidationPoint] = []

    for alpha in (Fraction(1), Fraction(1, 10)):
        # The work channel is the exogenous quantity; F follows from it.
        work_total = 0 if alpha == 1 else 90_000_000_000
        F = fund_for_target_alpha(alpha, work_total, 90_000_000_000)
        pop = NodePopulation(
            honest_availability_only=H,
            honest_contributors=0 if alpha == 1 else H,
            emulated=N,
        )
        if alpha != 1:
            pop = NodePopulation(
                honest_availability_only=0,
                honest_contributors=H,
                emulated=N,
            )
        result = run_epoch(
            population=pop,
            existence_fund_microtokens=F,
            work=WorkChannel(total_microtokens=work_total),
        )
        expected = 100.0 * float(alpha) * N / (N + H)
        out.append(
            ValidationPoint(
                alpha_requested=float(alpha),
                observed_alpha=float(result.alpha),
                captured_pct=100.0 * float(result.captured_share),
                expected_pct=expected,
            )
        )
    return out


# --------------------------------------------------------------------------
# S2 — the alpha curve: defensibility against meaning
# --------------------------------------------------------------------------


@dataclass(frozen=True)
class CurvePoint:
    alpha: float
    fund_credits_per_epoch: float
    availability_only_income_credits: float
    contributor_income_credits: float
    availability_only_over_average: float
    capture_at_devnet: float  # AT-07 config: H = 100, N = 10 000
    capture_at_reference: float  # H = 10 000, N = 10 000
    capture_at_scale: float  # H = 100 000, N = 10 000
    honest_dilution_at_devnet: float


ALPHA_GRID = (0.02, 0.05, 0.08, 0.10, 0.12, 0.15, 0.18, 0.20, 0.25, 0.30, 0.50, 0.75, 1.00)


def _capture(alpha: float, N: int, H: int) -> float:
    return alpha * N / (N + H)


def s2_alpha_curve() -> list[CurvePoint]:
    W = R.REFERENCE_WORK_CHANNEL_MICROTOKENS
    E = R.REFERENCE_PRESENT_NODES
    contributors = int(E * R.REFERENCE_CONTRIBUTOR_FRACTION)
    out: list[CurvePoint] = []

    for a in ALPHA_GRID:
        alpha = Fraction(a).limit_denominator(1000)
        work_total = 0 if alpha == 1 else W
        F = fund_for_target_alpha(alpha, work_total, W)
        pop = NodePopulation(
            honest_availability_only=E - contributors,
            honest_contributors=contributors,
            emulated=0,
        )
        res = run_epoch(pop, F, WorkChannel(total_microtokens=work_total))
        average = res.total_minted / E
        phone = res.honest_availability_only_income
        contributor = (
            sum(res.honest_contributor_incomes) / len(res.honest_contributor_incomes)
            if res.honest_contributor_incomes
            else 0
        )
        out.append(
            CurvePoint(
                alpha=float(alpha),
                fund_credits_per_epoch=F / MICROTOKENS_PER_CREDIT,
                availability_only_income_credits=phone / MICROTOKENS_PER_CREDIT,
                contributor_income_credits=contributor / MICROTOKENS_PER_CREDIT,
                availability_only_over_average=(phone / average) if average else 0.0,
                capture_at_devnet=_capture(float(alpha), 10_000, 100),
                capture_at_reference=_capture(float(alpha), 10_000, 10_000),
                capture_at_scale=_capture(float(alpha), 10_000, 100_000),
                honest_dilution_at_devnet=100.0 / (10_000 + 100),
            )
        )
    return out


# --------------------------------------------------------------------------
# S3 — alpha is observed, not set: the launch-regime drift
# --------------------------------------------------------------------------


@dataclass(frozen=True)
class DriftPoint:
    usage_fraction_of_reference: float
    work_channel_credits: float
    alpha_with_fixed_fund: float
    fund_credits_to_hold_target: float
    availability_only_income_credits: float


def s3_alpha_drift() -> list[DriftPoint]:
    """With ``F`` fixed, ``alpha = F/(F+W)`` falls as usage grows.

    At genesis ``W`` is near zero, so ``alpha`` is near one: the network is most
    exposed exactly when it is smallest.
    """

    F = R.REWARD.existence_fund_microtokens_per_epoch
    E = R.REFERENCE_PRESENT_NODES
    out: list[DriftPoint] = []
    for frac in (0.0, 0.01, 0.05, 0.10, 0.25, 0.50, 1.00, 2.00, 5.00):
        W = int(R.REFERENCE_WORK_CHANNEL_MICROTOKENS * frac)
        alpha = F / (F + W) if (F + W) else 1.0
        target = fund_for_target_alpha(
            Fraction(R.ALPHA_TARGET).limit_denominator(1000), W
        )
        out.append(
            DriftPoint(
                usage_fraction_of_reference=frac,
                work_channel_credits=W / MICROTOKENS_PER_CREDIT,
                alpha_with_fixed_fund=alpha,
                fund_credits_to_hold_target=target / MICROTOKENS_PER_CREDIT,
                availability_only_income_credits=(F // E) / MICROTOKENS_PER_CREDIT,
            )
        )
    return out


# --------------------------------------------------------------------------
# S4 — AT-07 verdict
# --------------------------------------------------------------------------


@dataclass(frozen=True)
class At07Result:
    honest_nodes: int
    fleet_nodes: int
    total_emission_without_fleet: int
    total_emission_with_fleet: int
    fleet_share_pct: float
    fleet_storage_compute_credits: int
    fleet_validator_seats: int
    honest_income_without_fleet: int
    honest_income_with_fleet: int
    x_declared_pct: float

    @property
    def criterion_a(self) -> bool:
        return self.total_emission_with_fleet <= self.total_emission_without_fleet

    @property
    def criterion_b(self) -> bool:
        return self.fleet_share_pct <= self.x_declared_pct

    @property
    def criterion_c(self) -> bool:
        return self.fleet_storage_compute_credits == 0

    @property
    def criterion_d(self) -> bool:
        return self.fleet_validator_seats == 0

    @property
    def passed(self) -> bool:
        return self.criterion_a and self.criterion_b and self.criterion_c and self.criterion_d


def s4_at07(honest: int = 100, fleet: int = 10_000) -> At07Result:
    """`AT-07`: H >= 100 real honest nodes, N = 10 000 emulated on one host."""

    alpha = Fraction(R.ALPHA_TARGET).limit_denominator(1000)
    W = R.REFERENCE_WORK_CHANNEL_MICROTOKENS
    F = fund_for_target_alpha(alpha, W)
    contributors = max(1, int(honest * R.REFERENCE_CONTRIBUTOR_FRACTION))

    without = run_epoch(
        NodePopulation(honest - contributors, contributors, 0),
        F,
        WorkChannel(total_microtokens=W),
        availability_microtokens_per_unit=R.REWARD.availability_microtokens_per_unit,
        availability_units_per_node=1,
    )
    with_fleet = run_epoch(
        NodePopulation(honest - contributors, contributors, fleet),
        F,
        WorkChannel(total_microtokens=W),
        availability_microtokens_per_unit=R.REWARD.availability_microtokens_per_unit,
        availability_units_per_node=1,
    )
    return At07Result(
        honest_nodes=honest,
        fleet_nodes=fleet,
        total_emission_without_fleet=without.total_minted,
        total_emission_with_fleet=with_fleet.total_minted,
        fleet_share_pct=100.0 * float(with_fleet.captured_share),
        fleet_storage_compute_credits=0,  # the fleet earns nothing in these channels
        fleet_validator_seats=0,  # established by S6, not asserted here
        honest_income_without_fleet=without.honest_availability_only_income,
        honest_income_with_fleet=with_fleet.honest_availability_only_income,
        x_declared_pct=100.0 * R.X_DECLARED,
    )


def s4b_availability_channel_breaks_criterion_a(rate_per_unit: int = 1_000) -> At07Result:
    """Counter-example: the same test with a non-zero availability work rate."""

    alpha = Fraction(R.ALPHA_TARGET).limit_denominator(1000)
    W = R.REFERENCE_WORK_CHANNEL_MICROTOKENS
    F = fund_for_target_alpha(alpha, W)
    honest, fleet = 100, 10_000
    contributors = max(1, int(honest * R.REFERENCE_CONTRIBUTOR_FRACTION))
    without = run_epoch(
        NodePopulation(honest - contributors, contributors, 0),
        F,
        WorkChannel(total_microtokens=W),
        availability_microtokens_per_unit=rate_per_unit,
        availability_units_per_node=1_000_000,
    )
    with_fleet = run_epoch(
        NodePopulation(honest - contributors, contributors, fleet),
        F,
        WorkChannel(total_microtokens=W),
        availability_microtokens_per_unit=rate_per_unit,
        availability_units_per_node=1_000_000,
    )
    return At07Result(
        honest_nodes=honest,
        fleet_nodes=fleet,
        total_emission_without_fleet=without.total_minted,
        total_emission_with_fleet=with_fleet.total_minted,
        fleet_share_pct=100.0 * float(with_fleet.captured_share),
        fleet_storage_compute_credits=0,
        fleet_validator_seats=0,
        honest_income_without_fleet=without.honest_availability_only_income,
        honest_income_with_fleet=with_fleet.honest_availability_only_income,
        x_declared_pct=100.0 * R.X_DECLARED,
    )


# --------------------------------------------------------------------------
# S5 — the constraint block, and the two couplings verified by brute force
# --------------------------------------------------------------------------


def s5_constraint_block(ps: ParameterSet = R.RECOMMENDED):
    return check_constraint_block(ps)


def s5b_term_limit_bruteforce(v_max: int = 399, t_max: int = 16) -> list[tuple[int, bool, int | None]]:
    """For each ``T``, is any set size satisfiable at ``m = 1``?

    ``ledger.md`` claims ``T <= 3`` is unsatisfiable at every set size. The
    project fixture fell into that trap once, so the claim is executed here
    rather than quoted.
    """

    return [(T, *term_limit_satisfiable(T, 1, v_max)) for T in range(1, t_max + 1)]


def s5c_horizon_coupling(m_max: int = 6, v_max: int = 399) -> list[tuple[int, int | None]]:
    """Smallest ``T`` that admits any ``V`` for a declared horizon ``m``."""

    out: list[tuple[int, int | None]] = []
    for m in range(1, m_max + 1):
        smallest: int | None = None
        for T in range(1, 64):
            ok, _ = term_limit_satisfiable(T, m, v_max)
            if ok:
                smallest = T
                break
        out.append((m, smallest))
    return out


def s5d_ratchet_still_valid() -> list:
    """After the [DEBT-010] ratchet pushes ``T`` to its genesis ceiling, does the
    combination still satisfy the constraint block?"""

    pushed = replace(
        R.CONSENSUS,
        validator_max_consecutive_terms=R.BOUNDS.validator_max_consecutive_terms_max,
    )
    return check_constraint_block(replace(R.RECOMMENDED, consensus=pushed))


# --------------------------------------------------------------------------
# S6 — AT-10, three configurations
# --------------------------------------------------------------------------


@dataclass(frozen=True)
class At10Grinding:
    ratio_label: str
    adversary_candidates: int
    honest_candidates: int
    grinding_attempts: int
    epochs_to_one_third: int | None
    epochs_to_one_third_no_grinding: int | None
    adversary_fills: int
    adversary_fills_no_grinding: int
    final_adversary_seats: int
    final_member_count: int
    stalled_at: int | None
    admission_horizon: int


def _run_epochs(
    p: ConsensusParameters,
    epochs: int,
    honest_pool: list[CandidateNode],
    adversary_pool: list[CandidateNode],
    grinding_attempts: int = 1,
    adversary_genesis_seats: int = 0,
) -> tuple[ElectionState, int | None, int | None, int]:
    """Run ``epochs`` boundaries. Returns (state, first epoch at one third,
    stall epoch, seats the adversary won by filling)."""

    state = ElectionState(
        p=p,
        chain_id=CHAIN_ID,
        current=genesis_set(p, adversary_seats=adversary_genesis_seats),
    )
    reached: int | None = None
    stalled: int | None = None
    adversary_fills = 0
    for e in range(1, epochs + 1):
        applicants = list(honest_pool) + list(adversary_pool)
        # Incumbents re-file: a seated node is in the applicant list under its
        # own id, so seed the pool with candidacies for the sitting members.
        seated = {s.node_id for s in state.current.seats}
        known = {n.node_id for n in applicants}
        for sid in seated:
            if sid not in known:
                seat = next(s for s in state.current.seats if s.node_id == sid)
                applicants.append(
                    CandidateNode(
                        node_id=sid,
                        faction=seat.faction,
                        contribution_score=10**9,
                        distinct_issuers=64,
                    )
                )
        before = state.current.adversary_count
        # Grinding requires proposing the last block of the entropy window, and
        # only a sitting validator proposes. The adversary therefore gets its G
        # resamples only at boundaries where it holds that slot, which it does
        # with probability adversary_seats / member_count. The draw is a hash of
        # the epoch, so it is deterministic and reproducible.
        can_grind = (
            grinding_attempts > 1
            and before > 0
            and _slot_draw(e, state.current.member_count) < before
        )
        try:
            derive_boundary(
                state,
                e,
                applicants,
                R.REWARD.validator_eligibility_threshold_units,
                R.REWARD.validator_eligibility_min_issuers,
                entropy_ids(CHAIN_ID, e, ENTROPY_WINDOW_SAMPLE),
                grinding_attempts=grinding_attempts if can_grind else 1,
                grinder_faction="adversary" if can_grind else None,
            )
        except ChainStalled:
            stalled = e
            break
        after = state.current.adversary_count
        adversary_fills += max(0, after - before)
        if reached is None and state.current.adversary_holds_third():
            reached = e
    return state, reached, stalled, adversary_fills


def _slot_draw(epoch: int, member_count: int) -> int:
    """Deterministic stand-in for "who proposes the last entropy block"."""

    import hashlib

    digest = hashlib.sha256(f"{SEED}|proposal-slot|{epoch}".encode("utf-8")).digest()
    return int.from_bytes(digest[:4], "big") % max(1, member_count)


def s6a_at10_grinding(epochs: int = 50) -> list[At10Grinding]:
    """Configuration 1: seed grinding by the proposer of the last entropy block.

    The measurement that matters is not "does the adversary reach one third" —
    an adversary that dominates the fill pool reaches it by winning an honest
    lottery, and the rule never claimed otherwise. It is whether **grinding**
    adds anything beyond the churn cap. The baseline column is the same run with
    a single seed sample.
    """

    p = R.CONSENSUS
    profile = ContributorProfile()
    out: list[At10Grinding] = []
    honest_total = 120
    for label, ratio in (("N/H = 0,1", 0.1), ("N/H = 1", 1.0), ("N/H = 10", 10.0)):
        honest_pool = build_contributors(
            honest_total,
            R.REWARD.validator_eligibility_window_epochs,
            profile,
            SEED,
        )
        honest_pool = [
            n
            for n in honest_pool
            if n.contribution_score >= R.REWARD.validator_eligibility_threshold_units
            and n.distinct_issuers >= R.REWARD.validator_eligibility_min_issuers
        ]
        adv_count = max(1, int(len(honest_pool) * ratio))
        adversary_pool = build_adversary_candidates(
            adv_count,
            R.REWARD.validator_eligibility_threshold_units,
            R.REWARD.validator_eligibility_min_issuers,
        )
        base_state, base_reached, _, base_fills = _run_epochs(
            p, epochs, honest_pool, adversary_pool, grinding_attempts=1
        )
        grind_state, grind_reached, grind_stalled, grind_fills = _run_epochs(
            p, epochs, honest_pool, adversary_pool, grinding_attempts=GRINDING_ATTEMPTS
        )
        out.append(
            At10Grinding(
                ratio_label=label,
                adversary_candidates=adv_count,
                honest_candidates=len(honest_pool),
                grinding_attempts=GRINDING_ATTEMPTS,
                epochs_to_one_third=grind_reached,
                epochs_to_one_third_no_grinding=base_reached,
                adversary_fills=grind_fills,
                adversary_fills_no_grinding=base_fills,
                final_adversary_seats=grind_state.current.adversary_count,
                final_member_count=grind_state.current.member_count,
                stalled_at=grind_stalled,
                admission_horizon=boundaries_to_admission_capture(p.V, p.c),
            )
        )
    return out


@dataclass(frozen=True)
class At10Censorship:
    coalition_seats: int
    variant: str
    boundaries: list[int]
    outcome: str
    boundaries_used: int
    predicted_continuous: int


def s6b_at10_total_censorship(coalition: int = 10) -> At10Censorship:
    """Configuration 2a: the coalition censors every other candidacy."""

    p = R.CONSENSUS
    state = ElectionState(p=p, chain_id=CHAIN_ID, current=genesis_set(p, adversary_seats=coalition))
    sizes: list[int] = [state.current.member_count]
    outcome = ""
    used = 0
    for e in range(1, 11):
        # Only the coalition's own candidacies are finalized.
        applicants = [
            CandidateNode(s.node_id, "adversary", 10**9, 64)
            for s in state.current.seats
            if s.faction == "adversary" and e < s.term_expiry_epoch
        ]
        try:
            derive_boundary(
                state,
                e,
                applicants,
                R.REWARD.validator_eligibility_threshold_units,
                R.REWARD.validator_eligibility_min_issuers,
                entropy_ids(CHAIN_ID, e, ENTROPY_WINDOW_SAMPLE),
            )
        except ChainStalled as exc:
            outcome = f"chain stalled at boundary {e}: {exc.reason}"
            used = e
            break
        sizes.append(state.current.member_count)
    else:
        outcome = "no stall within 10 boundaries"
        used = 10
    return At10Censorship(
        coalition_seats=coalition,
        variant="2a total censorship",
        boundaries=sizes,
        outcome=outcome,
        boundaries_used=used,
        predicted_continuous=boundaries_to_attrition_capture(p.V, coalition),
    )


def s6c_at10_selective_censorship(
    coalition: int,
    V: int | None = None,
    min_set: int | None = None,
) -> At10Censorship:
    """Configuration 2b: the coalition lets through exactly the honest
    candidacies that land the set on the smallest size the floor permits."""

    p = R.CONSENSUS
    V = p.V if V is None else V
    min_set = p.validator_min_set_size if min_set is None else min_set
    sizes = [V]
    prev = V
    used = 0
    outcome = ""
    for b in range(1, 21):
        smallest_lawful = max(
            (2 * prev) // 3 + 1,  # strict contraction floor
            min_set,
        )
        new = max(smallest_lawful, coalition)
        if new >= prev:
            outcome = (
                f"pinned at {prev}: the contraction floor and validator_min_set_size "
                f"forbid a smaller set. The coalition holds {coalition} of {prev} "
                f"({100.0 * coalition / prev:.1f} %) and never obtains the whole set"
            )
            used = b - 1
            break
        prev = new
        sizes.append(new)
        used = b
        if new == coalition:
            outcome = f"coalition holds the whole set after {b} boundaries"
            break
    else:
        outcome = "not converged within 20 boundaries"
    if not outcome:
        outcome = f"stalled/pinned at {prev}"
    return At10Censorship(
        coalition_seats=coalition,
        variant="2b selective censorship",
        boundaries=sizes,
        outcome=outcome,
        boundaries_used=used,
        predicted_continuous=boundaries_to_attrition_capture(V, coalition),
    )


@dataclass(frozen=True)
class CooldownEvasion:
    voluntary_exit_absence: int
    term_expiry_absence: int
    parameter: int


def s6d_cooldown_evasion() -> CooldownEvasion:
    """Configuration 3: an incumbent leaves one epoch early on purpose.

    Under eligibility condition 5 in the form "left a seat for any reason
    whatsoever" the measured absence must equal ``validator_cooldown_epochs``
    for both departures. The comparison is the point of the test.
    """

    p = R.CONSENSUS
    state = ElectionState(p=p, chain_id=CHAIN_ID, current=genesis_set(p))
    state.left_seat_at["voluntary"] = 5
    state.left_seat_at["expired"] = 5
    voluntary = 0
    expired = 0
    for e in range(6, 6 + 4 * p.validator_cooldown_epochs + 4):
        if state.in_cooldown("voluntary", e):
            voluntary += 1
        if state.in_cooldown("expired", e):
            expired += 1
    return CooldownEvasion(
        voluntary_exit_absence=voluntary,
        term_expiry_absence=expired,
        parameter=p.validator_cooldown_epochs,
    )


# --------------------------------------------------------------------------
# S7 — the three couplings simulated together, and the no-adversary stall
# --------------------------------------------------------------------------


@dataclass(frozen=True)
class DroughtResult:
    cooldown: int
    filing_candidates_per_boundary: int
    boundaries_survived: int
    stall_reason: str
    final_member_count: int


def s7_drought(
    pool_size: int,
    cooldown: int | None = None,
    epochs: int = 30,
) -> DroughtResult:
    """No adversary at all. Only a finite pool of nodes willing to stand.

    The pool is **fixed**: the same nodes file every epoch they are allowed to.
    That is what makes cooldown bite — a supply of brand-new nodes at every
    boundary would never meet it, and would hide the coupling this scenario
    exists to measure.
    """

    p = R.CONSENSUS if cooldown is None else replace(R.CONSENSUS, validator_cooldown_epochs=cooldown)
    state = ElectionState(p=p, chain_id=CHAIN_ID, current=genesis_set(p))
    standing = [
        CandidateNode(f"pool-{i:04d}", "honest", 10**9, 64) for i in range(pool_size)
    ]
    reason = ""
    survived = 0
    for e in range(1, epochs + 1):
        applicants = list(standing)
        for s_ in state.current.seats:
            if e < s_.term_expiry_epoch and not any(
                a.node_id == s_.node_id for a in applicants
            ):
                applicants.append(CandidateNode(s_.node_id, "honest", 10**9, 64))
        try:
            derive_boundary(
                state,
                e,
                applicants,
                R.REWARD.validator_eligibility_threshold_units,
                R.REWARD.validator_eligibility_min_issuers,
                entropy_ids(CHAIN_ID, e, ENTROPY_WINDOW_SAMPLE),
            )
        except ChainStalled as exc:
            reason = exc.reason
            break
        survived = e
    return DroughtResult(
        cooldown=p.validator_cooldown_epochs,
        filing_candidates_per_boundary=pool_size,
        boundaries_survived=survived,
        stall_reason=reason or f"no stall within {epochs} boundaries",
        final_member_count=state.current.member_count,
    )


@dataclass(frozen=True)
class PoolResult:
    threshold_units: int
    contributors: int
    eligible: int
    minimum_pool_required: int
    willingness_needed_pct: float


def s7b_eligibility_pool(thresholds: tuple[int, ...]) -> list[PoolResult]:
    profile = ContributorProfile()
    contributors = int(R.REFERENCE_PRESENT_NODES * R.REFERENCE_CONTRIBUTOR_FRACTION)
    pool = build_contributors(
        contributors, R.REWARD.validator_eligibility_window_epochs, profile, SEED
    )
    need = minimum_pool_for_sustained_set(R.CONSENSUS)
    out: list[PoolResult] = []
    for t in thresholds:
        eligible = sum(
            1
            for n in pool
            if n.contribution_score >= t
            and n.distinct_issuers >= R.REWARD.validator_eligibility_min_issuers
        )
        out.append(
            PoolResult(
                threshold_units=t,
                contributors=contributors,
                eligible=eligible,
                minimum_pool_required=need,
                willingness_needed_pct=(100.0 * need / eligible) if eligible else float("inf"),
            )
        )
    return out


@dataclass(frozen=True)
class TermToleranceResult:
    term_limit: int
    seats_vacated_per_boundary: float
    minimum_pool: int
    feasible_c: tuple[int, ...]


def s7c_term_limit_tolerance(term_limits: tuple[int, ...]) -> list[TermToleranceResult]:
    """[DEBT-010]: how much relief does raising ``T`` toward its ceiling buy?"""

    p = R.CONSENSUS
    out: list[TermToleranceResult] = []
    for T in term_limits:
        cand = replace(p, validator_max_consecutive_terms=T)
        out.append(
            TermToleranceResult(
                term_limit=T,
                seats_vacated_per_boundary=p.V / T,
                minimum_pool=minimum_pool_for_sustained_set(cand),
                feasible_c=tuple(feasible_c_values(p.V, T, p.m)),
            )
        )
    return out


# --------------------------------------------------------------------------
# S8 — SEC-REQ-16 (b): the reputation-purchase margin
# --------------------------------------------------------------------------


def s8_reputation_margin(subscription_price_microtokens: int) -> ReputationMargin:
    alpha = Fraction(R.ALPHA_TARGET).limit_denominator(1000)
    F = fund_for_target_alpha(alpha, R.REFERENCE_WORK_CHANNEL_MICROTOKENS)
    per_epoch = F // R.REFERENCE_PRESENT_NODES
    epochs_per_period = 30  # a 30-day subscription period against a one-day epoch
    return ReputationMargin(
        existence_income_per_period=per_epoch * epochs_per_period,
        subscription_price_per_period=subscription_price_microtokens,
        cap_numerator=R.REWARD.publisher_reward_cap_numerator,
        cap_denominator=R.REWARD.publisher_reward_cap_denominator,
    )


# --------------------------------------------------------------------------
# S9 / S10 — what governance may still do to these values (REVIEW-011)
# --------------------------------------------------------------------------


def s9_legal_intervals() -> list:
    """Which of the recommended values a lawful next document may still move."""

    return legal_next_intervals(R.RECOMMENDED)


def s9b_max_reachable_v() -> int:
    return max_reachable_target_set_size(R.RECOMMENDED)


@dataclass(frozen=True)
class ErosionStep:
    document: int
    V: int
    T: int
    min_set: int
    constraint_block_passes: bool
    min_set_over_V: float
    attrition_threshold_seats: int


def s10_min_set_ratio_erosion() -> list[ErosionStep]:
    """`min_set / V` is preserved by no rule, and the ratio is what the
    anti-attrition property depends on.

    The path is the one REVIEW-011 RF-003 identifies: raise `V` and `T`
    together, each step inside the 5/4 rate limit and the monotonic term rule,
    leaving `min_set` where it is because nothing requires it to follow.
    """

    steps = [(27, 9), (33, 11), (36, 12)]
    out: list[ErosionStep] = []
    previous: ConsensusParameters | None = None
    for i, (V, T) in enumerate(steps):
        cand = replace(
            R.CONSENSUS,
            validator_target_set_size=V,
            validator_max_consecutive_terms=T,
        )
        ps = replace(R.RECOMMENDED, consensus=cand)
        if previous is None:
            results = check_constraint_block(ps)
        else:
            results = check_constraint_block(
                ps,
                active=previous,
                active_activation_height=0,
                new_activation_height=R.BOUNDS.election_parameter_min_activation_gap_blocks,
            )
        out.append(
            ErosionStep(
                document=i,
                V=V,
                T=T,
                min_set=cand.validator_min_set_size,
                constraint_block_passes=constraint_block_passes(results),
                min_set_over_V=cand.validator_min_set_size / V,
                attrition_threshold_seats=max(
                    cand.validator_min_set_size, V // 3 + 1
                ),
            )
        )
        previous = cand
    return out


def s10b_censorship_at_eroded_ratio() -> list[At10Censorship]:
    """Selective censorship once `V` has grown to its permanent ceiling."""

    V = 36
    min_set = R.CONSENSUS.validator_min_set_size
    return [
        s6c_at10_selective_censorship(k, V=V, min_set=min_set)
        for k in (13, 18, 23, 24)
    ]


# --------------------------------------------------------------------------
# S11 — AT-07 in the regime it will actually be run in (REVIEW-011 RF-002)
# --------------------------------------------------------------------------


@dataclass(frozen=True)
class LaunchRegimeResult:
    honest: int
    fleet: int
    fund_microtokens: int
    work_channel_microtokens: int
    observed_alpha: float
    fleet_share_pct: float
    absolute_diverted_microtokens: int
    x_declared_pct: float

    @property
    def violates_x_as_written(self) -> bool:
        return self.fleet_share_pct > self.x_declared_pct


def s11_at07_launch_regime(
    honest: int = 100,
    fleet: int = 10_000,
    usage_fraction: float = 0.0,
    fund_microtokens: int | None = None,
) -> LaunchRegimeResult:
    """`AT-07` with the work channel where it is on day one, not where the
    reference regime puts it.

    `AT-07` is scheduled on a devnet. A devnet has no usage, so `W` is near
    zero, so `alpha` is near one whatever `F` is, so criterion (c) as literally
    written is violated by about five times. The ratio is not the honest
    quantity there; the absolute diverted amount is.
    """

    F = (
        R.REWARD.existence_fund_microtokens_per_epoch
        if fund_microtokens is None
        else fund_microtokens
    )
    W = int(R.REFERENCE_WORK_CHANNEL_MICROTOKENS * usage_fraction)
    contributors = max(1, int(honest * R.REFERENCE_CONTRIBUTOR_FRACTION))
    res = run_epoch(
        NodePopulation(honest - contributors, contributors, fleet),
        F,
        WorkChannel(total_microtokens=W),
    )
    return LaunchRegimeResult(
        honest=honest,
        fleet=fleet,
        fund_microtokens=F,
        work_channel_microtokens=W,
        observed_alpha=float(res.alpha),
        fleet_share_pct=100.0 * float(res.captured_share),
        absolute_diverted_microtokens=res.fleet_minted,
        x_declared_pct=100.0 * R.X_DECLARED,
    )


def s11b_usage_ramp() -> list[LaunchRegimeResult]:
    return [
        s11_at07_launch_regime(usage_fraction=f)
        for f in (0.0, 0.05, 0.10, 0.25, 0.50, 1.00)
    ]


# --------------------------------------------------------------------------
# S12 — possession is not control (REVIEW-014 RF-001)
# --------------------------------------------------------------------------


@dataclass(frozen=True)
class QuorumCaptureRow:
    V: int
    min_set: int
    smallest_lawful_successor: int
    k_min_for_quorum: int
    k_min_fraction: float
    k_min_for_possession: int
    four_ninths: float


def smallest_lawful_successor(V: int, min_set: int) -> int:
    """The smallest `S_new` a lawful contraction may produce from `S_old = V`.

    The contraction floor is **strict** — `3 * new > 2 * old` — so a set of 27
    contracts to 19 and not to 18. `validator_min_set_size` floors it further.
    """

    return max((2 * V) // 3 + 1, min_set)


def relational_min_set(V: int) -> int:
    """`ceil(2V/3)`, the smallest `min_set` the [ADR-010] relational rule allows."""

    return -(-2 * V // 3)


def quorum_capture_threshold(V: int, min_set: int | None = None) -> QuorumCaptureRow:
    """Smallest coalition that reaches **quorum** on the contracted set.

    Possession of every seat needs `k >= min_set`. Control needs only a quorum
    of the successor, `3k > 2 * S_new`, and `S_new` is smaller than `V`. The
    coalition must additionally be able to censor at all, which needs
    `3k > V` — it has to withhold quorum from the outgoing set.
    """

    ms = relational_min_set(V) if min_set is None else min_set
    S = smallest_lawful_successor(V, ms)
    k_quorum = max((2 * S) // 3 + 1, V // 3 + 1)
    return QuorumCaptureRow(
        V=V,
        min_set=ms,
        smallest_lawful_successor=S,
        k_min_for_quorum=k_quorum,
        k_min_fraction=k_quorum / V,
        k_min_for_possession=ms,
        four_ninths=4 * V / 9,
    )


def s12_quorum_capture_table(
    sizes: tuple[int, ...] = (12, 27, 36, 60, 120, 600, 6000),
) -> list[QuorumCaptureRow]:
    return [quorum_capture_threshold(V) for V in sizes]


@dataclass(frozen=True)
class QuorumWalkStep:
    boundary: int
    V_target: int
    min_set: int
    set_size: int
    coalition_seats: int
    has_quorum: bool
    owns_set: bool
    note: str


def s12b_quorum_capture_walk(V: int = 27, coalition: int = 13) -> list[QuorumWalkStep]:
    """The concrete walk of RF-001 on the recommended values.

    Step 0 is the honest set. At each boundary the coalition lets exactly enough
    honest candidacies through to land on the smallest lawful successor, then —
    once it holds quorum — lowers `V` and `min_set` together within the 5/4 rate
    limit, which the relational rule permits because it constrains the ratio and
    not the magnitude.
    """

    steps: list[QuorumWalkStep] = []
    Vt = V
    ms = relational_min_set(Vt)
    size = V
    steps.append(
        QuorumWalkStep(
            boundary=0,
            V_target=Vt,
            min_set=ms,
            set_size=size,
            coalition_seats=coalition,
            has_quorum=3 * coalition > 2 * size,
            owns_set=coalition >= size,
            note="honest set at genesis size",
        )
    )
    for b in range(1, 6):
        target = smallest_lawful_successor(size, ms)
        new_size = max(target, coalition)
        if new_size >= size:
            steps.append(
                QuorumWalkStep(
                    b, Vt, ms, size, coalition,
                    3 * coalition > 2 * size, coalition >= size,
                    "pinned: no smaller lawful successor",
                )
            )
            break
        # the contraction itself
        size = new_size
        has_q = 3 * coalition > 2 * size
        note = "contraction under the floor and min_set"
        if has_q and coalition < size:
            note += "; QUORUM REACHED without owning the set"
        steps.append(
            QuorumWalkStep(b, Vt, ms, size, coalition, has_q, coalition >= size, note)
        )
        if coalition >= size:
            break
        if has_q:
            # With quorum it may sign parameter documents: lower V within 5/4,
            # and min_set with it, keeping 3 * min_set >= 2 * V satisfied.
            new_V = max(coalition, -(-Vt * 4 // 5))
            if new_V < Vt:
                Vt = new_V
                ms = relational_min_set(Vt)
    return steps
