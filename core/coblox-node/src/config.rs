//! Node and Devnet configuration.

use std::path::PathBuf;

use coblox_core::consensus::ConsensusTimeouts;
use coblox_core::hash::{ChainId, Digest32};
use coblox_core::validator_set::{ValidatorEntry, ValidatorSet};

use crate::signer::SigningKey;

/// Complete configuration for a running validator node.
#[derive(Debug, Clone)]
pub struct NodeConfig {
    pub validator_id: String,
    pub node_id: String,
    pub signing_key: SigningKey,
    pub network_id: String,
    pub chain_id: ChainId,
    pub genesis_block_id: Digest32,
    pub listen_addr: String,
    pub seed_peers: Vec<String>,
    pub data_dir: PathBuf,
    pub validator_set: ValidatorSet,
    pub timeouts: ConsensusTimeouts,
    pub target_height: Option<u64>,
}

/// Helper to generate a deterministic 4-validator devnet set and keys.
///
/// # Panics
///
/// Va in panico se il set di quattro validatori generato non supera la validazione strutturale, che sarebbe un difetto di questa funzione e non un input dell'utente.
#[must_use]
pub fn devnet_4_validator_set() -> (ValidatorSet, Vec<SigningKey>) {
    let count = 4;
    let mut validators = Vec::with_capacity(count);
    let mut keys = Vec::with_capacity(count);
    for index in 0..count {
        let seed = [u8::try_from(index).expect("4 validators") + 1; 32];
        let key = SigningKey::from_seed(&seed);
        let validator_id = format!("val-{index:03}");
        validators.push(ValidatorEntry {
            validator_id: validator_id.clone(),
            node_id: validator_id,
            consensus_public_key: key.public_key(),
            key_binding_signature: [0u8; 64],
            seated_since_epoch: 1,
            term_expiry_epoch: 9,
            voting_power: 1,
        });
        keys.push(key);
    }
    let set = ValidatorSet {
        schema_version: "0.1".to_owned(),
        activation_height: 0,
        election: None,
        validators,
    };
    (set, keys)
}

/// Derives standard devnet timeouts.
///
/// All four are multiples of one declared quantity, `Delta_net` = 50 ms, the
/// loopback round-trip budget this devnet is tuned for:
/// - `propose_ms`: 4 * `Delta_net` = 200 ms
/// - `prevote_ms`: 3 * `Delta_net` = 150 ms
/// - `precommit_ms`: 3 * `Delta_net` = 150 ms
/// - `round_increment_ms`: 2 * `Delta_net` = 100 ms
///
/// [REVIEW-049] RF-020 found that the multipliers written here did not produce
/// three of the four values below, and asked which of the two was right. **The
/// values were**: they are the ones the four-process devnet has actually
/// finalized ten heights at, and the arithmetic beside them had been written
/// after the fact and never recomputed. The multipliers above are corrected to
/// the values, not the other way round.
///
/// These are local parameters in the sense [`ConsensusTimeouts`] documents: no
/// validity rule compares them, and two nodes with different values are both
/// conformant.
#[must_use]
pub fn devnet_timeouts() -> ConsensusTimeouts {
    ConsensusTimeouts {
        propose_ms: 200,
        prevote_ms: 150,
        precommit_ms: 150,
        round_increment_ms: 100,
    }
}
