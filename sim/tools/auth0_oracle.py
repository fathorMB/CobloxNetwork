#!/usr/bin/env python3
"""The second derivation of fixture `AUTH-0`, done from the rule and not from
the previous table.

`GATE-TWO-ORACLES` of [SPEC-022] requires the `AUTH-0` verdict table to be
derived **twice by independent routes, neither reading the output of the
other**. [REVIEW-042] established that the second derivation had never been
done: the transcript carried only the two digest oracles, and the diagnosis was
that the new table had been obtained by *flipping the rows of the old one* —
which reproduces the row set the previous reading needed and not the one the new
reading requires. Three symptoms of the same origin: row `20` was absent from
the table and from the suite, and the prose attributed the boundary of clause 2
to row `21`, which is the single row that does **not** separate the two
comparisons.

What this tool reads, stated because the gate requires it:

1. the **normative clause text** of `ledger.md` §"What `enrolled, unrevoked`
   means, and as of which height", clauses 1 and 2, which it re-implements as a
   two-line predicate — not the table, and not the Rust crate;
2. the **three declared facts of the fixture**, extracted from the fixture's own
   prose by regular expression: the `valid_from_height` of both certificates,
   the identity the revocation names, and the height of the block that included
   it;
3. the **table**, extracted from the document, *only in order to be compared
   with what (1) and (2) produce*.

It does not read `core/coblox-core/`, and no value in it is transcribed from the
table. `effective_height` is read for exactly one purpose — to assert that it is
**not** a boundary of either clause — and never to compute a verdict, which is
the whole content of part 1 of [ADR-017].

Route independence, stated as a limit rather than claimed: the first route is
the published table together with the Rust conformance suite
(`core/coblox-core/tests/authorization_unrevoked.rs`), which *transcribes* the
rows. This is the second route: verdicts computed from the clauses, and flip
heights found **by exhaustion** over the whole interval instead of by sampling.
Exhaustion is what the previous derivation could not have done, because a sweep
of every height finds `20` without anyone knowing to look for it.

Usage:
    python sim/tools/auth0_oracle.py              # derive and compare
    python sim/tools/auth0_oracle.py --negative   # prove the comparison bites
"""

from __future__ import annotations

import re
import sys
from dataclasses import dataclass
from pathlib import Path

LEDGER = Path(__file__).resolve().parents[2] / "docs" / "protocol" / "ledger.md"

# The height range the exhaustion runs over. It has to reach past
# `effective_height` so that the absence of a flip there is a measured fact and
# not an untested region.
SWEEP_TOP = 60


# --------------------------------------------------------------------------
# (1) the rule, re-implemented from the clause text
# --------------------------------------------------------------------------
#
# `ledger.md`, verbatim:
#
#   1. an enrollment certificate in that chain names `node_id` and its
#      `valid_from_height` is at most `h`; and
#   2. no `revoke_identity` in that chain names `node_id` at a height at most
#      `h` — the block at `h` included.
#
# Both comparisons are "at most", i.e. `<=`. Nothing in either clause names
# `effective_height`, and nothing in either clause names an execution order
# inside a block: the predicate is a function of `h` alone given the two record
# sets, which is why it is insensitive to intra-block ordering.

CLAUSE_1 = "an enrollment certificate in that chain names `node_id` and its"
CLAUSE_2 = "no `revoke_identity` in that chain names `node_id` at a height at most `h`"


def clause_1_holds(valid_from_height: int, h: int) -> bool:
    """Clause 1: `valid_from_height` is at most `h`."""
    return valid_from_height <= h


def clause_2_holds(revocation_included_heights: tuple[int, ...], h: int) -> bool:
    """Clause 2: no revocation of this identity is at a height at most `h`."""
    return all(included > h for included in revocation_included_heights)


def verdict(valid_from_height: int, revocations: tuple[int, ...], h: int) -> str:
    if not clause_1_holds(valid_from_height, h):
        return "invalid"
    if not clause_2_holds(revocations, h):
        return "invalid"
    return "valid"


# --------------------------------------------------------------------------
# (2) the declared facts of the fixture, extracted from its own prose
# --------------------------------------------------------------------------


@dataclass(frozen=True)
class FixtureFacts:
    valid_from_height: int
    revoked_node_id: str
    included_height: int
    effective_height: int


def extract_facts(text: str) -> FixtureFacts:
    enrolment = re.search(
        r"hold a finalized enrollment certificate with\s+`valid_from_height` `(\d+)`",
        text,
    )
    revocation = re.search(
        r"A `revoke_identity` naming `([A-Za-z0-9]+)` with\s+"
        r"`effective_height` `(\d+)` is included in the block at height `(\d+)`",
        text,
    )
    if enrolment is None:
        raise SystemExit("FAIL: the fixture's `valid_from_height` sentence was not found")
    if revocation is None:
        raise SystemExit("FAIL: the fixture's revocation sentence was not found")
    return FixtureFacts(
        valid_from_height=int(enrolment.group(1)),
        revoked_node_id=revocation.group(1),
        included_height=int(revocation.group(3)),
        effective_height=int(revocation.group(2)),
    )


def check_clause_text(text: str) -> None:
    """The clauses this tool re-implements must be the clauses the document
    states. Without this the oracle could drift into being an oracle for a rule
    the protocol no longer has."""
    for clause in (CLAUSE_1, CLAUSE_2):
        if clause not in text:
            raise SystemExit(
                f"FAIL: the clause this oracle implements is not in ledger.md: {clause!r}"
            )


# --------------------------------------------------------------------------
# (3) the table, extracted rather than transcribed
# --------------------------------------------------------------------------

ROW = re.compile(
    r"^\|\s*`([A-Za-z0-9]+)`\s*\|\s*`(\d+)`\s*\|([^|]*)\|([^|]*)\|\s*\*{0,2}(valid|invalid)\*{0,2}\s*\|\s*$"
)


@dataclass(frozen=True)
class TableRow:
    node_id: str
    height: int
    verdict: str
    line_number: int


def extract_table(text: str) -> list[TableRow]:
    lines = text.splitlines()
    # The `AUTH-0` table is the one that follows the fixture heading; anchoring
    # on the heading keeps this from picking up another table of the document.
    start = next(
        (i for i, line in enumerate(lines) if line.startswith("**Fixture `AUTH-0`")),
        None,
    )
    if start is None:
        raise SystemExit("FAIL: the `AUTH-0` fixture heading was not found")
    rows: list[TableRow] = []
    for offset, line in enumerate(lines[start:], start=start):
        match = ROW.match(line)
        if match:
            rows.append(
                TableRow(
                    node_id=match.group(1),
                    height=int(match.group(2)),
                    verdict=match.group(5),
                    line_number=offset + 1,
                )
            )
        elif rows and not line.startswith("|"):
            break
    if not rows:
        raise SystemExit("FAIL: no `AUTH-0` table rows were parsed")
    return rows


# --------------------------------------------------------------------------
# the derivation, and the comparison
# --------------------------------------------------------------------------


def flip_heights(valid_from: int, revocations: tuple[int, ...], top: int) -> list[int]:
    """Every height at which the verdict changes, found by exhaustion."""
    flips: list[int] = []
    previous = verdict(valid_from, revocations, 0)
    for h in range(1, top + 1):
        current = verdict(valid_from, revocations, h)
        if current != previous:
            flips.append(h)
            previous = current
    return flips


def run(strict_clause_2: bool = False, read_effective_height: bool = False) -> int:
    """`strict_clause_2` and `read_effective_height` are the two mutations the
    negative proof applies; both are False in ordinary operation."""
    text = LEDGER.read_text(encoding="utf-8")
    check_clause_text(text)
    facts = extract_facts(text)
    rows = extract_table(text)

    revoked_included: tuple[int, ...]
    if read_effective_height:
        revoked_included = (facts.effective_height,)
    else:
        revoked_included = (facts.included_height,)
    if strict_clause_2:
        revoked_included = tuple(i + 1 for i in revoked_included)

    print("facts read from the fixture prose (not from the table):")
    print(f"  valid_from_height   = {facts.valid_from_height}")
    print(f"  revoked identity    = {facts.revoked_node_id}")
    print(f"  included at height  = {facts.included_height}")
    print(f"  effective_height    = {facts.effective_height}  (read, never used in a verdict)")
    print()

    findings: list[str] = []

    print(f"verdicts derived from clauses 1 and 2, compared with the {len(rows)} table rows:")
    for row in rows:
        revocations = revoked_included if row.node_id == facts.revoked_node_id else ()
        derived = verdict(facts.valid_from_height, revocations, row.height)
        mark = "ok  " if derived == row.verdict else "FAIL"
        if derived != row.verdict:
            findings.append(
                f"row at h={row.height} ({row.node_id}, ledger.md:{row.line_number}): "
                f"table says {row.verdict}, the rule derives {derived}"
            )
        print(f"  {mark}  h={row.height:<3} {row.node_id[:24]:<24} table={row.verdict:<7} rule={derived}")
    print()

    flips = flip_heights(facts.valid_from_height, revoked_included, SWEEP_TOP)
    print(f"flip heights over 0..{SWEEP_TOP}, by exhaustion and not by sampling: {flips}")
    expected_flips = [facts.valid_from_height, facts.included_height]
    if flips != expected_flips:
        findings.append(
            f"the verdict changes at {flips}; clauses 1 and 2 place the two changes at "
            f"{expected_flips}"
        )
    # The check that a table obtained by flipping the rows of the previous one
    # cannot pass: under the previous reading the boundary was
    # `effective_height`, and under this one it must not be a boundary at all.
    if facts.effective_height in flips:
        findings.append(
            f"`effective_height` {facts.effective_height} is a boundary of the verdict, "
            "which is the previous reading and not part 1 of ADR-017"
        )
    # Every table row must sit on a height the sweep covers, or the comparison
    # above is silently partial.
    for row in rows:
        if row.height > SWEEP_TOP:
            findings.append(f"row h={row.height} lies outside the swept interval")
    # The boundary of each clause must appear as a row: a table that omits them
    # is the table this gate exists to reject.
    table_heights = {row.height for row in rows if row.node_id == facts.revoked_node_id}
    for boundary in expected_flips:
        if boundary not in table_heights:
            findings.append(
                f"the boundary height {boundary} found by exhaustion has no row in the table"
            )

    print()
    if findings:
        for finding in findings:
            print(f"FAIL {finding}")
        print(f"\nAUTH-0 second derivation: FAIL ({len(findings)} finding(s))")
        return 1
    print(
        f"AUTH-0 second derivation: PASS - {len(rows)} rows agree, "
        f"boundaries {expected_flips} found by exhaustion, "
        f"effective_height {facts.effective_height} is not a boundary"
    )
    return 0


def negative() -> int:
    """Each mutation must be observed failing. A comparison that cannot fail is
    not evidence, which is the standard `published_artifacts_negative.py` sets
    for every other guard in this repository."""
    mutations = [
        (
            "clause 2 read as `<` instead of `at most`: the revocation would not "
            "bite at its own inclusion height",
            dict(strict_clause_2=True),
        ),
        (
            "clause 2 anchored to `effective_height` instead of the inclusion "
            "height: the reading ADR-017 part 1 replaced",
            dict(read_effective_height=True),
        ),
    ]
    failures = 0
    for description, kwargs in mutations:
        print(f"=== mutation: {description} ===")
        code = run(**kwargs)  # type: ignore[arg-type]
        print(f"exit={code} (must be non-zero)\n")
        if code == 0:
            failures += 1
            print("FAIL: the mutation was not detected\n")
    if failures:
        print(f"negative proof: FAIL - {failures} mutation(s) went unnoticed")
        return 1
    print(f"negative proof: PASS - {len(mutations)} mutations, each observed failing")
    return 0


if __name__ == "__main__":
    if "--negative" in sys.argv:
        raise SystemExit(negative())
    raise SystemExit(run())
