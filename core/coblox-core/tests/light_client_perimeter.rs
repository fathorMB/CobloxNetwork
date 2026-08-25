//! The light client's perimeter: the checks it performs, and the closed list of
//! things it does not.
//!
//! The second list is tested here as deliberately as the first, because
//! `ledger.md` treats it as specification: "If you implement a check the
//! specification declares impossible for a light client, you have not added a
//! guarantee: you have introduced an assumption the protocol does not
//! authorize."

mod common;

use coblox_core::block::{BlockHeader, TransitionOccasion, transition_occasion};
use coblox_core::election::{self, CandidateFacts};
use coblox_core::error::{Error, ParameterError};
use coblox_core::hash::{AccountKey, ChainId, Digest32};
use coblox_core::json::JsonObject;
use coblox_core::light_client::{self, CANNOT_ESTABLISH, TrustedTip};
use coblox_core::merkle::{self, TaggedTree};
use coblox_core::params::{
    ActiveRewardPolicyDocument, ElectionBounds, RewardBounds, RewardPolicy,
    ValidatedConsensusParameters,
};
use coblox_core::registry::{self, DocumentKind};
use coblox_core::validator_set::{ElectionRecord, ValidatorEntry, ValidatorSet};

use common::{
    Pd0Kind, consensus_parameters_of, consensus_parameters_pd0, permissive_bounds,
    permissive_reward_bounds, protocol_document_pd0, reward_document_of, reward_policy_pd0,
    zero_chain_id,
};

/// The `consensus_parameters_hash` of the PD-0 document, from the published
/// registry table. Quoted rather than recomputed, so the header below commits
/// to a value that came from the specification.
const PD0_CONSENSUS_PARAMETERS_HASH: &str =
    "sha256:628c66f9ca8ac1a3161a0159201f7b6c6bf4c7500b390bc89b9b65a6c50ccbe9";

/// The `policy_hash` of the PD-0 reward document, from the same published
/// table and quoted for the same reason.
const PD0_REWARD_POLICY_HASH: &str =
    "sha256:89da35fbb8f0ba3c9ebffc0e3c5987045a005aaa7414356ef16a978a92025c48";

fn parameters() -> ValidatedConsensusParameters {
    consensus_parameters_of(&consensus_parameters_pd0())
}

fn node(index: u8) -> String {
    format!("cblx1node{index:02x}")
}

fn entry(index: u8, seated_since_epoch: u64, term_expiry_epoch: u64) -> ValidatorEntry {
    ValidatorEntry {
        validator_id: node(index),
        node_id: node(index),
        consensus_public_key: [index; 32],
        key_binding_signature: [index; 64],
        seated_since_epoch,
        term_expiry_epoch,
        voting_power: 1,
    }
}

fn fact(index: u8, eligible: bool) -> CandidateFacts {
    CandidateFacts {
        node_id: node(index),
        account_key: AccountKey::from_bytes([index; 32]),
        consensus_public_key: [index; 32],
        key_binding_signature: [index; 64],
        eligible,
    }
}

/// `P`, active at height 20 (the epoch-5 boundary, `L = 4`).
fn previous_set() -> ValidatorSet {
    ValidatorSet {
        schema_version: "0.1".to_owned(),
        activation_height: 20,
        election: Some(ElectionRecord {
            election_epoch: 5,
            previous_validator_set_hash: Digest32::repeated(0xee),
            candidate_root: Digest32::repeated(0xdd),
            candidate_count: 12,
            entropy_first_height: 18,
            entropy_block_ids: vec![Digest32::repeated(0x0a), Digest32::repeated(0x0b)],
            election_seed: Digest32::repeated(0xcc),
            retained_count: 12,
            filled_count: 0,
            member_count: 12,
        }),
        validators: (0..12).map(|i| entry(i, 4, 9)).collect(),
    }
}

fn entropy() -> Vec<Digest32> {
    vec![Digest32::repeated(0xaa), Digest32::repeated(0xbb)]
}

/// The epoch-6 successor, derived rather than hand-written so that the checks
/// run against a set the derivation actually produces.
fn derived_successor() -> ValidatorSet {
    let mut facts: Vec<CandidateFacts> = (0..12).map(|i| fact(i, i != 11)).collect();
    facts.extend((100..104).map(|i| fact(i, true)));
    election::derive(
        &zero_chain_id(),
        &parameters(),
        6,
        &previous_set(),
        &facts,
        &entropy(),
    )
    .expect("a valid epoch-6 set")
    .set
}

fn header(height: u64, active: &Digest32, next: &Digest32) -> BlockHeader {
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
        validator_set_hash: *active,
        next_validator_set_hash: *next,
        consensus_parameters_hash: Digest32::parse_prefixed(PD0_CONSENSUS_PARAMETERS_HASH).unwrap(),
    }
}

// --- Checks 1 to 11, the list the client can establish ----------------------

/// Check 1, and the boundary rule it rests on.
#[test]
fn check_1_the_set_changes_only_where_it_is_permitted_to() {
    let parameters = parameters();
    let active = Digest32::repeated(0x01);
    let next = Digest32::repeated(0x02);

    // `L = 4`: height 23's successor takes effect at 24, an election boundary.
    assert_eq!(
        transition_occasion(&parameters, 24, false),
        TransitionOccasion::ElectionBoundary
    );
    light_client::check_no_off_schedule_change(
        &header(23, &active, &next),
        transition_occasion(&parameters, 24, false),
    )
    .expect("a change at a boundary is permitted");

    // Height 21's successor takes effect at 22, which is neither occasion.
    assert_eq!(
        transition_occasion(&parameters, 22, false),
        TransitionOccasion::None
    );
    assert!(
        light_client::check_no_off_schedule_change(
            &header(21, &active, &next),
            transition_occasion(&parameters, 22, false),
        )
        .is_err()
    );
    // The same height with an established revocation-forced transition.
    light_client::check_no_off_schedule_change(
        &header(21, &active, &next),
        transition_occasion(&parameters, 22, true),
    )
    .expect("a removal-only transition is the second permitted occasion");

    // A permitted occasion allows a change; it does not require one.
    light_client::check_no_off_schedule_change(
        &header(23, &active, &active),
        transition_occasion(&parameters, 24, false),
    )
    .expect("an unchanged successor is always valid");

    // Epoch 0 is the genesis set, so height 0 is not an election boundary.
    assert!(!parameters.is_election_boundary(0));
    assert_eq!(parameters.epoch_at_boundary(24), Some(6));
    assert_eq!(parameters.epoch_at_boundary(23), None);
}

/// Checks 2 to 7 and 9, applied together as step 5 of the algorithm does.
#[test]
fn checks_2_to_9_over_a_lawful_transition() {
    let previous = previous_set();
    let new = derived_successor();
    light_client::check_transition(&zero_chain_id(), &parameters(), &previous, &new, &[])
        .expect("a lawfully shaped and lawfully rotating transition");
}

/// Each of those checks refuses its own violation.
#[test]
fn each_layer_one_check_refuses_its_own_violation() {
    let parameters = parameters();
    let chain = zero_chain_id();
    let previous = previous_set();
    let base = derived_successor();

    // Check 3: uniform voting power.
    let mut weighted = base.clone();
    weighted.validators[0].voting_power = 2;
    assert!(
        light_client::check_transition(&chain, &parameters, &previous, &weighted, &[]).is_err()
    );

    // Check 3: `validator_id` equal to `node_id`.
    let mut renamed = base.clone();
    renamed.validators[0].validator_id = "aaa-renamed".to_owned();
    renamed
        .validators
        .sort_by(|left, right| left.validator_id.cmp(&right.validator_id));
    assert!(light_client::check_transition(&chain, &parameters, &previous, &renamed, &[]).is_err());

    // Check 4: no member seated beyond its term.
    let mut expired = base.clone();
    expired.validators[0].term_expiry_epoch = 6;
    assert!(light_client::check_transition(&chain, &parameters, &previous, &expired, &[]).is_err());

    // Check 2: the activation height must be `election_epoch * L`.
    let mut misplaced = base.clone();
    misplaced.activation_height = 25;
    assert!(
        light_client::check_transition(&chain, &parameters, &previous, &misplaced, &[]).is_err()
    );

    // Check 2: `previous_validator_set_hash`.
    let mut disowned = base.clone();
    if let Some(record) = disowned.election.as_mut() {
        record.previous_validator_set_hash = Digest32::repeated(0x01);
    }
    assert!(
        light_client::check_transition(&chain, &parameters, &previous, &disowned, &[]).is_err()
    );

    // Check 6: the three counts must agree with the array, and `filled_count`
    // must not exceed the churn cap. "a set that lies about either contradicts
    // itself."
    let mut miscounted = base.clone();
    if let Some(record) = miscounted.election.as_mut() {
        record.filled_count += 1;
        record.retained_count -= 1;
    }
    assert!(
        light_client::check_transition(&chain, &parameters, &previous, &miscounted, &[]).is_err()
    );

    // Check 7: the seed must be the hash of the committed entropy IDs.
    let mut reseeded = base.clone();
    if let Some(record) = reseeded.election.as_mut() {
        record.election_seed = Digest32::repeated(0x01);
    }
    assert!(
        light_client::check_transition(&chain, &parameters, &previous, &reseeded, &[]).is_err()
    );

    // Check 7, the other half: the entropy window's first height.
    let mut shifted = base.clone();
    if let Some(record) = shifted.election.as_mut() {
        record.entropy_first_height += 1;
    }
    assert!(light_client::check_transition(&chain, &parameters, &previous, &shifted, &[]).is_err());

    // Check 9: the contraction floor.
    let mut shrunk = base.clone();
    shrunk.validators.truncate(7);
    if let Some(record) = shrunk.election.as_mut() {
        record.member_count = 7;
        record.retained_count = 7;
        record.filled_count = 0;
    }
    assert!(light_client::check_transition(&chain, &parameters, &previous, &shrunk, &[]).is_err());
}

/// Check 8: the drift is a quantity, reported without a verdict attached.
#[test]
fn check_8_reports_composition_drift_and_judges_nothing() {
    let (entered, left) = light_client::composition_drift(&previous_set(), &derived_successor());
    assert_eq!(left, vec![node(11)]);
    assert_eq!(entered.len(), 1);
    // The function returns lists, not a boolean: item (g) of the closed list
    // says a lawful contraction and a capture by attrition are
    // indistinguishable, so there is no verdict to return.
}

/// Check 10: the parameters come from an authenticated document that lies
/// within the genesis bounds, and every failure mode fails closed.
#[test]
fn check_10_authenticates_the_parameters_and_fails_closed() {
    let chain = zero_chain_id();
    let bounds = permissive_bounds();
    let document = protocol_document_pd0(Pd0Kind::Consensus);
    let trusted = header(24, &Digest32::repeated(1), &Digest32::repeated(1));

    let parameters =
        light_client::authenticate_consensus_parameters(&chain, &trusted, &document, &bounds)
            .expect("the document hashes to the header's commitment");
    assert_eq!(parameters.get(), &consensus_parameters_pd0());

    // A hash mismatch: the header commits to a different document.
    let mut other_header = trusted.clone();
    other_header.consensus_parameters_hash = Digest32::repeated(0x01);
    assert!(
        light_client::authenticate_consensus_parameters(&chain, &other_header, &document, &bounds)
            .is_err()
    );

    // Out of bounds: narrower genesis bounds reject a document the network
    // considers valid, and the client fails closed rather than proceeding.
    let narrow = ElectionBounds {
        validator_max_consecutive_terms_max: 3,
        ..bounds.clone()
    };
    assert!(
        light_client::authenticate_consensus_parameters(&chain, &trusted, &document, &narrow)
            .is_err()
    );

    // Bounds for another chain are refused before anything else is read.
    let foreign = ElectionBounds {
        chain_id: ChainId::from_digest(Digest32::repeated(0x01)),
        ..bounds.clone()
    };
    assert_eq!(
        light_client::authenticate_consensus_parameters(&chain, &trusted, &document, &foreign)
            .unwrap_err(),
        Error::Parameter(ParameterError::ChainIdMismatch)
    );

    // A document of the wrong kind cannot be laundered through this path.
    let wrong_kind = protocol_document_pd0(Pd0Kind::Reward);
    assert!(
        registry::protocol_document_hash(DocumentKind::ConsensusParameters, &chain, &wrong_kind)
            .is_err()
    );
}

/// [REVIEW-017] RF-001: the reward side has the composed entry point its twin
/// has, and a degenerate `RewardBounds` is **rejected** instead of silently
/// disabling the rule it carries.
///
/// The regression this pins down is not "a rule is wrong". It is "a rule is
/// vacuous and nothing says so": with
/// `reward_parameter_change_denominator = 0`, both halves of
/// `new * den <= old * num` and `old * den <= new * num` hold for every pair,
/// so the rate limit accepts any jump and returns `Ok`.
#[test]
fn the_reward_entry_point_validates_the_bounds_before_it_trusts_them() {
    let chain = zero_chain_id();
    let bounds = permissive_reward_bounds();
    let document = protocol_document_pd0(Pd0Kind::Reward);
    let policy_hash = Digest32::parse_prefixed(PD0_REWARD_POLICY_HASH).unwrap();

    let policy =
        light_client::authenticate_reward_policy(&chain, &policy_hash, &document, &bounds, None)
            .expect("PD-0 hashes to its published policy_hash and is within the bounds");
    assert_eq!(policy.get(), &reward_policy_pd0());

    // A hash mismatch: the signed object names a different document.
    assert!(
        light_client::authenticate_reward_policy(
            &chain,
            &Digest32::repeated(0x01),
            &document,
            &bounds,
            None
        )
        .is_err()
    );

    // Bounds for another chain are refused before anything else is read.
    let foreign = RewardBounds {
        chain_id: ChainId::from_digest(Digest32::repeated(0x01)),
        ..bounds.clone()
    };
    assert_eq!(
        light_client::authenticate_reward_policy(&chain, &policy_hash, &document, &foreign, None)
            .unwrap_err(),
        Error::Parameter(ParameterError::ChainIdMismatch)
    );

    // Out of bounds: a narrower ceiling rejects a document the network may well
    // consider valid, and the client fails closed rather than proceeding.
    let narrow = RewardBounds {
        existence_fund_microtokens_per_epoch_max: 0,
        ..bounds.clone()
    };
    assert!(
        light_client::authenticate_reward_policy(&chain, &policy_hash, &document, &narrow, None)
            .is_err()
    );

    // The finding itself: a bounds object whose change ratio is degenerate.
    let degenerate = RewardBounds {
        reward_parameter_change_denominator: 0,
        reward_parameter_change_numerator: 0,
        ..bounds.clone()
    };
    assert!(
        degenerate.validate(&chain).is_err(),
        "the object is invalid"
    );

    // A document that multiplies the existence fund by 100 000 in one step,
    // against an active document that is otherwise identical.
    let base = reward_policy_pd0();
    let active = ActiveRewardPolicyDocument {
        sequence: 1,
        activation_height: 1,
        policy: base,
    };
    let jump = RewardPolicy {
        existence_fund_microtokens_per_epoch: base.existence_fund_microtokens_per_epoch * 100_000,
        ..base
    };
    // Bounded by the genesis ceiling, so the magnitude rules do not object and
    // the rate-of-change rule is the only thing standing between the jump and
    // acceptance.
    assert!(
        jump.existence_fund_microtokens_per_epoch
            <= bounds.existence_fund_microtokens_per_epoch_max
    );

    // The entry point refuses to use the anchor at all.
    let jump_document = reward_document_of(&jump, 200_000, 2);
    let jump_hash =
        registry::protocol_document_hash(DocumentKind::RewardPolicy, &chain, &jump_document)
            .unwrap();
    assert_eq!(
        light_client::authenticate_reward_policy(
            &chain,
            &jump_hash,
            &jump_document,
            &degenerate,
            Some(&active)
        )
        .unwrap_err(),
        Error::Parameter(ParameterError::Bounds {
            rule: "reward_parameter_change_denominator MUST be positive",
        })
    );

    // And the rule itself refuses to be vacuous, for a caller that reached
    // `validate` without going through the entry point.
    assert_eq!(
        jump.validate(&degenerate, 200_000, 2, Some(&active))
            .unwrap_err(),
        Error::Parameter(ParameterError::Bounds {
            rule: "reward_parameter_change_numerator MUST exceed reward_parameter_change_denominator",
        })
    );

    // With a usable anchor the same document is rejected by the rule it was
    // meant to evade, and named by the parameter that moved.
    assert_eq!(
        jump.validate(&bounds, 200_000, 2, Some(&active))
            .unwrap_err(),
        Error::Parameter(ParameterError::ChangeRatio {
            parameter: "existence_fund_microtokens_per_epoch",
        })
    );
}

/// Check 11: membership against `candidate_root`, using the worked example's
/// own tree so that the proof is checked against a published root.
#[test]
fn check_11_verifies_candidate_membership_against_the_committed_root() {
    let key = |byte: u8| AccountKey::from_bytes([byte; 32]);
    let keys = [key(0x02), key(0x04), key(0x05), key(0x06), key(0x08)];
    let leaves: Vec<Digest32> = keys.iter().map(|k| merkle::candidate_leaf(3, k)).collect();
    let levels = TaggedTree::CANDIDATES.levels(&leaves);
    let root = merkle::candidate_root(3, &keys).unwrap();

    // `05` sits at index 2 of the sorted, padded tree.
    let siblings = vec![levels[0][3], levels[1][0], levels[2][1]];
    assert!(light_client::candidate_membership(
        &root,
        3,
        &key(0x05),
        2,
        &siblings
    ));
    // A candidate that was not committed does not verify.
    assert!(!light_client::candidate_membership(
        &root,
        3,
        &key(0x07),
        2,
        &siblings
    ));
    // Nor does the right leaf at the wrong index.
    assert!(!light_client::candidate_membership(
        &root,
        3,
        &key(0x05),
        4,
        &siblings
    ));
}

// --- Steps 1, 3 and 4 -------------------------------------------------------

/// Step 1 and the resolved parameter circularity.
#[test]
fn the_checkpoint_window_comes_from_the_checkpoint_and_must_match_the_chain() {
    // `max_weak_subjectivity_age_ms` is 1 in the PD-0 fixture.
    assert!(light_client::checkpoint_is_fresh(2, 1, 1).unwrap());
    assert!(!light_client::checkpoint_is_fresh(3, 1, 1).unwrap());
    // A checkpoint issued in the future is an error, not a very fresh one.
    assert!(light_client::checkpoint_is_fresh(1, 2, 1).is_err());

    assert!(light_client::checkpoint_agrees_with_chain(1, &parameters()));
    assert!(!light_client::checkpoint_agrees_with_chain(
        2,
        &parameters()
    ));
}

/// Step 3 and step 4's revocation application.
#[test]
fn non_regression_and_the_checkpoints_revocations() {
    let mut tip = TrustedTip::default();
    tip.accept(24, Digest32::repeated(0x01)).unwrap();
    assert!(tip.accept(23, Digest32::repeated(0x01)).is_err());
    assert!(tip.accept(24, Digest32::repeated(0x02)).is_err());

    let active = derived_successor();
    let revocations = vec![(active.validators[0].node_id.clone(), 24u64)];
    // "reject any header at height >= effective_height whose active set
    // contains it — including the set inherited from the checkpoint."
    assert!(
        light_client::check_header_against_revocations(
            &header(24, &Digest32::repeated(1), &Digest32::repeated(1)),
            &active,
            &revocations
        )
        .is_err()
    );
    light_client::check_header_against_revocations(
        &header(23, &Digest32::repeated(1), &Digest32::repeated(1)),
        &active,
        &revocations,
    )
    .expect("below the effective height the header stands");
}

// --- The second closed list -------------------------------------------------

/// The list of what a light client **cannot** establish is carried as data, and
/// nothing in the public surface of this crate offers any of it.
///
/// The eight entries are checked for shape here; the substantive guarantee is
/// the absence of the corresponding functions, which the module documentation
/// of `light_client` states and which a reviewer verifies by reading that
/// module's public items — deliberately few, and each mapped to a numbered
/// check of the first list.
#[test]
fn the_closed_list_of_non_capabilities_is_carried_with_the_code() {
    assert_eq!(CANNOT_ESTABLISH.len(), 8);
    assert!(CANNOT_ESTABLISH[0].contains("candidate_root contains every node"));
    assert!(CANNOT_ESTABLISH[1].contains("met the contribution threshold"));
    assert!(CANNOT_ESTABLISH[2].contains("lowest-ticket members"));
    assert!(CANNOT_ESTABLISH[3].contains("seed was not ground"));
    assert!(CANNOT_ESTABLISH[4].contains("cooldown"));
    assert!(CANNOT_ESTABLISH[5].contains("off-boundary transition was due"));
    assert!(CANNOT_ESTABLISH[6].contains("capture by attrition"));
    assert!(CANNOT_ESTABLISH[7].contains("never being finalized"));
}

/// Item `(a)`, made concrete: a record that omits a genuinely eligible node is
/// internally consistent and passes every Layer-1 check.
///
/// This is the specification's claim, asserted rather than assumed. If a future
/// change made this test fail by *rejecting* the censored transition, that
/// change would be claiming a guarantee the protocol does not give.
#[test]
fn a_censored_candidate_set_is_indistinguishable_to_a_light_client() {
    let chain = zero_chain_id();
    let parameters = parameters();
    let previous = previous_set();

    let mut honest: Vec<CandidateFacts> = (0..12).map(|i| fact(i, true)).collect();
    honest.extend((100..104).map(|i| fact(i, true)));
    let honest_set = election::derive(&chain, &parameters, 6, &previous, &honest, &entropy())
        .unwrap()
        .set;

    // The same boundary with two newcomers' candidacies never finalized.
    let mut censored: Vec<CandidateFacts> = (0..12).map(|i| fact(i, true)).collect();
    censored.extend((100..102).map(|i| fact(i, true)));
    let censored_set = election::derive(&chain, &parameters, 6, &previous, &censored, &entropy())
        .unwrap()
        .set;

    // Both are valid. The client accepts either, and has no basis to prefer one.
    light_client::check_transition(&chain, &parameters, &previous, &honest_set, &[]).unwrap();
    light_client::check_transition(&chain, &parameters, &previous, &censored_set, &[]).unwrap();
    assert_ne!(honest_set.hash().unwrap(), censored_set.hash().unwrap());
}

/// Item `(f)`, made concrete: a removal-only transition is checkable for
/// *shape*, and the list of revocations that would justify it is not something
/// a light client derives — it is passed in from its checkpoint.
#[test]
fn an_off_boundary_transition_is_checked_for_shape_and_not_for_being_due() {
    let previous = previous_set();
    let mut interim = previous.clone();
    interim.validators.truncate(9);
    interim.activation_height = 22;
    if let Some(record) = interim.election.as_mut() {
        // "its `election` record is copied verbatim from the set it replaces,
        // except `member_count`, which MUST equal the new, smaller array
        // length."
        record.member_count = 9;
    }

    // With no revocation list — which is a light client that has none for these
    // identities — the shape checks still pass, and that is all they establish.
    interim
        .check_removal_only_transition(&previous, &[])
        .expect("removal-only shape holds");

    // The header-level check still needs the occasion to be supplied, and no
    // function here derives it from the chain.
    let occasion = transition_occasion(&parameters(), 22, true);
    assert_eq!(occasion, TransitionOccasion::RevocationForced);
}

/// A light client verifies the set hash it retains, which is the one fact that
/// makes every check above bind to the header it walked.
#[test]
fn a_set_is_bound_to_the_header_by_its_hash() {
    let set = derived_successor();
    let hash = set.hash().unwrap();
    let object = JsonObject::parse_canonical(&set.to_json().unwrap().to_jcs()).unwrap();
    assert_eq!(registry::validator_set_hash(&object), hash);
    assert_eq!(ValidatorSet::from_json(&object).unwrap(), set);
}
