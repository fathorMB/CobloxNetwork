"""Generator and checker for the **Coblox extension** Ed25519 conformance vectors.

    python sim/tools/ed25519_coblox_extension_vectors.py            # check
    python sim/tools/ed25519_coblox_extension_vectors.py --write    # regenerate

**Why a second vector file exists at all.** Rule 1 of `docs/protocol/README.md`
§*Consensus-critical Ed25519 verification* prescribes two decoding behaviours in
which Coblox departs from RFC 8032. The twelve `novifinancial/ed25519-speccheck`
vectors exercise exactly one of them — `x = 0` with the sign bit set, at vectors
8-11 — and **none** of the twelve exercises the other: [REVIEW-019] inspected all
twenty-four point encodings in that fixture and found no `y` whose masked value
is `>= 2^255-19`. Half of rule 1 was normative and unvalidated.

The consequence is not theoretical, and the vectors below are that consequence
written down. Two implementations that both pass all twelve vectors — one
reducing `y >= p` as Coblox requires, one rejecting it as RFC 8032 §5.1.3 step 2
requires — return opposite verdicts on vectors 0-3 here. The second is the
*plausible* implementation, not a strawman: it follows ZIP-215 on the sign bit
(so it agrees with Coblox on vectors 8-11) and the RFC on the canonicity of `y`.

**Why exactly these seven, and why four of them are the whole attack.** An input
on which the two implementations diverge is one Coblox **accepts** while carrying
a non-canonical `y`. Producing one requires solving the cofactored equation
`[8][S]B = [8]R + [8][k]A` with `R_enc` or `A_enc` fixed before `k` is known:

* with a non-canonical `A_enc`, the forger would need the discrete logarithm of
  the reduced `A`, which is intractable and which no honest key can have either,
  because a public key is `[a]B` and cannot be steered to `y in [0, 18]`;
* with a non-canonical `R_enc` of large order, the forger would need the discrete
  logarithm of `R`, equally intractable;
* with a non-canonical `R_enc` whose reduced point satisfies `[8]R = O`, the
  equation collapses to `[8][S]B = [8][k]A` and `S = k·a mod L` solves it with the
  forger's *own* key and no secret beyond it.

The non-canonical encodings whose reduced point has `[8]R = O` are exactly four:
`y_raw in {p, p+1}` times the two sign bits (`y = 1` is the identity, `y = 0` is a
point of order 4; every other order-2/4/8 point has a canonical `y`). Vectors 0-3
are therefore not a sample of the divergent set — **they are the divergent set**,
up to the choice of key and message. Vectors 4-6 carry the remaining
non-canonical shapes that [REVIEW-019] asked for; they reject under both rules,
and that is stated rather than glossed, because a vector that cannot fail is
evidence of decoding coverage and nothing more.

**Independence, and its limit.** Outcomes are never transcribed from the Rust
implementation. Each vector is constructed algebraically here, its outcome is
re-derived by `ed25519_speccheck_oracle.verify` from the five published rules,
the outcome is published in the protocol document, and `speccheck_conformance.rs`
compares `coblox-core` against the *document*. This generator does share the
Edwards arithmetic of `ed25519_speccheck_oracle`, deliberately: the independence
that matters is from `verifier.rs`, and duplicating the arithmetic here would add
a fourth copy to audit without adding an independent party.

**Determinism.** Every secret is derived by SHA-512 from an ASCII label written in
this file, so the fixture is reproducible byte for byte by anyone who runs
`--write`, and `--check` fails if the committed file and this code disagree.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import sys
from pathlib import Path

sys.path.insert(0, str(Path(__file__).resolve().parent))

import ed25519_speccheck_oracle as oracle  # noqa: E402

REPO_ROOT = Path(__file__).resolve().parents[2]
FIXTURE = (
    REPO_ROOT / "core" / "coblox-core" / "tests" / "fixtures" / "ed25519_coblox_extension.json"
)

P = oracle.P
L = oracle.L

# The message every vector signs is a real finality-vote preimage, the shape
# `registry::block_vote_preimage` produces, because the scenario this file exists
# to describe is a validator splitting the network on a finality vote:
#   "coblox-block-vote-v0" || 0x00 || chain_id_32 || u64be(height) || u64be(round)
#   || block_id_32
DOMAIN = b"coblox-block-vote-v0"
CHAIN_ID = bytes([0x42]) * 32
HEIGHT = 4_211_337
ROUND = 2
BLOCK_ID = bytes([0x7A]) * 32


def block_vote_preimage() -> bytes:
    return (
        DOMAIN
        + b"\x00"
        + CHAIN_ID
        + HEIGHT.to_bytes(8, "big")
        + ROUND.to_bytes(8, "big")
        + BLOCK_ID
    )


MESSAGE = block_vote_preimage()


def non_canonical_y(offset: int, sign: int) -> bytes:
    """The 32-byte encoding of `y_raw = p + offset` with the given sign bit.

    `y_raw >= p` is precisely the class rule 1 says is reduced and RFC 8032
    §5.1.3 step 2 says is rejected. `offset` must stay in `[0, 18]` because
    `p + 19 = 2^255` does not fit in 255 bits.
    """
    assert 0 <= offset <= 18, "y_raw = p + offset must fit in 255 bits"
    assert sign in (0, 1)
    return ((P + offset) | (sign << 255)).to_bytes(32, "little")


def secret_scalar(label: str) -> int:
    """An RFC 8032 clamped scalar, derived from `label` so the fixture is
    reproducible. Clamping makes the scalar a multiple of 8, so `A = [a]B` has
    prime order and rule 3 (`[8]A != identity`) never fires on it by accident."""
    h = bytearray(hashlib.sha512(label.encode("ascii")).digest()[:32])
    h[0] &= 248
    h[31] &= 127
    h[31] |= 64
    return int.from_bytes(h, "little")


def public_key(a: int) -> bytes:
    return oracle.compress(oracle.point_mul(a, oracle.BASE))


def forge_with_torsion_r(label: str, r_enc: bytes) -> tuple[bytes, bytes]:
    """Return `(A_enc, signature)` accepted by the Coblox rule with this `R_enc`.

    Requires `[8]R = O` for the point `R_enc` reduces to. Then the cofactored
    equation `[8][S]B = [8]R + [8][k]A` is `[8][S]B = [8][k]A`, satisfied by
    `S = k·a mod L`. Nothing here is a break of Ed25519: it is a signature that
    is valid under the published rule and invalid under RFC 8032 decoding, which
    is the entire point.
    """
    a = secret_scalar(label)
    a_enc = public_key(a)
    r_point = oracle.decompress(r_enc)
    assert r_point is not None, "R_enc must decode under the Coblox rule"
    assert oracle.point_eq(
        oracle.point_mul(8, r_point), oracle.IDENTITY
    ), "this construction needs [8]R = identity"
    k = int.from_bytes(hashlib.sha512(r_enc + a_enc + MESSAGE).digest(), "little") % L
    s = (k * a) % L
    return a_enc, r_enc + s.to_bytes(32, "little")


def genuine_signature(label: str) -> tuple[bytes, bytes]:
    """An ordinary, entirely canonical Ed25519 signature over `MESSAGE`.

    Used as the base for the three rejecting vectors: each swaps one encoding for
    a non-canonical one, so what the vector isolates is the swap and not some
    other malformation.
    """
    a = secret_scalar(label)
    a_enc = public_key(a)
    r = int.from_bytes(hashlib.sha512(("nonce:" + label).encode("ascii")).digest(), "little") % L
    r_enc = oracle.compress(oracle.point_mul(r, oracle.BASE))
    k = int.from_bytes(hashlib.sha512(r_enc + a_enc + MESSAGE).digest(), "little") % L
    s = (r + k * a) % L
    return a_enc, r_enc + s.to_bytes(32, "little")


def build() -> list[dict[str, str | int]]:
    vectors: list[tuple[str, bytes, bytes]] = []

    # 0-3: the complete set of inputs that a key holder can forge and on which a
    # `y >= p`-rejecting implementation disagrees with Coblox.
    a_enc, sig = forge_with_torsion_r("coblox-ext-0", non_canonical_y(1, 0))
    vectors.append(
        (
            "R_enc = LE(p+1), sign 0: y >= p reduces to y = 1, the identity. "
            "Forged with S = k*a, accepted by rule 4 because [8]R = O. "
            "An implementation that rejects y >= p rejects this signature.",
            a_enc,
            sig,
        )
    )
    a_enc, sig = forge_with_torsion_r("coblox-ext-1", non_canonical_y(1, 1))
    vectors.append(
        (
            "R_enc = LE(p+1), sign 1: y >= p reduces to y = 1 and the sign bit is "
            "then applied to x = 0, so both departures from RFC 8032 are present "
            "at once. Forged with S = k*a; accepted.",
            a_enc,
            sig,
        )
    )
    a_enc, sig = forge_with_torsion_r("coblox-ext-2", non_canonical_y(0, 0))
    vectors.append(
        (
            "R_enc = LE(p), sign 0: y >= p reduces to y = 0, a point of order 4. "
            "Forged with S = k*a, accepted because [8]R = O. "
            "An implementation that rejects y >= p rejects this signature.",
            a_enc,
            sig,
        )
    )
    a_enc, sig = forge_with_torsion_r("coblox-ext-3", non_canonical_y(0, 1))
    vectors.append(
        (
            "R_enc = LE(p), sign 1: the other order-4 point with y = 0. "
            "Forged with S = k*a; accepted. Present so the sign bit is exercised "
            "on a reduced y that is not the identity.",
            a_enc,
            sig,
        )
    )

    # 4: non-canonical A of small order. Rejected by rule 3 under Coblox and by
    # decoding under RFC 8032: same verdict, different reason.
    _, sig = genuine_signature("coblox-ext-4")
    vectors.append(
        (
            "A_enc = LE(p), sign 0: y >= p reduces to y = 0, a point of order 4, "
            "so rule 3 ([8]A != identity) rejects. Rejected by a y >= p-rejecting "
            "implementation too, at decoding: this vector shows decoding coverage, "
            "not divergence.",
            non_canonical_y(0, 0),
            sig,
        )
    )

    # 5: non-canonical A of large order. Decodes, passes rule 3, fails rule 4.
    _, sig = genuine_signature("coblox-ext-5")
    vectors.append(
        (
            "A_enc = LE(p+3), sign 0: y >= p reduces to y = 3, a point of large "
            "order, so decoding succeeds and rule 3 does not fire; rule 4 rejects "
            "because no signature can be produced without the discrete logarithm "
            "of that point. Rejected under both rules.",
            non_canonical_y(3, 0),
            sig,
        )
    )

    # 6: non-canonical R of large order, same argument on the R side.
    a_enc, sig = genuine_signature("coblox-ext-6")
    vectors.append(
        (
            "R_enc = LE(p+3), sign 0: y >= p reduces to a point of large order, so "
            "[8]R != O and the cofactored equation cannot be satisfied without its "
            "discrete logarithm. Rejected under both rules.",
            a_enc,
            non_canonical_y(3, 0) + sig[32:],
        )
    )

    out: list[dict[str, str | int]] = []
    for index, (comment, a_enc, sig) in enumerate(vectors):
        accepted, reason = oracle.verify(a_enc, sig[:32], sig[32:], MESSAGE)
        out.append(
            {
                "index": index,
                "comment": comment,
                "message": MESSAGE.hex(),
                "pub_key": a_enc.hex(),
                "signature": sig.hex(),
                "expected_coblox": "accept" if accepted else "reject",
                "oracle_reason": reason,
            }
        )
    return out


def render(vectors: list[dict[str, str | int]]) -> str:
    return json.dumps(vectors, indent=2, ensure_ascii=True) + "\n"


def main() -> int:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--write",
        action="store_true",
        help="regenerate the fixture instead of checking the committed one",
    )
    args = parser.parse_args()

    rendered = render(build())
    if args.write:
        FIXTURE.write_text(rendered, encoding="utf-8")
        print(f"wrote {FIXTURE.relative_to(REPO_ROOT).as_posix()}")
        return 0

    if not FIXTURE.exists():
        print(f"{FIXTURE}: missing; run with --write")
        return 1
    committed = FIXTURE.read_text(encoding="utf-8")
    if committed != rendered:
        print(f"{FIXTURE.relative_to(REPO_ROOT).as_posix()}: does NOT reproduce")
        print("  the committed fixture and this generator disagree; neither is")
        print("  authoritative until the difference is explained.")
        return 1
    print(f"{FIXTURE.relative_to(REPO_ROOT).as_posix()}: reproduces byte for byte")
    return 0


if __name__ == "__main__":
    sys.exit(main())
