//! Node identity, transport keys, and transport key attestations.
//!
//! "La chiave di trasporto libp2p e distinta dalla chiave di identita Coblox,
//! subordinata a essa, ruotabile, e il suo legame non e pubblicato sul ledger"
//! (ADR-015, `docs/protocol/identity.md`).

use crate::SignatureVerifier;
use crate::block::BlockHeader;
use crate::cadence::AttestationClock;
use crate::encoding::{base64url_decode_fixed, base64url_encode};
use crate::error::{AttestationError, Error, JsonError, Result};
use crate::hash::{ChainId, Domain, NodeId};
use crate::json::{Json, JsonObject};
use crate::params::{ConsensusParameters, ElectionBounds};
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
    ///
    /// **`clock` is an [`AttestationClock`] and not a `u64`, and it carries two
    /// numbers because rule 5 has two halves that must not read the same one.**
    /// The exposure of this object is
    /// `max_validity_ms + max_future_skew_ms + (however far the receiver's clock
    /// is behind)`, and only the first two terms are bounded by a parameter. The
    /// third is **reduced** — not closed — by a floor under the expiry
    /// comparison, taken from the one clock no validator writes.
    ///
    /// The floor is spent on the expiry half only. Raising the clock rejects
    /// more attestations as expired, but it would *admit* more postdated ones
    /// through `created_at_ms > now_ms + max_future_skew_ms`, so that half reads
    /// [`AttestationClock::local_clock_ms`] instead. A floor is fail-closed on
    /// one half of this rule and fail-open on the other; spending it only where
    /// it rejects is what makes the whole rule fail closed.
    ///
    /// A receiver that holds no checkpoint passes
    /// [`AttestationClock::local_only`], for which the two numbers are equal,
    /// and behaves exactly as it did before [SPEC-020].
    #[allow(clippy::too_many_arguments)]
    pub fn verify(
        &self,
        chain_id: &ChainId,
        expected_network_id: &str,
        enrolled_identity_public_key: &[u8; 32],
        authenticated_transport_key: &[u8; 32],
        clock: AttestationClock,
        bounds: &AttestationBounds,
        verifier: &impl SignatureVerifier,
    ) -> Result<()> {
        let now_ms = clock.now_ms();
        let local_clock_ms = clock.local_clock_ms();
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
        //
        // **The two halves read two different clocks, and that is the rule.**
        // The floored reading rejects more attestations as expired and can never
        // revive an expired one, so the expiry half spends it. The admission
        // half runs the other way: raising the clock would *admit* an
        // attestation postdated further than the signed tolerance, and with an
        // anchor ahead of real time by `Δ` the real window becomes
        // `max_validity_ms + Δ` with `Δ` chosen by whoever signs the checkpoint.
        // So the admission half reads the receiver's own clock, unfloored. A
        // floor is fail-closed on one half of this rule and fail-open on the
        // other; both halves are fail-closed only once they are split.
        //
        // For a receiver holding no checkpoint the two readings are the same
        // number and this is the comparison that existed before [SPEC-020].
        let latest_acceptable_creation = local_clock_ms.saturating_add(bounds.max_future_skew_ms);
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
        // The checked entry point rather than `verify`: this call site builds
        // the preimage two lines above and so cannot get the context wrong
        // today, and that is the reason to write it this way now. The shape a
        // reader copies is the shape the next call site will have, and the next
        // one will receive its preimage from somewhere else.
        if !crate::verifier::verify_in_context(
            verifier,
            Domain::SIG_TRANSPORT_KEY_ATTESTATION,
            chain_id,
            enrolled_identity_public_key,
            &preimage,
            &self.signature,
        ) {
            return Err(AttestationError::InvalidSignature.into());
        }

        Ok(())
    }
}

/// The three valid values for `RevokeIdentityBody.reason`.
///
/// Governed by `docs/protocol/ledger.md#identity-revocation` and [ADR-017].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RevocationReason {
    /// "key_compromise"
    KeyCompromise,
    /// "validator_misconduct"
    ValidatorMisconduct,
    /// "operator_request"
    OperatorRequest,
}

impl RevocationReason {
    /// Returns the canonical wire string representation of this reason.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::KeyCompromise => "key_compromise",
            Self::ValidatorMisconduct => "validator_misconduct",
            Self::OperatorRequest => "operator_request",
        }
    }

    /// Parses a reason string from the wire.
    ///
    /// # Errors
    ///
    /// [`crate::error::RevocationError::UnknownReason`] when the string is not one of
    /// the three enumerated reasons.
    pub fn parse(s: &str) -> Result<Self> {
        match s {
            "key_compromise" => Ok(Self::KeyCompromise),
            "validator_misconduct" => Ok(Self::ValidatorMisconduct),
            "operator_request" => Ok(Self::OperatorRequest),
            other => Err(crate::error::RevocationError::UnknownReason(other.to_owned()).into()),
        }
    }
}

/// The body of a `revoke_identity` governance transaction.
///
/// Governed by `docs/protocol/ledger.md#identity-revocation` and [ADR-017].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RevokeIdentityBody {
    /// The identity to revoke.
    pub node_id: String,
    /// The declared reason for revocation.
    pub reason: RevocationReason,
    /// The height at which this revocation takes effect for validator set transitions.
    pub effective_height: u64,
    /// Optional informational replacement node ID.
    pub replacement_node_id: Option<String>,
}

impl RevokeIdentityBody {
    /// Builds a new `RevokeIdentityBody`.
    #[must_use]
    pub fn new(
        node_id: String,
        reason: RevocationReason,
        effective_height: u64,
        replacement_node_id: Option<String>,
    ) -> Self {
        Self {
            node_id,
            reason,
            effective_height,
            replacement_node_id,
        }
    }

    /// Parses a `RevokeIdentityBody` from a [`JsonObject`].
    ///
    /// # Errors
    ///
    /// Fails when any required field is missing or invalid, or an unknown field is present.
    pub fn from_json(body: &JsonObject) -> Result<Self> {
        body.reject_unknown_fields(&[
            "node_id",
            "reason",
            "effective_height",
            "replacement_node_id",
        ])?;
        let node_id = body.string("node_id")?.to_owned();
        let reason = RevocationReason::parse(body.string("reason")?)?;
        let effective_height = body.uint("effective_height")?;
        let replacement_node_id = match body.get("replacement_node_id") {
            Some(Json::Str(text)) => Some(text.clone()),
            Some(_) => {
                return Err(Error::from(JsonError::Field(
                    "replacement_node_id".to_owned(),
                )));
            }
            None => None,
        };
        Ok(Self {
            node_id,
            reason,
            effective_height,
            replacement_node_id,
        })
    }

    /// Serializes this body into a [`JsonObject`].
    pub fn to_json(&self) -> Result<JsonObject> {
        let mut builder = JsonObject::builder()
            .uint("effective_height", self.effective_height)
            .str("node_id", &self.node_id)
            .str("reason", self.reason.as_str());
        if let Some(ref replacement) = self.replacement_node_id {
            builder = builder.str("replacement_node_id", replacement);
        }
        builder.build()
    }

    /// Validates `effective_height` against the reason-dependent band, reading
    /// **both** the height and the parameter version from the header of the
    /// block that includes the revocation.
    ///
    /// This is clause 3 of [ADR-017] — *every constraint is evaluated against
    /// the consensus parameters in force at the height of the block that
    /// includes the `revoke_identity`* — expressed as code rather than as prose.
    /// [`Self::validate_effective_height`] takes the height and the parameters
    /// as two independent arguments and therefore states only the arithmetic;
    /// which pair a verifier must pass was, until [REVIEW-042] RF-004, written
    /// nowhere an implementation could read. Here the pair cannot be
    /// mismatched: `including_header.height` is the height, and the parameters
    /// are the ones whose document hashes to that same header's
    /// `consensus_parameters_hash`.
    ///
    /// The binding is the one
    /// [`crate::light_client::authenticate_consensus_parameters`] already
    /// performs, reused rather than reimplemented, so a document that is
    /// authentic but belongs to a different parameter epoch is rejected here
    /// exactly as it is on the light-client path. As there, verifying the
    /// quorum signature over the document is the caller's step: this crate
    /// ships a consensus-domain verifier ([`crate::verify_in_context`]), not one
    /// for governed documents.
    ///
    /// # Errors
    ///
    /// Whatever [`crate::light_client::authenticate_consensus_parameters`]
    /// returns when the document is not the one the including header commits
    /// to, is not for this chain, or falls outside the genesis bounds; and
    /// otherwise whatever [`Self::validate_effective_height`] returns.
    pub fn validate_effective_height_in_block(
        &self,
        chain_id: &ChainId,
        including_header: &BlockHeader,
        unsigned_consensus_document: &JsonObject,
        bounds: &ElectionBounds,
    ) -> Result<()> {
        let parameters = crate::light_client::authenticate_consensus_parameters(
            chain_id,
            including_header,
            unsigned_consensus_document,
            bounds,
        )?;
        self.validate_effective_height(including_header.height, parameters.get())
    }

    /// Validates `effective_height` against the reason-dependent band defined in
    /// `ledger.md#identity-revocation` and [ADR-017].
    ///
    /// `including_height` is the height of the block that includes the
    /// revocation and `params` MUST be the parameters in force at that height.
    /// **This function does not enforce that correspondence** — it reads the two
    /// arguments it is given — so a verifier applying clause 3 of [ADR-017]
    /// calls [`Self::validate_effective_height_in_block`], which derives both
    /// from the including block's header.
    ///
    /// # Errors
    ///
    /// [`crate::error::RevocationError::EffectiveHeightBelowFloor`] when `effective_height < p + F`.
    /// [`crate::error::RevocationError::EffectiveHeightAboveCeiling`] when `effective_height > p + F + G` (for key compromise)
    /// or `effective_height > p + P` (for misconduct / operator request).
    pub fn validate_effective_height(
        &self,
        including_height: u64,
        params: &ConsensusParameters,
    ) -> Result<()> {
        let floor = including_height
            .checked_add(params.min_revocation_effective_delay_blocks)
            .ok_or(crate::error::Error::Arithmetic("effective_height floor"))?;
        if self.effective_height < floor {
            return Err(crate::error::RevocationError::EffectiveHeightBelowFloor {
                including_height,
                effective_height: self.effective_height,
                floor,
            }
            .into());
        }
        let ceiling = match self.reason {
            RevocationReason::KeyCompromise => floor
                .checked_add(params.revocation_effective_grace_blocks)
                .ok_or(crate::error::Error::Arithmetic("effective_height ceiling"))?,
            RevocationReason::ValidatorMisconduct | RevocationReason::OperatorRequest => {
                including_height
                    .checked_add(params.max_planned_revocation_delay_blocks)
                    .ok_or(crate::error::Error::Arithmetic("effective_height ceiling"))?
            }
        };
        if self.effective_height > ceiling {
            return Err(crate::error::RevocationError::EffectiveHeightAboveCeiling {
                including_height,
                effective_height: self.effective_height,
                ceiling,
            }
            .into());
        }
        Ok(())
    }
}
