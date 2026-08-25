"""Agent-based emission accounting: the mint side of [ADR-005] under [ADR-007].

The model is deliberately *arithmetic on minted amounts*, not a network
simulator. Everything it computes is a sum of `mint` transactions that
``docs/protocol/ledger.md`` already defines exactly:

* ``existence_income`` — ``amount = F // E`` with the remainder discarded and
  never minted (`ledger.md` §"Existence income is a share of a capped fund");
* ``work_compensation`` — per-unit rates against measured, finalized evidence;
* ``publisher_reward`` — bounded by the creator-share cap, a validity rule.

`alpha` is **not** a schema field and is not a knob. `ledger.md` says so in as
many words: it is "an observed ratio between channels". The knob is
``existence_fund_microtokens_per_epoch`` (``F``); `alpha` is what comes out.
This module therefore takes `F` and the work channel as inputs and *reports*
`alpha`, and offers an inverse helper for the curve, which is the only place a
target `alpha` is allowed to drive `F`.
"""

from __future__ import annotations

from dataclasses import dataclass, field
from fractions import Fraction


MICROTOKENS_PER_CREDIT = 1_000_000


# --------------------------------------------------------------------------
# Population
# --------------------------------------------------------------------------


@dataclass(frozen=True)
class NodePopulation:
    """One epoch's population, grouped because only the group sizes matter.

    ``honest_availability_only`` are the project's characteristic devices: they
    prove presence and nothing else. ``honest_contributors`` additionally serve
    storage and compute and are paid for it. ``emulated`` is the Sybil fleet:
    it proves presence (a signature) and by construction earns nothing in the
    storage and compute channels.
    """

    honest_availability_only: int
    honest_contributors: int
    emulated: int

    @property
    def honest(self) -> int:
        return self.honest_availability_only + self.honest_contributors

    @property
    def eligible_count(self) -> int:
        """``E`` — every present node that met the availability threshold."""

        return self.honest + self.emulated


@dataclass(frozen=True)
class WorkChannel:
    """Storage and compute compensation actually earned in one epoch.

    ``total_microtokens`` is exogenous: it is set by how much the network is
    *used*, not by governance. Splitting it across contributors is what makes
    the median honest income computable.
    """

    total_microtokens: int
    contributor_share_weights: tuple[int, ...] = ()

    def split(self, contributors: int) -> list[int]:
        if contributors <= 0:
            return []
        if self.contributor_share_weights:
            weights = list(self.contributor_share_weights)
            if len(weights) != contributors:
                raise ValueError("weight vector length must equal contributor count")
        else:
            weights = [1] * contributors
        total_weight = sum(weights)
        if total_weight <= 0:
            raise ValueError("weights must sum to a positive value")
        # Integer split, remainder to the lowest-index contributors so the
        # result is deterministic and sums exactly to total_microtokens.
        base = [self.total_microtokens * w // total_weight for w in weights]
        remainder = self.total_microtokens - sum(base)
        for i in range(remainder):
            base[i % contributors] += 1
        return base


# --------------------------------------------------------------------------
# One epoch
# --------------------------------------------------------------------------


@dataclass(frozen=True)
class EpochResult:
    eligible_count: int
    existence_amount_per_node: int
    existence_minted_total: int
    existence_fund_cap: int
    existence_remainder_burned: int
    work_minted_total: int
    publisher_minted_total: int
    total_minted: int
    fleet_minted: int
    honest_availability_only_income: int
    honest_contributor_incomes: tuple[int, ...]

    @property
    def alpha(self) -> Fraction:
        """Observed ``alpha``: the share of emission through the existence
        channel."""

        if self.total_minted == 0:
            return Fraction(0)
        return Fraction(self.existence_minted_total, self.total_minted)

    @property
    def captured_share(self) -> Fraction:
        if self.total_minted == 0:
            return Fraction(0)
        return Fraction(self.fleet_minted, self.total_minted)

    @property
    def median_honest_income(self) -> int:
        incomes = sorted(
            [self.honest_availability_only_income] * self._availability_only_count
            + list(self.honest_contributor_incomes)
        )
        if not incomes:
            return 0
        mid = len(incomes) // 2
        if len(incomes) % 2:
            return incomes[mid]
        return (incomes[mid - 1] + incomes[mid]) // 2

    _availability_only_count: int = 0


def run_epoch(
    population: NodePopulation,
    existence_fund_microtokens: int,
    work: WorkChannel,
    publisher_minted_total: int = 0,
    availability_microtokens_per_unit: int = 0,
    availability_units_per_node: int = 0,
) -> EpochResult:
    """Mint one reward epoch and account for who received what.

    ``availability_microtokens_per_unit`` is present because
    ``RewardPolicyBody`` has the field and a non-zero value **breaks** the
    [ADR-007] metric: `work_compensation` with ``work_kind = "availability"``
    is a per-node amount with no cap, so ``N`` emulated identities increase
    total emission. The model keeps the channel so the failure is measurable
    rather than assumed away; the recommended value is ``0``.
    """

    E = population.eligible_count
    if E <= 0:
        raise ValueError("an epoch with no eligible node has no valid existence mint")

    per_node = existence_fund_microtokens // E  # ledger.md: integer division
    existence_total = per_node * E
    remainder = existence_fund_microtokens - existence_total

    contributor_incomes = work.split(population.honest_contributors)
    work_total = sum(contributor_incomes)

    avail_per_node = availability_microtokens_per_unit * availability_units_per_node
    availability_total = avail_per_node * E

    total = existence_total + work_total + publisher_minted_total + availability_total
    fleet = population.emulated * (per_node + avail_per_node)

    return EpochResult(
        eligible_count=E,
        existence_amount_per_node=per_node,
        existence_minted_total=existence_total,
        existence_fund_cap=existence_fund_microtokens,
        existence_remainder_burned=remainder,
        work_minted_total=work_total + availability_total,
        publisher_minted_total=publisher_minted_total,
        total_minted=total,
        fleet_minted=fleet,
        honest_availability_only_income=per_node + avail_per_node,
        honest_contributor_incomes=tuple(
            per_node + avail_per_node + w for w in contributor_incomes
        ),
        _availability_only_count=population.honest_availability_only,
    )


# --------------------------------------------------------------------------
# Inverse helper, used only to draw the curve
# --------------------------------------------------------------------------


def fund_for_target_alpha(
    alpha: Fraction,
    non_existence_microtokens: int,
    reference_scale_microtokens: int = 0,
) -> int:
    """``F`` such that ``F / (F + W) == alpha``, i.e. ``F = W * a / (1 - a)``.

    ``alpha == 1`` means the non-existence channel is empty, and then every
    positive ``F`` satisfies the ratio; the caller supplies
    ``reference_scale_microtokens`` so that the endpoint of the curve is drawn
    at the same emission scale as the rest of it.
    """

    if alpha < 0 or alpha > 1:
        raise ValueError("alpha must lie in [0, 1]")
    if alpha == 1:
        if non_existence_microtokens != 0:
            raise ValueError("alpha = 1 requires an empty non-existence channel")
        if reference_scale_microtokens <= 0:
            raise ValueError("alpha = 1 needs a reference emission scale")
        return reference_scale_microtokens
    return int(Fraction(non_existence_microtokens) * alpha / (1 - alpha))


# --------------------------------------------------------------------------
# SEC-REQ-16 (b): the reputation-purchase margin of threat-model.md 6.3
# --------------------------------------------------------------------------


@dataclass(frozen=True)
class ReputationMargin:
    existence_income_per_period: int
    subscription_price_per_period: int
    cap_numerator: int
    cap_denominator: int

    @property
    def net_cost_per_fake_subscriber(self) -> Fraction:
        """``S * (1 - kn/kd)`` — strictly positive because ``kn < kd``."""

        return Fraction(self.subscription_price_per_period) * (
            1 - Fraction(self.cap_numerator, self.cap_denominator)
        )

    @property
    def sustainable(self) -> bool:
        return Fraction(self.existence_income_per_period) >= self.net_cost_per_fake_subscriber

    @property
    def margin(self) -> Fraction:
        """How many fake subscribers one node-period of existence income funds."""

        cost = self.net_cost_per_fake_subscriber
        if cost == 0:
            return Fraction(-1)  # unbounded; kn == kd is rejected at acceptance
        return Fraction(self.existence_income_per_period) / cost
