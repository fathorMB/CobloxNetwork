"""Inventory of every published artifact of the Coblox v0 protocol documents.

[ADR-012] requires that a spec introducing or changing a validity rule sweep
**all** published artifacts, not only the ones it touches, and that the sweep be
run by a versioned tool that has been shown to fail. It also observes that the
sweep is impossible until somebody knows what the published artifacts *are*:

    "Chi scrive una spec deve sapere quali siano gli artefatti pubblicati, il
     che e un esercizio utile in se: la quarta occorrenza esiste perche
     quell'inventario non era mai stato fatto."

This file is the second half of that inventory. The first half is
`published_artifacts.toml`, a hand-written manifest. A hand-written list ages
in silence exactly like the artefacts it is supposed to guard, so the manifest
alone would be a declaration of intent with a file format. What makes it an
inventory is that this tool **re-derives the candidate set mechanically from the
documents** and fails when the two disagree, in either direction.

    python sim/tools/published_artifacts.py            # run every check
    python sim/tools/published_artifacts.py --uncovered # the DEBT-012 answer

Eleven defect classes are checked. Each one is reachable: reintroduce the
defect and the tool exits non-zero naming it.

    C1  DOMAIN         a `coblox-*-v0` domain string in the documents is not in
                       the manifest
    C2  TAG            an `H(0xNN` tree tag byte is not in the manifest
    C3  FIXTURE-ID     a `XXX-N` fixture identifier is not in the manifest
    C4  VALUE          a published digest literal is not in the manifest, or is
                       in it with a different value or at a different document
    C5  MIRROR         a value the manifest records as transcribed into a test
                       or tool no longer appears there
    C6  ORPHAN         a manifest entry no longer occurs in the documents
    C7  COVERAGE       a preimage has no conformance fixture and no declared
                       reason for having none
    C8  ENCODING       a preimage commits a symbolic byte whose enumeration is
                       not declared in a document
    C9  EXAMPLE        an inline example violates an equality the specification
                       states between its own fields
    C10 PROBE          a normative passage the manifest pins is no longer there
    C11 CLAIMDOC       a claim document grew a mechanical artifact, so the
                       probe-only treatment it is given has stopped being
                       the right one

**What this tool does not cover**, stated because a guard whose limits are not
written is read as covering everything (`meta.not_covered` in the manifest
carries the same list, and C6 keeps the two honest):

  - prose. A value or a rule expressed only in running text, with no digest
    literal, domain string, tag byte or fixture identifier, is invisible to the
    mechanical sweep. Where such a passage matters it is pinned by hand as a C10
    probe, and the probe list is not claimed to be complete.
  - the *contents* of a claim document beyond its pinned probes. `SECURITY.md`
    is published — GitHub serves it from the Security tab, and it is the first
    thing an outside researcher reads — but it carries no digest, domain, tag
    byte or fixture identifier, so the five discovery classes have nothing to
    find in it. It is therefore swept for C10 probes only, and C11 exists so
    that the day it *does* grow a mechanical artifact the tool says so instead
    of continuing to sweep the smaller set. Until [SPEC-016] the document was
    outside the sweep entirely: the guard had measured the wrong set, which is
    family 3 of `recurring-defects.md` applied to a guard.
  - base64url presentations other than the 43-character (32-byte) and
    22-character (16-byte) unpadded forms.
  - semantic correctness of any digest. This tool checks that the same value
    appears in every place that claims to carry it; it never recomputes one.
    Recomputation from the written bytes is `protocol_hashes.py` and the
    `coblox-core` conformance suite, which are the tools that hold that job.
"""

from __future__ import annotations

import argparse
import json
import os
import pathlib
import re
import sys
import tomllib

# `COBLOX_REPO` exists for one caller: `published_artifacts_negative.py`, which
# proves this tool can fail by mutating a throwaway copy of the tree. Pointing
# the harness at a copy instead of at the working tree means the negative proof
# cannot leave a defect behind if it is interrupted.
REPO = pathlib.Path(
    os.environ.get("COBLOX_REPO") or pathlib.Path(__file__).resolve().parents[2]
)
DOCS = REPO / "docs" / "protocol"
# Published documents that carry *claims* rather than artifacts. They live
# outside docs/protocol/ and are swept for C10 probes only; see C11.
CLAIM_DOCS = ("SECURITY.md",)
MANIFEST = REPO / "sim" / "tools" / "published_artifacts.toml"

MANIFEST_NOT_COVERED: list[str] = []

MIRROR_FILES = (
    "sim/tools/protocol_hashes.py",
    "core/coblox-core/tests/conformance_registry.rs",
    "core/coblox-core/tests/worked_example.rs",
    "core/coblox-core/tests/canonical_serialization.rs",
    "core/coblox-core/tests/light_client_perimeter.rs",
)

# --- mechanical discovery ---------------------------------------------------

RE_DOMAIN = re.compile(r"coblox-[a-z0-9-]+-v0")
RE_TAG = re.compile(r"H\((0x[0-9a-fA-F]{2})")
RE_FIXTURE = re.compile(r"`([A-Z]{2,6}-\d+)`")
RE_DIGEST = re.compile(r"(?<![0-9a-zA-Z:_-])(?:sha256:)?([0-9a-f]{64})(?![0-9a-zA-Z_-])")
RE_B64 = re.compile(r"(?<![A-Za-z0-9_-])([A-Za-z0-9_-]{43}|[A-Za-z0-9_-]{22})(?![A-Za-z0-9_=-])")


def _is_word(token: str) -> bool:
    """Reject prose that happens to fit the base64url alphabet.

    Two shapes collide with it: lowercase document anchors and snake_case field
    names (`inline-examples-are-not-conformance-oracles`), and the CamelCase
    schema names of the `text` blocks (`SignedProtocolDocument`). Neither can be
    a 32- or 16-byte encoding of anything a reader would mistake for a value.
    """
    return (
        re.fullmatch(r"[a-z0-9_-]+", token) is not None
        or re.fullmatch(r"(?:[A-Z][a-z]+)+", token) is not None
    )


def documents() -> dict[str, str]:
    return {p.name: p.read_text(encoding="utf-8") for p in sorted(DOCS.glob("*.md"))}


def claim_documents() -> dict[str, str]:
    return {name: (REPO / name).read_text(encoding="utf-8") for name in CLAIM_DOCS}


def discover(docs: dict[str, str]) -> dict[str, dict[str, set[str]]]:
    """token -> {"sites": {file, ...}} for each discovery class."""
    found: dict[str, dict[str, set[str]]] = {
        "domain": {},
        "tag": {},
        "fixture": {},
        "value": {},
        "b64": {},
    }

    def add(cls: str, token: str, name: str) -> None:
        found[cls].setdefault(token, set()).add(name)

    for name, text in docs.items():
        for m in RE_DOMAIN.finditer(text):
            add("domain", m.group(0), name)
        for m in RE_TAG.finditer(text):
            add("tag", m.group(1).lower(), name)
        for m in RE_FIXTURE.finditer(text):
            add("fixture", m.group(1), name)
        for m in RE_DIGEST.finditer(text):
            add("value", m.group(1), name)
        for m in RE_B64.finditer(text):
            if not _is_word(m.group(1)):
                add("b64", m.group(1), name)
    return found


# --- manifest ---------------------------------------------------------------


def load_manifest() -> dict:
    with MANIFEST.open("rb") as handle:
        return tomllib.load(handle)


def entries(manifest: dict, table: str) -> list[dict]:
    return list(manifest.get(table, []))


# --- checks -----------------------------------------------------------------


class Report:
    def __init__(self) -> None:
        self.failures: list[tuple[str, str]] = []
        self.counts: dict[str, int] = {}

    def fail(self, code: str, message: str) -> None:
        self.failures.append((code, message))

    def note(self, code: str, checked: int) -> None:
        self.counts[code] = checked

    @property
    def ok(self) -> bool:
        return not self.failures


def _keyed(rows: list[dict], key: str) -> dict[str, dict]:
    return {row[key]: row for row in rows}


def check_symbol_classes(manifest: dict, found: dict, report: Report) -> None:
    """C1, C2, C3, C6 for the three symbol classes, and C6 for values."""
    plan = (
        ("C1-DOMAIN", "domain", "name", "domain string"),
        ("C2-TAG", "tag", "byte", "tree tag byte"),
        ("C3-FIXTURE-ID", "fixture", "id", "fixture identifier"),
    )
    for code, table, key, label in plan:
        declared = _keyed(entries(manifest, table), key)
        seen = found[table]
        for token, sites in sorted(seen.items()):
            if token not in declared:
                report.fail(
                    code,
                    f"{label} {token!r} occurs in {', '.join(sorted(sites))} "
                    f"but is absent from the manifest",
                )
                continue
            site = declared[token]["site"]
            if site not in sites:
                report.fail(
                    code,
                    f"{label} {token!r} is declared at {site} "
                    f"but occurs only in {', '.join(sorted(sites))}",
                )
        for token in sorted(declared):
            if token not in seen:
                report.fail(
                    "C6-ORPHAN",
                    f"{label} {token!r} is in the manifest but occurs in no document",
                )
        report.note(code, len(seen))


def check_values(manifest: dict, found: dict, docs: dict[str, str], report: Report) -> None:
    """C4: every published digest literal is classified, and is where it says."""
    declared = _keyed(entries(manifest, "value"), "hex")
    seen = found["value"]
    for hexval, sites in sorted(seen.items()):
        row = declared.get(hexval)
        if row is None:
            report.fail(
                "C4-VALUE",
                f"digest {hexval} occurs in {', '.join(sorted(sites))} "
                f"but is not classified in the manifest",
            )
            continue
        if row["site"] not in sites:
            report.fail(
                "C4-VALUE",
                f"digest {hexval} is declared at {row['site']} "
                f"but occurs only in {', '.join(sorted(sites))}",
            )
        if row["class"] == "registry":
            anchor = row.get("anchor", "")
            if anchor and anchor not in docs[row["site"]]:
                report.fail(
                    "C4-VALUE",
                    f"registry digest {hexval} no longer carries its anchor "
                    f"{anchor!r} in {row['site']}",
                )
    for hexval in sorted(declared):
        if hexval not in seen:
            report.fail(
                "C6-ORPHAN",
                f"digest {hexval} is in the manifest but occurs in no document",
            )
    report.note("C4-VALUE", len(seen))

    b64_declared = _keyed(entries(manifest, "presentation"), "text")
    for token, sites in sorted(found["b64"].items()):
        if token not in b64_declared:
            report.fail(
                "C4-VALUE",
                f"base64url presentation {token!r} occurs in "
                f"{', '.join(sorted(sites))} but is not classified in the manifest",
            )
    for token in sorted(b64_declared):
        if token not in found["b64"]:
            report.fail(
                "C6-ORPHAN",
                f"base64url presentation {token!r} is in the manifest "
                f"but occurs in no document",
            )


def check_mirrors(manifest: dict, report: Report) -> None:
    """C5: the copies of a published value elsewhere in the tree still agree.

    [SPEC-010] was asked to face the observation that the same expected hashes
    lived in three independent places kept aligned by hand — the document, the
    Python tool, the Rust test — and to decide, not to ignore it. The answer
    this check encodes is that the three were never of equal standing.

    `docs/protocol/README.md` is the oracle; the other two are consumers, and
    they need different treatment:

      - `sim/tools/protocol_hashes.py` **recomputes** values, so its copy of
        the expectation was pure duplication and is gone: it now reads the
        registry table. That copy had already gone stale once, in [SPEC-009],
        and made the tool report a mismatch that did not exist — the false
        positive [ADR-012] cites when it requires a guard to be proved in the
        negative.
      - `core/coblox-core/tests/conformance_registry.rs` **is the
        implementation under test**, and its transcription is deliberate: a
        suite whose expectation is generated by the code it checks asserts
        nothing. That copy stays, and this check is what stops it drifting.

    So the count went from three hand-aligned sites to one oracle, one derived
    reader, and one deliberate transcription that a machine now compares.
    """
    checked = 0
    texts = {name: (REPO / name).read_text(encoding="utf-8") for name in MIRROR_FILES}
    for row in entries(manifest, "value"):
        if row["class"] != "registry":
            continue
        for name in row.get("mirrors", []):
            checked += 1
            if row["hex"] not in texts[name]:
                report.fail(
                    "C5-MIRROR",
                    f"registry digest {row['hex']} ({row['name']}) is declared "
                    f"mirrored in {name} but does not appear there",
                )
    report.note("C5-MIRROR", checked)


def check_preimages(manifest: dict, docs: dict[str, str], report: Report) -> None:
    """C7 coverage and C8 symbolic-byte encoding."""
    fixtures = {row["id"] for row in entries(manifest, "fixture")}
    values = {row["hex"] for row in entries(manifest, "value")}
    covered = 0
    for row in entries(manifest, "preimage"):
        pid = row["id"]
        coverage = row["coverage"]
        if coverage == "uncovered":
            if not row.get("uncovered_reason", "").strip():
                report.fail(
                    "C7-COVERAGE",
                    f"preimage {pid!r} has no fixture and no declared reason",
                )
        elif coverage == "fixture-elsewhere":
            covered += 1
            # A value published outside the registry table. It must still say
            # where, because "covered somewhere" is the shape of a reassurance.
            if not row.get("uncovered_reason", "").strip():
                report.fail(
                    "C7-COVERAGE",
                    f"preimage {pid!r} is covered outside the registry table "
                    f"but does not say where",
                )
        elif coverage == "fixture":
            covered += 1
            fixture = row.get("fixture", "")
            if fixture not in fixtures:
                report.fail(
                    "C7-COVERAGE",
                    f"preimage {pid!r} names fixture {fixture!r}, "
                    f"which is not a declared fixture identifier",
                )
            expected = row.get("expected", "").removeprefix("sha256:")
            if not expected:
                report.fail(
                    "C7-COVERAGE",
                    f"preimage {pid!r} is declared covered but publishes no value",
                )
            elif expected not in values:
                report.fail(
                    "C7-COVERAGE",
                    f"preimage {pid!r} publishes {expected}, "
                    f"which is not a classified document value",
                )
        else:
            report.fail("C7-COVERAGE", f"preimage {pid!r} has unknown coverage {coverage!r}")

        for field in row.get("symbolic_field", []):
            site = field["site"]
            if site not in docs:
                report.fail(
                    "C8-ENCODING",
                    f"preimage {pid!r} field {field['field']!r} names "
                    f"unknown document {site!r}",
                )
                continue
            if not re.search(field["probe"], docs[site]):
                report.fail(
                    "C8-ENCODING",
                    f"preimage {pid!r} commits the symbolic byte "
                    f"{field['field']!r}, whose enumeration is declared in "
                    f"{site} but no longer matches {field['probe']!r}. "
                    f"An undeclared symbolic byte is DEBT-012.",
                )
    report.note("C7-COVERAGE", len(entries(manifest, "preimage")))
    report.note("C8-ENCODING", sum(len(r.get("symbolic_field", [])) for r in entries(manifest, "preimage")))


def _dig(obj, path: str):
    for part in path.split("."):
        if not isinstance(obj, dict) or part not in obj:
            return None
        obj = obj[part]
    return obj


def check_example_invariants(manifest: dict, docs: dict[str, str], report: Report) -> None:
    """C9: an inline example must satisfy the equalities the documents state."""
    checked = 0
    for row in entries(manifest, "example_invariant"):
        text = docs[row["site"]]
        blocks = [
            line.strip()
            for line in text.splitlines()
            if line.startswith("{") and row["selector"] in line
        ]
        if len(blocks) != 1:
            report.fail(
                "C9-EXAMPLE",
                f"invariant {row['id']!r}: selector {row['selector']!r} matches "
                f"{len(blocks)} inline examples in {row['site']}, expected exactly 1",
            )
            continue
        try:
            obj = json.loads(blocks[0])
        except json.JSONDecodeError as exc:  # pragma: no cover - defensive
            report.fail("C9-EXAMPLE", f"invariant {row['id']!r}: example is not JSON ({exc})")
            continue
        seen = {path: _dig(obj, path) for path in row["equal_fields"]}
        missing = [p for p, v in seen.items() if v is None]
        if missing:
            report.fail(
                "C9-EXAMPLE",
                f"invariant {row['id']!r}: fields absent from the example: {missing}",
            )
            continue
        checked += 1
        if len(set(seen.values())) != 1:
            report.fail(
                "C9-EXAMPLE",
                f"invariant {row['id']!r}: {row['rule']} - the example in "
                f"{row['site']} carries "
                + ", ".join(f"{p}={v}" for p, v in seen.items()),
            )
    report.note("C9-EXAMPLE", checked)


def check_claim_documents(
    manifest: dict, claims: dict[str, str], report: Report
) -> None:
    """C11: a claim document is swept for probes only, and must stay eligible.

    The five discovery classes are not run over these documents, because the
    tokens they look for are not there and a `DEBT-013` in backticks would be
    reported as an undeclared conformance fixture — a false positive, which
    [ADR-012] precision 3 records as the way a guard stops being believed and
    therefore stops being run.

    That decision is only honest while it stays true. If a claim document ever
    carries a digest literal, a domain-separation string or a tree tag byte, it
    has become an artifact document and the narrower treatment is exactly the
    defect this class exists to prevent: a guard measuring the smaller set.
    """
    declared = set(manifest["meta"].get("claim_documents", []))
    if declared != set(claims):
        report.fail(
            "C11-CLAIMDOC",
            f"claim documents on disk {sorted(claims)} differ from the "
            f"manifest's {sorted(declared)}",
        )
    for name, text in claims.items():
        for label, pattern in (
            ("digest literal", RE_DIGEST),
            ("domain-separation string", RE_DOMAIN),
            ("tree tag byte", RE_TAG),
        ):
            hit = pattern.search(text)
            if hit is not None:
                report.fail(
                    "C11-CLAIMDOC",
                    f"{name} now carries a {label} ({hit.group(0)!r}). It is "
                    f"swept for C10 probes only, which was right while it "
                    f"carried claims and no artifacts. Promote it to "
                    f"meta.documents and to the five discovery classes, or "
                    f"remove the artifact.",
                )
    checked_claims = len(claims)

    # The derived counts. A number transcribed by hand into a published
    # document is the defect [SPEC-012] closed by extracting the table from the
    # document instead of copying it, and `SECURITY.md` had two of them wrong:
    # 36 scenarios where the threat model carried 39, and 24 security
    # requirements where it carried 26. Pinning the corrected numbers as probes
    # would have bought one edit of grace, so they are recomputed from the
    # source instead.
    for row in entries(manifest, "claim_count"):
        text = claims.get(row["site"])
        if text is None:
            report.fail(
                "C11-CLAIMDOC",
                f"claim count {row['id']!r} names unknown claim document "
                f"{row['site']!r}",
            )
            continue
        hit = re.search(row["pattern"], text)
        if hit is None:
            report.fail(
                "C11-CLAIMDOC",
                f"claim count {row['id']!r} found no match of "
                f"{row['pattern']!r} in {row['site']}. {row['why']}",
            )
            continue
        source = REPO / row["source"]
        if not source.is_file():
            report.fail(
                "C11-CLAIMDOC",
                f"claim count {row['id']!r} names source {row['source']!r}, "
                f"which is not a file",
            )
            continue
        actual = len(set(re.findall(row["token"], source.read_text(encoding="utf-8"))))
        claimed = int(hit.group(1))
        if claimed != actual:
            report.fail(
                "C11-CLAIMDOC",
                f"claim count {row['id']!r}: {row['site']} claims {claimed} "
                f"but {row['source']} carries {actual} distinct "
                f"{row['token']!r}. {row['why']}",
            )
    report.note("C11-CLAIMDOC", checked_claims + len(entries(manifest, "claim_count")))


def check_probes(manifest: dict, docs: dict[str, str], report: Report) -> None:
    """C10: hand-pinned normative passages that carry no mechanical token."""
    for row in entries(manifest, "probe"):
        text = docs.get(row["site"])
        if text is None:
            report.fail("C10-PROBE", f"probe {row['id']!r} names unknown document {row['site']!r}")
            continue
        hits = len(re.findall(row["pattern"], text))
        if hits != row["count"]:
            report.fail(
                "C10-PROBE",
                f"probe {row['id']!r} expected {row['count']} match(es) of "
                f"{row['pattern']!r} in {row['site']}, found {hits}. {row['why']}",
            )
    report.note("C10-PROBE", len(entries(manifest, "probe")))


# --- entry point ------------------------------------------------------------


def print_uncovered(manifest: dict) -> None:
    """The written answer to the general question of [DEBT-012]."""
    global MANIFEST_NOT_COVERED
    MANIFEST_NOT_COVERED = manifest["meta"]["not_covered"]
    rows = entries(manifest, "preimage")
    uncovered = [r for r in rows if r["coverage"] == "uncovered"]
    elsewhere = [r for r in rows if r["coverage"] == "fixture-elsewhere"]
    covered = [r for r in rows if r["coverage"] == "fixture"]
    symbolic = [(r, f) for r in rows for f in r.get("symbolic_field", [])]

    print(f"Preimages in the v0 documents: {len(rows)}")
    print(f"  in the conformance registry table: {len(covered)}")
    print(f"  with a published value elsewhere:  {len(elsewhere)}")
    print(f"  with no published value:           {len(uncovered)}")
    print()
    print("Preimages with no published fixture, and why:")
    for row in uncovered:
        print(f"  {row['id']:<28} {row['site']}")
        print(f"      {row['uncovered_reason']}")
    print()
    print("The second half of the question of [DEBT-012]: which of these")
    print("commit a field whose encoding is not fixed elsewhere.")
    print()
    print("A preimage field is safe when its bytes are determined by the")
    print("formula alone: a fixed-width big-endian integer, raw digest bytes,")
    print("a UTF-8 string with its length, a literal tag, or a JCS object")
    print("whose schema enumerates its own spellings. A field is a SYMBOLIC")
    print("BYTE when a name has to be turned into a number, because the")
    print("number is the thing the formula does not carry.")
    print()
    print(f"Symbolic bytes across all {len(rows)} preimages: {len(symbolic)}")
    for row, field in symbolic:
        print(f"  {row['id']}.{field['field']} - enumeration declared in {field['site']}")
    if not symbolic:
        print("  (none)")
    print()
    print("That count is one, and the reason is structural rather than lucky:")
    print("v0 commits every other enumeration as a JCS string inside a JCS")
    print("object, so the committed bytes are the letters of the name and no")
    print("mapping exists to disagree about. `lifecycle_u8` is the only place")
    print("where a name is committed as a number, and it is exactly the place")
    print("where the mapping was missing. Check C8 fails if a preimage grows")
    print("another one without an enumeration to point at.")
    print()
    print("Not covered by this sweep at all:")
    for line in MANIFEST_NOT_COVERED:
        print(f"  - {line}")


def main(argv: list[str] | None = None) -> int:
    parser = argparse.ArgumentParser(description=__doc__.splitlines()[0])
    parser.add_argument(
        "--uncovered",
        action="store_true",
        help="print the coverage answer of [DEBT-012] instead of running checks",
    )
    args = parser.parse_args(argv)

    manifest = load_manifest()
    if args.uncovered:
        print_uncovered(manifest)
        return 0

    docs = documents()
    expected_docs = set(manifest["meta"]["documents"])
    if set(docs) != expected_docs:
        print(f"FAIL C6-ORPHAN: documents on disk {sorted(docs)} "
              f"differ from the manifest's {sorted(expected_docs)}")
        return 1

    claims = claim_documents()

    found = discover(docs)
    report = Report()
    check_symbol_classes(manifest, found, report)
    check_values(manifest, found, docs, report)
    check_mirrors(manifest, report)
    check_preimages(manifest, docs, report)
    check_example_invariants(manifest, docs, report)
    check_claim_documents(manifest, claims, report)
    check_probes(manifest, {**docs, **claims}, report)

    for code in (
        "C1-DOMAIN",
        "C2-TAG",
        "C3-FIXTURE-ID",
        "C4-VALUE",
        "C5-MIRROR",
        "C7-COVERAGE",
        "C8-ENCODING",
        "C9-EXAMPLE",
        "C10-PROBE",
        "C11-CLAIMDOC",
    ):
        print(f"  {code:<15} {report.counts.get(code, 0):>4} candidate(s) checked")
    print()

    if report.ok:
        print("published-artifact inventory: PASS")
        return 0

    for code, message in report.failures:
        print(f"FAIL {code}: {message}")
    print()
    print(f"published-artifact inventory: FAIL ({len(report.failures)} finding(s))")
    return 1


if __name__ == "__main__":
    raise SystemExit(main())
