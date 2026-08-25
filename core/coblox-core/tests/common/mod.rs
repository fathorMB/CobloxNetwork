//! Conformance fixtures, built exactly as `docs/protocol/` defines them.
//!
//! Every fixture here is a transcription of a definition in the specification.
//! No expected hash is computed in this file: expected values are literal
//! constants at their assertion sites, quoted from the document that publishes
//! them. A fixture that derived its own expectation from the implementation
//! would pass unconditionally and prove nothing.
//!
//! `docs/protocol/README.md#hash-conformance-fixtures` also places an
//! obligation on this file directly: "Suites MUST therefore validate their own
//! parameter fixtures against the constraint block before using them, and a
//! case that fails validation is removed rather than adjusted."
//! [`consensus_parameters_of`] and [`worked_example_parameters`] are the two
//! places where that applies, and both go through
//! `ConsensusParameters::validate`.

#![allow(dead_code)]

use coblox_core::hash::{ChainId, Digest32};
use coblox_core::json::{Json, JsonObject};
use coblox_core::params::{ConsensusParameters, ElectionBounds, ValidatedConsensusParameters};

/// "Fixture `HASH-0` uses 32 zero bytes for `chain_id`."
///
/// Every fixture in the registry that carries a chain binding uses this value.
#[must_use]
pub fn zero_chain_id() -> ChainId {
    ChainId::from_digest(Digest32::repeated(0x00))
}

/// The identity fixture public key of `identity.md#node-identifier`,
/// `L_o1qZ06PPuxe7fB3FVhsYqNzKTfONxhPqhZw36xM2s`, decoded to 32 bytes.
#[must_use]
pub fn identity_fixture_public_key() -> [u8; 32] {
    coblox_core::encoding::base64url_decode_fixed::<32>(
        IDENTITY_FIXTURE_PUBLIC_KEY,
        "identity fixture public key",
    )
    .expect("the identity fixture key is 32 unpadded base64url bytes")
}

/// The base64url spelling of the identity fixture public key.
pub const IDENTITY_FIXTURE_PUBLIC_KEY: &str = "L_o1qZ06PPuxe7fB3FVhsYqNzKTfONxhPqhZw36xM2s";

/// The signed-object Peer ID of the same key.
pub const IDENTITY_FIXTURE_PEER_ID: &str = "12D3KooWD3eckifWpRn9wQpMG9R9hX3sD158z7EqHWmweQAJU5SA";

/// The node identifier `identity.md` shows for that key.
pub const IDENTITY_FIXTURE_NODE_ID: &str =
    "cblx176fmuouuc5v2xyqqxgef5uwrdqt53yqazdlxwcfl6a63bxarnuyq";

/// A 64-byte all-zero signature in unpadded base64url.
#[must_use]
pub fn zero_signature_base64url() -> String {
    coblox_core::encoding::base64url_encode(&[0u8; 64])
}

/// Fixture `ER-0`: the exact enrollment-request schema, with all timestamps and
/// recent height set to `"1"`, nonce `"0"`, network `"fixture"`, the identity
/// fixture Peer ID / public key, algorithm `argon2id-leading-zero-bits-v0`,
/// `difficulty_bits:"4"`, the RFC 9106 second recommended cost profile,
/// parameter hash `11` repeated 32 bytes, recent block hash `22` repeated 32
/// bytes, and a 64-zero-byte base64url signature.
#[must_use]
pub fn enrollment_request_er0() -> JsonObject {
    let pow = JsonObject::builder()
        .str("algorithm", "argon2id-leading-zero-bits-v0")
        .uint("difficulty_bits", 4)
        .uint("iterations", 3)
        .uint("lanes", 4)
        .uint("memory_kib", 65_536)
        .uint("nonce", 0)
        .digest("parameter_set_hash", &Digest32::repeated(0x11))
        .uint("recent_block_height", 1)
        .digest("recent_block_id", &Digest32::repeated(0x22))
        .build()
        .expect("ER-0 pow object");
    JsonObject::builder()
        .str("schema_version", "0.1")
        .str("network_id", "fixture")
        .str("node_id", IDENTITY_FIXTURE_NODE_ID)
        .str("libp2p_peer_id", IDENTITY_FIXTURE_PEER_ID)
        .str("public_key", IDENTITY_FIXTURE_PUBLIC_KEY)
        .object("pow", pow)
        .uint("created_at_ms", 1)
        .str("signature", &zero_signature_base64url())
        .build()
        .expect("ER-0")
}

/// The four `PD-0` document kinds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Pd0Kind {
    Enrollment,
    Reward,
    Hosting,
    Consensus,
}

/// Fixture `PD-0`, for one document kind.
///
/// Common fields: `schema_version:"0.1"`, `network_id:"fixture"`, zero
/// `chain_id`, `sequence:"1"`, `activation_height:"1"`. Every numeric body
/// value is `"1"` except the exceptions the fixture definition enumerates.
#[must_use]
pub fn protocol_document_pd0(kind: Pd0Kind) -> JsonObject {
    let (document_kind, body) = match kind {
        Pd0Kind::Enrollment => ("enrollment_parameters", enrollment_body_pd0()),
        Pd0Kind::Reward => ("reward_policy", reward_body_pd0()),
        Pd0Kind::Hosting => ("hosting_rate_card", hosting_body_pd0()),
        Pd0Kind::Consensus => ("consensus_parameters", consensus_body_pd0()),
    };
    JsonObject::builder()
        .str("schema_version", "0.1")
        .str("document_kind", document_kind)
        .str("network_id", "fixture")
        .digest("chain_id", &Digest32::repeated(0x00))
        .uint("sequence", 1)
        .uint("activation_height", 1)
        .object("body", body)
        .build()
        .expect("PD-0")
}

fn enrollment_body_pd0() -> JsonObject {
    JsonObject::builder()
        .str("pow_algorithm", "argon2id-leading-zero-bits-v0")
        .uint("difficulty_bits", 4)
        .uint("memory_kib", 65_536)
        .uint("iterations", 3)
        .uint("lanes", 4)
        .uint("tag_length_bytes", 32)
        .uint("max_request_age_ms", 1)
        .uint("max_future_skew_ms", 1)
        .uint("recent_block_window", 1)
        .build()
        .expect("enrollment PD-0 body")
}

fn reward_body_pd0() -> JsonObject {
    JsonObject::builder()
        .uint("reward_epoch_ms", 1)
        .uint("existence_fund_microtokens_per_epoch", 1)
        .uint("availability_microtokens_per_unit", 1)
        .uint("storage_microtokens_per_byte_epoch", 1)
        .uint("compute_microtokens_per_million_fuel", 1)
        .uint("publisher_microtokens_per_active_subscriber", 1)
        .uint("publisher_reward_cap_numerator", 1)
        // "the cap must be strictly below one, so `"1"`/`"1"` would not be a
        // structurally valid fixture"
        .uint("publisher_reward_cap_denominator", 2)
        .uint("storage_units_per_contribution_unit", 1)
        .uint("compute_units_per_contribution_unit", 1)
        .uint("validator_eligibility_threshold_units", 1)
        .uint("validator_eligibility_window_epochs", 1)
        // "a single-issuer score does not qualify, so `"1"` would not be
        // structurally valid either"
        .uint("validator_eligibility_min_issuers", 2)
        .build()
        .expect("reward PD-0 body")
}

fn hosting_body_pd0() -> JsonObject {
    JsonObject::builder()
        .uint("billing_epoch_ms", 1)
        .uint("minimum_billable_epochs", 1)
        .uint("microtokens_per_replica_epoch", 1)
        .uint("microtokens_per_gib_epoch", 1)
        .uint("microtokens_per_million_fuel", 1)
        .build()
        .expect("hosting PD-0 body")
}

/// The `PD-0` consensus parameter values, as the fixture definition fixes them.
///
/// `V:"12"`, `T:"4"`, `c:"3"`, `m:"1"`. The definition is explicit that these
/// are not free choices and that `T:"3"` or `V:"3"` would encode a state no
/// conformant network can reach.
#[must_use]
pub fn consensus_parameters_pd0() -> ConsensusParameters {
    ConsensusParameters {
        max_clock_drift_ms: 1,
        max_envelope_validity_ms: 1,
        replay_cache_entries_per_peer: 1,
        replay_cache_entries_global: 1,
        max_weak_subjectivity_age_ms: 1,
        max_current_balance_age_ms: 1,
        app_suspension_notice_epochs: 1,
        min_revocation_effective_delay_blocks: 1,
        election_epoch_blocks: 4,
        candidacy_close_blocks: 3,
        election_entropy_blocks: 2,
        validator_min_set_size: 1,
        validator_target_set_size: 12,
        validator_max_set_size: 12,
        validator_churn_cap_seats: 3,
        validator_max_consecutive_terms: 4,
        validator_cooldown_epochs: 1,
        validator_min_capture_epochs: 1,
    }
}

fn consensus_body_pd0() -> JsonObject {
    consensus_body(&consensus_parameters_pd0())
}

/// Serializes a `ConsensusParametersBody` from typed parameters.
#[must_use]
pub fn consensus_body(parameters: &ConsensusParameters) -> JsonObject {
    JsonObject::builder()
        .uint("max_clock_drift_ms", parameters.max_clock_drift_ms)
        .uint(
            "max_envelope_validity_ms",
            parameters.max_envelope_validity_ms,
        )
        .uint(
            "replay_cache_entries_per_peer",
            parameters.replay_cache_entries_per_peer,
        )
        .uint(
            "replay_cache_entries_global",
            parameters.replay_cache_entries_global,
        )
        .uint(
            "max_weak_subjectivity_age_ms",
            parameters.max_weak_subjectivity_age_ms,
        )
        .uint(
            "max_current_balance_age_ms",
            parameters.max_current_balance_age_ms,
        )
        .uint(
            "app_suspension_notice_epochs",
            parameters.app_suspension_notice_epochs,
        )
        .uint(
            "min_revocation_effective_delay_blocks",
            parameters.min_revocation_effective_delay_blocks,
        )
        .uint("election_epoch_blocks", parameters.election_epoch_blocks)
        .uint("candidacy_close_blocks", parameters.candidacy_close_blocks)
        .uint(
            "election_entropy_blocks",
            parameters.election_entropy_blocks,
        )
        .uint("validator_min_set_size", parameters.validator_min_set_size)
        .uint(
            "validator_target_set_size",
            parameters.validator_target_set_size,
        )
        .uint("validator_max_set_size", parameters.validator_max_set_size)
        .uint(
            "validator_churn_cap_seats",
            parameters.validator_churn_cap_seats,
        )
        .uint(
            "validator_max_consecutive_terms",
            parameters.validator_max_consecutive_terms,
        )
        .uint(
            "validator_cooldown_epochs",
            parameters.validator_cooldown_epochs,
        )
        .uint(
            "validator_min_capture_epochs",
            parameters.validator_min_capture_epochs,
        )
        .build()
        .expect("consensus parameters body")
}

/// Test-local `ElectionBounds`, wide enough not to constrain a fixture.
///
/// The specification is explicit that these values "are a genesis decision of
/// the network operator rather than a simulator output, and are deliberately
/// not fixed in this document". They are therefore chosen here to exercise the
/// *mechanism*, and no test asserts any particular magnitude.
#[must_use]
pub fn permissive_bounds() -> ElectionBounds {
    ElectionBounds {
        network_id: "fixture".to_owned(),
        chain_id: zero_chain_id(),
        election_epoch_blocks_max: 1_000_000,
        validator_max_consecutive_terms_max: 1_000,
        validator_max_set_size_max: 1_000,
        validator_min_set_size_min: 1,
        validator_min_capture_epochs_min: 1,
        election_parameter_change_numerator: 3,
        election_parameter_change_denominator: 2,
        election_parameter_min_activation_gap_blocks: 100,
    }
}

/// Validates a parameter fixture against the constraint block before it is
/// used, as the registry requires of every conformance suite.
///
/// # Panics
///
/// Panics when the fixture is inadmissible. That is the intended failure mode:
/// "a case that fails validation is removed rather than adjusted", so a suite
/// that would otherwise assert behaviour for an unreachable state stops here.
#[must_use]
pub fn consensus_parameters_of(parameters: &ConsensusParameters) -> ValidatedConsensusParameters {
    parameters
        .validate(&permissive_bounds(), 1, 1, None)
        .expect("the parameter fixture must satisfy the election constraint block")
}

/// The example parameters of `ledger.md#worked-example-of-the-derivation`.
///
/// "The example is normative in form and not in values: every parameter below
/// is instantiated only to make the derivation reproducible, and none of these
/// numbers is a proposal." They are reproduced here for that reason and are not
/// promoted to constants anywhere in the crate.
///
/// `validator_max_set_size` is the one value the example does not state. It is
/// set equal to `validator_target_set_size`, the smallest value that admits the
/// example's own set, and it plays no part in any assertion.
#[must_use]
pub fn worked_example_parameters() -> ValidatedConsensusParameters {
    let parameters = ConsensusParameters {
        max_clock_drift_ms: 1,
        max_envelope_validity_ms: 1,
        replay_cache_entries_per_peer: 1,
        replay_cache_entries_global: 1,
        max_weak_subjectivity_age_ms: 1,
        max_current_balance_age_ms: 1,
        app_suspension_notice_epochs: 1,
        min_revocation_effective_delay_blocks: 1,
        election_epoch_blocks: 100,
        candidacy_close_blocks: 10,
        election_entropy_blocks: 3,
        validator_min_set_size: 3,
        validator_target_set_size: 8,
        validator_max_set_size: 8,
        validator_churn_cap_seats: 2,
        validator_max_consecutive_terms: 4,
        validator_cooldown_epochs: 1,
        validator_min_capture_epochs: 1,
    };
    consensus_parameters_of(&parameters)
}

/// Fixture `WSC-0`: the unsigned weak subjectivity checkpoint.
#[must_use]
pub fn weak_subjectivity_checkpoint_wsc0() -> JsonObject {
    JsonObject::builder()
        .str("schema_version", "0.1")
        .str("network_id", "fixture")
        .digest("chain_id", &Digest32::repeated(0x00))
        .uint("height", 1)
        .digest("block_id", &Digest32::repeated(0x66))
        .uint("timestamp_ms", 1)
        .uint("issued_at_ms", 1)
        .digest("validator_set_hash", &Digest32::repeated(0x77))
        .uint("max_weak_subjectivity_age_ms", 1)
        .array("revoked_validators", Vec::new())
        .digest(
            "revocation_root",
            &coblox_core::merkle::revocation_root(&[]).expect("empty revocation root"),
        )
        .build()
        .expect("WSC-0")
}

/// Fixture `REQ-0`: an availability challenge request without `challenge_id`
/// and without `issuer_signature`.
#[must_use]
pub fn challenge_request_req0(
    randomness_rnd0: &Digest32,
    issuer_commitment_cmt0: &Digest32,
) -> JsonObject {
    let randomness_source = JsonObject::builder()
        .digest("beacon_block_id", &Digest32::repeated(0x55))
        .uint("beacon_height", 1)
        .uint("commitment_epoch", 1)
        .build()
        .expect("REQ-0 randomness source");
    let assignment = JsonObject::builder()
        .uint("response_bytes", 1)
        .build()
        .expect("REQ-0 assignment");
    JsonObject::builder()
        .str("kind", "availability")
        .str("issuer_node_id", "cblx1issuerfixture")
        .str("subject_node_id", "cblx1fixture")
        .uint("issued_at_ms", 1)
        .uint("deadline_ms", 2)
        .str(
            "randomness",
            &coblox_core::encoding::base64url_encode(randomness_rnd0.as_bytes()),
        )
        .object("randomness_source", randomness_source)
        .digest("issuer_commitment", issuer_commitment_cmt0)
        .object("assignment", assignment)
        .build()
        .expect("REQ-0")
}

/// Fixture `RESP-0`: an unsigned availability challenge response.
#[must_use]
pub fn challenge_response_resp0() -> JsonObject {
    let result = JsonObject::builder()
        .str("kind", "availability")
        .value("response", Json::bytes(&[0x00]))
        .build()
        .expect("RESP-0 result");
    JsonObject::builder()
        .digest("challenge_id", &Digest32::repeated(0x33))
        .str("subject_node_id", "cblx1fixture")
        .uint("completed_at_ms", 2)
        .object("result", result)
        .build()
        .expect("RESP-0")
}
