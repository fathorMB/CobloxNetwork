//! Canonical serialization, verified in **both** directions.
//!
//! Forward: the bytes this crate produces for the specification's own canonical
//! examples are those exact bytes. Backward: bytes that are not canonical are
//! rejected, and are never quietly normalized into something acceptable.
//!
//! The forward oracle is unusually good and is used deliberately. Each
//! `CANONICAL_*` constant is a one-line canonical serialization published in
//! `docs/protocol/`, copied verbatim. Feeding one of them to
//! [`JsonObject::parse_canonical`] asserts, in a single call, that this
//! implementation agrees with the document about key order, string escaping,
//! integer spelling, base64url padding and hash presentation — because that
//! function parses, re-serializes, and requires byte equality.

mod common;

use coblox_core::block::BlockHeader;
use coblox_core::hash::Digest32;
use coblox_core::json::{Json, JsonObject};

// --- ledger.md#block-format ------------------------------------------------
const CANONICAL_BLOCK_HEADER: &str = r#"{"consensus_parameters_hash":"sha256:7df04d03b60f62852f0d76c847d0181a2b17b43a50f987c0b9f814e70f064bcc","height":"42","network_id":"coblox-devnet-0","next_validator_set_hash":"sha256:1df0a6454faaa5985b7f98c48d3c60d2ed62d5b3b24fe8e97d3dca1dd36f1120","previous_block_id":"sha256:7e0694f564afa2d047db4eb58f4f2b3d322d71db808f6bbf5313ee2d2a4a95af","protocol_version":"0.1","round":"0","schema_version":"0.1","state_root":"sha256:993b24bf6115fbf5651d615ca57a1baa825baf304b1dcc4d52debbc7fa3bd6d8","timestamp_ms":"1787654600000","transactions_root":"sha256:00811b3f6ae09c7acdb2e5c92fb273a05481f75fd477901fd43f76a9290b19b7","validator_set_hash":"sha256:1df0a6454faaa5985b7f98c48d3c60d2ed62d5b3b24fe8e97d3dca1dd36f1120"}"#;

// --- ledger.md, canonical transaction examples ------------------------------
const CANONICAL_EXISTENCE_MINT: &str = r#"{"authorization":{"quorum_certificate":{"signatures":[{"signature":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA","validator_id":"val-001"}],"validator_set_hash":"sha256:1df0a6454faaa5985b7f98c48d3c60d2ed62d5b3b24fe8e97d3dca1dd36f1120"}},"body":{"amount_microtokens":"250000","beneficiary_node_id":"cblx1ci6q36gqm6u3spknxzr7p5r2y4xw7n25d5icm7rsoq7lq6ka","eligible_node_count":"4000","eligible_set_root":"sha256:2f0e2c8a9d4b6f1c3e5a7b9d0f2468ace13579bdf02468ace13579bdf02468ac","evidence_tx_ids":["sha256:313eb3d86d8c049838543325910bccb953b828da764b5f18bff11d7a123b0e68"],"policy_hash":"sha256:7df04d03b60f62852f0d76c847d0181a2b17b43a50f987c0b9f814e70f064bcc","reason":"existence_income","reward_epoch":"17"},"created_at_ms":"1787654500000","expires_at_ms":"1787740900000","kind":"mint","network_id":"coblox-devnet-0","schema_version":"0.1"}"#;
const CANONICAL_PUBLISHER_MINT: &str = r#"{"authorization":{"quorum_certificate":{"signatures":[{"signature":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA","validator_id":"val-001"}],"validator_set_hash":"sha256:1df0a6454faaa5985b7f98c48d3c60d2ed62d5b3b24fe8e97d3dca1dd36f1120"}},"body":{"active_subscriber_count":"128","active_subscription_root":"sha256:fc9cd19c4f7b32970a7c870e821dbca915d204c09a496d60b17f66ec8790ad3a","amount_microtokens":"6400000","app_id":"sha256:77a1d5d603f675f8b8a3f63ac94d14f9ea04c86d5e216ac4f0e1bd5ebac0ecf8","beneficiary_node_id":"cblx1ci6q36gqm6u3spknxzr7p5r2y4xw7n25d5icm7rsoq7lq6ka","counted_subscription_burn_microtokens":"38400000","policy_hash":"sha256:7df04d03b60f62852f0d76c847d0181a2b17b43a50f987c0b9f814e70f064bcc","reason":"publisher_reward","reward_epoch":"17"},"created_at_ms":"1787654502000","expires_at_ms":"1787740902000","kind":"mint","network_id":"coblox-devnet-0","schema_version":"0.1"}"#;
const CANONICAL_FUND_APP: &str = r#"{"authorization":{"public_key":"11qYAYdk9J0L5Z-6hB4qMTPBSAE5nK1G0IU2n6z1V9g","signature":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"},"body":{"amount_microtokens":"2400000","app_id":"sha256:77a1d5d603f675f8b8a3f63ac94d14f9ea04c86d5e216ac4f0e1bd5ebac0ecf8","payer_account_nonce":"8","payer_node_id":"cblx1ci6q36gqm6u3spknxzr7p5r2y4xw7n25d5icm7rsoq7lq6ka"},"created_at_ms":"1787654505000","expires_at_ms":"1787654805000","kind":"fund_app","network_id":"coblox-devnet-0","schema_version":"0.1"}"#;
const CANONICAL_SUBSCRIPTION_BURN: &str = r#"{"authorization":{"public_key":"11qYAYdk9J0L5Z-6hB4qMTPBSAE5nK1G0IU2n6z1V9g","signature":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"},"body":{"account_nonce":"9","amount_microtokens":"300000","app_id":"sha256:77a1d5d603f675f8b8a3f63ac94d14f9ea04c86d5e216ac4f0e1bd5ebac0ecf8","payer_node_id":"cblx1ci6q36gqm6u3spknxzr7p5r2y4xw7n25d5icm7rsoq7lq6ka","pricing_hash":"sha256:2d1e35bf61f89fc50cb9cafe158f44ad63d522898971e0211d59708331c4b404","reason":"app_subscription","service_period_end_ms":"1790332800000","service_period_start_ms":"1787654400000"},"created_at_ms":"1787654520000","expires_at_ms":"1787654820000","kind":"burn","network_id":"coblox-devnet-0","schema_version":"0.1"}"#;
const CANONICAL_CHALLENGE_COMMITMENT: &str = r#"{"authorization":{"public_key":"11qYAYdk9J0L5Z-6hB4qMTPBSAE5nK1G0IU2n6z1V9g","signature":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"},"body":{"commitment_epoch":"17","issuer_commitment":"sha256:19556b209c36de1940340bd3ada4a4c821fe70cde0fd3906af2b71f31445e4d5","issuer_node_id":"cblx1issuerfixture"},"created_at_ms":"1787654400000","expires_at_ms":"1787654700000","kind":"challenge_commitment","network_id":"coblox-devnet-0","schema_version":"0.1"}"#;
const CANONICAL_REVOKE_IDENTITY: &str = r#"{"authorization":{"quorum_certificate":{"signatures":[{"signature":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA","validator_id":"val-001"}],"validator_set_hash":"sha256:1df0a6454faaa5985b7f98c48d3c60d2ed62d5b3b24fe8e97d3dca1dd36f1120"}},"body":{"effective_height":"50","node_id":"cblx1revokedfixture","reason":"key_compromise","replacement_node_id":"cblx1replacementfixture"},"created_at_ms":"1787654550000","expires_at_ms":"1787740950000","kind":"revoke_identity","network_id":"coblox-devnet-0","schema_version":"0.1"}"#;
const CANONICAL_VALIDATOR_CANDIDACY: &str = r#"{"authorization":{"public_key":"11qYAYdk9J0L5Z-6hB4qMTPBSAE5nK1G0IU2n6z1V9g","signature":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"},"body":{"consensus_public_key":"IjIiMiIyIjIiMiIyIjIiMiIyIjIiMiIyIjIiMiIyIjI","election_epoch":"3","key_binding_signature":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA","node_id":"cblx1ci6q36gqm6u3spknxzr7p5r2y4xw7n25d5icm7rsoq7lq6ka"},"created_at_ms":"1787654530000","expires_at_ms":"1787740930000","kind":"validator_candidacy","network_id":"coblox-devnet-0","schema_version":"0.1"}"#;
const CANONICAL_CHALLENGE_EVIDENCE: &str = r#"{"authorization":{"quorum_certificate":{"signatures":[{"signature":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA","validator_id":"val-001"}],"validator_set_hash":"sha256:1df0a6454faaa5985b7f98c48d3c60d2ed62d5b3b24fe8e97d3dca1dd36f1120"}},"body":{"auditor_signatures":[{"signature":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA","validator_id":"val-001"}],"challenge_id":"sha256:3d56e5dd5104a2ad5c733fa4f0b6d8f35de2f68509e9c10a3d473128eaec0b21","completed_at_ms":"1787654416000","issuer_node_id":"cblx1issuerfixture","issuer_reveal":"REREREREREREREREREREREREREREREREREREREREREQ","kind":"availability","measured_units":"1","outcome":"passed","request":{"assignment":{"response_bytes":"32"},"challenge_id":"sha256:3d56e5dd5104a2ad5c733fa4f0b6d8f35de2f68509e9c10a3d473128eaec0b21","deadline_ms":"1787654420000","issued_at_ms":"1787654415000","issuer_commitment":"sha256:19556b209c36de1940340bd3ada4a4c821fe70cde0fd3906af2b71f31445e4d5","issuer_node_id":"cblx1issuerfixture","issuer_signature":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA","kind":"availability","randomness":"AAECAwQFBgcICQoLDA0ODxAREhMUFRYXGBkaGxwdHh8","randomness_source":{"beacon_block_id":"sha256:7e0694f564afa2d047db4eb58f4f2b3d322d71db808f6bbf5313ee2d2a4a95af","beacon_height":"40","commitment_epoch":"17"},"subject_node_id":"cblx1ci6q36gqm6u3spknxzr7p5r2y4xw7n25d5icm7rsoq7lq6ka"},"request_hash":"sha256:3d56e5dd5104a2ad5c733fa4f0b6d8f35de2f68509e9c10a3d473128eaec0b21","response":{"challenge_id":"sha256:3d56e5dd5104a2ad5c733fa4f0b6d8f35de2f68509e9c10a3d473128eaec0b21","completed_at_ms":"1787654416000","result":{"kind":"availability","response":"MzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzM"},"subject_node_id":"cblx1ci6q36gqm6u3spknxzr7p5r2y4xw7n25d5icm7rsoq7lq6ka","subject_signature":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"},"response_hash":"sha256:8bc23b6277b0892c0eea482c835359a2ad975ac18af9832b727738a880f2400f","subject_node_id":"cblx1ci6q36gqm6u3spknxzr7p5r2y4xw7n25d5icm7rsoq7lq6ka"},"created_at_ms":"1787654420000","expires_at_ms":"1787740820000","kind":"challenge_evidence","network_id":"coblox-devnet-0","schema_version":"0.1"}"#;
const CANONICAL_ACCOUNT_PROOF: &str = r#"{"account_key":"AAECAwQFBgcICQoLDA0ODxAREhMUFRYXGBkaGxwdHh8","account_kind":"node","account_nonce":"0","balance_microtokens":"0","present":false,"sibling_bitmap":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA","siblings":[],"subject_id":"cblx1absentfixture"}"#;

// --- identity.md -----------------------------------------------------------
const CANONICAL_ENROLLMENT_REQUEST: &str = r#"{"created_at_ms":"1787654400000","libp2p_peer_id":"12D3KooWD3eckifWpRn9wQpMG9R9hX3sD158z7EqHWmweQAJU5SA","network_id":"coblox-devnet-0","node_id":"cblx176fmuouuc5v2xyqqxgef5uwrdqt53yqazdlxwcfl6a63bxarnuyq","pow":{"algorithm":"argon2id-leading-zero-bits-v0","difficulty_bits":"4","iterations":"3","lanes":"4","memory_kib":"65536","nonce":"11","parameter_set_hash":"sha256:7df04d03b60f62852f0d76c847d0181a2b17b43a50f987c0b9f814e70f064bcc","recent_block_height":"41","recent_block_id":"sha256:7e0694f564afa2d047db4eb58f4f2b3d322d71db808f6bbf5313ee2d2a4a95af"},"public_key":"L_o1qZ06PPuxe7fB3FVhsYqNzKTfONxhPqhZw36xM2s","schema_version":"0.1","signature":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"}"#;
const CANONICAL_ENROLLMENT_CERTIFICATE: &str = r#"{"enrollment_request_hash":"sha256:44dc2df246a89f42d9a9da10f621c86f5141b597b1a6f08cc78b5e61a8388eb1","issued_at_ms":"1787654405000","libp2p_peer_id":"12D3KooWD3eckifWpRn9wQpMG9R9hX3sD158z7EqHWmweQAJU5SA","network_id":"coblox-devnet-0","node_id":"cblx176fmuouuc5v2xyqqxgef5uwrdqt53yqazdlxwcfl6a63bxarnuyq","public_key":"L_o1qZ06PPuxe7fB3FVhsYqNzKTfONxhPqhZw36xM2s","schema_version":"0.1","signatures":[{"signature":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA","validator_id":"val-001"}],"valid_from_height":"42","validator_set_hash":"sha256:1df0a6454faaa5985b7f98c48d3c60d2ed62d5b3b24fe8e97d3dca1dd36f1120"}"#;

// --- wire.md ---------------------------------------------------------------
const CANONICAL_ENVELOPE: &str = r#"{"created_at_ms":"1787654410000","expires_at_ms":"1787654470000","message_id":"sha256:56d2aa0cd4c2ff0b06c47b478b6bfc2dff88b2c162c6cff1e33f9bf3284c7308","message_type":"ledger_status_request","network_id":"coblox-devnet-0","nonce":"AAECAwQFBgcICQoLDA0ODw","payload":{"finalized_height":"41","want_validator_set":true},"schema_version":"0.1","sender_node_id":"cblx1ci6q36gqm6u3spknxzr7p5r2y4xw7n25d5icm7rsoq7lq6ka","signature":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"}"#;
const CANONICAL_CHALLENGE_REQUEST: &str = r#"{"assignment":{"response_bytes":"32"},"challenge_id":"sha256:3d56e5dd5104a2ad5c733fa4f0b6d8f35de2f68509e9c10a3d473128eaec0b21","deadline_ms":"1787654420000","issued_at_ms":"1787654415000","issuer_commitment":"sha256:19556b209c36de1940340bd3ada4a4c821fe70cde0fd3906af2b71f31445e4d5","issuer_node_id":"cblx1issuerfixture","issuer_signature":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA","kind":"availability","randomness":"AAECAwQFBgcICQoLDA0ODxAREhMUFRYXGBkaGxwdHh8","randomness_source":{"beacon_block_id":"sha256:7e0694f564afa2d047db4eb58f4f2b3d322d71db808f6bbf5313ee2d2a4a95af","beacon_height":"40","commitment_epoch":"17"},"subject_node_id":"cblx1ci6q36gqm6u3spknxzr7p5r2y4xw7n25d5icm7rsoq7lq6ka"}"#;
const CANONICAL_CHALLENGE_RESPONSE: &str = r#"{"challenge_id":"sha256:3d56e5dd5104a2ad5c733fa4f0b6d8f35de2f68509e9c10a3d473128eaec0b21","completed_at_ms":"1787654416000","result":{"kind":"availability","response":"MzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzMzM"},"subject_node_id":"cblx1ci6q36gqm6u3spknxzr7p5r2y4xw7n25d5icm7rsoq7lq6ka","subject_signature":"AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA"}"#;
const CANONICAL_LEDGER_RANGE_REQUEST: &str = r#"{"expected_previous_block_id":"sha256:7e0694f564afa2d047db4eb58f4f2b3d322d71db808f6bbf5313ee2d2a4a95af","from_height":"42","max_blocks":"64"}"#;
const CANONICAL_ERROR_RESPONSE: &str = r#"{"error_code":"invalid_request","message_id":"sha256:56d2aa0cd4c2ff0b06c47b478b6bfc2dff88b2c162c6cff1e33f9bf3284c7308","retry_after_ms":"0"}"#;

/// Every published one-line canonical serialization in `docs/protocol/`.
fn published_canonical_examples() -> Vec<(&'static str, &'static str)> {
    vec![
        ("ledger.md block header", CANONICAL_BLOCK_HEADER),
        ("ledger.md existence-income mint", CANONICAL_EXISTENCE_MINT),
        ("ledger.md publisher-reward mint", CANONICAL_PUBLISHER_MINT),
        ("ledger.md fund_app", CANONICAL_FUND_APP),
        ("ledger.md subscription burn", CANONICAL_SUBSCRIPTION_BURN),
        (
            "ledger.md challenge_commitment",
            CANONICAL_CHALLENGE_COMMITMENT,
        ),
        ("ledger.md revoke_identity", CANONICAL_REVOKE_IDENTITY),
        (
            "ledger.md validator_candidacy",
            CANONICAL_VALIDATOR_CANDIDACY,
        ),
        ("ledger.md challenge_evidence", CANONICAL_CHALLENGE_EVIDENCE),
        ("ledger.md account proof", CANONICAL_ACCOUNT_PROOF),
        (
            "identity.md enrollment request",
            CANONICAL_ENROLLMENT_REQUEST,
        ),
        (
            "identity.md enrollment certificate",
            CANONICAL_ENROLLMENT_CERTIFICATE,
        ),
        ("wire.md signed envelope", CANONICAL_ENVELOPE),
        ("wire.md challenge request", CANONICAL_CHALLENGE_REQUEST),
        ("wire.md challenge response", CANONICAL_CHALLENGE_RESPONSE),
        (
            "wire.md ledger range request",
            CANONICAL_LEDGER_RANGE_REQUEST,
        ),
        ("wire.md error response", CANONICAL_ERROR_RESPONSE),
    ]
}

/// Forward direction, part one: every published canonical example is accepted
/// and re-serializes to exactly its own bytes.
#[test]
fn every_published_canonical_example_round_trips_byte_for_byte() {
    for (label, text) in published_canonical_examples() {
        let object = JsonObject::parse_canonical(text.as_bytes())
            .unwrap_or_else(|error| panic!("{label} was rejected: {error}"));
        assert_eq!(
            object.to_jcs(),
            text.as_bytes(),
            "{label} did not re-serialize to its own bytes"
        );
    }
}

/// Forward direction, part two: a typed object built from its fields serializes
/// to the document's bytes without ever being told what those bytes are.
#[test]
fn a_typed_block_header_serializes_to_the_published_bytes() {
    let header = BlockHeader {
        schema_version: "0.1".to_owned(),
        protocol_version: "0.1".to_owned(),
        network_id: "coblox-devnet-0".to_owned(),
        height: 42,
        round: 0,
        timestamp_ms: 1_787_654_600_000,
        previous_block_id: Digest32::parse_prefixed(
            "sha256:7e0694f564afa2d047db4eb58f4f2b3d322d71db808f6bbf5313ee2d2a4a95af",
        )
        .unwrap(),
        transactions_root: Digest32::parse_prefixed(
            "sha256:00811b3f6ae09c7acdb2e5c92fb273a05481f75fd477901fd43f76a9290b19b7",
        )
        .unwrap(),
        state_root: Digest32::parse_prefixed(
            "sha256:993b24bf6115fbf5651d615ca57a1baa825baf304b1dcc4d52debbc7fa3bd6d8",
        )
        .unwrap(),
        validator_set_hash: Digest32::parse_prefixed(
            "sha256:1df0a6454faaa5985b7f98c48d3c60d2ed62d5b3b24fe8e97d3dca1dd36f1120",
        )
        .unwrap(),
        next_validator_set_hash: Digest32::parse_prefixed(
            "sha256:1df0a6454faaa5985b7f98c48d3c60d2ed62d5b3b24fe8e97d3dca1dd36f1120",
        )
        .unwrap(),
        consensus_parameters_hash: Digest32::parse_prefixed(
            "sha256:7df04d03b60f62852f0d76c847d0181a2b17b43a50f987c0b9f814e70f064bcc",
        )
        .unwrap(),
    };
    assert_eq!(
        header.to_json().unwrap().to_jcs(),
        CANONICAL_BLOCK_HEADER.as_bytes()
    );
}

/// Backward direction: each of these is a *second spelling* of an object that
/// already has a canonical one, and each is rejected rather than normalized.
#[test]
fn non_canonical_bytes_are_rejected_and_never_normalized() {
    let cases: Vec<(&str, String)> = vec![
        (
            "keys out of canonical order",
            r#"{"round":"0","height":"42"}"#.to_owned(),
        ),
        (
            "insignificant whitespace between members",
            r#"{"height": "42","round":"0"}"#.to_owned(),
        ),
        (
            "indentation and newlines",
            "{\n  \"height\": \"42\"\n}".to_owned(),
        ),
        (
            "a trailing newline after the object",
            format!("{CANONICAL_LEDGER_RANGE_REQUEST}\n"),
        ),
        (
            "a UTF-8 byte order mark",
            format!("\u{feff}{CANONICAL_LEDGER_RANGE_REQUEST}"),
        ),
        (
            "a duplicate key",
            r#"{"height":"42","height":"43"}"#.to_owned(),
        ),
        ("an integer as a JSON number", r#"{"height":42}"#.to_owned()),
        ("a floating-point value", r#"{"ratio":1.5}"#.to_owned()),
        ("a null value", r#"{"height":null}"#.to_owned()),
        (
            "an unnecessarily escaped solidus",
            r#"{"path":"a\/b"}"#.to_owned(),
        ),
        (
            "a long \\u escape where a short one exists",
            "{\"text\":\"\\u000a\"}".to_owned(),
        ),
        (
            "an uppercase hexadecimal escape",
            "{\"text\":\"\\u001F\"}".to_owned(),
        ),
        (
            "a raw control character in a string",
            "{\"text\":\"a\u{0001}b\"}".to_owned(),
        ),
        ("an upper-case object key", r#"{"Height":"42"}"#.to_owned()),
        (
            "a kebab-case object key",
            r#"{"height-ms":"42"}"#.to_owned(),
        ),
        (
            "trailing bytes after the value",
            r#"{"height":"42"} {}"#.to_owned(),
        ),
        ("a top-level array", r#"["height"]"#.to_owned()),
        ("a lone surrogate escape", r#"{"text":"\ud800"}"#.to_owned()),
    ];
    for (label, text) in cases {
        let outcome = JsonObject::parse_canonical(text.as_bytes());
        assert!(outcome.is_err(), "{label} was accepted: {text}");
    }

    // The "non-shortest integer" case deserves its own note: the *bytes* above
    // are structurally canonical JSON, so `parse_canonical` accepts the string
    // and the rejection happens at the typed accessor. Both layers are checked
    // so that neither is relied on alone.
    let object = JsonObject::parse_canonical(br#"{"height":"042"}"#).unwrap();
    assert!(object.uint("height").is_err());
    assert_eq!(
        JsonObject::parse_canonical(br#"{"height":"42"}"#)
            .unwrap()
            .uint("height")
            .unwrap(),
        42
    );
}

/// Non-canonical *field* encodings are rejected by the typed accessors, which
/// is where a signature verifier would otherwise be handed a second spelling.
#[test]
fn non_canonical_field_encodings_are_rejected_by_the_typed_accessors() {
    // Padded base64url for a 32-byte key.
    assert!(
        coblox_core::encoding::base64url_decode_fixed::<32>(
            "AAECAwQFBgcICQoLDA0ODxAREhMUFRYXGBkaGxwdHh8=",
            "test"
        )
        .is_err()
    );
    // Upper-case hexadecimal in a hash.
    assert!(
        Digest32::parse_prefixed(
            "sha256:7E0694F564AFA2D047DB4EB58F4F2B3D322D71DB808F6BBF5313EE2D2A4A95AF"
        )
        .is_err()
    );
    // A hash without its presentation prefix.
    assert!(
        Digest32::parse_prefixed(
            "7e0694f564afa2d047db4eb58f4f2b3d322d71db808f6bbf5313ee2d2a4a95af"
        )
        .is_err()
    );
}

/// The canonical serializer is total over the values [`Json`] can hold, and the
/// escape forms it emits are the ones RFC 8785 section 3.2.2.2 prescribes.
#[test]
fn string_escaping_uses_the_short_forms_and_leaves_everything_else_literal() {
    let object = JsonObject::builder()
        .value(
            "text",
            Json::str("\"\\\u{8}\t\n\u{c}\r\u{1}\u{1f} \u{e9}\u{1f600}"),
        )
        .build()
        .unwrap();
    assert_eq!(
        object.to_jcs_string(),
        "{\"text\":\"\\\"\\\\\\b\\t\\n\\f\\r\\u0001\\u001f \u{e9}\u{1f600}\"}"
    );
    // And the result is itself canonical.
    let bytes = object.to_jcs();
    assert_eq!(JsonObject::parse_canonical(&bytes).unwrap(), object);
}
