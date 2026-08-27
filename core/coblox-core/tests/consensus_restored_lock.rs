//! The lock a restarted validator resumes with.
//!
//! [REVIEW-049] RF-002: a validator that precommitted a block and then died came
//! back with `locked: None`, because `Engine::start` had no way to be told
//! otherwise. It could then prevote a *different* value in a later round of the
//! same height with no polka for it — not a double signature, so the write-ahead
//! log saw nothing, and a violation of the locking rule the
//! quorum-intersection argument of [ADR-018] rests on. With n = 4 and f = 1 one
//! `kill -9` spent the whole fault budget with no adversary present.
//!
//! These tests are about the engine's half of the repair: two new
//! `EngineConfig` fields. The other half — that the write-ahead log already
//! holds what they are filled from — is `coblox-node`'s
//! `wal_lock_restore.rs`.

mod consensus_support;

use coblox_core::consensus::BlockProposal;
use coblox_core::consensus::{
    Action, ConsensusTimeouts, Engine, EngineConfig, Event, Outbound, TimeoutKind, Validity,
    VotePhase, proposer_at, verify_proposal,
};
use coblox_core::error::{ConsensusError, Error};
use coblox_core::hash::{ChainId, Digest32};

use consensus_support::devnet::{devnet_set, harness_header, harness_timeouts};

const HEIGHT: u64 = 5;

fn chain_id() -> ChainId {
    ChainId::from_digest(Digest32::repeated(0x7a))
}

fn previous_block_id() -> Digest32 {
    Digest32::repeated(0x01)
}

/// Starts `val-000` at `HEIGHT` with the given lock, then walks it to round 2
/// with two precommit timeouts.
fn engine_at_round_two(locked: Option<(u64, Digest32)>) -> Engine {
    let (set, _) = devnet_set(4);
    let (mut engine, _actions) = Engine::start(EngineConfig {
        chain_id: chain_id(),
        set,
        validator_id: "val-000".to_owned(),
        timeouts: harness_timeouts(),
        height: HEIGHT,
        previous_block_id: previous_block_id(),
        locked_round: locked.map(|(round, _)| round),
        locked_block_id: locked.map(|(_, block_id)| block_id),
    })
    .expect("the engine must start");

    for round in [0, 1] {
        engine
            .step_event(Event::Timeout {
                kind: TimeoutKind::Precommit,
                height: HEIGHT,
                round,
            })
            .expect("a precommit timeout advances the round");
    }
    assert_eq!(engine.round(), 2, "the walk must reach round 2");
    engine
}

/// A fresh proposal for `(HEIGHT, 2)` — `valid_round` absent — from whoever the
/// round-robin makes the proposer, together with its `block_id`.
fn fresh_proposal_at_round_two() -> (BlockProposal, Digest32) {
    let (set, _) = devnet_set(4);
    let set_hash = set.hash().expect("set hash");
    let proposer = proposer_at(&set, HEIGHT, 2).expect("a proposer for (HEIGHT, 2)");
    let proposer_index = set
        .validators
        .iter()
        .position(|v| v.validator_id == proposer.validator_id)
        .expect("the proposer is a member");
    let header = harness_header(
        &set_hash,
        HEIGHT,
        2,
        u64::try_from(proposer_index).expect("index fits"),
        &previous_block_id(),
    );
    let proposal = BlockProposal {
        height: HEIGHT,
        round: 2,
        valid_round: None,
        header,
        transactions: Vec::new(),
    };
    let block_id = proposal.block_id(&chain_id()).expect("block id");
    (proposal, block_id)
}

fn deliver_proposal(engine: &mut Engine, proposal: BlockProposal) -> Vec<Action> {
    let (set, _) = devnet_set(4);
    let proposer = proposer_at(&set, proposal.height, proposal.round)
        .expect("a proposer")
        .validator_id
        .clone();
    let verified = verify_proposal(&chain_id(), &set, &proposer, proposal, Validity::Valid)
        .expect("the proposal is well formed");
    engine
        .step_event(Event::Message(verified))
        .expect("the engine accepts a verified proposal")
}

fn prevotes_in(actions: &[Action]) -> Vec<Digest32> {
    actions
        .iter()
        .filter_map(|action| match action {
            Action::Broadcast(Outbound::Vote {
                phase: VotePhase::Prevote,
                block_id,
                ..
            }) => Some(*block_id),
            _ => None,
        })
        .collect()
}

#[test]
fn a_restored_lock_refuses_a_different_value_at_a_later_round() {
    let (proposal, block_id_c) = fresh_proposal_at_round_two();
    // Locked at round 1 on a block that is *not* the one now proposed.
    let block_id_b = Digest32::repeated(0xB0);
    assert_ne!(block_id_b, block_id_c);

    let mut engine = engine_at_round_two(Some((1, block_id_b)));
    assert_eq!(engine.locked_round(), Some(1));
    assert_eq!(engine.locked_block_id(), Some(block_id_b));

    let actions = deliver_proposal(&mut engine, proposal);

    assert!(
        prevotes_in(&actions).is_empty(),
        "a node locked on B at round 1 must not prevote C at round 2 with no polka for C"
    );
    assert!(
        !actions.iter().any(|a| matches!(
            a,
            Action::Broadcast(Outbound::Vote {
                phase: VotePhase::Precommit,
                ..
            })
        )),
        "and it must not precommit it either"
    );
    assert_eq!(
        engine.locked_block_id(),
        Some(block_id_b),
        "the lock is unchanged by a proposal it refuses"
    );
}

#[test]
fn the_same_engine_without_the_restored_lock_does_prevote_it() {
    // The twin. Without it, the test above would pass on an engine that refuses
    // everything, which is precisely the failure mode [REVIEW-049] RF-003
    // describes for a gate marked satisfied by a test that observes nothing.
    let (proposal, block_id_c) = fresh_proposal_at_round_two();
    let mut engine = engine_at_round_two(None);
    assert_eq!(engine.locked_round(), None);

    let actions = deliver_proposal(&mut engine, proposal);

    assert_eq!(
        prevotes_in(&actions),
        vec![block_id_c],
        "an unlocked node prevotes the fresh proposal of its round"
    );
}

#[test]
fn a_lock_restored_on_the_proposed_value_does_not_block_it() {
    // Algorithm 1 line 23's second disjunct: `lockedValue_p = v`.
    let (proposal, block_id_c) = fresh_proposal_at_round_two();
    let mut engine = engine_at_round_two(Some((1, block_id_c)));

    let actions = deliver_proposal(&mut engine, proposal);

    assert_eq!(
        prevotes_in(&actions),
        vec![block_id_c],
        "a node locked on the very value proposed prevotes it"
    );
}

#[test]
fn a_half_specified_restored_lock_is_refused_at_construction() {
    let (set, _) = devnet_set(4);
    let error = Engine::start(EngineConfig {
        chain_id: chain_id(),
        set,
        validator_id: "val-000".to_owned(),
        timeouts: harness_timeouts(),
        height: HEIGHT,
        previous_block_id: previous_block_id(),
        locked_round: Some(1),
        locked_block_id: None,
    })
    .expect_err("a round without a block is not a lock");
    assert!(
        matches!(
            error,
            Error::Consensus(ConsensusError::IncompleteRestoredLock {
                has_round: true,
                has_block_id: false
            })
        ),
        "unexpected error: {error:?}"
    );
}

#[test]
fn a_restored_lock_is_readable_through_the_accessors() {
    // The name says only what it checks. Whether the lock is *dropped* on
    // decision is Algorithm 1 line 53 and is covered by the height chain in
    // `consensus_devnet.rs`; asserting it here would need a quorum this test
    // does not build, and a test named for a property it does not exercise is
    // the defect [REVIEW-049] RF-003 and RF-004 were opened for.
    let timeouts: ConsensusTimeouts = harness_timeouts();
    let (set, _) = devnet_set(4);
    let (engine, _) = Engine::start(EngineConfig {
        chain_id: chain_id(),
        set,
        validator_id: "val-000".to_owned(),
        timeouts,
        height: HEIGHT,
        previous_block_id: previous_block_id(),
        locked_round: Some(3),
        locked_block_id: Some(Digest32::repeated(0xB0)),
    })
    .expect("the engine must start");
    assert_eq!(engine.locked_round(), Some(3));
    assert_eq!(engine.height(), HEIGHT);
}
