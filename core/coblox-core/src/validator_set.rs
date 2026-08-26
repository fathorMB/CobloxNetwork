//! `ValidatorSet`, `ElectionRecord`, and the validity rules on a set and on a
//! transition between two sets.
//!
//! Everything here is checkable from the set documents themselves plus the
//! active parameters. That perimeter is deliberate and is the reason
//! `ledger.md` splits its election rule into two layers: "Layer 1 is a function
//! of the validator-set documents alone, so a light client that never sees a
//! transaction verifies all of it."

use crate::error::{Error, Result, SetError};
use crate::hash::{ChainId, Digest32, Domain};
use crate::json::{Json, JsonObject};
use crate::params::ValidatedConsensusParameters;
use crate::quorum;
use crate::registry;

/// One member of a validator set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatorEntry {
    /// Sort key of the set. In an elected set it equals `node_id`.
    pub validator_id: String,
    /// The enrolled identity this seat belongs to.
    pub node_id: String,
    /// The subordinate consensus key, distinct from the identity key.
    pub consensus_public_key: [u8; 32],
    /// Proof of possession over the binding object, re-issued per
    /// `activation_height`.
    pub key_binding_signature: [u8; 64],
    /// The epoch at which this seat was filled.
    pub seated_since_epoch: u64,
    /// The stamp written when the seat was filled; carried unchanged for the
    /// whole tenure.
    pub term_expiry_epoch: u64,
    /// Voting power. In an elected set it equals 1.
    pub voting_power: u64,
}

const ENTRY_FIELDS: [&str; 7] = [
    "validator_id",
    "node_id",
    "consensus_public_key",
    "key_binding_signature",
    "seated_since_epoch",
    "term_expiry_epoch",
    "voting_power",
];

impl ValidatorEntry {
    /// The canonical object for this entry.
    pub fn to_json(&self) -> Result<JsonObject> {
        JsonObject::builder()
            .str("validator_id", &self.validator_id)
            .str("node_id", &self.node_id)
            .bytes("consensus_public_key", &self.consensus_public_key)
            .bytes("key_binding_signature", &self.key_binding_signature)
            .uint("seated_since_epoch", self.seated_since_epoch)
            .uint("term_expiry_epoch", self.term_expiry_epoch)
            .uint("voting_power", self.voting_power)
            .build()
    }

    /// Reads an entry from a canonical object.
    pub fn from_json(object: &JsonObject) -> Result<Self> {
        object.reject_unknown_fields(&ENTRY_FIELDS)?;
        Ok(Self {
            validator_id: object.string("validator_id")?.to_owned(),
            node_id: object.string("node_id")?.to_owned(),
            consensus_public_key: crate::encoding::base64url_decode_fixed::<32>(
                object.string("consensus_public_key")?,
                "consensus_public_key",
            )?,
            key_binding_signature: crate::encoding::base64url_decode_fixed::<64>(
                object.string("key_binding_signature")?,
                "key_binding_signature",
            )?,
            seated_since_epoch: object.uint("seated_since_epoch")?,
            term_expiry_epoch: object.uint("term_expiry_epoch")?,
            voting_power: object.uint("voting_power")?,
        })
    }

    /// The object a `key_binding_signature` is computed over.
    pub fn binding_object(&self, network_id: &str, activation_height: u64) -> Result<JsonObject> {
        consensus_key_binding_object(
            network_id,
            activation_height,
            &self.consensus_public_key,
            &self.node_id,
            &self.validator_id,
        )
    }
}

/// The JCS object of the consensus key binding proof of possession.
///
/// `network_id` is in the object and not only in the preimage's `chain_id_32`
/// prefix, and the reason is the genesis window. See
/// [`consensus_key_binding_preimage`].
pub fn consensus_key_binding_object(
    network_id: &str,
    activation_height: u64,
    consensus_public_key: &[u8; 32],
    node_id: &str,
    validator_id: &str,
) -> Result<JsonObject> {
    JsonObject::builder()
        .uint("activation_height", activation_height)
        .bytes("consensus_public_key", consensus_public_key)
        .str("network_id", network_id)
        .str("node_id", node_id)
        .str("validator_id", validator_id)
        .build()
}

/// The exact bytes a `key_binding_signature` signs.
///
/// The signature is made by the **consensus** key and is verified with the
/// identity public key from the finalized enrollment certificate — the
/// specification's wording — so the caller supplies both; this function only
/// builds the message.
///
/// # The genesis set signs under the placeholder, and that is why `network_id`
/// # is in the object
///
/// A `ValidatorSet`'s bytes are an input to `validator_set_hash`, which is a
/// field of the height-0 header, so a genesis binding cannot be signed under a
/// `chain_id` that does not exist yet: `chain_id` here is
/// [`ChainId::GENESIS_PLACEHOLDER`], per
/// `README.md#genesis-derivation-and-the-placeholder-chain-id`. **A caller
/// building a genesis set and passing a derived `chain_id` reopens the
/// circularity**, and the symptom is not a digest a conformance suite catches:
/// it is a different `genesis_block_id`, so the two implementations simply
/// disagree about which chain they are on.
///
/// The placeholder is the same 32 zero bytes on every network, so inside that
/// window the prefix separates domains and not chains. Without `network_id` in
/// the object, a genesis entry signed on one network would produce a
/// **byte-identical** preimage on another, and the published signature could be
/// replayed to seat that validator in a genesis it never consented to — the one
/// thing this signature exists to prove ([REVIEW-029] RF-002). `network_id` is
/// in the object at every height rather than only at genesis, because a shape
/// that changes at one height is a shape to get wrong; after genesis it is
/// redundant with `chain_id_32` and harmless.
///
/// What it buys is bounded and the bound is stated in the document: attribution
/// at the level of the **network name**, which is an operational convention and
/// not a replay control. Two chains that share a `network_id` still share this
/// preimage inside the genesis window, and nothing available before
/// `genesis_block_id` exists could do better.
pub fn consensus_key_binding_preimage(
    chain_id: &ChainId,
    network_id: &str,
    activation_height: u64,
    consensus_public_key: &[u8; 32],
    node_id: &str,
    validator_id: &str,
) -> Result<registry::SigningPreimage> {
    let object = consensus_key_binding_object(
        network_id,
        activation_height,
        consensus_public_key,
        node_id,
        validator_id,
    )?;
    Ok(registry::signing_preimage(
        Domain::SIG_CONSENSUS_KEY_BINDING,
        chain_id,
        &object.to_jcs(),
    ))
}

/// The record an elected set carries.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ElectionRecord {
    /// `e`.
    pub election_epoch: u64,
    /// Hash of `P`, the set active at `boundary - 1`.
    pub previous_validator_set_hash: Digest32,
    /// Root of the committed candidate set `C`.
    pub candidate_root: Digest32,
    /// `|C|`.
    pub candidate_count: u64,
    /// First height of the entropy window.
    pub entropy_first_height: u64,
    /// Canonical finalized block IDs of the entropy window, ascending.
    pub entropy_block_ids: Vec<Digest32>,
    /// The committed seed.
    pub election_seed: Digest32,
    /// Entries whose `seated_since_epoch` is below `election_epoch`.
    pub retained_count: u64,
    /// Entries whose `seated_since_epoch` equals `election_epoch`.
    pub filled_count: u64,
    /// Array length of `validators`.
    pub member_count: u64,
}

const RECORD_FIELDS: [&str; 10] = [
    "election_epoch",
    "previous_validator_set_hash",
    "candidate_root",
    "candidate_count",
    "entropy_first_height",
    "entropy_block_ids",
    "election_seed",
    "retained_count",
    "filled_count",
    "member_count",
];

impl ElectionRecord {
    /// The canonical object for this record.
    pub fn to_json(&self) -> Result<JsonObject> {
        JsonObject::builder()
            .uint("election_epoch", self.election_epoch)
            .digest(
                "previous_validator_set_hash",
                &self.previous_validator_set_hash,
            )
            .digest("candidate_root", &self.candidate_root)
            .uint("candidate_count", self.candidate_count)
            .uint("entropy_first_height", self.entropy_first_height)
            .array(
                "entropy_block_ids",
                self.entropy_block_ids.iter().map(Json::digest).collect(),
            )
            .digest("election_seed", &self.election_seed)
            .uint("retained_count", self.retained_count)
            .uint("filled_count", self.filled_count)
            .uint("member_count", self.member_count)
            .build()
    }

    /// Reads a record from a canonical object.
    pub fn from_json(object: &JsonObject) -> Result<Self> {
        object.reject_unknown_fields(&RECORD_FIELDS)?;
        let entropy_block_ids = object
            .array("entropy_block_ids")?
            .iter()
            .map(|item| match item {
                Json::Str(text) => Digest32::parse_prefixed(text),
                _ => Err(crate::error::JsonError::Field("entropy_block_ids".to_owned()).into()),
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(Self {
            election_epoch: object.uint("election_epoch")?,
            previous_validator_set_hash: object.digest("previous_validator_set_hash")?,
            candidate_root: object.digest("candidate_root")?,
            candidate_count: object.uint("candidate_count")?,
            entropy_first_height: object.uint("entropy_first_height")?,
            entropy_block_ids,
            election_seed: object.digest("election_seed")?,
            retained_count: object.uint("retained_count")?,
            filled_count: object.uint("filled_count")?,
            member_count: object.uint("member_count")?,
        })
    }
}

/// A validator set document.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ValidatorSet {
    /// Object schema version.
    pub schema_version: String,
    /// Height at which this set becomes active.
    pub activation_height: u64,
    /// Absent only for the genesis set, which is a trust anchor rather than a
    /// derived object.
    pub election: Option<ElectionRecord>,
    /// Members, sorted by `validator_id` and unique.
    pub validators: Vec<ValidatorEntry>,
}

const SET_FIELDS: [&str; 4] = [
    "schema_version",
    "activation_height",
    "election",
    "validators",
];

impl ValidatorSet {
    /// The canonical object for this set.
    pub fn to_json(&self) -> Result<JsonObject> {
        let mut builder = JsonObject::builder()
            .str("schema_version", &self.schema_version)
            .uint("activation_height", self.activation_height);
        if let Some(record) = &self.election {
            builder = builder.object("election", record.to_json()?);
        }
        let entries = self
            .validators
            .iter()
            .map(|entry| entry.to_json().map(Json::Object))
            .collect::<Result<Vec<_>>>()?;
        builder.array("validators", entries).build()
    }

    /// Reads a set from a canonical object.
    pub fn from_json(object: &JsonObject) -> Result<Self> {
        object.reject_unknown_fields(&SET_FIELDS)?;
        let election = match object.get("election") {
            Some(Json::Object(record)) => Some(ElectionRecord::from_json(record)?),
            Some(_) => return Err(crate::error::JsonError::Field("election".to_owned()).into()),
            None => None,
        };
        let validators = object
            .array("validators")?
            .iter()
            .map(|item| match item {
                Json::Object(entry) => ValidatorEntry::from_json(entry),
                _ => Err(crate::error::JsonError::Field("validators".to_owned()).into()),
            })
            .collect::<Result<Vec<_>>>()?;
        Ok(Self {
            schema_version: object.string("schema_version")?.to_owned(),
            activation_height: object.uint("activation_height")?,
            election,
            validators,
        })
    }

    /// `validator_set_hash` over this set's JCS bytes.
    pub fn hash(&self) -> Result<Digest32> {
        Ok(registry::validator_set_hash(&self.to_json()?))
    }

    /// The number of members.
    pub fn member_count(&self) -> Result<u64> {
        u64::try_from(self.validators.len()).map_err(|_| Error::Arithmetic("member_count"))
    }

    /// The summed voting power, which the quorum predicate operates on.
    pub fn total_voting_power(&self) -> Result<u64> {
        quorum::total_voting_power(self.validators.iter().map(|entry| entry.voting_power))
    }

    /// Finds a member by node ID.
    #[must_use]
    pub fn find(&self, node_id: &str) -> Option<&ValidatorEntry> {
        self.validators
            .iter()
            .find(|entry| entry.node_id == node_id)
    }

    /// Structural rules that hold for every set, elected or genesis.
    ///
    /// Sortedness, uniqueness, positive power and a non-overflowing sum. Size
    /// bounds are checked separately because they need the active parameters.
    pub fn check_structure(&self) -> Result<()> {
        if self.validators.is_empty() {
            return Err(SetError::Size { member_count: 0 }.into());
        }
        for pair in self.validators.windows(2) {
            if pair[0].validator_id >= pair[1].validator_id {
                return Err(SetError::NotSortedOrUnique.into());
            }
        }
        self.total_voting_power()
            .map_err(|_| Error::ValidatorSet(SetError::VotingPower))?;
        Ok(())
    }

    /// Layer-1 checks 3 and 4 of the light-client perimeter, plus the elected
    /// set's uniformity rule.
    ///
    /// * every member has `voting_power` 1 and `validator_id == node_id`;
    /// * `member_count` lies in `[validator_min_set_size, validator_max_set_size]`;
    /// * `election_epoch < term_expiry_epoch` for every entry.
    pub fn check_elected_shape(&self, parameters: &ValidatedConsensusParameters) -> Result<()> {
        self.check_structure()?;
        let record = self
            .election
            .as_ref()
            .ok_or(Error::ValidatorSet(SetError::Genesis {
                rule: "an elected set MUST carry an election record",
            }))?;
        let member_count = self.member_count()?;
        let params = parameters.get();
        if member_count < params.validator_min_set_size
            || member_count > params.validator_max_set_size
        {
            return Err(SetError::Size { member_count }.into());
        }
        for entry in &self.validators {
            if entry.validator_id != entry.node_id || entry.voting_power != 1 {
                return Err(SetError::NotUniformElectedEntry {
                    validator_id: entry.validator_id.clone(),
                }
                .into());
            }
            if record.election_epoch >= entry.term_expiry_epoch {
                return Err(SetError::TermExpired {
                    validator_id: entry.validator_id.clone(),
                }
                .into());
            }
        }
        Ok(())
    }

    /// Layer-1 checks 2, 6 and 7: activation height, the three counts, the
    /// rotation cap and the seed derivation.
    pub fn check_election_record(
        &self,
        chain_id: &ChainId,
        parameters: &ValidatedConsensusParameters,
    ) -> Result<()> {
        let record = self
            .election
            .as_ref()
            .ok_or(Error::ValidatorSet(SetError::Genesis {
                rule: "an elected set MUST carry an election record",
            }))?;
        let expected_height = parameters.election_boundary_height(record.election_epoch)?;
        if self.activation_height != expected_height {
            return Err(SetError::ActivationHeight {
                expected: expected_height,
                actual: self.activation_height,
            }
            .into());
        }

        let member_count = self.member_count()?;
        if record.member_count != member_count {
            return Err(SetError::CountMismatch {
                field: "member_count",
            }
            .into());
        }
        let retained = self
            .validators
            .iter()
            .filter(|entry| entry.seated_since_epoch < record.election_epoch)
            .count();
        let filled = self
            .validators
            .iter()
            .filter(|entry| entry.seated_since_epoch == record.election_epoch)
            .count();
        if u64::try_from(retained).map_err(|_| Error::Arithmetic("retained_count"))?
            != record.retained_count
        {
            return Err(SetError::CountMismatch {
                field: "retained_count",
            }
            .into());
        }
        if u64::try_from(filled).map_err(|_| Error::Arithmetic("filled_count"))?
            != record.filled_count
        {
            return Err(SetError::CountMismatch {
                field: "filled_count",
            }
            .into());
        }
        if record
            .retained_count
            .checked_add(record.filled_count)
            .ok_or(Error::Arithmetic("retained + filled"))?
            != member_count
        {
            return Err(SetError::CountMismatch {
                field: "retained_count + filled_count",
            }
            .into());
        }
        let cap = parameters.get().validator_churn_cap_seats;
        if record.filled_count > cap {
            return Err(SetError::ChurnCapExceeded {
                filled: record.filled_count,
                cap,
            }
            .into());
        }

        let (first, _) = parameters.entropy_window(record.election_epoch)?;
        if record.entropy_first_height != first {
            return Err(SetError::EntropyWindow.into());
        }
        let entropy = registry::election_entropy(
            chain_id,
            record.election_epoch,
            parameters.get().election_entropy_blocks,
            &record.entropy_block_ids,
        )
        .map_err(|_| Error::ValidatorSet(SetError::EntropyWindow))?;
        let seed = registry::election_seed(chain_id, record.election_epoch, &entropy);
        if seed != record.election_seed {
            return Err(SetError::SeedMismatch.into());
        }
        Ok(())
    }

    /// Layer-1 check 5: `seated_since_epoch` and `term_expiry_epoch`
    /// consistency across two adjacent sets.
    ///
    /// "A member present in both keeps both values unchanged; a member present
    /// only in the newer set has `seated_since_epoch` exactly `election_epoch`
    /// and `term_expiry_epoch` exactly `election_epoch +
    /// validator_max_consecutive_terms`."
    pub fn check_stamps_against_previous(
        &self,
        previous: &Self,
        parameters: &ValidatedConsensusParameters,
    ) -> Result<()> {
        let record = self
            .election
            .as_ref()
            .ok_or(Error::ValidatorSet(SetError::Genesis {
                rule: "an elected set MUST carry an election record",
            }))?;
        let epoch = record.election_epoch;
        let expected_expiry = epoch
            .checked_add(parameters.get().validator_max_consecutive_terms)
            .ok_or(Error::Arithmetic("term_expiry_epoch"))?;
        for entry in &self.validators {
            match previous.find(&entry.node_id) {
                Some(before) => {
                    if before.seated_since_epoch != entry.seated_since_epoch
                        || before.term_expiry_epoch != entry.term_expiry_epoch
                    {
                        return Err(SetError::StampInconsistent {
                            validator_id: entry.validator_id.clone(),
                        }
                        .into());
                    }
                }
                None => {
                    if entry.seated_since_epoch != epoch
                        || entry.term_expiry_epoch != expected_expiry
                    {
                        return Err(SetError::StampInconsistent {
                            validator_id: entry.validator_id.clone(),
                        }
                        .into());
                    }
                }
            }
        }
        Ok(())
    }

    /// Layer-1 check 9: the contraction floor across any transition.
    pub fn check_contraction_floor(&self, previous: &Self) -> Result<()> {
        let new = self.member_count()?;
        let old = previous.member_count()?;
        if quorum::contraction_floor(new, old)? {
            Ok(())
        } else {
            Err(SetError::ContractionFloor { new, old }.into())
        }
    }

    /// Check 2's remaining half: `previous_validator_set_hash` equals the hash
    /// of the set being replaced.
    pub fn check_previous_hash(&self, previous: &Self) -> Result<()> {
        let record = self
            .election
            .as_ref()
            .ok_or(Error::ValidatorSet(SetError::Genesis {
                rule: "an elected set MUST carry an election record",
            }))?;
        if record.previous_validator_set_hash != previous.hash()? {
            return Err(SetError::PreviousHashMismatch.into());
        }
        Ok(())
    }

    /// The genesis stagger rule.
    ///
    /// "In the genesis set, every entry's `term_expiry_epoch` lies in
    /// `[1, validator_max_consecutive_terms]`, and no more than
    /// `validator_churn_cap_seats` entries share the same value. A genesis set
    /// violating either condition is not a valid trust anchor and a client MUST
    /// refuse it."
    pub fn check_genesis_stagger(&self, parameters: &ValidatedConsensusParameters) -> Result<()> {
        self.check_structure()?;
        if self.election.is_some() {
            return Err(SetError::Genesis {
                rule: "the genesis set carries no election record",
            }
            .into());
        }
        let params = parameters.get();
        let mut counts: std::collections::BTreeMap<u64, u64> = std::collections::BTreeMap::new();
        for entry in &self.validators {
            if entry.seated_since_epoch != 0 {
                return Err(SetError::Genesis {
                    rule: "genesis entries carry seated_since_epoch 0",
                }
                .into());
            }
            if entry.term_expiry_epoch < 1
                || entry.term_expiry_epoch > params.validator_max_consecutive_terms
            {
                return Err(SetError::Genesis {
                    rule: "every genesis term_expiry_epoch lies in [1, validator_max_consecutive_terms]",
                }
                .into());
            }
            *counts.entry(entry.term_expiry_epoch).or_insert(0) += 1;
        }
        if counts
            .values()
            .any(|count| *count > params.validator_churn_cap_seats)
        {
            return Err(SetError::Genesis {
                rule: "no more than validator_churn_cap_seats genesis entries share a term_expiry_epoch",
            }
            .into());
        }
        Ok(())
    }

    /// The removal-only, revocation-forced transition.
    ///
    /// Rules 7 to 10 of
    /// `ledger.md#revocation-forces-a-validator-set-transition`. The successor
    /// must be a strict subset, entry-for-entry identical except for a re-issued
    /// `key_binding_signature`; its election record is copied verbatim except
    /// `member_count`; and it satisfies the contraction floor.
    ///
    /// Rule 8 — that every removed `node_id` has a finalized `revoke_identity`
    /// with a low enough `effective_height` — needs data a light client does not
    /// have, so it is a separate argument rather than something this function
    /// invents. Passing an empty list checks only what a light client can check;
    /// see [`crate::light_client`] entry `(f)`.
    pub fn check_removal_only_transition(
        &self,
        previous: &Self,
        revocations: &[(String, u64)],
    ) -> Result<()> {
        self.check_structure()?;
        if self.validators.len() >= previous.validators.len() {
            return Err(SetError::RemovalOnlyViolated {
                reason: "the successor must be a strict subset",
            }
            .into());
        }
        for entry in &self.validators {
            let before = previous.find(&entry.node_id).ok_or(Error::ValidatorSet(
                SetError::RemovalOnlyViolated {
                    reason: "a removal-only transition admits no member",
                },
            ))?;
            let identical = before.validator_id == entry.validator_id
                && before.consensus_public_key == entry.consensus_public_key
                && before.seated_since_epoch == entry.seated_since_epoch
                && before.term_expiry_epoch == entry.term_expiry_epoch
                && before.voting_power == entry.voting_power;
            if !identical {
                return Err(SetError::RemovalOnlyViolated {
                    reason: "retained entries are identical except for key_binding_signature",
                }
                .into());
            }
        }
        match (&previous.election, &self.election) {
            (Some(before), Some(after)) => {
                let mut expected = before.clone();
                expected.member_count = self.member_count()?;
                if expected != *after {
                    return Err(SetError::RemovalOnlyViolated {
                        reason: "the election record is copied verbatim except member_count",
                    }
                    .into());
                }
            }
            (None, None) => {}
            _ => {
                return Err(SetError::RemovalOnlyViolated {
                    reason: "the election record presence must match the set it replaces",
                }
                .into());
            }
        }
        self.check_contraction_floor(previous)?;

        for entry in previous
            .validators
            .iter()
            .filter(|entry| self.find(&entry.node_id).is_none())
        {
            if !revocations.is_empty()
                && !revocations.iter().any(|(node_id, effective_height)| {
                    node_id == &entry.node_id && *effective_height <= self.activation_height
                })
            {
                return Err(SetError::Revocation {
                    node_id: entry.node_id.clone(),
                }
                .into());
            }
        }
        Ok(())
    }

    /// Rules 1 and 2 of the revocation rule, in the form a light client applies
    /// them with the `revoked_validators` list of its checkpoint.
    ///
    /// A set with `activation_height >= effective_height` containing a revoked
    /// `node_id` is invalid; so is any block at height `>= effective_height`
    /// whose active set contains it. "The revoked entry's voting power is never
    /// counted in either `signed_power` or `total_power` — it is not
    /// reweighted, the set is simply rejected."
    pub fn check_against_revocations(&self, revocations: &[(String, u64)]) -> Result<()> {
        for (node_id, effective_height) in revocations {
            if self.activation_height >= *effective_height && self.find(node_id).is_some() {
                return Err(SetError::Revocation {
                    node_id: node_id.clone(),
                }
                .into());
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn entry(id: &str, seated: u64, expiry: u64) -> ValidatorEntry {
        ValidatorEntry {
            validator_id: id.to_owned(),
            node_id: id.to_owned(),
            consensus_public_key: [1u8; 32],
            key_binding_signature: [0u8; 64],
            seated_since_epoch: seated,
            term_expiry_epoch: expiry,
            voting_power: 1,
        }
    }

    #[test]
    fn a_set_must_be_sorted_and_unique_by_validator_id() {
        let set = ValidatorSet {
            schema_version: "0.1".to_owned(),
            activation_height: 0,
            election: None,
            validators: vec![entry("b", 0, 1), entry("a", 0, 2)],
        };
        assert!(set.check_structure().is_err());
    }

    #[test]
    fn set_json_round_trips_through_canonical_bytes() {
        let set = ValidatorSet {
            schema_version: "0.1".to_owned(),
            activation_height: 7,
            election: None,
            validators: vec![entry("a", 0, 1), entry("b", 0, 2)],
        };
        let bytes = set.to_json().unwrap().to_jcs();
        let parsed = JsonObject::parse_canonical(&bytes).unwrap();
        assert_eq!(ValidatorSet::from_json(&parsed).unwrap(), set);
    }
}
