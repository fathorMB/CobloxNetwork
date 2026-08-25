"""The negative proof of `published_artifacts.py`.

Precision 3 of [ADR-012]:

    "Lo strumento deve saper fallire, e va verificato in negativo. [...] Un
     falso positivo insegna a non fidarsi, e uno strumento in cui nessuno ha
     fiducia non viene eseguito - il che riporta al punto di partenza. La
     guardia va quindi provata reintroducendo il difetto e osservandola
     fallire: una guardia che non sa fallire non e una guardia."

The proof is versioned rather than pasted into a transcript once, because a
transcript proves the guard failed on the day somebody ran it and this proves
it every time. For each of the ten defect classes the harness copies the tree
to a temporary directory, reintroduces exactly one defect there, runs the tool
against the copy, and requires that it exit non-zero **naming that class**. It
then runs the unmutated copy and requires a clean pass, which is the other half
and the one [SPEC-009] paid for: a guard that fails on everything is as useless
as one that fails on nothing.

    python sim/tools/published_artifacts_negative.py

The working tree is never modified. Every mutation happens in a copy under the
system temporary directory and is deleted afterwards.
"""

from __future__ import annotations

import pathlib
import re
import shutil
import subprocess
import sys
import tempfile

REPO = pathlib.Path(__file__).resolve().parents[2]
COPIED = ("docs/protocol", "sim/tools", "core/coblox-core/tests")

Mutation = tuple[str, str, str]  # code, description, target path relative to repo


def _sub(root: pathlib.Path, rel: str, old: str, new: str, *, count: int = 1) -> None:
    path = root / rel
    text = path.read_text(encoding="utf-8")
    if text.count(old) < count:
        raise SystemExit(
            f"negative proof cannot set up: {old!r} not found in {rel}. "
            f"The mutation is stale, which means this harness is now lying "
            f"about what it proves."
        )
    path.write_text(text.replace(old, new, count), encoding="utf-8")


# Each entry: defect class, what the defect is, and how to reintroduce it.
MUTATIONS: list[tuple[str, str, callable]] = [
    (
        "C1-DOMAIN",
        "a new domain-separation string is added to a document and nobody "
        "records it as a published artifact",
        lambda root: _sub(
            root,
            "docs/protocol/wire.md",
            "## Signed envelope",
            "## Signed envelope\n\nDomain: `coblox-brand-new-v0`.",
        ),
    ),
    (
        "C2-TAG",
        "a new tagged tree is introduced with a tag byte the inventory has "
        "never seen",
        lambda root: _sub(
            root,
            "docs/protocol/ledger.md",
            "merkle_node = H(0x01 || left_32 || right_32)",
            "merkle_node = H(0x01 || left_32 || right_32)\nnew_node    = H(0x50 || left_32 || right_32)",
        ),
    ),
    (
        "C3-FIXTURE-ID",
        "a document names a conformance fixture that is in no inventory",
        lambda root: _sub(
            root,
            "docs/protocol/README.md",
            "Conformance suites MUST reconstruct",
            "Fixture `NEW-9` is left undefined. Conformance suites MUST reconstruct",
        ),
    ),
    (
        "C4-VALUE",
        "a published digest is edited, which is the shape of a fixture that "
        "silently stops matching what it claims",
        lambda root: _sub(
            root,
            "docs/protocol/README.md",
            "sha256:2eac8b0a7955a70543eddf975843fb8e4ddf377daef08b61c7b8cde469515697",
            "sha256:2eac8b0a7955a70543eddf975843fb8e4ddf377daef08b61c7b8cde469515698",
        ),
    ),
    (
        "C5-MIRROR",
        "the transcription in the coblox-core conformance suite drifts away "
        "from the document it transcribes",
        lambda root: _sub(
            root,
            "core/coblox-core/tests/conformance_registry.rs",
            "sha256:a881e2e0907aa86b225aaa2a2e1898afda1ce4733bd6d9cb390475ded4737e9d",
            "sha256:a881e2e0907aa86b225aaa2a2e1898afda1ce4733bd6d9cb390475ded4737e9e",
        ),
    ),
    (
        "C6-ORPHAN",
        "a published artifact is deleted from the documents while the "
        "inventory keeps asserting it - the tool going stale, which is the "
        "false positive of [SPEC-009]",
        lambda root: _sub(
            root,
            "docs/protocol/ledger.md",
            "eligible_empty = H(0x26)",
            "eligible_empty = (removed)",
        ),
    ),
    (
        "C7-COVERAGE",
        "a preimage is declared covered by a fixture that does not exist",
        lambda root: _sub(
            root,
            "sim/tools/published_artifacts.toml",
            'id = "app_leaf"\nsite = "README.md"\ncoverage = "fixture"\nfixture = "APP-0"',
            'id = "app_leaf"\nsite = "README.md"\ncoverage = "fixture"\nfixture = "APP-9"',
        ),
    ),
    (
        "C8-ENCODING",
        "the lifecycle_u8 encoding table is removed from the document while "
        "the preimage still commits the byte - [DEBT-012] exactly",
        lambda root: _sub(
            root,
            "docs/protocol/ledger.md",
            "| `active` | `0x01` |",
            "| `active` | (unspecified) |",
        ),
    ),
    (
        "C9-EXAMPLE",
        "an inline example stops satisfying an equality the specification "
        "states between its own fields - the defect this pass found",
        lambda root: _sub(
            root,
            "docs/protocol/ledger.md",
            '"request_hash":"sha256:3d56e5dd5104a2ad5c733fa4f0b6d8f35de2f68509e9c10a3d473128eaec0b21"',
            '"request_hash":"sha256:e14d4c02c41a950c9f4f4464e9f98a6652c64e6c992efc36c97f01d2f4ca2dc2"',
        ),
    ),
    (
        "C10-PROBE",
        "the declaration that v0 does not enforce the block interval is "
        "quietly dropped, leaving a cadence that reads as enforced",
        lambda root: _sub(
            root,
            "docs/protocol/README.md",
            "**`block_interval_seconds = 5` is declared, not enforced.**",
            "The block interval is 5 seconds.",
        ),
    ),
]


def make_copy(tmp: pathlib.Path) -> pathlib.Path:
    root = tmp / "tree"
    for rel in COPIED:
        shutil.copytree(REPO / rel, root / rel, dirs_exist_ok=True)
    return root


def run_tool(root: pathlib.Path) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        [sys.executable, str(root / "sim/tools/published_artifacts.py")],
        capture_output=True,
        text=True,
        env={**dict(__import__("os").environ), "COBLOX_REPO": str(root)},
        check=False,
    )


def main() -> int:
    failures: list[str] = []

    with tempfile.TemporaryDirectory() as tmpdir:
        clean = make_copy(pathlib.Path(tmpdir) / "clean")
        result = run_tool(clean)
        print("=== control: the unmutated copy ===")
        print(result.stdout.strip().splitlines()[-1] if result.stdout else result.stderr)
        if result.returncode != 0:
            failures.append(
                "control run failed; a guard that rejects a correct tree is the "
                "false positive [ADR-012] was written about"
            )
        print()

    for code, description, mutate in MUTATIONS:
        with tempfile.TemporaryDirectory() as tmpdir:
            root = make_copy(pathlib.Path(tmpdir) / "case")
            mutate(root)
            result = run_tool(root)
            named = f"FAIL {code}" in result.stdout
            caught = result.returncode != 0 and named
            print(f"=== {code} ===")
            print(f"defect reintroduced: {description}")
            for line in result.stdout.splitlines():
                if line.startswith("FAIL "):
                    print(f"  {line}")
            print(f"  exit={result.returncode} names {code}: {named}")
            if not caught:
                failures.append(
                    f"{code}: the tool did not fail naming this class "
                    f"(exit={result.returncode})"
                )
                if result.stderr.strip():
                    print(f"  stderr: {result.stderr.strip()}")
            print()

    if failures:
        print("negative proof: FAIL")
        for line in failures:
            print(f"  {line}")
        return 1
    print(f"negative proof: PASS - {len(MUTATIONS)} defect classes, each observed failing")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
