"""Recompute the genesis derivation of `GEN-0` from the bytes the document states.

This tool exists for one reason, and it is not convenience. `chain_id` is
derived from `genesis_block_id`, which is the `block_id` of the height-0 header,
whose preimage carries `chain_id_32`: the derivation is circular at genesis, and
`docs/protocol/README.md` §"Genesis derivation and the placeholder chain ID" is
the rule that breaks the circle. A rule that breaks a circularity is verifiable
only by **two derivations that share no code**, because one implementation is
internally consistent by construction — which is precisely why [DEBT-012] stayed
invisible until [SPEC-010].

So this file is the second road. It was written from the document text and not
from `coblox-core`: the Rust derivation lives in `ChainId::genesis` and
`ChainId::derive`, is reached through `PreimageWriter` and `JsonObject`, and
shares nothing with the `hashlib` and `json.dumps` used here. The two agree on
the published values or this tool exits non-zero.

Method, quoted from the document it reads:

    chain_id = SHA-256("coblox-chain-id-v0\\0" || u32be(len(network_id_utf8))
                       || network_id_utf8 || raw_32_bytes(genesis_block_id))
    block_id = H("coblox-block-id-v0\\0" || chain_id_32 || JCS(block_header))
    <kind>_hash = H(domain || chain_id_32 || JCS(UnsignedProtocolDocument))
    dht_namespace_key = H("coblox-dht-v0\\0" || raw_32_bytes(genesis_block_id))

and the rule itself:

    "The genesis placeholder chain ID is 32 zero bytes. A value that is an input
     to genesis_block_id, and any signature taken over such a value, is computed
     with the placeholder in place of chain_id_32 and in place of any chain_id
     field."

    python sim/tools/genesis_chain_id.py

**The expected values are read from `docs/protocol/README.md`, not copied into
this file**, for the reason `protocol_hashes.py` gives at length: a tool that
carries its own copy of the oracle can disagree with the oracle while looking
authoritative.

**The method is validated on a value this pass did not change before it is used
on one it did.** Section 1 recomputes the consensus `PD-0` hash, which no part
of [SPEC-017] touches. A procedure that cannot reproduce an untouched value is
not evidence for a new one.

**Sections 3 and 4 vary `network_id`, and that is not decoration.** A gate whose
cases all share one value on a quantity that is not the quantity under test has
never seen the case that breaks it — the lesson `GATE-MEASURE-BINDS` of
[SPEC-016] paid for. Here the quantity under test is the derivation and the
quantity that must not stay constant is `network_id`, so `GEN-1` is the same
genesis on a network name of a different byte length.

**`GEN-1` is published, and that is the point of [REVIEW-028] RF-001.** It was
first derived on both roads and only printed, with the two outputs compared by
eye — which showed that the values *move* and asserted nothing about the two
roads moving *together*. Hardcoding the other road's answer would have been
worse, because a road that copies the other's result has stopped being a second
road. Publishing the values in the registry table gives both roads the same
third party to meet, which is exactly the arrangement `GEN-0` already had.

Section 4 then removes the length prefix and swaps the placeholder for 32 `ff`
bytes, and requires the result to move: a clause nobody has watched fail is
arithmetic, not a rule.
"""

from __future__ import annotations

import hashlib
import json
import pathlib
import re
import sys

from protocol_hashes import CONSENSUS_BODY

README = (
    pathlib.Path(__file__).resolve().parents[2] / "docs" / "protocol" / "README.md"
)

# "The genesis placeholder chain ID is 32 zero bytes."
PLACEHOLDER = bytes(32)

DOMAIN_CHAIN_ID = b"coblox-chain-id-v0\x00"
DOMAIN_BLOCK_ID = b"coblox-block-id-v0\x00"
DOMAIN_CONSENSUS_PARAMETERS = b"coblox-consensus-parameters-v0\x00"
DOMAIN_DHT = b"coblox-dht-v0\x00"

GEN0_NETWORK_ID = "genesis-fixture"
# `GEN-1`. Deliberately a different byte length from the one above: a derivation
# fixture that fixes one network name never exercises
# `u32be(len(network_id_utf8))`, and never shows that the name enters the
# height-0 header as well as the `chain_id` preimage.
GEN1_NETWORK_ID = "genesis-fixture-b"

# The row of the registry table that publishes each value, by its `Hash` cell.
REGISTRY_ROWS = {
    "consensus_parameters_pd0": "`consensus_parameters_hash`",
    "consensus_parameters_gen0": "`consensus_parameters_hash` (genesis)",
    "empty_transactions_root": "`empty_transactions_root`",
    "genesis_block_id": "`block_id` (genesis)",
    "chain_id": "`chain_id`",
    "dht_namespace_key": "`dht_namespace_key`",
    "consensus_parameters_gen1": "`consensus_parameters_hash` (genesis, `GEN-1`)",
    "genesis_block_id_gen1": "`block_id` (genesis, `GEN-1`)",
    "chain_id_gen1": "`chain_id` (`GEN-1`)",
    "dht_namespace_key_gen1": "`dht_namespace_key` (`GEN-1`)",
}


def read_registry() -> dict[str, str]:
    """Read the expected values out of the published table.

    A missing row is a hard error rather than a skipped check, for the reason
    `protocol_hashes.py` states: the point of reading the document is that this
    tool cannot silently keep asserting a value the document no longer
    publishes.
    """
    text = README.read_text(encoding="utf-8")
    published: dict[str, str] = {}
    for key, cell in REGISTRY_ROWS.items():
        pattern = re.compile(
            r"^\|\s*" + re.escape(cell) + r"\s*\|[^|]*\|\s*`(sha256:[0-9a-f]{64})`\s*\|$",
            re.MULTILINE,
        )
        match = pattern.search(text)
        if match is None:
            raise SystemExit(
                f"genesis_chain_id: no registry row for {cell} in README.md. "
                f"Either the row was removed or its shape changed; either way "
                f"this tool has stopped reading the oracle it claims to read."
            )
        published[key] = match.group(1)
    return published


def jcs(obj) -> bytes:
    """RFC 8785 canonical JSON, for the subset the protocol admits.

    Every value in these objects is a string or a nested object of strings, so
    JCS reduces to sorted keys, no whitespace and minimal escaping.
    """
    return json.dumps(
        obj, sort_keys=True, separators=(",", ":"), ensure_ascii=False
    ).encode("utf-8")


def sha(*parts: bytes) -> str:
    digest = hashlib.sha256()
    for part in parts:
        digest.update(part)
    return "sha256:" + digest.hexdigest()


def raw(prefixed: str) -> bytes:
    """`raw_32_bytes`: strip the `sha256:` presentation prefix."""
    return bytes.fromhex(prefixed.removeprefix("sha256:"))


def u32be(value: int) -> bytes:
    return value.to_bytes(4, "big")


def consensus_document(network_id: str, chain_id: bytes) -> dict:
    """The genesis `consensus_parameters` document of `GEN-0`.

    `sequence:"1"`, `activation_height:"0"`, and the consensus `PD-0` body
    unchanged, as the fixture definition states.
    """
    return {
        "schema_version": "0.1",
        "document_kind": "consensus_parameters",
        "network_id": network_id,
        "chain_id": "sha256:" + chain_id.hex(),
        "sequence": "1",
        "activation_height": "0",
        "body": CONSENSUS_BODY,
    }


def pd0_consensus_document() -> dict:
    """The consensus `PD-0`, which this pass does not change.

    `network_id:"fixture"`, zero `chain_id`, `sequence:"1"`,
    `activation_height:"1"`.
    """
    return {
        "schema_version": "0.1",
        "document_kind": "consensus_parameters",
        "network_id": "fixture",
        "chain_id": "sha256:" + bytes(32).hex(),
        "sequence": "1",
        "activation_height": "1",
        "body": CONSENSUS_BODY,
    }


def genesis_header(
    network_id: str, consensus_parameters_hash: str, transactions_root: str
) -> dict:
    """The height-0 header of `GEN-0`."""
    return {
        "schema_version": "0.1",
        "protocol_version": "0.1",
        "network_id": network_id,
        "height": "0",
        "round": "0",
        "timestamp_ms": "1",
        "previous_block_id": "sha256:" + bytes(32).hex(),
        "transactions_root": transactions_root,
        "state_root": "sha256:" + (bytes([0xEE]) * 32).hex(),
        "validator_set_hash": "sha256:" + (bytes([0xDD]) * 32).hex(),
        "next_validator_set_hash": "sha256:" + (bytes([0xDD]) * 32).hex(),
        "consensus_parameters_hash": consensus_parameters_hash,
    }


def derive(network_id: str, placeholder: bytes = PLACEHOLDER,
           length_prefix: bool = True) -> dict[str, str]:
    """The whole genesis derivation, in the order the rule imposes it.

    `placeholder` and `length_prefix` are parameters only so that section 4 can
    break each clause and watch the result move. Nothing but that section passes
    anything other than the defaults.
    """
    empty_transactions_root = sha(bytes([0x03]))
    document = consensus_document(network_id, placeholder)
    consensus_parameters_hash = sha(
        DOMAIN_CONSENSUS_PARAMETERS, placeholder, jcs(document)
    )
    header = genesis_header(
        network_id, consensus_parameters_hash, empty_transactions_root
    )
    genesis_block_id = sha(DOMAIN_BLOCK_ID, placeholder, jcs(header))
    network_bytes = network_id.encode("utf-8")
    prefix = u32be(len(network_bytes)) if length_prefix else b""
    chain_id = sha(DOMAIN_CHAIN_ID, prefix, network_bytes, raw(genesis_block_id))
    return {
        "empty_transactions_root": empty_transactions_root,
        "consensus_parameters_gen0": consensus_parameters_hash,
        "genesis_block_id": genesis_block_id,
        "chain_id": chain_id,
        "dht_namespace_key": sha(DOMAIN_DHT, raw(genesis_block_id)),
    }


def report(label: str, computed: str, published: str) -> bool:
    ok = computed == published
    print(f"  {'ok  ' if ok else 'FAIL'}  {label}")
    print(f"          computed  {computed}")
    if not ok:
        print(f"          published {published}")
    return ok


def differs(label: str, changed: str, baseline: str) -> bool:
    ok = changed != baseline
    print(f"  {'ok  ' if ok else 'FAIL'}  {label}")
    print(f"          {'moved to' if ok else 'STAYED  '} {changed}")
    return ok


def main() -> int:
    published = read_registry()
    ok = True

    print("1. the method, on a value this pass did not change")
    ok &= report(
        "consensus_parameters_hash / consensus PD-0",
        sha(
            DOMAIN_CONSENSUS_PARAMETERS,
            bytes(32),
            jcs(pd0_consensus_document()),
        ),
        published["consensus_parameters_pd0"],
    )

    print("2. GEN-0, derived under the genesis placeholder rule")
    derived = derive(GEN0_NETWORK_ID)
    for key, label in (
        ("empty_transactions_root", "empty_transactions_root / H(0x03)"),
        ("consensus_parameters_gen0", "consensus_parameters_hash / GEN-0 document"),
        ("genesis_block_id", "block_id / GEN-0 genesis header"),
        ("chain_id", "chain_id / GEN-0"),
        ("dht_namespace_key", "dht_namespace_key / DHT-0"),
    ):
        ok &= report(label, derived[key], published[key])

    print("3. GEN-1, the same genesis on a network name of a different length")
    variant = derive(GEN1_NETWORK_ID)
    for key, published_key, label in (
        ("consensus_parameters_gen0", "consensus_parameters_gen1",
         "consensus_parameters_hash / GEN-1 document"),
        ("genesis_block_id", "genesis_block_id_gen1", "block_id / GEN-1 header"),
        ("chain_id", "chain_id_gen1", "chain_id / GEN-1"),
        ("dht_namespace_key", "dht_namespace_key_gen1", "dht_namespace_key / GEN-1"),
    ):
        ok &= report(label, variant[key], published[published_key])

    print("4. every clause, watched failing")
    ok &= differs(
        "network_id enters the header, so genesis_block_id moves with it",
        variant["genesis_block_id"],
        derived["genesis_block_id"],
    )
    ok &= differs(
        "network_id enters chain_id twice over, so chain_id moves too",
        variant["chain_id"],
        derived["chain_id"],
    )
    ok &= differs(
        "dropping u32be(len(network_id_utf8)) changes chain_id",
        derive(GEN0_NETWORK_ID, length_prefix=False)["chain_id"],
        derived["chain_id"],
    )
    ok &= differs(
        "a placeholder of 32 ff bytes changes genesis_block_id",
        derive(GEN0_NETWORK_ID, placeholder=bytes([0xFF]) * 32)["genesis_block_id"],
        derived["genesis_block_id"],
    )
    ok &= differs(
        "and changes chain_id with it",
        derive(GEN0_NETWORK_ID, placeholder=bytes([0xFF]) * 32)["chain_id"],
        derived["chain_id"],
    )

    print("ok" if ok else "MISMATCH")
    return 0 if ok else 1


if __name__ == "__main__":
    sys.exit(main())
