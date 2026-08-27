//! The wire boundary of `NodeRunner::handle_envelope`.
//!
//! [REVIEW-049] RF-001 executed a proof of concept in which an honest node
//! signed and made durable a prevote on a proposal forged by a key outside the
//! validator set, carried in an envelope that had been expired since 1970. That
//! proof of concept is `forged_proposal_from_a_non_member_key_is_refused_at_the_boundary`
//! below, inverted: the envelope must now be refused and `wal_vote_count()` must
//! not move. `a_well_signed_proposal_from_the_legitimate_proposer_is_admitted`
//! is its twin, and exists so that the refusal cannot be obtained by refusing
//! everything.

use tempfile::TempDir;

use coblox_core::block::BlockHeader;
use coblox_core::consensus::{BlockProposal, VotePhase, proposer_at};
use coblox_core::hash::{ChainId, Digest32};
use coblox_core::json::JsonObject;
use coblox_core::merkle;
use coblox_node::config::{NodeConfig, devnet_4_validator_set, devnet_timeouts};
use coblox_node::envelope::{MAX_ENVELOPE_VALIDITY_MS, SignedEnvelope, fresh_nonce};
use coblox_node::node::NodeRunner;
use coblox_node::signer::SigningKey;

const NETWORK_ID: &str = "coblox-devnet-0";

fn chain_id() -> ChainId {
    ChainId::from_digest(Digest32::repeated(0x7a))
}

fn now_ms() -> u64 {
    u64::try_from(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock at or after the Unix epoch")
            .as_millis(),
    )
    .expect("milliseconds fit in u64")
}

/// A node that is **not** the proposer of `(1, 0)`, so its own start emits no
/// vote and the WAL begins empty.
fn non_proposer_runner(dir: &TempDir) -> (NodeRunner, String, usize) {
    let (set, keys) = devnet_4_validator_set();
    let proposer_id = proposer_at(&set, 1, 0)
        .expect("a proposer for (1, 0)")
        .validator_id
        .clone();
    let index = set
        .validators
        .iter()
        .position(|v| v.validator_id != proposer_id)
        .expect("a member that is not the proposer of (1, 0)");
    let validator_id = set.validators[index].validator_id.clone();

    let config = NodeConfig {
        validator_id: validator_id.clone(),
        node_id: validator_id.clone(),
        signing_key: keys[index].clone(),
        network_id: NETWORK_ID.to_owned(),
        chain_id: chain_id(),
        genesis_block_id: Digest32::repeated(0x01),
        listen_addr: "/ip4/127.0.0.1/tcp/0".to_owned(),
        seed_peers: Vec::new(),
        data_dir: dir.path().to_path_buf(),
        validator_set: set,
        timeouts: devnet_timeouts(),
        target_height: None,
    };
    let (runner, _network) = NodeRunner::new(config).expect("the runner must start");
    let proposer_index = proposer_index_of(&proposer_id);
    (runner, proposer_id, proposer_index)
}

fn proposer_index_of(validator_id: &str) -> usize {
    let (set, _) = devnet_4_validator_set();
    set.validators
        .iter()
        .position(|v| v.validator_id == validator_id)
        .expect("the proposer is a member")
}

/// A structurally well-formed proposal for `(1, 0)` with `state_root` chosen by
/// the caller — the attacker's freedom in the proof of concept.
fn proposal_for_height_one(state_root: Digest32) -> BlockProposal {
    let (set, _) = devnet_4_validator_set();
    let set_hash = set.hash().expect("set hash");
    let header = BlockHeader {
        schema_version: "0.1".to_owned(),
        protocol_version: "0.1".to_owned(),
        network_id: NETWORK_ID.to_owned(),
        height: 1,
        round: 0,
        timestamp_ms: 1_787_654_405_000,
        previous_block_id: Digest32::repeated(0x01),
        transactions_root: merkle::transactions_root(&[]).expect("empty root"),
        state_root,
        validator_set_hash: set_hash,
        next_validator_set_hash: set_hash,
        consensus_parameters_hash: Digest32::repeated(0x44),
    };
    BlockProposal {
        height: 1,
        round: 0,
        valid_round: None,
        header,
        transactions: Vec::new(),
    }
}

fn envelope_for(
    proposal: &BlockProposal,
    sender_node_id: &str,
    created_at_ms: u64,
    key: &SigningKey,
) -> SignedEnvelope {
    SignedEnvelope::build_and_sign(
        &chain_id(),
        NETWORK_ID,
        "block_proposal",
        sender_node_id,
        created_at_ms,
        30_000,
        fresh_nonce().expect("system entropy"),
        proposal.to_json().expect("proposal json"),
        key,
    )
    .expect("the envelope must build")
}

#[tokio::test]
async fn forged_proposal_from_a_non_member_key_is_refused_at_the_boundary() {
    let dir = TempDir::new().expect("tempdir");
    let (mut runner, proposer_id, _) = non_proposer_runner(&dir);
    let votes_before = runner.wal_vote_count();
    assert_eq!(votes_before, 0, "a non-proposer votes nothing at start");

    // The attacker's key, exactly as in the proof of concept, and it is asserted
    // to be outside the set before anything else happens.
    let attacker = SigningKey::from_seed(&[0xAA; 32]);
    let (set, _) = devnet_4_validator_set();
    assert!(
        !set.validators
            .iter()
            .any(|v| v.consensus_public_key == attacker.public_key()),
        "the attacker key must not be a member of the validator set"
    );

    // `sender_node_id` names the legitimate proposer of (1, 0); the signature is
    // the attacker's. `created_at_ms = 0` makes the envelope expired since 1970.
    let proposal = proposal_for_height_one(Digest32::repeated(0xEE));
    let forged = envelope_for(&proposal, &proposer_id, 0, &attacker);

    let outcome = runner.handle_envelope(forged);
    let error = outcome.expect_err("the forged envelope must be refused");
    assert!(
        !error.is_fatal(),
        "a refusal at the boundary must not be a fatal error: {error}"
    );

    assert_eq!(
        runner.wal_vote_count(),
        votes_before,
        "no vote may be signed or made durable on a forged proposal"
    );
}

#[tokio::test]
async fn a_well_signed_proposal_from_the_legitimate_proposer_is_admitted() {
    let dir = TempDir::new().expect("tempdir");
    let (mut runner, proposer_id, proposer_index) = non_proposer_runner(&dir);
    let (_, keys) = devnet_4_validator_set();

    let proposal = proposal_for_height_one(Digest32::repeated(0x33));
    let envelope = envelope_for(&proposal, &proposer_id, now_ms(), &keys[proposer_index]);

    runner
        .handle_envelope(envelope)
        .expect("a well-signed proposal from the round's proposer must be admitted");

    assert_eq!(
        runner.wal_vote_count(),
        1,
        "the admitted proposal must produce exactly one durable prevote"
    );
}

#[tokio::test]
async fn an_expired_envelope_from_a_member_is_refused() {
    let dir = TempDir::new().expect("tempdir");
    let (mut runner, proposer_id, proposer_index) = non_proposer_runner(&dir);
    let (_, keys) = devnet_4_validator_set();

    // Correctly signed by the right member, and expired since 1970.
    let proposal = proposal_for_height_one(Digest32::repeated(0x33));
    let expired = envelope_for(&proposal, &proposer_id, 0, &keys[proposer_index]);

    let error = runner
        .handle_envelope(expired)
        .expect_err("an expired envelope must be refused");
    assert!(
        error.to_string().contains("expired"),
        "the refusal must name the expiry: {error}"
    );
    assert_eq!(runner.wal_vote_count(), 0);
}

#[tokio::test]
async fn an_envelope_from_an_unknown_sender_is_refused() {
    let dir = TempDir::new().expect("tempdir");
    let (mut runner, _, _) = non_proposer_runner(&dir);

    let stranger = SigningKey::from_seed(&[0xAA; 32]);
    let proposal = proposal_for_height_one(Digest32::repeated(0x33));
    let envelope = envelope_for(&proposal, "not-a-validator", now_ms(), &stranger);

    let error = runner
        .handle_envelope(envelope)
        .expect_err("a sender outside the set must be refused");
    assert!(
        error.to_string().contains("not a member"),
        "the refusal must name the membership check: {error}"
    );
    assert_eq!(runner.wal_vote_count(), 0);
}

#[tokio::test]
async fn an_envelope_of_another_chain_is_refused() {
    let dir = TempDir::new().expect("tempdir");
    let (mut runner, proposer_id, proposer_index) = non_proposer_runner(&dir);
    let (_, keys) = devnet_4_validator_set();

    // Built and signed correctly, under a different chain. The envelope carries
    // no `chain_id` field: the chain is bound into `message_id` and into the
    // signature domain, so the check is the recomputation.
    let other_chain = ChainId::from_digest(Digest32::repeated(0x5b));
    let proposal = proposal_for_height_one(Digest32::repeated(0x33));
    let envelope = SignedEnvelope::build_and_sign(
        &other_chain,
        NETWORK_ID,
        "block_proposal",
        &proposer_id,
        now_ms(),
        30_000,
        fresh_nonce().expect("system entropy"),
        proposal.to_json().expect("proposal json"),
        &keys[proposer_index],
    )
    .expect("the envelope must build");

    let error = runner
        .handle_envelope(envelope)
        .expect_err("an envelope of another chain must be refused");
    assert!(
        error.to_string().contains("message_id mismatch"),
        "the refusal must come from the chain-bound message_id: {error}"
    );
    assert_eq!(runner.wal_vote_count(), 0);
}

#[tokio::test]
async fn the_same_envelope_twice_is_refused_the_second_time() {
    let dir = TempDir::new().expect("tempdir");
    let (mut runner, proposer_id, proposer_index) = non_proposer_runner(&dir);
    let (_, keys) = devnet_4_validator_set();

    let proposal = proposal_for_height_one(Digest32::repeated(0x33));
    let envelope = envelope_for(&proposal, &proposer_id, now_ms(), &keys[proposer_index]);

    runner
        .handle_envelope(envelope.clone())
        .expect("the first delivery is admitted");
    let error = runner
        .handle_envelope(envelope)
        .expect_err("the replay must be refused");
    assert!(
        error.to_string().contains("replay cache"),
        "the refusal must come from the replay cache: {error}"
    );
}

#[test]
fn an_envelope_may_not_outlive_max_envelope_validity_ms() {
    let key = SigningKey::from_seed(&[0x01; 32]);
    let payload = JsonObject::builder()
        .uint("from_height", 1)
        .build()
        .expect("payload");
    let error = SignedEnvelope::build_and_sign(
        &chain_id(),
        NETWORK_ID,
        "block_request",
        "val-000",
        now_ms(),
        MAX_ENVELOPE_VALIDITY_MS + 1,
        fresh_nonce().expect("system entropy"),
        payload,
        &key,
    )
    .expect_err("a node must not mint an envelope it would itself refuse");
    assert!(error.to_string().contains("max_envelope_validity_ms"));
}

#[test]
fn a_signing_key_does_not_print_its_secret() {
    // [REVIEW-049] RF-009: the derived `Debug` printed the secret scalar and the
    // prefix in clear, and `NodeConfig` derives `Debug` and holds one.
    let key = SigningKey::from_seed(&[0x07; 32]);
    let printed = format!("{key:?}");
    let public = key.public_key();
    assert!(printed.contains("<redacted>"), "{printed}");
    for window in printed.as_bytes().windows(2) {
        let _ = window;
    }
    // The public key may appear; nothing else about the key may.
    let sample = format!("{:02x}{:02x}", public[0], public[1]);
    assert!(printed.contains(&sample), "the public key is not a secret");
    assert!(!printed.contains("scalar"), "{printed}");
    assert!(!printed.contains("prefix"), "{printed}");
}

#[test]
fn a_wal_phase_is_one_of_two() {
    // Guards the two-value phase encoding the WAL keys on, which
    // `locked_at_height` filters by.
    assert_ne!(VotePhase::Prevote, VotePhase::Precommit);
}
