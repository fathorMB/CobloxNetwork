//! Block headers and the header-only checks a light client performs.
//!
//! Everything here is a function of header fields. That is the point: the
//! boundary rule of `ledger.md#validator-set-continuity` was written so that "a
//! light client checks this with two fields it already reads, without seeing a
//! single transaction", and a header type that needed a transaction to answer
//! the question would have given that property away.

use crate::error::{Result, SetError};
use crate::hash::{ChainId, Digest32};
use crate::json::JsonObject;
use crate::params::ValidatedConsensusParameters;
use crate::registry;

/// A `BlockHeader`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockHeader {
    /// Object schema version, `"0.1"` in v0.
    pub schema_version: String,
    /// Ledger rule selector. "Nodes MUST NOT apply rules from an unrecognized
    /// version."
    pub protocol_version: String,
    /// Network identifier.
    pub network_id: String,
    /// Block height; genesis is 0.
    pub height: u64,
    /// Consensus round.
    pub round: u64,
    /// Header timestamp in Unix milliseconds.
    pub timestamp_ms: u64,
    /// Previous block ID; genesis uses the configured all-zero ID.
    pub previous_block_id: Digest32,
    /// Merkle root over the transactions, in canonical execution order.
    pub transactions_root: Digest32,
    /// Account state root after all transactions execute atomically.
    pub state_root: Digest32,
    /// Hash of the set active at this height.
    pub validator_set_hash: Digest32,
    /// Hash of the set committed as the successor.
    pub next_validator_set_hash: Digest32,
    /// Hash of the active `consensus_parameters` document.
    pub consensus_parameters_hash: Digest32,
}

/// The field names of `BlockHeader`, in schema order.
const HEADER_FIELDS: [&str; 12] = [
    "schema_version",
    "protocol_version",
    "network_id",
    "height",
    "round",
    "timestamp_ms",
    "previous_block_id",
    "transactions_root",
    "state_root",
    "validator_set_hash",
    "next_validator_set_hash",
    "consensus_parameters_hash",
];

impl BlockHeader {
    /// The canonical object this header serializes to.
    pub fn to_json(&self) -> Result<JsonObject> {
        JsonObject::builder()
            .str("schema_version", &self.schema_version)
            .str("protocol_version", &self.protocol_version)
            .str("network_id", &self.network_id)
            .uint("height", self.height)
            .uint("round", self.round)
            .uint("timestamp_ms", self.timestamp_ms)
            .digest("previous_block_id", &self.previous_block_id)
            .digest("transactions_root", &self.transactions_root)
            .digest("state_root", &self.state_root)
            .digest("validator_set_hash", &self.validator_set_hash)
            .digest("next_validator_set_hash", &self.next_validator_set_hash)
            .digest("consensus_parameters_hash", &self.consensus_parameters_hash)
            .build()
    }

    /// Reads a header from a canonical object, rejecting unknown fields.
    pub fn from_json(object: &JsonObject) -> Result<Self> {
        object.reject_unknown_fields(&HEADER_FIELDS)?;
        Ok(Self {
            schema_version: object.string("schema_version")?.to_owned(),
            protocol_version: object.string("protocol_version")?.to_owned(),
            network_id: object.string("network_id")?.to_owned(),
            height: object.uint("height")?,
            round: object.uint("round")?,
            timestamp_ms: object.uint("timestamp_ms")?,
            previous_block_id: object.digest("previous_block_id")?,
            transactions_root: object.digest("transactions_root")?,
            state_root: object.digest("state_root")?,
            validator_set_hash: object.digest("validator_set_hash")?,
            next_validator_set_hash: object.digest("next_validator_set_hash")?,
            consensus_parameters_hash: object.digest("consensus_parameters_hash")?,
        })
    }

    /// `block_id = H("coblox-block-id-v0\0" || chain_id_32 || JCS(header))`.
    pub fn block_id(&self, chain_id: &ChainId) -> Result<Digest32> {
        Ok(registry::block_id(chain_id, &self.to_json()?))
    }
}

/// Why a header is permitted to commit a successor set different from the
/// active one.
///
/// There are exactly two occasions, and a verifier must name which one it
/// believes applies rather than inferring it from the fact that a change
/// happened.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TransitionOccasion {
    /// The next height is an election boundary.
    ElectionBoundary,
    /// The transition is a removal-only, revocation-forced one.
    RevocationForced,
    /// No transition is due at this height.
    None,
}

/// The boundary rule of `ledger.md#validator-set-continuity`.
///
/// "At every finalized height `h` that is neither an election boundary nor a
/// revocation-forced transition, `next_validator_set_hash` MUST equal
/// `validator_set_hash`. A block that changes the committed successor set
/// outside those two occasions is invalid."
///
/// Note the asymmetry the specification intends and this function preserves: a
/// permitted occasion *allows* a change, it does not *require* one. A boundary
/// at which the derivation happens to reproduce the same set is valid.
pub fn check_successor_commitment(
    header: &BlockHeader,
    occasion: TransitionOccasion,
) -> Result<()> {
    let changed = header.next_validator_set_hash != header.validator_set_hash;
    if changed && occasion == TransitionOccasion::None {
        return Err(SetError::OffScheduleChange {
            height: header.height,
        }
        .into());
    }
    Ok(())
}

/// Classifies a height for the boundary rule, given the active parameters and
/// whether the verifier has established a revocation-forced transition.
///
/// A light client has no way to establish the second condition from headers
/// alone — see [`crate::light_client`] entry `(f)` — so `revocation_forced` is
/// an explicit input rather than something this function guesses.
#[must_use]
pub fn transition_occasion(
    parameters: &ValidatedConsensusParameters,
    next_height: u64,
    revocation_forced: bool,
) -> TransitionOccasion {
    if parameters.is_election_boundary(next_height) {
        TransitionOccasion::ElectionBoundary
    } else if revocation_forced {
        TransitionOccasion::RevocationForced
    } else {
        TransitionOccasion::None
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn header(active: u8, next: u8) -> BlockHeader {
        BlockHeader {
            schema_version: "0.1".to_owned(),
            protocol_version: "0.1".to_owned(),
            network_id: "fixture".to_owned(),
            height: 42,
            round: 0,
            timestamp_ms: 1,
            previous_block_id: Digest32::repeated(0x11),
            transactions_root: Digest32::repeated(0x22),
            state_root: Digest32::repeated(0x33),
            validator_set_hash: Digest32::repeated(active),
            next_validator_set_hash: Digest32::repeated(next),
            consensus_parameters_hash: Digest32::repeated(0x44),
        }
    }

    #[test]
    fn a_successor_change_outside_the_two_occasions_is_invalid() {
        assert!(check_successor_commitment(&header(1, 2), TransitionOccasion::None).is_err());
        assert!(check_successor_commitment(&header(1, 1), TransitionOccasion::None).is_ok());
        assert!(
            check_successor_commitment(&header(1, 2), TransitionOccasion::ElectionBoundary).is_ok()
        );
        assert!(
            check_successor_commitment(&header(1, 2), TransitionOccasion::RevocationForced).is_ok()
        );
    }

    #[test]
    fn header_json_round_trips_through_canonical_bytes() {
        let original = header(1, 1);
        let bytes = original.to_json().unwrap().to_jcs();
        let parsed = JsonObject::parse_canonical(&bytes).unwrap();
        assert_eq!(BlockHeader::from_json(&parsed).unwrap(), original);
    }
}
