"""The containment of the non-consensus signing-preimage escape hatch.

`SigningPreimage::from_raw_bytes_non_consensus` builds a signing preimage out of
arbitrary bytes, that is, without the `domain || 0x00 || chain_id` prefix that
binds a Coblox signature to its domain and to its chain. It exists because the
upstream `ed25519-speccheck` vectors sign raw non-Coblox messages and the
conformance suite has to be able to express them. On a consensus path it is an
acceptance rule, not an error: a vote gathered on a devnet verifies on mainnet.

[REVIEW-023] RF-001 found that the hatch was named but not contained -- `pub`,
in a `pub mod`, behind no feature -- and therefore reachable from `coblox-node`,
`coblox-ffi` and the desktop shell in a **production** build. The primary remedy
is a compilation boundary: the constructor is now behind the non-default
`conformance-testing` feature, enabled only by the dev-dependency `coblox-core`
declares on itself. That boundary is real and is proved in the negative in the
SPEC-014 evidence.

**This tool is the second half, and it is a lint, not a boundary.** It exists
because the boundary has one declared limit: `cargo test --workspace` builds
dev-dependencies and cargo unifies features across a single invocation, so the
feature is on for the whole graph during that command and a call written in the
*test* code of another crate would compile. It also exists because a boundary
made of one line in one manifest can be undone by one line in another manifest,
and nothing would go red.

    python sim/tools/non_consensus_containment.py     # run every check
    python sim/tools/non_consensus_containment.py --negative   # prove it fails

Three defect classes are checked. Each one is reachable: reintroduce the defect
and the tool exits non-zero naming it.

    N1  CALL-SITE   the constructor is named in a file that is not allowed to
                    name it -- anything outside `core/coblox-core/src/registry.rs`
                    and `core/coblox-core/tests/`
    N2  GATE        the `#[cfg(feature = "conformance-testing")]` attribute no
                    longer guards the constructor, or the feature has been made
                    default, or the feature is no longer declared at all
    N3  ENABLED     a manifest other than `coblox-core`'s own dev-dependency on
                    itself turns the feature on, which would re-open the hatch
                    for that crate's production build

**What this tool does not cover**, stated because a guard whose limits are not
written is read as covering everything:

  - it is textual. A call reached through a re-export under another name, or
    through a macro that assembles the identifier, is invisible to it. The
    compilation boundary is what covers those, wherever the feature is off.
  - it says nothing about *semantic* misuse of `signing_preimage` itself, that
    is, a preimage built with the wrong `Domain` for the message at hand. That
    residual is [REVIEW-023] RF-002 and is a debt of its own, not this check.
  - `.lmbrain/` is excluded: specs and reviews discuss the constructor by name,
    which is the point of them. Only source, manifests, scripts and workflows
    are scanned.
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

# `COBLOX_REPO` exists for one caller: the `--negative` harness below, which
# proves this tool can fail by mutating a throwaway copy of the tree. Pointing
# the harness at a copy instead of at the working tree means the negative proof
# cannot leave a defect behind if it is interrupted.
REPO = pathlib.Path(
    os.environ.get("COBLOX_REPO") or pathlib.Path(__file__).resolve().parents[2]
)

HATCH = "from_raw_bytes_non_consensus"
FEATURE = "conformance-testing"

CORE_MANIFEST = "core/coblox-core/Cargo.toml"
REGISTRY = "core/coblox-core/src/registry.rs"
TESTS_DIR = "core/coblox-core/tests/"

# The two places allowed to name the hatch in code: where it is defined, and the
# conformance suite it exists for. This tool and the core manifest name it in
# prose about the containment itself, which is not a call site.
ALLOWED_PREFIXES = (REGISTRY, TESTS_DIR)
ALLOWED_PROSE = (CORE_MANIFEST, "sim/tools/non_consensus_containment.py")

SCANNED_SUFFIXES = (".rs", ".toml", ".py", ".yml", ".yaml", ".sh", ".kt", ".ts", ".tsx")
SKIPPED_DIRS = {".git", ".lmbrain", "target", "node_modules", "dist", "__pycache__"}

findings: list[str] = []


def fail(code: str, message: str) -> None:
    findings.append(f"{code}: {message}")


def read(rel: str) -> str:
    path = REPO / rel
    if not path.is_file():
        fail("N2-GATE", f"{rel} does not exist; the containment cannot be checked.")
        return ""
    return path.read_text(encoding="utf-8")


def scanned_files() -> list[pathlib.Path]:
    out: list[pathlib.Path] = []
    for root, dirs, files in os.walk(REPO):
        dirs[:] = [d for d in dirs if d not in SKIPPED_DIRS]
        for name in files:
            path = pathlib.Path(root) / name
            if path.suffix in SCANNED_SUFFIXES:
                out.append(path)
    return sorted(out)


def check_call_sites() -> None:
    """N1: the hatch is named only where it is allowed to be named."""
    for path in scanned_files():
        rel = path.relative_to(REPO).as_posix()
        if rel.startswith(ALLOWED_PREFIXES) or rel in ALLOWED_PROSE:
            continue
        try:
            text = path.read_text(encoding="utf-8")
        except UnicodeDecodeError:
            continue
        for number, line in enumerate(text.splitlines(), start=1):
            if HATCH in line:
                fail(
                    "N1-CALL-SITE",
                    f"{rel}:{number} names `{HATCH}`. The non-consensus "
                    f"constructor may be named only in {REGISTRY} and under "
                    f"{TESTS_DIR}. A preimage built from raw bytes carries no "
                    f"domain and no chain_id, so a signature verified over it "
                    f"is bound to neither.",
                )


def check_gate() -> None:
    """N2: the compilation boundary is still in place."""
    manifest = read(CORE_MANIFEST)
    if not manifest:
        return

    if not re.search(rf"^{re.escape(FEATURE)}\s*=", manifest, re.MULTILINE):
        fail(
            "N2-GATE",
            f"{CORE_MANIFEST} no longer declares the `{FEATURE}` feature. "
            f"Without it the `#[cfg(feature = ...)]` on the constructor is a "
            f"cfg that is never true, or worse, was removed with it.",
        )

    default = re.search(r"^default\s*=\s*\[(.*?)\]", manifest, re.MULTILINE | re.DOTALL)
    if default and FEATURE in default.group(1):
        fail(
            "N2-GATE",
            f"{CORE_MANIFEST} lists `{FEATURE}` in the default feature set. "
            f"The whole point of the feature is that a production build of a "
            f"dependant does not have it.",
        )

    registry = read(REGISTRY)
    if not registry:
        return

    definition = re.search(rf"^\s*pub fn {re.escape(HATCH)}\b", registry, re.MULTILINE)
    if definition is None:
        fail(
            "N2-GATE",
            f"{REGISTRY} no longer defines `{HATCH}`. If the hatch was removed "
            f"on purpose this tool should go with it; if it moved, the "
            f"containment moved with it and is no longer checked.",
        )
        return

    preceding = registry[: definition.start()]
    attributes = preceding.rsplit("///", 1)[-1]
    if f'#[cfg(feature = "{FEATURE}")]' not in attributes:
        fail(
            "N2-GATE",
            f"{REGISTRY} defines `{HATCH}` without an immediately preceding "
            f'`#[cfg(feature = "{FEATURE}")]`. The constructor is compiled into '
            f"every dependant's production build again.",
        )


def check_who_enables_it() -> None:
    """N3: only coblox-core's dev-dependency on itself turns the feature on."""
    for path in scanned_files():
        rel = path.relative_to(REPO).as_posix()
        if path.name != "Cargo.toml" or rel == "sim/tools/non_consensus_containment.py":
            continue
        try:
            text = path.read_text(encoding="utf-8")
        except UnicodeDecodeError:
            continue
        for number, line in enumerate(text.splitlines(), start=1):
            stripped = line.strip()
            if stripped.startswith("#") or FEATURE not in stripped:
                continue
            # The feature's own declaration, and the prose above it, are not an
            # enablement. An enablement is a `features = [...]` list.
            if "features" not in stripped or "=" not in stripped:
                continue
            if rel == CORE_MANIFEST and 'path = "."' in stripped:
                continue
            fail(
                "N3-ENABLED",
                f"{rel}:{number} enables `{FEATURE}` on a dependency. Only "
                f"{CORE_MANIFEST}'s dev-dependency on itself may do so; any "
                f"other enablement compiles the non-consensus constructor into "
                f"that crate's production build, which is the containment "
                f"undone by one line in a manifest.",
            )


# --------------------------------------------------------------------------
# The negative proof. Precision 3 of [ADR-012]: a guard that cannot be shown to
# fail is not a guard. For each defect class the harness copies the tree to a
# temporary directory, reintroduces exactly one defect there, runs this tool
# against the copy, and requires a non-zero exit **naming that class**. It then
# runs the unmutated copy and requires a clean pass, which is the other half:
# a guard that fails on everything is as useless as one that fails on nothing.
# The working tree is never modified.
# --------------------------------------------------------------------------

COPIED = ("core", "sim/tools", ".github")


def _sub(root: pathlib.Path, rel: str, old: str, new: str) -> None:
    path = root / rel
    text = path.read_text(encoding="utf-8")
    if old not in text:
        raise SystemExit(
            f"negative proof cannot set up: {old!r} not found in {rel}. "
            f"The mutation is stale, which means this harness is now lying "
            f"about what it proves."
        )
    path.write_text(text.replace(old, new, 1), encoding="utf-8")


def _append(root: pathlib.Path, rel: str, text: str) -> None:
    path = root / rel
    path.write_text(path.read_text(encoding="utf-8") + text, encoding="utf-8")


MUTATIONS = [
    (
        "N1-CALL-SITE",
        "the first consensus caller takes the shortest conversion that "
        "compiles and builds a vote preimage from bytes off the wire",
        lambda root: _append(
            root,
            "core/coblox-node/src/main.rs",
            "\nfn verify_vote(bytes: &[u8]) -> bool {\n"
            "    let p = coblox_core::SigningPreimage::from_raw_bytes_non_consensus(bytes);\n"
            "    coblox_core::verifier::verify_consensus_ed25519(&[0u8; 32], &p, &[0u8; 64])\n"
            "}\n",
        ),
    ),
    (
        "N2-GATE",
        "the cfg attribute is dropped from the constructor, which puts it back "
        "into every dependant's production build",
        lambda root: _sub(
            root,
            REGISTRY,
            f'#[cfg(feature = "{FEATURE}")]\n    #[must_use]\n    pub fn {HATCH}',
            f"#[must_use]\n    pub fn {HATCH}",
        ),
    ),
    (
        "N2-GATE",
        "the feature is made default, which enables it for every dependant",
        lambda root: _sub(
            root,
            CORE_MANIFEST,
            f"{FEATURE} = []",
            f'{FEATURE} = []\ndefault = ["{FEATURE}"]',
        ),
    ),
    (
        "N3-ENABLED",
        "a dependant turns the feature on for itself, undoing the containment "
        "with one line in a manifest",
        lambda root: _sub(
            root,
            "core/coblox-node/Cargo.toml",
            'coblox-core = { path = "../coblox-core" }',
            'coblox-core = { path = "../coblox-core", features = ["'
            + FEATURE
            + '"] }',
        ),
    ),
]


def _run_against(root: pathlib.Path) -> subprocess.CompletedProcess[str]:
    environment = dict(os.environ, COBLOX_REPO=str(root))
    return subprocess.run(
        [sys.executable, str(root / "sim" / "tools" / pathlib.Path(__file__).name)],
        capture_output=True,
        text=True,
        env=environment,
        check=False,
    )


def negative_proof() -> int:
    failures = 0
    with tempfile.TemporaryDirectory(prefix="coblox-containment-") as temporary:
        pristine = pathlib.Path(temporary) / "pristine"
        for rel in COPIED:
            shutil.copytree(REPO / rel, pristine / rel, dirs_exist_ok=True)

        clean = _run_against(pristine)
        if clean.returncode != 0:
            failures += 1
            print("NOT PROVED: the unmutated copy does not pass.")
            print(clean.stdout + clean.stderr)
        else:
            print("ok   unmutated copy passes")

        for index, (code, description, mutate) in enumerate(MUTATIONS):
            root = pathlib.Path(temporary) / f"mutant{index}"
            shutil.copytree(pristine, root)
            mutate(root)
            result = _run_against(root)
            if result.returncode == 0 or code not in result.stdout:
                failures += 1
                print(f"NOT PROVED {code}: {description}")
                print(result.stdout + result.stderr)
            else:
                print(f"ok   {code} caught: {description}")

    if failures:
        print(f"\n{failures} defect class(es) not proved reachable.")
        return 1
    print("\nEvery defect class is reachable and the guard names it.")
    return 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--negative",
        action="store_true",
        help="prove the guard fails, one defect class at a time, on a copy",
    )
    arguments = parser.parse_args()

    if arguments.negative:
        return negative_proof()

    check_call_sites()
    check_gate()
    check_who_enables_it()

    if findings:
        for finding in findings:
            print(finding)
        print(f"\n{len(findings)} finding(s).")
        return 1
    print(
        f"ok  `{HATCH}` is named only in {REGISTRY} and under {TESTS_DIR}, "
        f"is gated on the non-default `{FEATURE}` feature, and no manifest "
        f"other than coblox-core's dev-dependency on itself enables it."
    )
    return 0


if __name__ == "__main__":
    sys.exit(main())
