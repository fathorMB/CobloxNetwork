//! Wire protocol `SignedEnvelope` format conforming to `docs/protocol/wire.md`.

use coblox_core::SignatureVerifier;
use coblox_core::encoding;
use coblox_core::error::JsonError;
use coblox_core::hash::{ChainId, Digest32, Domain};
use coblox_core::json::{Json, JsonObject};
use coblox_core::registry::{message_id, signing_preimage};
use coblox_core::verifier::verify_in_context;

use crate::error::{NodeError, Result};
use crate::signer::SigningKey;

/// A signed wire envelope carrying an application message.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignedEnvelope {
    pub schema_version: String,
    pub network_id: String,
    pub message_type: String,
    pub message_id: Digest32,
    pub sender_node_id: String,
    pub created_at_ms: u64,
    pub expires_at_ms: u64,
    pub nonce: [u8; 16],
    pub payload: JsonObject,
    pub signature: [u8; 64],
}

const ENVELOPE_FIELDS: [&str; 10] = [
    "created_at_ms",
    "expires_at_ms",
    "message_id",
    "message_type",
    "network_id",
    "nonce",
    "payload",
    "schema_version",
    "sender_node_id",
    "signature",
];

impl SignedEnvelope {
    // Nove argomenti, e non sono sette piu' due di comodo: sono i campi che la busta
    // firmata di `wire.md` impone, piu' il verificatore. Raggrupparli in una struct
    // di parametri sposterebbe il conteggio senza cambiare cio' che il chiamante deve
    // fornire, e aggiungerebbe un tipo che esiste solo per compiacere un lint.
    // Dichiarato dal Lead nella presa in carico correttiva del 2026-08-27.
    #[allow(clippy::too_many_arguments)]
    /// Builds and signs an envelope for `payload`.
    ///
    /// # Errors
    ///
    /// Restituisce errore se la preimmagine non e' costruibile o se la firma fallisce.
    pub fn build_and_sign(
        chain_id: &ChainId,
        network_id: &str,
        message_type: &str,
        sender_node_id: &str,
        created_at_ms: u64,
        validity_duration_ms: u64,
        nonce: [u8; 16],
        payload: JsonObject,
        signer: &SigningKey,
    ) -> Result<Self> {
        let expires_at_ms = created_at_ms.saturating_add(validity_duration_ms);

        // 1. Build unsigned envelope without message_id and signature for message_id calculation
        let unsigned_for_id = JsonObject::builder()
            .uint("created_at_ms", created_at_ms)
            .uint("expires_at_ms", expires_at_ms)
            .str("message_type", message_type)
            .str("network_id", network_id)
            .bytes("nonce", &nonce)
            .object("payload", payload.clone())
            .str("schema_version", "0.1")
            .str("sender_node_id", sender_node_id)
            .build()?;

        let msg_id = message_id(chain_id, &unsigned_for_id);

        // 2. Build envelope with message_id for signature preimage calculation
        let unsigned_for_sig = JsonObject::builder()
            .uint("created_at_ms", created_at_ms)
            .uint("expires_at_ms", expires_at_ms)
            .digest("message_id", &msg_id)
            .str("message_type", message_type)
            .str("network_id", network_id)
            .bytes("nonce", &nonce)
            .object("payload", payload.clone())
            .str("schema_version", "0.1")
            .str("sender_node_id", sender_node_id)
            .build()?;

        let preimage = signing_preimage(
            Domain::SIG_WIRE_ENVELOPE,
            chain_id,
            &unsigned_for_sig.to_jcs(),
        );
        let signature = signer.sign(preimage.as_bytes());

        Ok(Self {
            schema_version: "0.1".to_owned(),
            network_id: network_id.to_owned(),
            message_type: message_type.to_owned(),
            message_id: msg_id,
            sender_node_id: sender_node_id.to_owned(),
            created_at_ms,
            expires_at_ms,
            nonce,
            payload,
            signature,
        })
    }

    /// Serializes envelope to canonical JSON object.
    ///
    /// # Errors
    ///
    /// Restituisce errore se un campo non e' rappresentabile nella forma canonica.
    pub fn to_json(&self) -> Result<JsonObject> {
        Ok(JsonObject::builder()
            .uint("created_at_ms", self.created_at_ms)
            .uint("expires_at_ms", self.expires_at_ms)
            .digest("message_id", &self.message_id)
            .str("message_type", &self.message_type)
            .str("network_id", &self.network_id)
            .bytes("nonce", &self.nonce)
            .object("payload", self.payload.clone())
            .str("schema_version", &self.schema_version)
            .str("sender_node_id", &self.sender_node_id)
            .bytes("signature", &self.signature)
            .build()?)
    }

    /// Serializes envelope to canonical JCS byte string.
    ///
    /// # Errors
    ///
    /// Restituisce errore se la serializzazione canonica JCS fallisce.
    pub fn to_jcs(&self) -> Result<Vec<u8>> {
        Ok(self.to_json()?.to_jcs())
    }

    /// Parses envelope from JSON object.
    ///
    /// # Errors
    ///
    /// Restituisce errore se un campo obbligatorio manca o ha il tipo sbagliato.
    pub fn from_json(object: &JsonObject) -> Result<Self> {
        object.reject_unknown_fields(&ENVELOPE_FIELDS)?;
        let nonce =
            encoding::base64url_decode_fixed::<16>(object.string("nonce")?, "envelope nonce")?;
        let signature = encoding::base64url_decode_fixed::<64>(
            object.string("signature")?,
            "envelope signature",
        )?;
        let payload = match object.get("payload") {
            Some(Json::Object(p)) => p.clone(),
            _ => return Err(NodeError::Core(JsonError::NotAnObject.into())),
        };

        Ok(Self {
            schema_version: object.string("schema_version")?.to_owned(),
            network_id: object.string("network_id")?.to_owned(),
            message_type: object.string("message_type")?.to_owned(),
            message_id: object.digest("message_id")?,
            sender_node_id: object.string("sender_node_id")?.to_owned(),
            created_at_ms: object.uint("created_at_ms")?,
            expires_at_ms: object.uint("expires_at_ms")?,
            nonce,
            payload,
            signature,
        })
    }

    /// Parses envelope from JCS byte slice.
    ///
    /// # Errors
    ///
    /// Restituisce errore se i byte non sono JSON valido o non hanno la forma della busta.
    pub fn from_slice(bytes: &[u8]) -> Result<Self> {
        let object = JsonObject::parse_canonical(bytes)?;
        Self::from_json(&object)
    }

    /// Verifies envelope `message_id`, timing, and signature against sender public key.
    ///
    /// # Errors
    ///
    /// Restituisce errore se la preimmagine non e' ricostruibile o se la firma non verifica.
    pub fn verify<V: SignatureVerifier + ?Sized>(
        &self,
        chain_id: &ChainId,
        public_key: &[u8; 32],
        now_ms: u64,
        verifier: &V,
    ) -> Result<()> {
        if self.expires_at_ms < self.created_at_ms {
            return Err(NodeError::Protocol(
                "envelope expires before creation".into(),
            ));
        }
        if now_ms > self.expires_at_ms {
            return Err(NodeError::Protocol("envelope expired".into()));
        }

        // Recompute message_id
        let unsigned_for_id = JsonObject::builder()
            .uint("created_at_ms", self.created_at_ms)
            .uint("expires_at_ms", self.expires_at_ms)
            .str("message_type", &self.message_type)
            .str("network_id", &self.network_id)
            .bytes("nonce", &self.nonce)
            .object("payload", self.payload.clone())
            .str("schema_version", &self.schema_version)
            .str("sender_node_id", &self.sender_node_id)
            .build()?;

        let computed_id = message_id(chain_id, &unsigned_for_id);
        if computed_id != self.message_id {
            return Err(NodeError::Protocol("envelope message_id mismatch".into()));
        }

        // Recompute preimage and check signature
        let unsigned_for_sig = JsonObject::builder()
            .uint("created_at_ms", self.created_at_ms)
            .uint("expires_at_ms", self.expires_at_ms)
            .digest("message_id", &self.message_id)
            .str("message_type", &self.message_type)
            .str("network_id", &self.network_id)
            .bytes("nonce", &self.nonce)
            .object("payload", self.payload.clone())
            .str("schema_version", &self.schema_version)
            .str("sender_node_id", &self.sender_node_id)
            .build()?;

        let preimage = signing_preimage(
            Domain::SIG_WIRE_ENVELOPE,
            chain_id,
            &unsigned_for_sig.to_jcs(),
        );

        if !verify_in_context(
            verifier,
            Domain::SIG_WIRE_ENVELOPE,
            chain_id,
            public_key,
            &preimage,
            &self.signature,
        ) {
            return Err(NodeError::Protocol("envelope signature invalid".into()));
        }

        Ok(())
    }
}
