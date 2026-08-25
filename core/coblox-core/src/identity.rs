//! Node identity, transport keys, and transport key attestations.
//!
//! "La chiave di trasporto libp2p e distinta dalla chiave di identita Coblox,
//! subordinata a essa, ruotabile, e il suo legame non e pubblicato sul ledger"
//! (ADR-015, `docs/protocol/identity.md`).

use crate::SignatureVerifier;
use crate::encoding::{base64url_decode_fixed, base64url_encode};
use crate::error::{AttestationError, Result};
use crate::hash::{ChainId, Domain, NodeId};
use crate::json::JsonObject;
use crate::params::ConsensusParameters;
use crate::registry::signing_preimage;

/// The two signed network parameters that bound an attestation in time.
///
/// Both are read from the active `consensus_parameters` document
/// (`crate::params::ConsensusParameters`) and never from local policy: a bound
/// each operator picks is a preference, not a property.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AttestationBounds {
    /// `max_transport_attestation_validity_ms`.
    pub max_validity_ms: u64,
    /// `max_transport_attestation_future_skew_ms`.
    pub max_future_skew_ms: u64,
}

impl AttestationBounds {
    /// Reads both bounds from a set of consensus parameters.
    #[must_use]
    pub const fn from_consensus_parameters(parameters: &ConsensusParameters) -> Self {
        Self {
            max_validity_ms: parameters.max_transport_attestation_validity_ms,
            max_future_skew_ms: parameters.max_transport_attestation_future_skew_ms,
        }
    }
}

/// A signed transport key attestation presented in-session.
///
/// An enrolled identity authorizes an ephemeral/rotatable transport key by signing
/// this object. The receiver verifies the attestation against the peer's
/// finalized enrollment certificate and verifies that the transport key matches
/// the authenticated libp2p connection (Noise / QUIC).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TransportKeyAttestation {
    pub schema_version: String,
    pub network_id: String,
    pub node_id: NodeId,
    pub transport_public_key: [u8; 32],
    pub created_at_ms: u64,
    pub expires_at_ms: u64,
    pub signature: [u8; 64],
}

impl TransportKeyAttestation {
    /// Builds a new attestation from its fields.
    pub fn new(
        network_id: String,
        node_id: NodeId,
        transport_public_key: [u8; 32],
        created_at_ms: u64,
        expires_at_ms: u64,
        signature: [u8; 64],
    ) -> Self {
        Self {
            schema_version: "0.1".to_owned(),
            network_id,
            node_id,
            transport_public_key,
            created_at_ms,
            expires_at_ms,
            signature,
        }
    }

    /// Serializes this attestation into a [`JsonObject`].
    pub fn to_json(&self) -> Result<JsonObject> {
        JsonObject::builder()
            .uint("created_at_ms", self.created_at_ms)
            .uint("expires_at_ms", self.expires_at_ms)
            .str("network_id", &self.network_id)
            .str("node_id", self.node_id.as_str())
            .str("schema_version", &self.schema_version)
            .str("signature", &base64url_encode(&self.signature))
            .str(
                "transport_public_key",
                &base64url_encode(&self.transport_public_key),
            )
            .build()
    }

    /// Serializes the unsigned object (without `signature`) for signing.
    pub fn to_unsigned_json(&self) -> Result<JsonObject> {
        JsonObject::builder()
            .uint("created_at_ms", self.created_at_ms)
            .uint("expires_at_ms", self.expires_at_ms)
            .str("network_id", &self.network_id)
            .str("node_id", self.node_id.as_str())
            .str("schema_version", &self.schema_version)
            .str(
                "transport_public_key",
                &base64url_encode(&self.transport_public_key),
            )
            .build()
    }

    /// Parses a [`JsonObject`] into a [`TransportKeyAttestation`].
    pub fn from_json(object: &JsonObject) -> Result<Self> {
        let schema_version = object.string("schema_version")?.to_owned();
        if schema_version != "0.1" {
            return Err(AttestationError::UnsupportedVersion(schema_version).into());
        }
        let network_id = object.string("network_id")?.to_owned();
        let node_id = NodeId::from_string(object.string("node_id")?.to_owned());
        let transport_public_key = base64url_decode_fixed::<32>(
            object.string("transport_public_key")?,
            "transport_public_key",
        )?;
        let created_at_ms = object.uint("created_at_ms")?;
        let expires_at_ms = object.uint("expires_at_ms")?;
        let signature = base64url_decode_fixed::<64>(object.string("signature")?, "signature")?;

        Ok(Self {
            schema_version,
            network_id,
            node_id,
            transport_public_key,
            created_at_ms,
            expires_at_ms,
            signature,
        })
    }

    /// Verifies the attestation against the enrolled identity key, expected
    /// network, time window, and the transport key authenticated on the
    /// connection.
    ///
    /// The rejection conditions are those of
    /// `identity.md#mandatory-rejection-rules`, in that order. Two of them are
    /// worth naming here because they are the ones an implementation is likely
    /// to omit and no test would notice: the attested transport key MUST NOT be
    /// the enrolled identity key, and the window MUST be no longer than
    /// `bounds.max_validity_ms`.
    #[allow(clippy::too_many_arguments)]
    pub fn verify(
        &self,
        chain_id: &ChainId,
        expected_network_id: &str,
        enrolled_identity_public_key: &[u8; 32],
        authenticated_transport_key: &[u8; 32],
        now_ms: u64,
        bounds: &AttestationBounds,
        verifier: &impl SignatureVerifier,
    ) -> Result<()> {
        if self.schema_version != "0.1" {
            return Err(AttestationError::UnsupportedVersion(self.schema_version.clone()).into());
        }
        if self.network_id != expected_network_id {
            return Err(AttestationError::NetworkIdMismatch {
                expected: expected_network_id.to_owned(),
                actual: self.network_id.clone(),
            }
            .into());
        }
        let derived_node_id = NodeId::derive(enrolled_identity_public_key);
        if self.node_id != derived_node_id {
            return Err(AttestationError::NodeIdMismatch {
                expected: derived_node_id.as_str().to_owned(),
                actual: self.node_id.as_str().to_owned(),
            }
            .into());
        }
        if self.transport_public_key == *enrolled_identity_public_key {
            return Err(AttestationError::TransportKeyEqualsIdentityKey.into());
        }
        if self.transport_public_key != *authenticated_transport_key {
            return Err(AttestationError::TransportKeyMismatch.into());
        }
        if self.created_at_ms > self.expires_at_ms {
            return Err(AttestationError::InvalidValidityWindow {
                created_at_ms: self.created_at_ms,
                expires_at_ms: self.expires_at_ms,
            }
            .into());
        }
        // `created_at_ms <= expires_at_ms` is established above, so `None` is
        // unreachable. It is written as a rejection rather than as a subtraction
        // so that removing the check above turns this into a rejection too,
        // instead of a debug panic and a wrapped duration in release.
        let Some(duration_ms) = self.expires_at_ms.checked_sub(self.created_at_ms) else {
            return Err(AttestationError::InvalidValidityWindow {
                created_at_ms: self.created_at_ms,
                expires_at_ms: self.expires_at_ms,
            }
            .into());
        };
        if duration_ms > bounds.max_validity_ms {
            return Err(AttestationError::ValidityWindowTooLong {
                duration_ms,
                maximum_ms: bounds.max_validity_ms,
            }
            .into());
        }
        // The tolerance is granted in one direction only. A receiver whose
        // clock is slightly behind must still accept a freshly issued
        // attestation, or it loses `ledger-sync` and with it the only source
        // from which it could correct its clock. Past `expires_at_ms` there is
        // no slack, because slack there extends the exposure window that
        // `max_validity_ms` exists to bound.
        let latest_acceptable_creation = now_ms.saturating_add(bounds.max_future_skew_ms);
        if now_ms > self.expires_at_ms || self.created_at_ms > latest_acceptable_creation {
            return Err(AttestationError::Expired {
                now_ms,
                created_at_ms: self.created_at_ms,
                expires_at_ms: self.expires_at_ms,
            }
            .into());
        }

        let unsigned = self.to_unsigned_json()?;
        let preimage = signing_preimage(
            Domain::SIG_TRANSPORT_KEY_ATTESTATION,
            chain_id,
            &unsigned.to_jcs(),
        );
        if !verifier.verify(enrolled_identity_public_key, &preimage, &self.signature) {
            return Err(AttestationError::InvalidSignature.into());
        }

        Ok(())
    }
}
