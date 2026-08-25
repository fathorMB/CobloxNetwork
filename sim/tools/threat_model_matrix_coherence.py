"""Check that the threat-model matrix and its scenarios cannot disagree.

Versioned on the Lead's decision of 2026-08-26, during the review of [SPEC-018].
It was written in a scratchpad while [SPEC-018] ran, and it found **eight**
misalignments the pass would otherwise have shipped: six of them predated the
spec, two were introduced by the pass itself. A check that catches its own
author is a check worth keeping, and a check that lives in a scratchpad is not
evidence anybody else can re-run — the same reason [REVIEW-014] RF-007 gave for
`sim/tools/protocol_hashes.py` existing at all.

**What this does not do, and it matters.** It never judges whether a cell is
*correct*. [ADR-012] declares in its own header that its tooling verifies shapes
and coherence between copies and never the semantic correctness of a value, and
`recurring-defects.md` classifies the family this document belongs to — the
claim left behind by the rule — as not mechanisable. What *is* mechanisable is
the coherence between two copies of the same fact: a scenario declares the
assets it hits, and the matrix places that scenario in rows. Those are two
copies, and they used to drift.

    python sim/tools/threat_model_matrix_coherence.py

Not wired into CI. Wiring it is [ADR-012] work, under the gate that applies to
tooling, and the Lead opened a debt for it rather than letting [SPEC-018] — a
spec with no [ADR-012] gate — smuggle it in.

Checks, in the order they fire:

  C1  the matrix is 13 asset rows by 8 actor columns
  C2  every non-`n/a` cell names at least one `TM-xx`
  C3  every `TM-xx` named in a cell is defined by a `#### TM-xx` heading
  C4  every scenario defined in §5 appears in at least one cell — a scenario
      outside the grid is a hole in the evidence `GATE-COVERAGE` rests on, and
      TM-39 sat outside it from the day it was written until 2026-08-26
  C5  each scenario's declared **Asset:** list equals exactly the set of asset
      rows in which the scenario appears
  C6  every `n/a` cites `R-NA`, the rule of §4 that admits it
  C7  no `n/a` argues from motive — §3 requires actors described by capability
      and budget, and R-NA.2 makes a motive inadmissible even when its
      conclusion is true

Exit code is 0 when every check passes and 1 otherwise; the counts are printed
either way, because the count is the thing a reader wants and the thing nobody
should retype by hand.
"""

from __future__ import annotations

import collections
import pathlib
import re
import sys

DOC = pathlib.Path(__file__).resolve().parents[2] / ".lmbrain" / "knowledge" / "threat-model.md"

ACTORS = ["T-01", "T-02", "T-03", "T-04", "T-05", "T-06", "T-07", "T-08"]

# R-NA.2: a motive is not a proposition about the system, so it cannot be
# falsified by reading a rule. These are the exact forms the matrix used before
# [SPEC-018]; the list grows only when a new one is caught.
MOTIVE_FORMS = [
    "non ha vantaggio",
    "non gli conviene",
    "ci guadagna",
    "risparmia le proprie",
    "vuole la rete",
]


def main() -> int:
    text = DOC.read_text(encoding="utf-8")
    lines = text.split("\n")
    failures: list[str] = []

    rows = [l for l in lines if l.startswith("| **A-")]
    if len(rows) != 13:
        failures.append(f"C1: {len(rows)} righe di matrice, attese 13")

    placed: dict[str, set[str]] = collections.defaultdict(set)
    n_na = n_cov = n_cells = 0

    for row in rows:
        cells = [c.strip() for c in row.strip().strip("|").split("|")]
        asset = re.search(r"A-\d\d", cells[0]).group(0)
        body = cells[1:]
        if len(body) != len(ACTORS):
            failures.append(f"C1: riga {asset} ha {len(body)} colonne, attese {len(ACTORS)}")
            continue
        for actor, cell in zip(ACTORS, body):
            n_cells += 1
            where = f"{asset} x {actor}"
            if cell.startswith("n/a"):
                n_na += 1
                if "R-NA" not in cell:
                    failures.append(f"C6: {where} e` n/a e non cita R-NA")
                for form in MOTIVE_FORMS:
                    if form in cell:
                        failures.append(f"C7: {where} argomenta dal movente: «{form}»")
            else:
                n_cov += 1
                names = set(re.findall(r"TM-\d\d", cell))
                if not names:
                    failures.append(f"C2: {where} non e` n/a e non nomina alcuno scenario")
                for name in names:
                    placed[name].add(asset)

    defined = set(re.findall(r"^#### (TM-\d\d) — ", text, re.M))

    for missing in sorted(set(placed) - defined):
        failures.append(f"C3: {missing} e` citato in matrice e non definito in §5")
    for orphan in sorted(defined - set(placed)):
        failures.append(f"C4: {orphan} e` definito in §5 e non compare in alcuna cella")

    for name in sorted(defined):
        head = text.index(f"#### {name} — ")
        segment = text[head:head + 1200]
        match = re.search(r"\*\*Asset:\*\*(.*?)·\s*\*\*(?:Attore|Severità)", segment, re.S)
        if match is None:
            match = re.search(r"\*\*Attore:\*\*.*?\*\*Asset:\*\*(.*?)·\s*\*\*Severità", segment, re.S)
        declared = set(re.findall(r"A-\d\d", match.group(1))) if match else set()
        actual = placed.get(name, set())
        if declared != actual:
            failures.append(
                f"C5: {name} dichiara {sorted(declared)} e compare in {sorted(actual)}"
                f" — dichiarati non collocati {sorted(declared - actual)},"
                f" collocati non dichiarati {sorted(actual - declared)}"
            )

    print(f"celle: {n_cells}  coperte: {n_cov}  n/a: {n_na}  scenari: {len(defined)}")
    if failures:
        for line in failures:
            print(f"FAIL {line}")
        return 1
    print("OK: matrice e scenari coerenti")
    return 0


if __name__ == "__main__":
    sys.exit(main())
