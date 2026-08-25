"""Deterministic synthetic populations.

Randomness here is derived from SHA-256 rather than from ``random``, so the
figures are reproducible across Python versions and platforms, not merely
within one interpreter build. Every draw is a function of a declared seed
string and an index.
"""

from __future__ import annotations

import hashlib
import math
from dataclasses import dataclass
from statistics import NormalDist

from .election import CandidateNode


_UNIT = float(1 << 53)


def uniform(seed: str, index: int, stream: str = "") -> float:
    """A deterministic uniform draw in [0, 1)."""

    digest = hashlib.sha256(
        f"{seed}|{stream}|{index}".encode("utf-8")
    ).digest()
    return int.from_bytes(digest[:7], "big") / float(1 << 56)


def lognormal(seed: str, index: int, median: float, sigma: float, stream: str = "") -> float:
    u = uniform(seed, index, stream)
    # Keep the draw strictly inside (0, 1) so inv_cdf is finite.
    u = min(max(u, 1e-12), 1 - 1e-12)
    z = NormalDist().inv_cdf(u)
    return median * math.exp(sigma * z)


@dataclass(frozen=True)
class ContributorProfile:
    """Distribution of proven storage/compute capacity among contributors.

    ``median_units_per_epoch`` is expressed in contribution units per reward
    epoch: one unit is one GiB proven for one epoch, or one million fuel
    re-executed, per the recommended divisors.
    """

    median_units_per_epoch: float = 32.0
    sigma: float = 1.5
    issuer_pool: int = 64


def build_contributors(
    count: int,
    window_epochs: int,
    profile: ContributorProfile,
    seed: str,
    faction: str = "honest",
    id_prefix: str = "hon",
) -> list[CandidateNode]:
    """Build ``count`` contributor nodes with a heavy-tailed capacity draw."""

    out: list[CandidateNode] = []
    for i in range(count):
        units = lognormal(seed, i, profile.median_units_per_epoch, profile.sigma, "capacity")
        score = int(units * window_epochs)
        # Distinct issuers seen over the window: a node with more evidence sees
        # more issuers, saturating at the issuer pool.
        expected = min(profile.issuer_pool, 1 + int(math.log1p(units) * 2))
        out.append(
            CandidateNode(
                node_id=f"{id_prefix}-{i:06d}",
                faction=faction,  # type: ignore[arg-type]
                contribution_score=score,
                distinct_issuers=expected,
            )
        )
    return out


def build_adversary_candidates(
    count: int,
    threshold_units: int,
    min_issuers: int,
    id_prefix: str = "adv",
) -> list[CandidateNode]:
    """Attacker candidates that buy their way over the eligibility bar.

    Per the threat model's own correction to `AT-10`, the attacker profile is no
    longer "datacenter uptime": uptime contributes zero to the contribution
    score. These candidates supply real storage and compute just above the
    threshold, and enroll ``min_issuers`` colluding issuers each — the price the
    eligibility rule actually charges.
    """

    return [
        CandidateNode(
            node_id=f"{id_prefix}-{i:06d}",
            faction="adversary",
            contribution_score=threshold_units,
            distinct_issuers=min_issuers,
        )
        for i in range(count)
    ]
