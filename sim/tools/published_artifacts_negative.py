"""The negative proof of `published_artifacts.py`.

Precision 3 of [ADR-012]:

    "Lo strumento deve saper fallire, e va verificato in negativo. [...] Un
     falso positivo insegna a non fidarsi, e uno strumento in cui nessuno ha
     fiducia non viene eseguito - il che riporta al punto di partenza. La
     guardia va quindi provata reintroducendo il difetto e osservandola
     fallire: una guardia che non sa fallire non e una guardia."

The proof is versioned rather than pasted into a transcript once, because a
transcript proves the guard failed on the day somebody ran it and this proves
it every time. For each of the eleven defect classes the harness copies the
tree to a temporary directory, reintroduces exactly one defect there, runs the
tool against the copy, and requires that it exit non-zero **naming that class**.
It then runs the unmutated copy and requires a clean pass, which is the other
half and the one [SPEC-009] paid for: a guard that fails on everything is as
useless as one that fails on nothing.

**The class-level proof is not enough for C10, and [SPEC-016] added the rest.**
Proving that *one* probe can fail says nothing about the other ninety-seven: a
pattern written against text that has since been rewritten still parses, still
runs, and still passes — it has simply stopped pinning anything. So
`prove_every_probe` deletes each probe's own pinned passage from its own
document and requires the tool to fail **naming that probe by id**. A probe
that survives the deletion of what it claims to pin is reported as
`UNREACHABLE`, because it is a calculation wearing a guard's name.

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
import tomllib

REPO = pathlib.Path(__file__).resolve().parents[2]
COPIED = ("docs/protocol", "sim/tools", "core/coblox-core/tests")
# Single files, copied alongside the trees above. `SECURITY.md` is a claim
# document (C11) and `.lmbrain/knowledge/threat-model.md` is the source the
# derived counts are recomputed from, so both must exist in the copy.
COPIED_FILES = (
    "SECURITY.md",
    "README.md",
    "AGENTS.md",
    ".lmbrain/knowledge/threat-model.md",
)

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
    (
        "C11-CLAIMDOC",
        "SECURITY.md grows a digest literal, so the probe-only treatment it is "
        "given has stopped being the right one and the sweep would otherwise "
        "keep measuring the smaller set - [SPEC-016] RF-001 exactly",
        lambda root: _sub(
            root,
            "SECURITY.md",
            "## Supply chain",
            "Digest: sha256:"
            "993b24bf6115fbf5651d615ca57a1baa825baf304b1dcc4d52debbc7fa3bd6d8 "
            "## Supply chain",
        ),
    ),
    (
        "C11-CLAIMDOC",
        "a published markdown appears that is in none of the three "
        "classifications - the [REVIEW-027] RF-005 scenario, in which a new "
        "SECURITY-OVERVIEW.md claims Sybil resistance and the sweep stays green",
        lambda root: (root / "SECURITY-OVERVIEW.md").write_text(
            "# Overview\n\nCoblox is Sybil-resistant and prevents a validator "
            "cartel from stretching the chain.\n",
            encoding="utf-8",
        ),
    ),
    (
        "C11-CLAIMDOC",
        "a document parked in the `unswept` bucket starts making a security "
        "claim, which is the way the closure check above would be bypassed by "
        "reclassifying instead of by hiding",
        lambda root: _sub(
            root,
            "README.md",
            "## Build",
            "Coblox prevents Sybil attacks.\n\n## Build",
        ),
    ),
    (
        "C5-MIRROR",
        "a source file transcribes a published digest and no one records it as "
        "a mirror - the same defect as RF-005 on the other declared list",
        lambda root: _sub(
            root,
            "core/coblox-core/tests/election_degenerate.rs",
            "use coblox_core::",
            "// sha256:2eac8b0a7955a70543eddf975843fb8e4ddf377daef08b61c7b8cde469515697\n"
            "use coblox_core::",
        ),
    ),
    (
        "C10-PROBE",
        "a probe's `claims` field is written in a shape no consumer can read, "
        "which is worse than a wrong claim: check-guide-pairs.mjs reads this "
        "file with a deliberately minimal reader and would SKIP the entry, and "
        "a skipped anchor is indistinguishable from one that holds",
        lambda root: _sub(
            root,
            "sim/tools/published_artifacts.toml",
            'claims = ["there is no fee"]',
            "claims = 1",
        ),
    ),
    (
        "C10-PROBE",
        "two probes claim the same sentence of the guide, which is the shape "
        "the `claims` list exists to avoid - one rule per probe is what "
        "[DEBT-032] will walk, and two probes on one sentence puts the cost "
        "back where it will matter more",
        lambda root: _sub(
            root,
            "sim/tools/published_artifacts.toml",
            'claims = ["there is no fee"]',
            'claims = ["there is no fee", "The register has a closed list of '
            'entry types and none of them confiscates."]',
        ),
    ),
    (
        "C11-CLAIMDOC",
        "a count SECURITY.md transcribes from the threat model drifts away "
        "from it, which is how it came to claim 36 scenarios against 39",
        lambda root: _sub(
            root,
            ".lmbrain/knowledge/threat-model.md",
            "## 10. Test di attacco",
            "### TM-99 - uno scenario nuovo che nessuno ha ricontato\n\n"
            "## 10. Test di attacco",
        ),
    ),
]


def make_copy(tmp: pathlib.Path) -> pathlib.Path:
    root = tmp / "tree"
    for rel in COPIED:
        shutil.copytree(REPO / rel, root / rel, dirs_exist_ok=True)
    for rel in COPIED_FILES:
        target = root / rel
        target.parent.mkdir(parents=True, exist_ok=True)
        shutil.copy2(REPO / rel, target)
    return root


def run_tool(root: pathlib.Path) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        [sys.executable, str(root / "sim/tools/published_artifacts.py")],
        capture_output=True,
        text=True,
        env={**dict(__import__("os").environ), "COBLOX_REPO": str(root)},
        check=False,
    )




def prove_every_probe(failures: list[str]) -> None:
    """The second half of the negative proof, and the one [SPEC-016] added.

    The class-level mutation above proves that *a* C10 probe can fail. It says
    nothing about the other ninety-seven, and a probe that has never been seen
    to fail is a calculation: its pattern may have been written against text
    that no longer exists in that shape, or may match something incidental that
    the passage it claims to pin does not control.

    So each probe is proved individually. The passage its own pattern matches is
    deleted from its own document, and the tool must exit non-zero **naming that
    probe by id**. One tree copy is made and the mutated file is restored after
    each case, because ninety-eight copies of the tree cost more than the proof
    is worth.
    """
    manifest_path = REPO / "sim/tools/published_artifacts.toml"
    with manifest_path.open("rb") as handle:
        manifest = tomllib.load(handle)
    probes = manifest.get("probe", [])
    claim_docs = set(manifest["meta"].get("claim_documents", []))

    def path_of(site: str) -> str:
        # Claim documents are named by their path from the repository root;
        # protocol documents by their bare name inside docs/protocol/.
        return site if site in claim_docs else f"docs/protocol/{site}"

    print("=== C10-PROBE, every probe individually ===")
    print(
        f"deleting each probe's own pinned passage from its own document, "
        f"{len(probes)} case(s)"
    )
    unreachable: list[str] = []
    with tempfile.TemporaryDirectory() as tmpdir:
        root = make_copy(pathlib.Path(tmpdir) / "probes")
        for row in probes:
            target = root / path_of(row["site"])
            original = target.read_text(encoding="utf-8")
            mutated, removed = re.subn(row["pattern"], "", original)
            if removed == 0:
                unreachable.append(
                    f"{row['id']}: its own pattern matches nothing in "
                    f"{row['site']}, so the probe cannot be proved"
                )
                continue
            target.write_text(mutated, encoding="utf-8")
            try:
                result = run_tool(root)
            finally:
                target.write_text(original, encoding="utf-8")
            if result.returncode == 0 or f"probe {row['id']!r}" not in result.stdout:
                unreachable.append(
                    f"{row['id']}: deleting its pinned passage did not make the "
                    f"tool fail naming it (exit={result.returncode})"
                )
    if unreachable:
        for line in unreachable:
            print(f"  UNREACHABLE {line}")
        failures.extend(unreachable)
    else:
        print(f"  every one of the {len(probes)} probes was observed failing")
    print()

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

    prove_every_probe(failures)

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
    classes = len({code for code, _, _ in MUTATIONS})
    print(
        f"negative proof: PASS - {len(MUTATIONS)} mutations across {classes} "
        f"defect classes, plus every probe individually, each observed failing"
    )
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
