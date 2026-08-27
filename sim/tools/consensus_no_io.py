"""The consensus engine has no I/O, checked over the source instead of claimed.

`SPEC-025` requires that the engine's freedom from I/O be **shown by the shape
of its interface, not asserted in a comment**. The primary demonstration is the
interface itself: `Engine::step_event` takes an `Event` and returns
`Vec<Action>`, both plain-data enums, so there is no callback to hand it and no
seam an implementor could route a socket through.

This tool is the second half, and it exists for the reason [REVIEW-022] gave
about `pub(crate)`: a guarantee held by a name is a guarantee one line can undo,
and no build fails when it is undone. It is a **lint, not a boundary**, and it
says so here rather than being described as one later.

    python sim/tools/consensus_no_io.py
    python sim/tools/consensus_no_io.py --negative

Three defect classes are checked. Each one is reachable: reintroduce the defect
and the tool exits non-zero naming it.

    N1  IO-PATH    a file under core/coblox-core/src/consensus/ names a clock, a
                   socket, a file, a process, a thread, an environment variable
                   or a randomness source
    N2  ENGINE-SEAM
                   engine.rs contains a generic parameter, a trait object, a
                   closure parameter, a function pointer, or an interior-mutability
                   or shared-ownership container. Any of those is a place a caller
                   could supply behaviour, and behaviour supplied from outside is
                   where I/O would enter a type that otherwise cannot reach it
    N3  BOUND      a generic parameter elsewhere under the consensus module is
                   bounded by something other than `SignatureVerifier`. That one
                   seam is deliberate, pre-dates this module, and lives at the
                   message boundary rather than inside the engine; a second seam
                   would be a new one, arriving without a decision

**What this tool does not cover**, stated because a guard whose limits are not
written is read as covering everything. It reads text. It does not prove that
`ValidatorSet`, `JsonObject` or any other type named in a signature is itself
free of I/O — that follows from those types being plain data in this crate, which
is a property of the crate and not something a regular expression establishes.
It does not run the compiler. And it says nothing about `coblox-node`, which is
where I/O is supposed to be.
"""

from __future__ import annotations

import argparse
import pathlib
import re
import shutil
import sys
import tempfile

REPO = pathlib.Path(__file__).resolve().parents[2]
MODULE = pathlib.Path("core/coblox-core/src/consensus")
ENGINE = MODULE / "engine.rs"

# Every path by which Rust standard library code reaches outside the process, and
# the two non-std spellings a consensus engine would plausibly grow. The list is
# of *names*, because that is what a text guard can see; a name reached through an
# alias would pass, which is the residual named in the module docstring.
IO_PATHS = (
    "std::fs",
    "std::net",
    "std::io",
    "std::process",
    "std::thread",
    "std::env",
    "std::time",
    "SystemTime",
    "Instant",
    "TcpStream",
    "TcpListener",
    "UdpSocket",
    "File::",
    "OpenOptions",
    "include_str!",
    "include_bytes!",
    "env!",
    "option_env!",
    "thread_rng",
    "OsRng",
    "getrandom",
)

# Shapes through which a caller supplies behaviour rather than data.
SEAM_PATTERNS = (
    (r"\bdyn\s", "a trait object"),
    (r"\bimpl\s+Fn(Mut|Once)?\b", "a closure parameter"),
    (r"\bfn\s*\(", "a function-pointer type"),
    (r"\bRc<", "a shared-ownership container"),
    (r"\bArc<", "a shared-ownership container"),
    (r"\bRefCell<", "an interior-mutability container"),
    (r"\bCell<", "an interior-mutability container"),
    (r"\bMutex<", "an interior-mutability container"),
    (r"\bRwLock<", "an interior-mutability container"),
    (r"\*const\s", "a raw pointer"),
    (r"\*mut\s", "a raw pointer"),
)

# A generic parameter declaration: `fn name<...>` or `impl<...>` or `struct X<...>`.
GENERIC = re.compile(r"\b(fn|struct|enum|impl|trait)\s+[A-Za-z_][A-Za-z0-9_]*\s*<([^>]*)>")
IMPL_GENERIC = re.compile(r"\bimpl\s*<([^>]*)>")

# The one bound the consensus module is allowed to be generic over.
ALLOWED_BOUND = "SignatureVerifier"


class Report:
    def __init__(self) -> None:
        self.findings: list[tuple[str, str]] = []
        self.notes: list[tuple[str, int]] = []

    def fail(self, code: str, message: str) -> None:
        self.findings.append((code, message))

    def note(self, code: str, count: int) -> None:
        self.notes.append((code, count))

    def render(self, stream) -> int:
        for code, count in self.notes:
            print(f"  {code:<16} {count} candidate(s) checked", file=stream)
        print(file=stream)
        for code, message in self.findings:
            print(f"FAIL {code}: {message}", file=stream)
        if self.findings:
            print(
                f"\nconsensus engine no-I/O lint: FAIL ({len(self.findings)} finding(s))",
                file=stream,
            )
            return 1
        print("consensus engine no-I/O lint: PASS", file=stream)
        return 0


def module_files(root: pathlib.Path) -> list[pathlib.Path]:
    directory = root / MODULE
    if not directory.is_dir():
        raise SystemExit(
            f"{MODULE} is not a directory under {root}. The consensus module has "
            f"moved or been renamed, and this lint is now checking nothing."
        )
    return sorted(directory.glob("*.rs"))


def strip_comments(text: str) -> str:
    """Removes line comments, so a docstring may *name* what the code may not.

    The module documentation of `consensus/mod.rs` has to be able to say the word
    `SystemTime` in order to explain that the engine never reaches one, and a
    guard that forbade the explanation along with the behaviour would be the
    trade [REVIEW-029] describes: a working fence given up for a nicer paragraph.
    Block comments are not stripped, because nothing in this module uses them.
    """
    return "\n".join(
        "" if line.lstrip().startswith("//") else line for line in text.splitlines()
    )


def check_io_paths(root: pathlib.Path, report: Report) -> None:
    checked = 0
    for path in module_files(root):
        code = strip_comments(path.read_text(encoding="utf-8"))
        for lineno, line in enumerate(code.splitlines(), start=1):
            checked += 1
            for needle in IO_PATHS:
                if needle in line:
                    report.fail(
                        "N1-IO-PATH",
                        f"{path.relative_to(root).as_posix()}:{lineno} names {needle!r}. "
                        f"The consensus engine is a total function of its inputs; a "
                        f"clock, a socket, a file or a randomness source in it would "
                        f"make the adversarial schedule of SPEC-025 unreproducible and "
                        f"the determinism criterion unmeasurable.",
                    )
    report.note("N1-IO-PATH", checked)


def check_engine_seams(root: pathlib.Path, report: Report) -> None:
    path = root / ENGINE
    if not path.is_file():
        raise SystemExit(f"{ENGINE} is missing; this lint is checking nothing.")
    code = strip_comments(path.read_text(encoding="utf-8"))
    checked = 0
    for lineno, line in enumerate(code.splitlines(), start=1):
        checked += 1
        for pattern, description in SEAM_PATTERNS:
            if re.search(pattern, line):
                report.fail(
                    "N2-ENGINE-SEAM",
                    f"{ENGINE.as_posix()}:{lineno} introduces {description}: {line.strip()!r}. "
                    f"The engine takes data and returns data; a seam here is where a "
                    f"caller would supply behaviour, and behaviour supplied from "
                    f"outside is how I/O reaches a type that otherwise cannot name it.",
                )
    for match in GENERIC.finditer(code):
        checked += 1
        report.fail(
            "N2-ENGINE-SEAM",
            f"{ENGINE.as_posix()} declares generic parameters {match.group(2)!r} on "
            f"{match.group(1)} {match.group(0).split()[1]!r}. The engine is generic "
            f"over nothing, and that is the property GATE-NO-IO reads off its "
            f"interface.",
        )
    report.note("N2-ENGINE-SEAM", checked)


def check_bounds(root: pathlib.Path, report: Report) -> None:
    checked = 0
    for path in module_files(root):
        code = strip_comments(path.read_text(encoding="utf-8"))
        for match in list(GENERIC.finditer(code)) + list(IMPL_GENERIC.finditer(code)):
            parameters = match.groups()[-1]
            if not parameters.strip():
                continue
            checked += 1
            bounds = [
                token
                for token in re.split(r"[,:+<>]", parameters)
                if token.strip()
                and token.strip() not in {"?Sized"}
                and token.strip()[0].isupper()
                and len(token.strip()) > 1
            ]
            for bound in bounds:
                if bound.strip() != ALLOWED_BOUND:
                    report.fail(
                        "N3-BOUND",
                        f"{path.relative_to(root).as_posix()} is generic over "
                        f"{bound.strip()!r}. The consensus module has exactly one "
                        f"declared seam, {ALLOWED_BOUND!r}, it lives at the message "
                        f"boundary rather than inside the engine, and it pre-dates "
                        f"this module. A second seam is a decision, not a refactor.",
                    )
    report.note("N3-BOUND", checked)


def run(root: pathlib.Path, stream) -> int:
    report = Report()
    check_io_paths(root, report)
    check_engine_seams(root, report)
    check_bounds(root, report)
    return report.render(stream)


# --------------------------------------------------------------- negative ---

Mutation = tuple[str, str, str, str, str]  # code, description, rel path, old, new

MUTATIONS: tuple[Mutation, ...] = (
    (
        "N1-IO-PATH",
        "the engine learns the time by itself, which is the single change that "
        "would make every adversarial schedule in the suite unreproducible while "
        "leaving the happy path green",
        (MODULE / "engine.rs").as_posix(),
        "        self.round = round;",
        "        let _ = std::time::SystemTime::now();\n        self.round = round;",
    ),
    (
        "N2-ENGINE-SEAM",
        "the engine takes a callback for the value to propose instead of asking "
        "for it and being told - the shape in which a mempool, and with it a "
        "socket, arrives inside the state machine",
        (MODULE / "engine.rs").as_posix(),
        "    /// The current round.",
        "    /// A value source supplied by the caller.\n"
        "    pub fn with_value_source(&mut self, _source: Box<dyn Iterator<Item = u64>>) {}\n\n"
        "    /// The current round.",
    ),
    (
        "N3-BOUND",
        "a second seam appears at the message boundary, so the module is generic "
        "over a trait nobody decided on",
        (MODULE / "messages.rs").as_posix(),
        "pub fn verify_vote<V: SignatureVerifier + ?Sized>(",
        "pub fn verify_vote<V: SignatureVerifier + ?Sized, C: Clone>(",
    ),
)


def prove_negative(stream) -> int:
    failures = 0
    for code, description, rel, old, new in MUTATIONS:
        with tempfile.TemporaryDirectory() as raw:
            root = pathlib.Path(raw) / "tree"
            (root / MODULE).mkdir(parents=True)
            for path in module_files(REPO):
                shutil.copy2(path, root / MODULE / path.name)
            target = root / rel
            text = target.read_text(encoding="utf-8")
            if old not in text:
                print(
                    f"negative proof cannot set up: {old!r} not found in {rel}. "
                    f"The mutation is stale, which means this harness is now lying "
                    f"about what it proves.",
                    file=stream,
                )
                return 1
            target.write_text(text.replace(old, new, 1), encoding="utf-8")

            print(f"\n=== {code} ===", file=stream)
            print(f"defect reintroduced: {description}", file=stream)
            report = Report()
            check_io_paths(root, report)
            check_engine_seams(root, report)
            check_bounds(root, report)
            named = any(finding[0] == code for finding in report.findings)
            for found_code, message in report.findings:
                print(f"  FAIL {found_code}: {message}", file=stream)
            print(f"  names {code}: {named}", file=stream)
            if not named:
                failures += 1

            # The other half: the unmutated copy must pass, or the guard fails on
            # everything and proves nothing about the mutation.
            target.write_text(text, encoding="utf-8")
            clean = Report()
            check_io_paths(root, clean)
            check_engine_seams(root, clean)
            check_bounds(root, clean)
            if clean.findings:
                print(
                    f"  the unmutated copy did NOT pass: {clean.findings}",
                    file=stream,
                )
                failures += 1

    if failures:
        print(f"\nnegative proof: FAIL ({failures})", file=stream)
        return 1
    print(
        f"\nnegative proof: PASS - {len(MUTATIONS)} mutations across "
        f"{len({m[0] for m in MUTATIONS})} defect classes, each observed failing",
        file=stream,
    )
    return 0


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--negative",
        action="store_true",
        help="reintroduce each defect in a copy and require this tool to fail",
    )
    arguments = parser.parse_args()
    if arguments.negative:
        return prove_negative(sys.stdout)
    return run(REPO, sys.stdout)


if __name__ == "__main__":
    raise SystemExit(main())
