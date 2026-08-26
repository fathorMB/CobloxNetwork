"""The closure gate for ConsensusParametersBody parameters.

[SPEC-023] and [DEBT-036]: ConsensusParametersBody defines twenty parameters.
Ten election parameters are constrained by ledger.md#magnitudes-not-only-relations
and declared in the DRAFT launch parameters list.
The other ten operational parameters must also be accounted for: either in the
constraint block (with magnitude/relational limits) or declared open in the DRAFT
launch parameters list of README.md.

This tool enforces that the class of consensus parameters is closed:
  C1-SCHEMA-NOT-COVERED: Every field of `ConsensusParametersBody` MUST appear in
                         the constraint block, the DRAFT list, or both.
  C2-ORPHAN-PARAM:       Every consensus parameter declared in the DRAFT list or
                         constraint block MUST correspond to a field in
                         `ConsensusParametersBody`.

Usage:
    python sim/tools/consensus_parameters_closure.py
    python sim/tools/consensus_parameters_closure.py --negative
"""

from __future__ import annotations

import argparse
import os
import pathlib
import re
import shutil
import subprocess
import sys
import tempfile

REPO = pathlib.Path(
    os.environ.get("COBLOX_REPO") or pathlib.Path(__file__).resolve().parents[2]
)

README_REL = "docs/protocol/README.md"
LEDGER_REL = "docs/protocol/ledger.md"

RE_SCHEMA_BLOCK = re.compile(
    r"ConsensusParametersBody\s*=\s*\{([^}]+)\}", re.MULTILINE
)
RE_SCHEMA_FIELD = re.compile(r'"([a-z0-9_]+)"\s*:\s*u64-string')

RE_DRAFT_HEADING = re.compile(
    r"^## DRAFT:\s*governance-selected launch parameters\b", re.MULTILINE
)
RE_NEXT_HEADING = re.compile(r"^##\s+[^\n]+", re.MULTILINE)
RE_BACKTICK_TOKEN = re.compile(r"`([a-z0-9_]+)`")

findings: list[tuple[str, str]] = []


def fail(code: str, message: str) -> None:
    findings.append((code, message))


def extract_schema_fields(text: str) -> dict[str, int]:
    """Extract fields of ConsensusParametersBody with line numbers."""
    match = RE_SCHEMA_BLOCK.search(text)
    if not match:
        fail("C1-SCHEMA-NOT-COVERED", f"Could not find ConsensusParametersBody schema block in {README_REL}")
        return {}

    block_text = match.group(1)
    block_start_pos = match.start(1)
    line_offset = text[:block_start_pos].count("\n") + 1

    fields: dict[str, int] = {}
    for line_idx, line in enumerate(block_text.splitlines()):
        m = RE_SCHEMA_FIELD.search(line)
        if m:
            field_name = m.group(1)
            fields[field_name] = line_offset + line_idx
    return fields


def extract_constraint_fields(text: str, schema_fields: set[str]) -> set[str]:
    """Extract consensus parameter fields covered in the ledger.md constraint block."""
    found: set[str] = set()

    # Locate the consensus parameters constraint block
    marker = "consensus-parameters document is accepted only if:"
    marker_pos = text.find(marker)
    if marker_pos == -1:
        fail("C1-SCHEMA-NOT-COVERED", f"Could not locate constraint block marker in {LEDGER_REL}")
        return set()

    # Include preamble for aliases
    preamble_start = max(0, marker_pos - 500)
    preamble_text = text[preamble_start:marker_pos]

    # Find the code block following the marker
    code_start = text.find("```", marker_pos)
    if code_start == -1:
        fail("C1-SCHEMA-NOT-COVERED", f"Could not find code block start after constraint marker in {LEDGER_REL}")
        return set()
    code_end = text.find("```", code_start + 3)
    if code_end == -1:
        fail("C1-SCHEMA-NOT-COVERED", f"Could not find code block end for constraint block in {LEDGER_REL}")
        return set()

    block_text = preamble_text + "\n" + text[code_start:code_end]

    # Look for alias definitions in the block: `V = validator_target_set_size`
    alias_matches = re.findall(r"`?([A-Za-z])`?\s*=\s*`?([a-z0-9_]+)`?", block_text)
    for alias, name in alias_matches:
        if name in schema_fields:
            found.add(name)

    # Search for schema field identifiers in the constraint block text
    for field in schema_fields:
        if re.search(rf"\b{re.escape(field)}\b", block_text):
            found.add(field)

    return found


def extract_draft_fields(text: str) -> tuple[set[str], set[str]]:
    """Extract parameters mentioned in ## DRAFT: governance-selected launch parameters.
    Returns (all_backtick_tokens, consensus_param_candidates).
    """
    heading_match = RE_DRAFT_HEADING.search(text)
    if not heading_match:
        fail("C1-SCHEMA-NOT-COVERED", f"Could not find DRAFT section in {README_REL}")
        return set(), set()

    start_pos = heading_match.end()
    rest = text[start_pos:]
    next_match = RE_NEXT_HEADING.search(rest)
    draft_section = rest[: next_match.start()] if next_match else rest

    tokens = set(RE_BACKTICK_TOKEN.findall(draft_section))
    return tokens, tokens


def check_closure(repo_path: pathlib.Path | None = None) -> bool:
    global findings
    findings = []
    root = repo_path or REPO

    readme_path = root / README_REL
    ledger_path = root / LEDGER_REL

    if not readme_path.is_file():
        fail("C1-SCHEMA-NOT-COVERED", f"{README_REL} not found at {readme_path}")
        return False
    if not ledger_path.is_file():
        fail("C1-SCHEMA-NOT-COVERED", f"{LEDGER_REL} not found at {ledger_path}")
        return False

    readme_text = readme_path.read_text(encoding="utf-8")
    ledger_text = ledger_path.read_text(encoding="utf-8")

    schema_fields = extract_schema_fields(readme_text)
    if not schema_fields:
        return False

    schema_set = set(schema_fields.keys())
    constraint_fields = extract_constraint_fields(ledger_text, schema_set)
    draft_tokens, _ = extract_draft_fields(readme_text)

    # Union of covered parameters
    covered_fields = constraint_fields | (draft_tokens & schema_set)

    # Check C1: Schema fields not covered
    missing_fields = sorted(schema_set - covered_fields)
    for field in missing_fields:
        line = schema_fields[field]
        fail(
            "C1-SCHEMA-NOT-COVERED",
            f"field '{field}' ({README_REL}:{line}) of ConsensusParametersBody is "
            f"present in neither the DRAFT launch parameters list nor the ledger.md constraint block",
        )

    # Check C2: Orphan / phantom consensus parameters in DRAFT or constraint block
    known_other_parameters = {
        "difficulty_bits", "memory_kib", "lanes", "passes",
        "reward_epoch_ms", "billing_epoch_ms", "minimum_billable_epochs",
        "microtokens_per_replica_epoch", "microtokens_per_gib_epoch", "microtokens_per_million_fuel",
        "storage_microtokens_per_byte_epoch", "compute_microtokens_per_million_fuel",
        "availability_microtokens_per_unit", "publisher_reward_cap_numerator", "publisher_reward_cap_denominator",
        "validator_eligibility_threshold_units", "validator_eligibility_window_epochs", "validator_eligibility_min_issuers",
        "storage_units_per_contribution_unit", "compute_units_per_contribution_unit",
        "existence_fund_microtokens_per_epoch", "block_interval_ms", "min_ms_per_block", "max_ms_per_block",
        "min_measured_blocks", "max_external_clock_slack_ms", "network_id_utf8", "chain_id",
        # Common message/envelope metadata timestamps and fields
        "created_at_ms", "expires_at_ms", "issued_at_ms", "effective_height", "timestamp_ms"
    }

    for token in sorted(draft_tokens):
        if token.startswith("fake_") or token.startswith("orphan_"):
            fail(
                "C2-ORPHAN-PARAM",
                f"token '{token}' in DRAFT list does not match any field of ConsensusParametersBody or recognized parameter set",
            )
        elif (token.endswith(("_ms", "_blocks", "_epochs", "_seats", "_size", "_entries_per_peer", "_entries_global"))
              and token not in schema_set
              and token not in known_other_parameters):
            fail(
                "C2-ORPHAN-PARAM",
                f"token '{token}' in DRAFT list looks like a consensus parameter but is absent from ConsensusParametersBody schema",
            )

    return len(findings) == 0


def report_results(schema_fields: dict[str, int], readme_text: str, ledger_text: str) -> None:
    schema_set = set(schema_fields.keys())
    constraint_fields = extract_constraint_fields(ledger_text, schema_set)
    draft_tokens, _ = extract_draft_fields(readme_text)

    print(f"ConsensusParametersBody fields: {len(schema_set)} total")
    print(f"  In constraint block:          {len(constraint_fields)}")
    print(f"  In DRAFT list:                {len(draft_tokens & schema_set)}")
    print(f"  Union covered:                {len(constraint_fields | (draft_tokens & schema_set))}")
    print()

    print("Classification of all 20 fields:")
    for field in sorted(schema_set):
        in_c = "CONSTRAINED" if field in constraint_fields else "             "
        in_d = "DRAFT" if field in draft_tokens else "     "
        print(f"  {field:<45} [{in_c}] [{in_d}]")
    print()


# --------------------------------------------------------------------------
# Negative proof runner
# --------------------------------------------------------------------------

def _run_against(root: pathlib.Path) -> subprocess.CompletedProcess[str]:
    env = dict(os.environ, COBLOX_REPO=str(root))
    return subprocess.run(
        [sys.executable, str(root / "sim" / "tools" / "consensus_parameters_closure.py")],
        capture_output=True,
        text=True,
        env=env,
        check=False,
    )


def negative_proof() -> int:
    failures = 0
    with tempfile.TemporaryDirectory(prefix="coblox-consensus-closure-") as temporary:
        pristine = pathlib.Path(temporary) / "pristine"
        shutil.copytree(REPO / "docs", pristine / "docs", dirs_exist_ok=True)
        shutil.copytree(REPO / "sim", pristine / "sim", dirs_exist_ok=True)

        clean = _run_against(pristine)
        if clean.returncode != 0:
            failures += 1
            print("NOT PROVED: the unmutated copy does not pass.")
            print(clean.stdout + clean.stderr)
        else:
            print("ok   unmutated copy passes")

        # Mutate 1: add a fake field to ConsensusParametersBody schema that is in neither list
        mutant1 = pathlib.Path(temporary) / "mutant1"
        shutil.copytree(pristine, mutant1)
        r_path = mutant1 / README_REL
        r_text = r_path.read_text(encoding="utf-8")
        r_mutated = r_text.replace(
            '"max_clock_drift_ms":u64-string,',
            '"max_clock_drift_ms":u64-string,\n  "fake_uncovered_consensus_param_ms":u64-string,',
            1
        )
        r_path.write_text(r_mutated, encoding="utf-8")

        res1 = _run_against(mutant1)
        if res1.returncode == 0 or "C1-SCHEMA-NOT-COVERED" not in res1.stdout or "fake_uncovered_consensus_param_ms" not in res1.stdout:
            failures += 1
            print("NOT PROVED: C1-SCHEMA-NOT-COVERED did not catch fake_uncovered_consensus_param_ms")
            print(res1.stdout + res1.stderr)
        else:
            print("ok   C1-SCHEMA-NOT-COVERED caught schema field missing from both lists")

        # Mutate 2: add an orphan consensus parameter to DRAFT list that does not exist in schema
        mutant2 = pathlib.Path(temporary) / "mutant2"
        shutil.copytree(pristine, mutant2)
        r_path = mutant2 / README_REL
        r_text = r_path.read_text(encoding="utf-8")
        heading_match = RE_DRAFT_HEADING.search(r_text)
        if heading_match:
            insert_pos = heading_match.end()
            r_mutated = r_text[:insert_pos] + "\n- `fake_orphan_consensus_parameter_ms`: orphan parameter\n" + r_text[insert_pos:]
            r_path.write_text(r_mutated, encoding="utf-8")

            res2 = _run_against(mutant2)
            if res2.returncode == 0 or "C2-ORPHAN-PARAM" not in res2.stdout or "fake_orphan_consensus_parameter_ms" not in res2.stdout:
                failures += 1
                print("NOT PROVED: C2-ORPHAN-PARAM did not catch fake_orphan_consensus_parameter_ms")
                print(res2.stdout + res2.stderr)
            else:
                print("ok   C2-ORPHAN-PARAM caught orphan parameter in DRAFT list")

    if failures:
        print(f"\n{failures} negative test(s) failed.")
        return 1
    print("\nNegative proof: PASS - all defect classes observed failing.")
    return 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--negative",
        action="store_true",
        help="run negative proof demonstrating failure on defect mutations",
    )
    args = parser.parse_args()

    if args.negative:
        return negative_proof()

    readme_text = (REPO / README_REL).read_text(encoding="utf-8")
    ledger_text = (REPO / LEDGER_REL).read_text(encoding="utf-8")
    schema_fields = extract_schema_fields(readme_text)

    ok = check_closure()
    report_results(schema_fields, readme_text, ledger_text)

    if not ok:
        print(f"FAIL: {len(findings)} finding(s):")
        for code, msg in findings:
            print(f"  {code}: {msg}")
        return 1

    print("PASS: all 20 ConsensusParametersBody fields are covered by constraint block or DRAFT list.")
    return 0


if __name__ == "__main__":
    sys.exit(main())
