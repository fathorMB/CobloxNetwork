"""Recompute the protocol-document hashes of the `PD-0` conformance fixtures.

Versioned because [REVIEW-014] RF-007 found the transcript of [SPEC-009] citing
`scratch/*.py` scripts that were never in the tree: evidence nobody else can
re-run is not evidence.

Method, from `docs/protocol/README.md` §"Hash preimage registry":

    <kind>_hash = H(domain || chain_id_32 || JCS(UnsignedProtocolDocument))

`H` is SHA-256 and `JCS` is RFC 8785 canonical JSON. Every `PD-0` body is a flat
object of ASCII keys and string values, for which JCS reduces to sorted keys,
no whitespace, and minimal escaping — which is what `json.dumps` produces with
`sort_keys=True` and compact separators.

    python tools/protocol_hashes.py

The script validates the method on the fixtures that did **not** change before
reporting the one that did, which is the discipline `GATE-FIXTURES-RECOMPUTED`
exists to enforce: a procedure that cannot reproduce an untouched value is not
evidence for a touched one.
"""

from __future__ import annotations

import hashlib
import json
import sys

CHAIN_ID = bytes(32)

DOMAINS = {
    "enrollment_parameters": b"coblox-enrollment-parameter-set-v0\x00",
    "reward_policy": b"coblox-reward-policy-v0\x00",
    "hosting_rate_card": b"coblox-hosting-rate-card-v0\x00",
    "consensus_parameters": b"coblox-consensus-parameters-v0\x00",
}

# The registry values published in docs/protocol/README.md.
PUBLISHED = {
    "enrollment_parameters": "sha256:a2553f36f496d30a7773b9f6424c3ffd5ef22e3f8620bf0cca88a9bcdccd4f63",
    "reward_policy": "sha256:89da35fbb8f0ba3c9ebffc0e3c5987045a005aaa7414356ef16a978a92025c48",
    "hosting_rate_card": "sha256:9b10204164f4197fb368f0f6ad6c186ae7af1a85b7b6383eeac412a10b8b3ae8",
    "consensus_parameters": "sha256:628c66f9ca8ac1a3161a0159201f7b6c6bf4c7500b390bc89b9b65a6c50ccbe9",
}

ENROLLMENT_BODY = {
    "pow_algorithm": "argon2id-leading-zero-bits-v0",
    "difficulty_bits": "4",
    "memory_kib": "65536",
    "iterations": "3",
    "lanes": "4",
    "tag_length_bytes": "32",
    "max_request_age_ms": "1",
    "max_future_skew_ms": "1",
    "recent_block_window": "1",
}

REWARD_BODY_KEYS = (
    "reward_epoch_ms",
    "existence_fund_microtokens_per_epoch",
    "availability_microtokens_per_unit",
    "storage_microtokens_per_byte_epoch",
    "compute_microtokens_per_million_fuel",
    "publisher_microtokens_per_active_subscriber",
    "publisher_reward_cap_numerator",
    "publisher_reward_cap_denominator",
    "storage_units_per_contribution_unit",
    "compute_units_per_contribution_unit",
    "validator_eligibility_threshold_units",
    "validator_eligibility_window_epochs",
    "validator_eligibility_min_issuers",
)

HOSTING_BODY = {
    "billing_epoch_ms": "1",
    "minimum_billable_epochs": "1",
    "microtokens_per_replica_epoch": "1",
    "microtokens_per_gib_epoch": "1",
    "microtokens_per_million_fuel": "1",
}

CONSENSUS_BODY = {
    "max_clock_drift_ms": "1",
    "max_envelope_validity_ms": "1",
    "replay_cache_entries_per_peer": "1",
    "replay_cache_entries_global": "1",
    "max_weak_subjectivity_age_ms": "1",
    "max_current_balance_age_ms": "1",
    "app_suspension_notice_epochs": "1",
    "min_revocation_effective_delay_blocks": "1",
    "election_epoch_blocks": "4",
    "candidacy_close_blocks": "3",
    "election_entropy_blocks": "2",
    "validator_min_set_size": "8",
    "validator_target_set_size": "12",
    "validator_max_set_size": "12",
    "validator_churn_cap_seats": "3",
    "validator_max_consecutive_terms": "4",
    "validator_cooldown_epochs": "1",
    "validator_min_capture_epochs": "1",
}


def reward_body(availability: str) -> dict[str, str]:
    body = {k: "1" for k in REWARD_BODY_KEYS}
    body["availability_microtokens_per_unit"] = availability
    body["publisher_reward_cap_denominator"] = "2"
    body["validator_eligibility_min_issuers"] = "2"
    return body


def jcs(obj) -> bytes:
    return json.dumps(obj, sort_keys=True, separators=(",", ":"),
                      ensure_ascii=False).encode("utf-8")


def document_hash(kind: str, body: dict) -> str:
    unsigned = {
        "schema_version": "0.1",
        "document_kind": kind,
        "network_id": "fixture",
        "chain_id": "sha256:" + CHAIN_ID.hex(),
        "sequence": "1",
        "activation_height": "1",
        "body": body,
    }
    digest = hashlib.sha256(DOMAINS[kind] + CHAIN_ID + jcs(unsigned)).hexdigest()
    return "sha256:" + digest


def main() -> int:
    print("Method validation on fixtures this pass did NOT change:")
    unchanged_ok = True
    for kind, body in (
        ("enrollment_parameters", ENROLLMENT_BODY),
        ("hosting_rate_card", HOSTING_BODY),
    ):
        got = document_hash(kind, body)
        ok = got == PUBLISHED[kind]
        unchanged_ok = unchanged_ok and ok
        print(f"  {kind:<24} {'MATCH' if ok else 'MISMATCH'}")
        print(f"    published {PUBLISHED[kind]}")
        print(f"    computed  {got}")

    print()
    print("Fixtures this pass changed:")
    for kind, body in (
        ("consensus_parameters", CONSENSUS_BODY),
        ("reward_policy", reward_body("0")),
    ):
        got = document_hash(kind, body)
        ok = got == PUBLISHED[kind]
        print(f"  {kind:<24} {'MATCH' if ok else 'DIFFERS from registry'}")
        print(f"    published {PUBLISHED[kind]}")
        print(f"    computed  {got}")

    print()
    print("The reward fixture with the pre-[ADR-010] availability tariff, for")
    print("comparison — this is the shape the new validity rule forbids:")
    print(f"    availability=1 -> {document_hash('reward_policy', reward_body('1'))}")

    print()
    print(f"method validated on unchanged fixtures: {'PASS' if unchanged_ok else 'FAIL'}")
    return 0 if unchanged_ok else 1


if __name__ == "__main__":
    raise SystemExit(main())
