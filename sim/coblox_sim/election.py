"""Executable model of the election derivation of `docs/protocol/ledger.md`.

Every step follows §"Validator election and rotation" literally: retain, commit
the candidates, form the fill pool, derive the seed, rank by ticket, fill under
the cap, then check the contraction floor and the minimum set size. The ticket
uses the protocol's own preimage and SHA-256, so the ordering is the protocol's
ordering and not a stand-in drawn from a pseudo-random generator.

The model does not implement blocks, signatures, or networking. It implements
the *derivation*, because the derivation is what the parameters are being tuned
against.
"""

from __future__ import annotations

import hashlib
import math
from dataclasses import dataclass, field, replace
from typing import Iterable, Literal

from .params import ConsensusParameters


# --------------------------------------------------------------------------
# Hashing, matching ledger.md byte for byte
# --------------------------------------------------------------------------


def _h(*parts: bytes) -> bytes:
    d = hashlib.sha256()
    for p in parts:
        d.update(p)
    return d.digest()


def _u64be(n: int) -> bytes:
    return n.to_bytes(8, "big")


def account_key(node_id: str) -> bytes:
    """A 32-byte stand-in for the node's account key, derived deterministically."""

    return _h(b"coblox-sim-account-key-v0\x00", node_id.encode("utf-8"))


def election_seed(chain_id: bytes, epoch: int, entropy_block_ids: Iterable[bytes]) -> bytes:
    ids = list(entropy_block_ids)
    entropy = _h(
        b"coblox-election-entropy-v0\x00",
        chain_id,
        _u64be(epoch),
        _u64be(len(ids)),
        *ids,
    )
    return _h(b"coblox-election-seed-v0\x00", chain_id, _u64be(epoch), entropy)


def election_ticket(chain_id: bytes, seed: bytes, key: bytes) -> bytes:
    return _h(b"coblox-election-ticket-v0\x00", chain_id, seed, key)


# --------------------------------------------------------------------------
# Nodes and sets
# --------------------------------------------------------------------------

Faction = Literal["honest", "adversary"]


@dataclass(frozen=True)
class CandidateNode:
    node_id: str
    faction: Faction
    contribution_score: int
    distinct_issuers: int

    @property
    def key(self) -> bytes:
        return account_key(self.node_id)


@dataclass(frozen=True)
class Seat:
    node_id: str
    faction: Faction
    seated_since_epoch: int
    term_expiry_epoch: int


@dataclass(frozen=True)
class ValidatorSet:
    epoch: int
    seats: tuple[Seat, ...]

    @property
    def member_count(self) -> int:
        return len(self.seats)

    @property
    def adversary_count(self) -> int:
        return sum(1 for s in self.seats if s.faction == "adversary")

    def adversary_holds_third(self) -> bool:
        """Strictly above one third of uniform voting power."""

        return 3 * self.adversary_count > self.member_count

    def adversary_holds_all(self) -> bool:
        return self.member_count > 0 and self.adversary_count == self.member_count


class ChainStalled(Exception):
    """No valid set exists for the boundary; the chain stalls (ledger.md)."""

    def __init__(self, epoch: int, reason: str, previous: ValidatorSet):
        super().__init__(f"epoch {epoch}: {reason}")
        self.epoch = epoch
        self.reason = reason
        self.previous = previous


# --------------------------------------------------------------------------
# Genesis
# --------------------------------------------------------------------------


def genesis_set(p: ConsensusParameters, adversary_seats: int = 0) -> ValidatorSet:
    """A genesis set obeying the **genesis stagger** of ledger.md.

    Every entry's ``term_expiry_epoch`` lies in ``[1, T]`` and no more than
    ``c`` entries share the same value. Refusing to build one that violates
    either condition is the model's version of "a client MUST refuse it".
    """

    V, T, c = p.V, p.T, p.c
    if V > T * c:
        raise ValueError(
            f"no staggered genesis exists: V={V} exceeds T*c={T * c}; "
            "at most c entries may share each of the T expiry values"
        )
    seats: list[Seat] = []
    for i in range(V):
        expiry = (i % T) + 1
        faction: Faction = "adversary" if i < adversary_seats else "honest"
        seats.append(
            Seat(
                node_id=f"gen-{i:04d}",
                faction=faction,
                seated_since_epoch=0,
                term_expiry_epoch=expiry,
            )
        )
    counts: dict[int, int] = {}
    for s in seats:
        counts[s.term_expiry_epoch] = counts.get(s.term_expiry_epoch, 0) + 1
    if max(counts.values()) > c:
        raise ValueError("genesis stagger violated: more than c entries share an expiry")
    return ValidatorSet(epoch=0, seats=tuple(seats))


# --------------------------------------------------------------------------
# The derivation
# --------------------------------------------------------------------------


@dataclass
class BoundaryOutcome:
    epoch: int
    previous_count: int
    retained_count: int
    filled_count: int
    member_count: int
    candidate_count: int
    fill_pool_size: int
    adversary_count: int
    stalled: bool = False
    stall_reason: str = ""


@dataclass
class ElectionState:
    """Mutable state carried across boundaries."""

    p: ConsensusParameters
    chain_id: bytes
    current: ValidatorSet
    # node_id -> epoch at which it left a seat (cooldown starts after it)
    left_seat_at: dict[str, int] = field(default_factory=dict)
    history: list[BoundaryOutcome] = field(default_factory=list)

    def in_cooldown(self, node_id: str, epoch: int) -> bool:
        left = self.left_seat_at.get(node_id)
        if left is None:
            return False
        # "did not leave a seat in any of the cooldown_epochs epochs before e"
        return epoch - left <= self.p.validator_cooldown_epochs


def eligible_candidates(
    state: ElectionState,
    epoch: int,
    applicants: Iterable[CandidateNode],
    threshold_units: int,
    min_issuers: int,
) -> list[CandidateNode]:
    """Eligibility conditions 2-5 of ledger.md. Condition 1 (enrolled and not
    revoked) is assumed for every applicant in the model."""

    out: list[CandidateNode] = []
    for n in applicants:
        if n.contribution_score < threshold_units:
            continue
        if n.distinct_issuers < min_issuers:
            continue
        if state.in_cooldown(n.node_id, epoch):
            continue
        out.append(n)
    return out


def derive_boundary(
    state: ElectionState,
    epoch: int,
    applicants: list[CandidateNode],
    threshold_units: int,
    min_issuers: int,
    entropy_block_ids: list[bytes],
    grinding_attempts: int = 1,
    grinder_faction: Faction | None = None,
) -> BoundaryOutcome:
    """Run one election boundary. Raises ``ChainStalled`` when no valid set exists."""

    p = state.p
    P = state.current
    applicant_by_id = {n.node_id: n for n in applicants}

    # 1. Retain.
    retained: list[Seat] = []
    for seat in P.seats:
        n = applicant_by_id.get(seat.node_id)
        if n is None:
            continue  # no candidacy filed for this epoch
        if epoch >= seat.term_expiry_epoch:
            continue  # term expired
        if n.contribution_score < threshold_units:
            continue
        if n.distinct_issuers < min_issuers:
            continue
        retained.append(seat)
    retained_ids = {s.node_id for s in retained}

    # Everyone who held a seat and is not retained leaves it: cooldown starts,
    # "for any reason whatsoever" (eligibility condition 5).
    for seat in P.seats:
        if seat.node_id not in retained_ids:
            state.left_seat_at[seat.node_id] = epoch

    # 2. Commit the candidates: C is the eligible set, which contains R too.
    C = eligible_candidates(state, epoch, applicants, threshold_units, min_issuers)
    C_ids = {n.node_id for n in C}
    # A retained member is in C by construction: it filed, it met the score, and
    # it did not leave a seat.
    for s in retained:
        C_ids.add(s.node_id)
    candidate_count = len(C_ids)

    # 3. Fill pool.
    Nw = [n for n in C if n.node_id not in retained_ids]

    # 4. Seed, with optional grinding by whoever proposes the last block.
    best_seed = election_seed(state.chain_id, epoch, entropy_block_ids)
    if grinding_attempts > 1 and grinder_faction is not None and Nw:
        best_score = -1
        for attempt in range(grinding_attempts):
            ids = list(entropy_block_ids)
            ids[-1] = _h(ids[-1], _u64be(attempt))  # resample the last block ID
            seed = election_seed(state.chain_id, epoch, ids)
            ranked = _rank(state.chain_id, seed, Nw)
            take = min(
                max(0, p.validator_target_set_size - len(retained)),
                p.c,
                len(Nw),
            )
            score = sum(1 for n in ranked[:take] if n.faction == grinder_faction)
            if score > best_score:
                best_score = score
                best_seed = seed
    seed = best_seed

    # 5. Rank.
    ranked = _rank(state.chain_id, seed, Nw)

    # 6. Fill, under the cap.
    fills = min(
        max(0, p.validator_target_set_size - len(retained)),
        p.validator_churn_cap_seats,
        len(ranked),
    )
    new_seats = list(retained)
    for n in ranked[:fills]:
        new_seats.append(
            Seat(
                node_id=n.node_id,
                faction=n.faction,
                seated_since_epoch=epoch,
                term_expiry_epoch=epoch + p.validator_max_consecutive_terms,
            )
        )

    # 7. Assemble, then the floor and the minimum.
    new_set = ValidatorSet(epoch=epoch, seats=tuple(sorted(new_seats, key=lambda s: s.node_id)))
    outcome = BoundaryOutcome(
        epoch=epoch,
        previous_count=P.member_count,
        retained_count=len(retained),
        filled_count=fills,
        member_count=new_set.member_count,
        candidate_count=candidate_count,
        fill_pool_size=len(Nw),
        adversary_count=new_set.adversary_count,
    )
    if not (3 * new_set.member_count > 2 * P.member_count):
        outcome.stalled = True
        outcome.stall_reason = (
            f"contraction floor: 3*{new_set.member_count} <= 2*{P.member_count}"
        )
        state.history.append(outcome)
        raise ChainStalled(epoch, outcome.stall_reason, P)
    if new_set.member_count < p.validator_min_set_size:
        outcome.stalled = True
        outcome.stall_reason = (
            f"below validator_min_set_size: {new_set.member_count} < {p.validator_min_set_size}"
        )
        state.history.append(outcome)
        raise ChainStalled(epoch, outcome.stall_reason, P)

    state.current = new_set
    state.history.append(outcome)
    return outcome


def _rank(chain_id: bytes, seed: bytes, pool: list[CandidateNode]) -> list[CandidateNode]:
    """Order by (ticket ascending, account_key ascending) — a total order."""

    return sorted(
        pool,
        key=lambda n: (election_ticket(chain_id, seed, n.key), n.key),
    )


def entropy_ids(chain_id: bytes, epoch: int, count: int) -> list[bytes]:
    """Deterministic stand-in block IDs for the entropy window."""

    return [_h(b"coblox-sim-block-id-v0\x00", chain_id, _u64be(epoch), _u64be(i))
            for i in range(count)]


# --------------------------------------------------------------------------
# Steady-state arithmetic used by the pool analysis
# --------------------------------------------------------------------------


def steady_state_seat_demand(p: ConsensusParameters) -> float:
    """Average seats vacated per boundary in steady state: ``V / T``."""

    return p.V / p.T


def minimum_pool_for_sustained_set(p: ConsensusParameters) -> int:
    """Smallest number of distinct eligible, candidacy-filing nodes that keeps
    a set of ``V`` members from shrinking.

    ``V`` seated, plus the cohort in cooldown (``ceil(V/T)`` leavers per
    boundary for ``validator_cooldown_epochs`` boundaries), plus enough free
    candidates to refill the current boundary.
    """

    per_boundary = math.ceil(p.V / p.T)
    return p.V + per_boundary * p.validator_cooldown_epochs + per_boundary


def boundaries_to_attrition_capture(V: int, k: int) -> int:
    """``ceil(log(V/k) / log(3/2))`` — the fixed attrition horizon."""

    if k >= V:
        return 0
    return math.ceil(math.log(V / k) / math.log(1.5))


def boundaries_to_admission_capture(V: int, c: int) -> int:
    """``ceil((V/3)/c)`` — the tunable admission horizon."""

    return math.ceil((V / 3) / c)
