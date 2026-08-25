"""Execute the rejection cases for the validity rules of [ADR-010] and [ADR-011].

Versioned because [REVIEW-014] RF-007 found `GATE-RULES-REJECT` resting on a
script that was never committed, and because the gate's own justification is
that a validity rule whose rejection is not exhibited is a recommendation with a
different name. Every case below is one row of a conformance table in
`docs/protocol/README.md` or `docs/protocol/ledger.md`.

    python tools/reward_rules.py

Exit code is non-zero if any case does not produce its documented verdict.
"""

from __future__ import annotations

from dataclasses import dataclass


# --------------------------------------------------------------------------
# The rules, as acceptance predicates
# --------------------------------------------------------------------------


@dataclass(frozen=True)
class RewardBounds:
    """Illustrative genesis magnitudes. The shape is normative, not the values."""

    existence_fund_microtokens_per_epoch_max: int = 15_882_352_941
    reward_epoch_ms_min: int = 3_600_000  # one hour
    reward_epoch_ms_max: int = 604_800_000  # one week
    publisher_reward_cap_numerator_max: int = 1
    publisher_reward_cap_denominator_min: int = 2
    validator_eligibility_threshold_units_min: int = 512
    validator_eligibility_window_epochs_max: int = 90
    validator_eligibility_min_issuers_min: int = 2
    storage_units_per_contribution_unit_max: int = 1_073_741_824
    compute_units_per_contribution_unit_max: int = 1_000_000
    storage_microtokens_per_byte_epoch_min: int = 1
    compute_microtokens_per_million_fuel_min: int = 1
    reward_parameter_change_numerator: int = 5
    reward_parameter_change_denominator: int = 4
    reward_parameter_min_activation_gap_blocks: int = 120_960


def accept_reward_policy(
    body: dict[str, int],
    bounds: RewardBounds,
    active: dict[str, int] | None = None,
    active_activation_height: int | None = None,
    activation_height: int | None = None,
) -> tuple[bool, str]:
    """The acceptance predicate for a `reward_policy` document."""

    b, B = body, bounds

    # Rule 1 - the availability tariff is zero ([ADR-010]).
    if b["availability_microtokens_per_unit"] != 0:
        return False, "availability tariff must be zero"

    # The creator-share cap, pre-existing.
    if not (
        b["publisher_reward_cap_denominator"] > 0
        and b["publisher_reward_cap_numerator"] < b["publisher_reward_cap_denominator"]
    ):
        return False, "creator-share cap not strictly lossy"

    # Rule 2 - RewardBounds magnitudes.
    checks = [
        (
            b["existence_fund_microtokens_per_epoch"]
            <= B.existence_fund_microtokens_per_epoch_max,
            "above the existence fund ceiling",
        ),
        (
            b["reward_epoch_ms"] >= B.reward_epoch_ms_min,
            "epoch below the floor inflates real issuance",
        ),
        (
            b["reward_epoch_ms"] <= B.reward_epoch_ms_max,
            "epoch above the ceiling freezes issuance",
        ),
        (
            b["validator_eligibility_threshold_units"]
            >= B.validator_eligibility_threshold_units_min,
            "eligibility threshold below the floor",
        ),
        (
            b["validator_eligibility_window_epochs"]
            <= B.validator_eligibility_window_epochs_max,
            "window above the ceiling drives the required rate toward zero",
        ),
        (
            b["validator_eligibility_min_issuers"]
            >= B.validator_eligibility_min_issuers_min,
            "issuer diversity below the floor",
        ),
        (
            b["storage_units_per_contribution_unit"]
            <= B.storage_units_per_contribution_unit_max,
            "redenominates the eligibility unit",
        ),
        (
            b["compute_units_per_contribution_unit"]
            <= B.compute_units_per_contribution_unit_max,
            "redenominates the eligibility unit",
        ),
        (
            b["storage_microtokens_per_byte_epoch"]
            >= B.storage_microtokens_per_byte_epoch_min,
            "empties the denominator of the surveilled ratio",
        ),
        (
            b["compute_microtokens_per_million_fuel"]
            >= B.compute_microtokens_per_million_fuel_min,
            "empties the denominator of the surveilled ratio",
        ),
        (
            b["publisher_reward_cap_numerator"] <= B.publisher_reward_cap_numerator_max,
            "creator-share numerator above the ceiling",
        ),
        (
            b["publisher_reward_cap_denominator"]
            >= B.publisher_reward_cap_denominator_min,
            "creator-share denominator below the floor",
        ),
    ]
    for ok, why in checks:
        if not ok:
            return False, why

    # Rule 3 - rate of change and activation spacing, against the active document.
    if active is not None:
        num = B.reward_parameter_change_numerator
        den = B.reward_parameter_change_denominator
        for key, new in b.items():
            old = active[key]
            if not (new * den <= old * num and old * den <= new * num):
                return False, f"rate of change exceeded on {key}"
        if activation_height is None or active_activation_height is None:
            return False, "missing activation heights"
        if (
            activation_height
            < active_activation_height + B.reward_parameter_min_activation_gap_blocks
        ):
            return False, "activation gap not respected"
    return True, "accepted"


def accept_consensus_min_set(V: int, min_set: int) -> tuple[bool, str]:
    """The relational rule of [ADR-010] on `consensus_parameters`."""

    if not 0 < min_set <= V:
        return False, "min_set outside (0, V]"
    if 3 * min_set < 2 * V:
        return False, "3 * min_set < 2 * V"
    return True, "accepted"


# --------------------------------------------------------------------------
# The cases, mirroring the published tables
# --------------------------------------------------------------------------

B = RewardBounds()

BASE = {
    "reward_epoch_ms": 86_400_000,
    "existence_fund_microtokens_per_epoch": 300_000_000,
    "availability_microtokens_per_unit": 0,
    "storage_microtokens_per_byte_epoch": 1,
    "compute_microtokens_per_million_fuel": 1,
    "publisher_microtokens_per_active_subscriber": 1,
    "publisher_reward_cap_numerator": 1,
    "publisher_reward_cap_denominator": 2,
    "storage_units_per_contribution_unit": 1_073_741_824,
    "compute_units_per_contribution_unit": 1_000_000,
    "validator_eligibility_threshold_units": 512,
    "validator_eligibility_window_epochs": 28,
    "validator_eligibility_min_issuers": 3,
}


def variant(**over) -> dict[str, int]:
    d = dict(BASE)
    d.update(over)
    return d


CASES: list[tuple[str, dict, bool]] = [
    ("availability tariff 0", variant(), True),
    ("availability tariff 1", variant(availability_microtokens_per_unit=1), False),
    ("availability tariff 1000", variant(availability_microtokens_per_unit=1000), False),
    ("creator cap 1/2", variant(), True),
    (
        "creator cap 2/2",
        variant(publisher_reward_cap_numerator=2, publisher_reward_cap_denominator=2),
        False,
    ),
    ("creator cap 1/0", variant(publisher_reward_cap_denominator=0), False),
    (
        "F exactly at the ceiling",
        variant(
            existence_fund_microtokens_per_epoch=B.existence_fund_microtokens_per_epoch_max
        ),
        True,
    ),
    (
        "F one above the ceiling",
        variant(
            existence_fund_microtokens_per_epoch=B.existence_fund_microtokens_per_epoch_max
            + 1
        ),
        False,
    ),
    ("epoch exactly at the floor", variant(reward_epoch_ms=B.reward_epoch_ms_min), True),
    (
        "epoch one below the floor",
        variant(reward_epoch_ms=B.reward_epoch_ms_min - 1),
        False,
    ),
    ("epoch of 86 400 ms (the x1000 attack)", variant(reward_epoch_ms=86_400), False),
    (
        "epoch one above the ceiling",
        variant(reward_epoch_ms=B.reward_epoch_ms_max + 1),
        False,
    ),
    ("storage divisor at the ceiling", variant(), True),
    (
        "storage divisor x 10^6",
        variant(storage_units_per_contribution_unit=1_073_741_824 * 10**6),
        False,
    ),
    (
        "compute divisor above the ceiling",
        variant(compute_units_per_contribution_unit=1_000_001),
        False,
    ),
    (
        "window at the ceiling",
        variant(
            validator_eligibility_window_epochs=B.validator_eligibility_window_epochs_max
        ),
        True,
    ),
    (
        "window of 3000 epochs",
        variant(validator_eligibility_window_epochs=3000),
        False,
    ),
    ("storage tariff at the floor", variant(storage_microtokens_per_byte_epoch=1), True),
    ("storage tariff zero", variant(storage_microtokens_per_byte_epoch=0), False),
    ("compute tariff zero", variant(compute_microtokens_per_million_fuel=0), False),
    ("threshold at the floor", variant(validator_eligibility_threshold_units=512), True),
    (
        "threshold below the floor",
        variant(validator_eligibility_threshold_units=511),
        False,
    ),
]

RATE_CASES: list[tuple[str, dict, int, bool]] = [
    (
        "F at exactly 5/4",
        variant(existence_fund_microtokens_per_epoch=375_000_000),
        120_960,
        True,
    ),
    (
        "F one above 5/4",
        variant(existence_fund_microtokens_per_epoch=375_000_001),
        120_960,
        False,
    ),
    (
        "epoch 86 400 000 -> 86 400 in one document",
        variant(reward_epoch_ms=86_400),
        120_960,
        False,
    ),
    ("activation exactly at the gap", variant(), 120_960, True),
    ("activation one block short", variant(), 120_959, False),
]

# --------------------------------------------------------------------------
# The rate of change, swept in both directions ([REVIEW-017] RF-002)
#
# The published tables carry one row that names a downward move, and that row is
# rejected by the `reward_epoch_ms` floor before the ratio is ever reached. The
# consequence measured in [REVIEW-017]: deleting `old * den <= new * num` from
# this file and from `coblox-core` left both oracles fully green. Two oracles
# written from the same table are independent in implementation and not in which
# cases exist.
#
# These cases are therefore derived, not transcribed — and derived differently
# from the Rust suite, which computes the two boundaries in closed form. Here the
# flip point is *searched for*: the sweep asks the predicate itself where it
# stops accepting, in each direction, and then checks that the answer is the one
# the ratio implies and that the refusal names the rate rule. If a direction is
# not enforced at all, the search runs to the end of the range and the case
# fails, which is exactly what did not happen before.
#
# The bounds are wide on every magnitude on purpose: a rejection in this sweep
# must come from the ratio and never from a floor or a ceiling.
# --------------------------------------------------------------------------

WIDE = RewardBounds(
    existence_fund_microtokens_per_epoch_max=2**60,
    reward_epoch_ms_min=1,
    reward_epoch_ms_max=2**60,
    publisher_reward_cap_numerator_max=2**60,
    publisher_reward_cap_denominator_min=1,
    validator_eligibility_threshold_units_min=1,
    validator_eligibility_window_epochs_max=2**60,
    validator_eligibility_min_issuers_min=2,
    storage_units_per_contribution_unit_max=2**60,
    compute_units_per_contribution_unit_max=2**60,
    storage_microtokens_per_byte_epoch_min=1,
    compute_microtokens_per_million_fuel_min=1,
)

# 5 000 divides by both 4 and 5, so both boundaries are exact integers and no
# rounding hides a case. The creator-share denominator is larger so that kn < kd
# holds at every point of its own sweep.
SWEEP_V = 5_000
SWEEP_BASE = {
    "reward_epoch_ms": SWEEP_V,
    "existence_fund_microtokens_per_epoch": SWEEP_V,
    # Pinned at zero by [ADR-010]: 0 -> 0 satisfies both inequalities and any
    # other value is refused before the ratio. Excluded from the sweep for that
    # reason, not by oversight.
    "availability_microtokens_per_unit": 0,
    "storage_microtokens_per_byte_epoch": SWEEP_V,
    "compute_microtokens_per_million_fuel": SWEEP_V,
    "publisher_microtokens_per_active_subscriber": SWEEP_V,
    "publisher_reward_cap_numerator": SWEEP_V,
    "publisher_reward_cap_denominator": 100_000,
    "storage_units_per_contribution_unit": SWEEP_V,
    "compute_units_per_contribution_unit": SWEEP_V,
    "validator_eligibility_threshold_units": SWEEP_V,
    "validator_eligibility_window_epochs": SWEEP_V,
    "validator_eligibility_min_issuers": SWEEP_V,
}

SWEEP_HEIGHT = WIDE.reward_parameter_min_activation_gap_blocks


def sweep_accept(key: str, value: int) -> tuple[bool, str]:
    body = dict(SWEEP_BASE)
    body[key] = value
    return accept_reward_policy(
        body,
        WIDE,
        active=SWEEP_BASE,
        active_activation_height=0,
        activation_height=SWEEP_HEIGHT,
    )


def highest_accepted(key: str, old: int) -> int:
    """The largest value of `key` the predicate still accepts, found by search."""

    lo, hi = old, old * 4
    assert not sweep_accept(key, hi)[0], f"{key}: the search range must end rejected"
    while lo < hi:
        mid = (lo + hi + 1) // 2
        if sweep_accept(key, mid)[0]:
            lo = mid
        else:
            hi = mid - 1
    return lo


def lowest_accepted(key: str, old: int) -> int:
    """The smallest value of `key` the predicate still accepts, found by search."""

    lo, hi = 0, old
    while lo < hi:
        mid = (lo + hi) // 2
        if sweep_accept(key, mid)[0]:
            hi = mid
        else:
            lo = mid + 1
    return lo


def rate_sweep() -> list[tuple[str, bool, str]]:
    """One result row per direction per governed parameter."""

    num = WIDE.reward_parameter_change_numerator
    den = WIDE.reward_parameter_change_denominator
    rows: list[tuple[str, bool, str]] = []
    for key, old in SWEEP_BASE.items():
        if old == 0:
            continue

        found = highest_accepted(key, old)
        expected = old * num // den
        _, why = sweep_accept(key, found + 1)
        ok = found == expected and why == f"rate of change exceeded on {key}"
        rows.append((f"{key} upward", ok, f"stops accepting above {found:,} ({why})"))

        found = lowest_accepted(key, old)
        expected = -(-old * den // num)
        _, why = sweep_accept(key, found - 1) if found > 0 else (True, "accepted")
        ok = found == expected and why == f"rate of change exceeded on {key}"
        rows.append((f"{key} downward", ok, f"stops accepting below {found:,} ({why})"))
    return rows


MIN_SET_CASES = [
    (12, 8, True),
    (12, 7, False),
    (12, 1, False),
    (27, 18, True),
    (27, 17, False),
    (36, 24, True),
    (36, 18, False),
]


def main() -> int:
    failures = 0
    print("Rules 1 and 2 - reward_policy acceptance against RewardBounds")
    print(f"  {'case':<42} {'expected':>9} {'got':>9}  reason")
    for name, body, expected in CASES:
        ok, why = accept_reward_policy(body, B)
        bad = ok != expected
        failures += bad
        print(
            f"  {name:<42} {'valid' if expected else 'INVALID':>9} "
            f"{'valid' if ok else 'INVALID':>9}  {why}"
            + ("   <-- MISMATCH" if bad else "")
        )

    print()
    print("Rule 3 - rate of change and activation spacing")
    for name, body, height, expected in RATE_CASES:
        ok, why = accept_reward_policy(
            body, B, active=BASE, active_activation_height=0, activation_height=height
        )
        bad = ok != expected
        failures += bad
        print(
            f"  {name:<42} {'valid' if expected else 'INVALID':>9} "
            f"{'valid' if ok else 'INVALID':>9}  {why}"
            + ("   <-- MISMATCH" if bad else "")
        )

    print()
    print("Rule 3 - both halves of the rate of change, per governed parameter")
    print("  (boundaries searched for, not transcribed; [REVIEW-017] RF-002)")
    sweep = rate_sweep()
    for name, ok, detail in sweep:
        failures += not ok
        print(
            f"  {name:<50} {'BOUND' if ok else 'UNBOUND':>9}  {detail}"
            + ("   <-- MISMATCH" if not ok else "")
        )

    print()
    print("Relational rule on consensus_parameters - 3 * min_set >= 2 * V")
    for V, ms, expected in MIN_SET_CASES:
        ok, _ = accept_consensus_min_set(V, ms)
        bad = ok != expected
        failures += bad
        print(
            f"  V={V:<4} min_set={ms:<4} 3*{ms}={3 * ms:<5} vs 2*{V}={2 * V:<5} "
            f"{'valid' if expected else 'INVALID':>9} {'valid' if ok else 'INVALID':>9}"
            + ("   <-- MISMATCH" if bad else "")
        )

    total = len(CASES) + len(RATE_CASES) + len(sweep) + len(MIN_SET_CASES)
    print()
    print(f"cases: {total}, mismatches: {failures}")
    print(f"GATE-RULES-REJECT: {'PASS' if failures == 0 else 'FAIL'}")
    return 0 if failures == 0 else 1


if __name__ == "__main__":
    raise SystemExit(main())
