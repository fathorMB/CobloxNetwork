//! Tests for `RevokeIdentityBody` and reason-dependent revocation delay validation
//! per ADR-017 / SPEC-022.

mod common;

use coblox_core::block::BlockHeader;
use coblox_core::error::{Error, ParameterError, RevocationError};
use coblox_core::hash::Digest32;
use coblox_core::identity::{RevocationReason, RevokeIdentityBody};
use coblox_core::json::JsonObject;
use coblox_core::params::ConsensusParameters;
use coblox_core::registry::{self, DocumentKind};

use common::{consensus_body, consensus_parameters_pd0, permissive_bounds, zero_chain_id};

fn custom_params(f: u64, g: u64, p: u64) -> ConsensusParameters {
    ConsensusParameters {
        min_revocation_effective_delay_blocks: f,
        revocation_effective_grace_blocks: g,
        max_planned_revocation_delay_blocks: p,
        ..consensus_parameters_pd0()
    }
}

#[test]
fn revocation_reason_parsing_and_display() {
    assert_eq!(
        RevocationReason::parse("key_compromise").unwrap(),
        RevocationReason::KeyCompromise
    );
    assert_eq!(
        RevocationReason::parse("validator_misconduct").unwrap(),
        RevocationReason::ValidatorMisconduct
    );
    assert_eq!(
        RevocationReason::parse("operator_request").unwrap(),
        RevocationReason::OperatorRequest
    );
    assert_eq!(
        RevocationReason::parse("unknown_reason").unwrap_err(),
        Error::Revocation(RevocationError::UnknownReason("unknown_reason".to_string()))
    );

    assert_eq!(RevocationReason::KeyCompromise.as_str(), "key_compromise");
    assert_eq!(
        RevocationReason::ValidatorMisconduct.as_str(),
        "validator_misconduct"
    );
    assert_eq!(
        RevocationReason::OperatorRequest.as_str(),
        "operator_request"
    );
}

#[test]
fn revoke_identity_body_json_roundtrip_and_validation() {
    let body = RevokeIdentityBody {
        node_id: "cblx1fixturenode".to_string(),
        reason: RevocationReason::KeyCompromise,
        effective_height: 105,
        replacement_node_id: Some("cblx1replacement".to_string()),
    };
    let json = body.to_json().unwrap();
    let parsed = RevokeIdentityBody::from_json(&json).expect("valid json object");
    assert_eq!(parsed, body);

    // Reject unknown fields
    let invalid_json = JsonObject::builder()
        .str("node_id", "cblx1fixturenode")
        .str("reason", "key_compromise")
        .uint("effective_height", 105)
        .str("extra_field", "unexpected")
        .build()
        .unwrap();
    assert!(RevokeIdentityBody::from_json(&invalid_json).is_err());
}

#[test]
fn key_compromise_effective_height_band() {
    // Parameters: F = 10, G = 5, P = 20
    let params = custom_params(10, 5, 20);
    let p = 100u64; // Proposing/including height

    // Floor: p + F = 110
    // Ceiling: p + F + G = 115

    let make_body = |effective_height| RevokeIdentityBody {
        node_id: "cblx1testnode".to_string(),
        reason: RevocationReason::KeyCompromise,
        effective_height,
        replacement_node_id: None,
    };

    // Below floor (109 < 110)
    assert_eq!(
        make_body(109)
            .validate_effective_height(p, &params)
            .unwrap_err(),
        Error::Revocation(RevocationError::EffectiveHeightBelowFloor {
            including_height: p,
            effective_height: 109,
            floor: 110,
        })
    );

    // Exact floor (110)
    assert!(make_body(110).validate_effective_height(p, &params).is_ok());

    // Interior (112)
    assert!(make_body(112).validate_effective_height(p, &params).is_ok());

    // Exact ceiling (115)
    assert!(make_body(115).validate_effective_height(p, &params).is_ok());

    // Above ceiling (116 > 115)
    assert_eq!(
        make_body(116)
            .validate_effective_height(p, &params)
            .unwrap_err(),
        Error::Revocation(RevocationError::EffectiveHeightAboveCeiling {
            including_height: p,
            effective_height: 116,
            ceiling: 115,
        })
    );
}

#[test]
fn validator_misconduct_and_operator_request_effective_height_band() {
    // Parameters: F = 10, G = 5, P = 20
    let params = custom_params(10, 5, 20);
    let p = 100u64;

    // Floor: p + F = 110
    // Ceiling: p + P = 120

    for reason in [
        RevocationReason::ValidatorMisconduct,
        RevocationReason::OperatorRequest,
    ] {
        let make_body = |effective_height| RevokeIdentityBody {
            node_id: "cblx1testnode".to_string(),
            reason,
            effective_height,
            replacement_node_id: None,
        };

        // Below floor (109 < 110)
        assert_eq!(
            make_body(109)
                .validate_effective_height(p, &params)
                .unwrap_err(),
            Error::Revocation(RevocationError::EffectiveHeightBelowFloor {
                including_height: p,
                effective_height: 109,
                floor: 110,
            })
        );

        // Exact floor (110)
        assert!(make_body(110).validate_effective_height(p, &params).is_ok());

        // Value in key compromise forbidden band (118) is valid for misconduct/operator_request
        assert!(make_body(118).validate_effective_height(p, &params).is_ok());

        // Exact ceiling (120)
        assert!(make_body(120).validate_effective_height(p, &params).is_ok());

        // Above ceiling (121 > 120)
        assert_eq!(
            make_body(121)
                .validate_effective_height(p, &params)
                .unwrap_err(),
            Error::Revocation(RevocationError::EffectiveHeightAboveCeiling {
                including_height: p,
                effective_height: 121,
                ceiling: 120,
            })
        );
    }
}

/// The band is read from the `params` argument and from nothing else.
///
/// **Renamed on [REVIEW-042] RF-004, which is the honest half of that finding.**
/// The name it carried — *evaluated at inclusion height against active
/// parameters* — claimed clause 3 of [ADR-017]. It never proved it: both calls
/// pass the *same* height and differ only in the parameters, so what it
/// establishes is that the function reads its own argument, not which argument a
/// verifier must pass. That second half is clause 3, and it is now enforced and
/// exercised by `validate_effective_height_in_block` below.
#[test]
fn the_band_is_read_from_the_parameters_argument() {
    // Parameter epoch 1: F = 5, G = 2, P = 10
    let params_epoch1 = custom_params(5, 2, 10);
    // Parameter epoch 2: F = 20, G = 10, P = 50
    let params_epoch2 = custom_params(20, 10, 50);

    let body = RevokeIdentityBody {
        node_id: "cblx1testnode".to_string(),
        reason: RevocationReason::KeyCompromise,
        effective_height: 106,
        replacement_node_id: None,
    };

    // At inclusion height p = 100 under epoch 1:
    // Floor: 100 + 5 = 105, Ceiling: 100 + 5 + 2 = 107. 106 is valid.
    assert!(body.validate_effective_height(100, &params_epoch1).is_ok());

    // Under epoch 2 parameters at inclusion height p = 100:
    // Floor: 100 + 20 = 120. 106 is below floor.
    assert_eq!(
        body.validate_effective_height(100, &params_epoch2)
            .unwrap_err(),
        Error::Revocation(RevocationError::EffectiveHeightBelowFloor {
            including_height: 100,
            effective_height: 106,
            floor: 120,
        })
    );
}

// --- Clause 3 of [ADR-017]: which version of the parameters governs ---------
//
// [REVIEW-042] RF-004: the clause was documented and not implemented.
// `validate_effective_height` takes the height and the parameters as two free
// arguments, so it states the arithmetic and says nothing about which pair a
// verifier must use — and it had no caller outside the tests. The clause is a
// fact about the **including block**: `BlockHeader` carries
// `consensus_parameters_hash`, so "the parameters in force at `p`" is readable
// from the same header that carries `p`.

/// Wraps a parameter set into a `consensus_parameters` protocol document of the
/// `PD-0` shape, so that it has a hash a header can commit to.
fn consensus_document(parameters: &ConsensusParameters) -> JsonObject {
    JsonObject::builder()
        .str("schema_version", "0.1")
        .str("document_kind", "consensus_parameters")
        .str("network_id", "fixture")
        .digest("chain_id", &Digest32::repeated(0x00))
        .uint("sequence", 1)
        .uint("activation_height", 1)
        .object("body", consensus_body(parameters))
        .build()
        .expect("consensus_parameters document")
}

/// A header at `height` committing to `document` through
/// `consensus_parameters_hash`.
fn including_header(height: u64, document: &JsonObject) -> BlockHeader {
    let hash = registry::protocol_document_hash(
        DocumentKind::ConsensusParameters,
        &zero_chain_id(),
        document,
    )
    .expect("the document hashes");
    BlockHeader {
        schema_version: "0.1".to_owned(),
        protocol_version: "0.1".to_owned(),
        network_id: "fixture".to_owned(),
        height,
        round: 0,
        timestamp_ms: 1,
        previous_block_id: Digest32::repeated(0x11),
        transactions_root: Digest32::repeated(0x22),
        state_root: Digest32::repeated(0x33),
        validator_set_hash: Digest32::repeated(0x44),
        next_validator_set_hash: Digest32::repeated(0x44),
        consensus_parameters_hash: hash,
    }
}

/// The verdict is decided by the parameter document the **including block**
/// commits to, at the height that block carries.
///
/// This is the half [REVIEW-042] RF-004 found missing. The two calls below use
/// the same body and the same inclusion height and differ only in which
/// parameter epoch the header commits to: under the first the revocation is
/// inside the `key_compromise` band, under the second it is below the floor.
/// Neither height nor parameters are the caller's to choose.
#[test]
fn clause_three_selects_the_parameters_the_including_header_commits_to() {
    let chain = zero_chain_id();
    let bounds = permissive_bounds();

    // Epoch 1: F = 5, G = 2 — floor 105, ceiling 107 at p = 100.
    let epoch1 = custom_params(5, 2, 10);
    // Epoch 2: F = 20, G = 10 — floor 120 at p = 100.
    let epoch2 = custom_params(20, 10, 50);
    let doc1 = consensus_document(&epoch1);
    let doc2 = consensus_document(&epoch2);

    let body = RevokeIdentityBody {
        node_id: "cblx1testnode".to_string(),
        reason: RevocationReason::KeyCompromise,
        effective_height: 106,
        replacement_node_id: None,
    };

    let header1 = including_header(100, &doc1);
    body.validate_effective_height_in_block(&chain, &header1, &doc1, &bounds)
        .expect("106 is inside the band the including header's parameters define");

    let header2 = including_header(100, &doc2);
    assert_eq!(
        body.validate_effective_height_in_block(&chain, &header2, &doc2, &bounds)
            .unwrap_err(),
        Error::Revocation(RevocationError::EffectiveHeightBelowFloor {
            including_height: 100,
            effective_height: 106,
            floor: 120,
        })
    );

    // The height comes from the same header as the parameters, so the two
    // cannot be paired by hand: at p = 115 under epoch 1 the floor is 120 and
    // the same body is refused.
    let later = including_header(115, &doc1);
    assert_eq!(
        body.validate_effective_height_in_block(&chain, &later, &doc1, &bounds)
            .unwrap_err(),
        Error::Revocation(RevocationError::EffectiveHeightBelowFloor {
            including_height: 115,
            effective_height: 106,
            floor: 120,
        })
    );
}

/// A parameter document the including header does not commit to is refused
/// before any band is computed.
///
/// This is what makes the selector a rule rather than a convenience: a verifier
/// cannot reach a favourable verdict by supplying an authentic document from
/// another parameter epoch.
#[test]
fn clause_three_refuses_a_parameter_document_the_header_does_not_commit_to() {
    let chain = zero_chain_id();
    let bounds = permissive_bounds();
    let doc1 = consensus_document(&custom_params(5, 2, 10));
    let doc2 = consensus_document(&custom_params(20, 10, 50));

    let body = RevokeIdentityBody {
        node_id: "cblx1testnode".to_string(),
        reason: RevocationReason::KeyCompromise,
        effective_height: 106,
        replacement_node_id: None,
    };

    // The header commits to epoch 2, which refuses this body; substituting the
    // epoch-1 document does not buy the permissive verdict.
    let header2 = including_header(100, &doc2);
    assert_eq!(
        body.validate_effective_height_in_block(&chain, &header2, &doc1, &bounds)
            .unwrap_err(),
        Error::Parameter(ParameterError::Constraint {
            rule: "consensus_parameters_hash MUST equal the hash of the trusted header",
        })
    );
}
