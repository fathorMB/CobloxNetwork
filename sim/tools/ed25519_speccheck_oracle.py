"""Independent oracle for the published `ed25519-speccheck` outcome table.

    python sim/tools/ed25519_speccheck_oracle.py
    python sim/tools/ed25519_speccheck_oracle.py --explain
    python sim/tools/ed25519_speccheck_oracle.py --decoder strict_y   # negative proof

**Why a second implementation exists at all.** `speccheck_conformance.rs` now
compares the published table in `docs/protocol/README.md` against what
`coblox-core` does, vector by vector, and that comparison is real because the
published column is parsed out of the document rather than transcribed. But two
agreeing parties are still only two: if a vector were fabricated or mis-copied,
the document and any implementation derived from the same reading of it would
agree on a wrong answer, and the agreement would look like a proof. [REVIEW-018]
made this argument concrete by refusing to re-run the implementer's oracle and
writing one from scratch instead. That oracle is this file.

This code shares nothing with `verifier.rs`: no `curve25519-dalek`, no `sha2`
crate, no constant, no helper. Edwards arithmetic on `2^255-19` in Python
integers, the base point pinned by its published coordinates and checked against
the curve equation, and the five rules read off `README.md` §*Consensus-critical
Ed25519 verification*:

    1. decode `A_enc`, `R_enc` with `y >= 2^255-19` reduced and `x = 0` with the
       sign bit set accepted
    2. `0 <= S < L`
    3. `[8]A != identity`
    4. `[8][S]B == [8]R + [8][k]A`
       with `k = SHA-512(R_enc || A_enc || M) mod L` over the **original**
       encodings, never over re-encoded points

It is slow (seconds, not microseconds) and deliberately unoptimised: it is read
by people deciding whether to trust a table, so it is written to be read.

**Two tables, and the reason for the second.** [REVIEW-019] established that the
twelve upstream vectors exercise the second half of rule 1 and never the first:
no `y` in their twenty-four point encodings is `>= 2^255-19`. This tool therefore
runs both the upstream table and the Coblox extension table, and on every run it
also proves what the extension vectors exist to prove — that an implementation
identical to Coblox except for rejecting `y >= p` agrees on all twelve upstream
vectors and disagrees on the extension ones. The negative proof is executed, not
transcribed, so it cannot go stale. The fully strict RFC 8032 decoder is checked
too, and is expected to be excluded by upstream vector 9 alone: that is what
makes the *intermediate* implementation the dangerous class rather than the
strict one.

**What it is not.** It is not a verifier and must never be used as one — no
constant-time discipline, no side-channel care. It reads the document, runs both
tables, and exits non-zero if any outcome disagrees.

**Provenance of the vectors.** `core/coblox-core/tests/fixtures/`, whose README
records authors, licence, paper, upstream commit and digests. This tool reads
them and does not supply its own: fabricating vectors here would defeat the
purpose of an independent check by making it independent of the wrong thing. The
extension vectors are constructed by `ed25519_coblox_extension_vectors.py`, which
imports this module's arithmetic on purpose — see its header for why that is the
right dependency and where the independence that matters actually lives.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import re
import sys
from pathlib import Path

REPO_ROOT = Path(__file__).resolve().parents[2]
PROTOCOL_README = REPO_ROOT / "docs" / "protocol" / "README.md"
FIXTURES = REPO_ROOT / "core" / "coblox-core" / "tests" / "fixtures"
VECTORS = FIXTURES / "ed25519_speccheck.json"
EXTENSION_VECTORS = FIXTURES / "ed25519_coblox_extension.json"

TABLE_SECTION = "### Consensus-critical Ed25519 verification"
TABLE_ROW_LABEL = "| Coblox v0 |"
EXTENSION_TABLE_SECTION = "#### Coblox extension vectors"
EXTENSION_TABLE_ROW_LABEL = "| Coblox v0 |"

# ---------------------------------------------------------------------------
# Field and curve
# ---------------------------------------------------------------------------

P = 2**255 - 19
L = 2**252 + 27742317777372353535851937790883648493
D = (-121665 * pow(121666, P - 2, P)) % P
SQRT_M1 = pow(2, (P - 1) // 4, P)

# RFC 8032 base point, pinned by value and checked below rather than derived, so
# a sign-convention slip cannot silently change what this oracle verifies.
BASE_X = 15112221349535400772501151409588531511454012693041857206046113283949847762202
BASE_Y = 46316835694926478169428394003475163141307993866256225615783033603165251855960

# Extended twisted-Edwards coordinates (X, Y, Z, T) with x = X/Z, y = Y/Z,
# xy = T/Z.
Point = tuple[int, int, int, int]
IDENTITY: Point = (0, 1, 1, 0)


def on_curve(x: int, y: int) -> bool:
    """`-x^2 + y^2 = 1 + d x^2 y^2`, the Ed25519 curve equation."""
    return (-x * x + y * y - 1 - D * x * x * y * y) % P == 0


assert on_curve(BASE_X, BASE_Y), "pinned base point is not on the curve"
BASE: Point = (BASE_X, BASE_Y, 1, (BASE_X * BASE_Y) % P)


def point_add(p: Point, q: Point) -> Point:
    """Unified addition (add-2008-hwcd-3); complete for this curve."""
    x1, y1, z1, t1 = p
    x2, y2, z2, t2 = q
    a = ((y1 - x1) * (y2 - x2)) % P
    b = ((y1 + x1) * (y2 + x2)) % P
    c = (t1 * 2 * D * t2) % P
    d = (z1 * 2 * z2) % P
    e, f, g, h = b - a, d - c, d + c, b + a
    return ((e * f) % P, (g * h) % P, (f * g) % P, (e * h) % P)


def point_mul(scalar: int, p: Point) -> Point:
    result = IDENTITY
    while scalar > 0:
        if scalar & 1:
            result = point_add(result, p)
        p = point_add(p, p)
        scalar >>= 1
    return result


def point_eq(p: Point, q: Point) -> bool:
    x1, y1, z1, _ = p
    x2, y2, z2, _ = q
    return (x1 * z2 - x2 * z1) % P == 0 and (y1 * z2 - y2 * z1) % P == 0


def recover_x(y: int) -> int | None:
    """The non-negative root of `x^2 = (y^2 - 1) / (d y^2 + 1)`, or None."""
    denom = (D * y * y + 1) % P
    if denom == 0:
        return None
    xx = ((y * y - 1) * pow(denom, P - 2, P)) % P
    x = pow(xx, (P + 3) // 8, P)
    if (x * x - xx) % P != 0:
        x = (x * SQRT_M1) % P
    if (x * x - xx) % P != 0:
        return None
    return x


# Decoding variants. `COBLOX` is the published rule; the other two exist so the
# document can be compared against the implementations it competes with, which is
# the only way a conformance table can say anything about a *second* implementer.
#
#   COBLOX    ZIP-215 on both counts: `y >= p` is reduced, `x = 0` with sign bit 1
#             is accepted. This is rule 1 of the protocol document.
#   STRICT_Y  ZIP-215 on the sign bit, RFC 8032 §5.1.3 step 2 on `y`: a masked
#             `y >= p` is rejected. **This is the dangerous class.** It agrees
#             with COBLOX on all twelve speccheck vectors and disagrees on the
#             extension vectors, and it is the shape a careful implementer
#             reaches by reading ZIP-215 for the sign bit and the RFC for
#             canonicity.
#   RFC8032   fully strict: also rejects `x = 0` with sign bit 1. Included
#             because it is the variant a reader expects to be the dangerous one
#             and is not: it fails speccheck vector 9, so the twelve already
#             exclude it.
COBLOX = "coblox"
STRICT_Y = "strict_y"
RFC8032 = "rfc8032"


def decompress(encoding: bytes, mode: str = COBLOX) -> Point | None:
    """Rule 1. Bit 255 is the sign of `x`; the low 255 bits are `y`.

    Under `COBLOX`, a `y` that is not canonically reduced is reduced mod `P`
    instead of rejecting the encoding, and the conditional negation is applied
    unconditionally so `x = 0` with sign bit 1 decodes to the order-2 point.
    Those two `if mode` lines below are the entire difference between the three
    rules, and between accepting and rejecting a forged finality vote.
    """
    n = int.from_bytes(encoding, "little")
    sign = (n >> 255) & 1
    y_raw = n & ((1 << 255) - 1)
    if mode in (STRICT_Y, RFC8032) and y_raw >= P:
        return None
    y = y_raw % P
    x = recover_x(y)
    if x is None:
        return None
    if mode == RFC8032 and x == 0 and sign == 1:
        return None
    if x % 2 != sign:
        x = (P - x) % P
    return (x, y, 1, (x * y) % P)


def compress(p: Point) -> bytes:
    """Only used by the counterfactual in `verify_over_reencoded_points`."""
    x, y, z, _ = p
    z_inv = pow(z, P - 2, P)
    x, y = (x * z_inv) % P, (y * z_inv) % P
    return (y | ((x & 1) << 255)).to_bytes(32, "little")


def verify(
    a_enc: bytes, r_enc: bytes, s_enc: bytes, message: bytes, mode: str = COBLOX
) -> tuple[bool, str]:
    """The Coblox v0 rule. Returns `(accepted, reason)`.

    `mode` changes rule 1 and nothing else; rules 2-4 are shared, because the
    whole question the extension vectors settle is what a difference confined to
    rule 1 does to the verdict.
    """
    a_point = decompress(a_enc, mode)
    if a_point is None:
        return False, "A_enc does not decode to a curve point"
    r_point = decompress(r_enc, mode)
    if r_point is None:
        return False, "R_enc does not decode to a curve point"

    s = int.from_bytes(s_enc, "little")
    if not 0 <= s < L:
        return False, "S >= L (rule 2)"

    if point_eq(point_mul(8, a_point), IDENTITY):
        return False, "[8]A == identity, small-order key (rule 3)"

    # Rule 4, and the sentence after it: the hash consumes the encodings as they
    # arrived, never a re-encoding of the decoded points.
    k = int.from_bytes(hashlib.sha512(r_enc + a_enc + message).digest(), "little") % L

    lhs = point_mul(8, point_mul(s, BASE))
    rhs = point_add(point_mul(8, r_point), point_mul(8, point_mul(k, a_point)))
    if not point_eq(lhs, rhs):
        return False, "[8][S]B != [8]R + [8][k]A (rule 4)"
    return True, "all five conditions hold"


def verify_over_reencoded_points(
    a_enc: bytes, r_enc: bytes, s_enc: bytes, message: bytes
) -> bool:
    """The counterfactual the specification forbids: `k` over re-encoded points.

    Kept because it is what makes vectors 8 and 9 legible. They carry the same
    non-canonical `R_enc` and differ only in which `R` their signer digested, so
    this function and `verify` disagree on both of them, in opposite directions.
    A reader who wants to know why vector 8 must be `reject` can read that
    disagreement instead of taking it on trust.
    """
    a_point = decompress(a_enc)
    r_point = decompress(r_enc)
    if a_point is None or r_point is None:
        return False
    s = int.from_bytes(s_enc, "little")
    if not 0 <= s < L:
        return False
    if point_eq(point_mul(8, a_point), IDENTITY):
        return False
    k = (
        int.from_bytes(
            hashlib.sha512(compress(r_point) + compress(a_point) + message).digest(),
            "little",
        )
        % L
    )
    lhs = point_mul(8, point_mul(s, BASE))
    rhs = point_add(point_mul(8, r_point), point_mul(8, point_mul(k, a_point)))
    return point_eq(lhs, rhs)


# ---------------------------------------------------------------------------
# The document
# ---------------------------------------------------------------------------


def _published_row(section_heading: str, row_label: str, expected_cells: int) -> list[bool]:
    """Parse one outcome row out of the published document.

    Same contract as `published_outcomes_from_document` in
    `speccheck_conformance.rs`, and for the same reason: a transcription here
    would make this oracle agree with a table it had copied rather than with a
    table it had read.
    """
    text = PROTOCOL_README.read_text(encoding="utf-8")
    start = text.find(section_heading)
    if start < 0:
        raise SystemExit(f"{PROTOCOL_README}: section '{section_heading}' not found")
    section = text[start + len(section_heading) :]
    # Stop at the next heading of any level. The two tables live in a section and
    # a subsection of it, so a bound that ignored `####` would let the first
    # parser walk into the second table.
    end = re.search(r"\n#{1,6} ", section)
    if end:
        section = section[: end.start()]

    rows = [ln.strip() for ln in section.splitlines() if ln.strip().startswith(row_label)]
    if len(rows) != 1:
        raise SystemExit(
            f"{PROTOCOL_README}: expected exactly one '{row_label}' row in "
            f"'{section_heading}', found {len(rows)}"
        )
    cells = [c.strip() for c in rows[0].strip("|").split("|")][1:]
    if len(cells) != expected_cells:
        raise SystemExit(
            f"{PROTOCOL_README}: expected {expected_cells} outcome cells in "
            f"'{section_heading}', found {len(cells)}"
        )
    for i, cell in enumerate(cells):
        if cell not in ("accept", "reject"):
            raise SystemExit(f"{PROTOCOL_README}: '{section_heading}' vector {i} reads '{cell}'")
    return [cell == "accept" for cell in cells]


def published_outcomes() -> list[bool]:
    """The twelve upstream `ed25519-speccheck` outcomes, from the document."""
    return _published_row(TABLE_SECTION, TABLE_ROW_LABEL, 12)


def published_extension_outcomes() -> list[bool]:
    """The Coblox extension outcomes, from the document."""
    return _published_row(EXTENSION_TABLE_SECTION, EXTENSION_TABLE_ROW_LABEL, 7)


def _load(path) -> list[dict]:
    vectors = json.loads(path.read_text(encoding="utf-8"))
    for i, entry in enumerate(vectors):
        if entry["index"] != i:
            raise SystemExit(f"{path}: vector {i} carries index {entry['index']}")
    return vectors


def _parts(entry: dict) -> tuple[bytes, bytes, bytes, bytes]:
    signature = bytes.fromhex(entry["signature"])
    return (
        bytes.fromhex(entry["pub_key"]),
        signature[:32],
        signature[32:],
        bytes.fromhex(entry["message"]),
    )


def _run_table(
    title: str, vectors: list[dict], published: list[bool], mode: str = COBLOX
) -> int:
    print(title)
    print(f"{'V':>2}  {'published':<9}  {'oracle':<9}  {'status':<8}  reason")
    failures = 0
    for entry in vectors:
        a_enc, r_enc, s_enc, message = _parts(entry)
        accepted, reason = verify(a_enc, r_enc, s_enc, message, mode)
        want = published[entry["index"]]
        if accepted != want:
            failures += 1
        print(
            f"{entry['index']:>2}  {'accept' if want else 'reject':<9}  "
            f"{'accept' if accepted else 'reject':<9}  "
            f"{'MATCH' if accepted == want else 'MISMATCH':<8}  {reason}"
        )
    return failures


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--explain",
        action="store_true",
        help="also print the 8/9 differential and the decoder divergence proof",
    )
    parser.add_argument(
        "--decoder",
        choices=(COBLOX, STRICT_Y, RFC8032),
        default=COBLOX,
        help=(
            "run the tables under a different rule 1. The default is the published "
            "one; 'strict_y' is the negative proof of [REVIEW-019] RF-001 made "
            "runnable by anyone — it passes the upstream twelve and fails the "
            "extension vectors, which is the whole finding in one command."
        ),
    )
    args = parser.parse_args()

    if args.decoder != COBLOX:
        print(f"!! running under rule 1 variant '{args.decoder}', NOT the published rule")
        print("!! a FAIL below is the expected result, not a defect\n")

    published = published_outcomes()
    vectors = _load(VECTORS)
    if len(vectors) != 12:
        raise SystemExit(f"{VECTORS}: expected 12 vectors, found {len(vectors)}")

    published_ext = published_extension_outcomes()
    extension = _load(EXTENSION_VECTORS)
    if len(extension) != 7:
        raise SystemExit(f"{EXTENSION_VECTORS}: expected 7 vectors, found {len(extension)}")

    doc = PROTOCOL_README.relative_to(REPO_ROOT).as_posix()
    failures = _run_table(
        f"independent oracle vs {doc} (upstream speccheck 0-11)",
        vectors,
        published,
        args.decoder,
    )
    print()
    failures += _run_table(
        f"independent oracle vs {doc} (Coblox extension 0-6)",
        extension,
        published_ext,
        args.decoder,
    )

    # The negative proof RF-001 of [REVIEW-019] requires, run every time rather
    # than transcribed once: the twelve upstream vectors cannot tell the Coblox
    # rule apart from an implementation that rejects `y >= p`, and the extension
    # vectors can.
    print()
    print("decoder divergence: Coblox vs an implementation that rejects y >= p")
    strict_disagreements_upstream = 0
    for entry in vectors:
        a_enc, r_enc, s_enc, message = _parts(entry)
        if verify(a_enc, r_enc, s_enc, message, COBLOX)[0] != (
            verify(a_enc, r_enc, s_enc, message, STRICT_Y)[0]
        ):
            strict_disagreements_upstream += 1
    strict_disagreements_extension = []
    for entry in extension:
        a_enc, r_enc, s_enc, message = _parts(entry)
        if verify(a_enc, r_enc, s_enc, message, COBLOX)[0] != (
            verify(a_enc, r_enc, s_enc, message, STRICT_Y)[0]
        ):
            strict_disagreements_extension.append(entry["index"])
    print(f"  upstream 0-11 : {strict_disagreements_upstream} disagreement(s)")
    print(
        f"  extension 0-6 : {len(strict_disagreements_extension)} disagreement(s) "
        f"at {strict_disagreements_extension}"
    )
    if strict_disagreements_upstream != 0 or not strict_disagreements_extension:
        failures += 1
        print("  FAIL - the extension vectors do not discriminate what they exist to discriminate")

    # The Lead's precision, kept executable: the *fully* strict RFC 8032 decoder
    # is already excluded by the twelve, so it is not the class the extension
    # vectors are aimed at.
    rfc_disagreements_upstream = [
        entry["index"]
        for entry in vectors
        if verify(*_parts(entry), COBLOX)[0] != verify(*_parts(entry), RFC8032)[0]
    ]
    print("  fully strict RFC 8032 decoder vs Coblox on the upstream twelve: "
          f"disagrees at {rfc_disagreements_upstream}")
    if rfc_disagreements_upstream != [9]:
        failures += 1
        print("  FAIL - expected the fully strict decoder to be excluded by vector 9 alone")

    if args.explain:
        print()
        print("vectors 8 and 9: same R_enc (order-2 point, sign bit 1), different k preimage")
        for index in (8, 9):
            a_enc, r_enc, s_enc, message = _parts(vectors[index])
            normative, _ = verify(a_enc, r_enc, s_enc, message)
            counterfactual = verify_over_reencoded_points(a_enc, r_enc, s_enc, message)
            print(
                f"  vector {index}: k over original encodings -> "
                f"{'accept' if normative else 'reject'}; "
                f"k over re-encoded points -> "
                f"{'accept' if counterfactual else 'reject'}"
            )
        print("  the rule mandates the first column, so 8 rejects and 9 accepts")
        print()
        print("extension vectors, per decoder:")
        print(f"  {'V':>2}  {'coblox':<8}  {'strict_y':<8}  {'rfc8032':<8}")
        for entry in extension:
            parts = _parts(entry)
            row = [verify(*parts, mode)[0] for mode in (COBLOX, STRICT_Y, RFC8032)]
            cells = "  ".join(f"{'accept' if v else 'reject':<8}" for v in row)
            print(f"  {entry['index']:>2}  {cells}")

    print()
    if failures:
        print(f"independent oracle: FAIL - {failures} disagreement(s) with the document")
        print("  Which side is wrong has to be settled by derivation before either is changed.")
        return 1
    print("independent oracle: PASS - both published tables agree with the rule, and the")
    print("  extension vectors separate Coblox from a y >= p-rejecting implementation")
    return 0


if __name__ == "__main__":
    sys.exit(main())
