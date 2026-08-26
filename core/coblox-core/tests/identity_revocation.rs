//! Tests for `RevokeIdentityBody` and reason-dependent revocation delay validation
//! per ADR-017 / SPEC-022.

mod common;

use coblox_core::error::{Error, RevocationError};
use coblox_core::identity::{RevocationReason, RevokeIdentityBody};
use coblox_core::json::JsonObject;
use coblox_core::params::ConsensusParameters;

use common::consensus_parameters_pd0;

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

#[test]
fn effective_height_evaluated_at_inclusion_height_against_active_parameters() {
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
