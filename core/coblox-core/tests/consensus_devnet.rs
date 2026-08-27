//! Four validators, one process, a transport the test controls.
//!
//! The five criteria of [SPEC-025] that need a running network live here: a real
//! chain, safety under an adversarial scheduler, liveness after a proposer that
//! says nothing, equivocation refused, and determinism.

mod consensus_support;

use std::collections::BTreeSet;

use coblox_core::consensus::VotePhase;
use coblox_core::hash::Digest32;

use consensus_support::devnet::{Adversary, Devnet, Wire};

/// The number of adversarial executions the **always-on** safety test performs.
///
/// It is a named constant because `GATE-SAFETY-UNDER-ADVERSARY` requires the
/// transcript to **declare** the number: an unstated number is not evidence. The
/// test prints it rather than leaving it to be transcribed by hand, so the
/// evidence cannot drift away from what ran.
///
/// It is sized so that `cargo test --workspace` in the **debug** profile — the
/// profile CI runs — finishes it in about a minute. Every message in every
/// execution goes through a real Ed25519 verification at the boundary, which is
/// the cost, and which is also the reason the number is not larger: making the
/// sweep cheaper by waving messages through would trade a number for a
/// measurement of nothing. The larger sweep is
/// [`the_extended_adversarial_sweep`], which is `#[ignore]`d and is run
/// explicitly.
const ADVERSARIAL_EXECUTIONS: u64 = 30;

/// The event budget one adversarial execution is given.
const ADVERSARIAL_BUDGET: u64 = 500;

/// The extended sweep's counts. Not run by `cargo test` unless asked for by
/// name; see [`the_extended_adversarial_sweep`].
const EXTENDED_EXECUTIONS: u64 = 500;
const EXTENDED_BUDGET: u64 = 8_000;

// ------------------------------------------------------------ the chain ---

/// **Four validators produce a chain of at least ten finalized blocks**, and
/// every certificate is accepted by the shipped verifier.
#[test]
fn four_validators_finalize_a_chain_of_ten_blocks() {
    let mut devnet = Devnet::start(
        4,
        20_260_827,
        Adversary {
            max_delay_ms: 5,
            ..Adversary::default()
        },
    );
    assert!(
        devnet.verifier_is_the_shipped_one(),
        "the harness must be verifying with the consensus-critical rule"
    );

    let processed = devnet.run_until_chain_length(10, 200_000);

    for node in &devnet.nodes {
        assert!(
            node.chain.len() >= 10,
            "node {} finalized only {} block(s)",
            node.validator_id,
            node.chain.len()
        );
    }
    devnet.assert_chains_agree();
    devnet.assert_no_conflicting_finality();
    let checked = devnet.assert_all_certificates_verify();

    println!("--- GATE-CHAIN-EXISTS ---");
    println!(
        "4 validators, {processed} scheduled events, virtual clock {} ms, \
         {} messages admitted through the boundary, {checked} certificates verified",
        devnet.now_ms(),
        devnet.delivered,
    );
    for line in devnet.transcript(0) {
        println!("  {line}");
    }
    // Heights are consecutive from 1 and every block links to the one before it,
    // which `Devnet::finalize` asserts on every push and this restates as a
    // property of the finished chain.
    for (index, block) in devnet.nodes[0].chain.iter().enumerate() {
        assert_eq!(block.header.height, 1 + u64::try_from(index).unwrap());
    }
}

// ---------------------------------------------------------- the adversary ---

/// **Safety under an adversarial scheduler**: over
/// [`ADVERSARIAL_EXECUTIONS`] executions that reorder, delay, duplicate and
/// partition, no height ever carries two different finalized blocks.
///
/// The partition is drawn per execution and is a **directed** pair set, so it
/// includes the asymmetric cases — a validator that can send but not receive —
/// which a symmetric split would not produce.
#[test]
fn no_two_blocks_are_ever_finalized_at_one_height() {
    adversarial_sweep(ADVERSARIAL_EXECUTIONS, ADVERSARIAL_BUDGET, "always-on");
}

/// The same sweep, larger, run explicitly rather than on every `cargo test`.
///
/// ```text
/// cargo test --release --test consensus_devnet -- --ignored --nocapture
/// ```
///
/// It is `#[ignore]`d and not deleted because the honest shape of this evidence
/// has two halves: what CI re-runs on every commit, and what was run once and
/// recorded. Presenting only the second would claim a guard the pipeline does
/// not have; presenting only the first would understate the search that was
/// actually performed.
#[test]
#[ignore = "the extended sweep: minutes, not seconds. Run it with --ignored."]
fn the_extended_adversarial_sweep() {
    adversarial_sweep(EXTENDED_EXECUTIONS, EXTENDED_BUDGET, "extended");
}

fn adversarial_sweep(execution_count: u64, budget: u64, label: &str) {
    let mut executions = 0u64;
    let mut with_progress = 0u64;
    let mut blocks_finalized = 0u64;
    let mut partitioned_executions = 0u64;

    for seed in 0..execution_count {
        let mut chooser = consensus_support::devnet::Prng::new(seed ^ 0xC0B1_0000);
        // A directed block set over four nodes: each of the twelve ordered pairs
        // of distinct nodes is cut with probability 1/4.
        let mut blocked = BTreeSet::new();
        for from in 0..4usize {
            for to in 0..4usize {
                if from != to && chooser.below(4) == 0 {
                    blocked.insert((from, to));
                }
            }
        }
        if !blocked.is_empty() {
            partitioned_executions += 1;
        }
        let adversary = Adversary {
            max_delay_ms: 1 + chooser.below(400),
            blocked,
            duplicate: chooser.below(2) == 0,
            ..Adversary::default()
        };
        let mut devnet = Devnet::start(4, seed, adversary);
        devnet.run(budget);

        devnet.assert_no_conflicting_finality();
        devnet.assert_chains_agree();
        devnet.assert_all_certificates_verify();

        executions += 1;
        let produced: usize = devnet.nodes.iter().map(|node| node.chain.len()).sum();
        blocks_finalized += u64::try_from(produced).unwrap();
        if produced > 0 {
            with_progress += 1;
        }
    }

    println!("--- GATE-SAFETY-UNDER-ADVERSARY ({label}) ---");
    println!("executions percorse: {executions}");
    println!(
        "  event budget per execution: {budget}; \
         executions with at least one finalized block: {with_progress}; \
         executions with a directed partition: {partitioned_executions}; \
         total finalized blocks across all nodes and executions: {blocks_finalized}"
    );
    assert_eq!(executions, execution_count);
    // A safety suite in which nothing ever finalized would be vacuous: it would
    // prove that a protocol which decides nothing never decides two things. The
    // floor is a ratio of executions rather than a magic number, and it is
    // deliberately low, because an execution spends its budget on whatever the
    // adversary drew and a partition that denies every quorum is a legitimate
    // draw — `a_split_that_denies_both_sides_a_quorum_finalizes_nothing` is that
    // same draw, made deliberately.
    assert!(
        with_progress * 5 >= executions && blocks_finalized > 0,
        "only {with_progress} of {executions} adversarial executions finalized anything \
         ({blocks_finalized} blocks in total); the suite would be proving safety about a \
         protocol that never decided"
    );
}

/// The same claim, stated the other way: a **partition that cannot be survived**
/// stalls rather than forking.
///
/// Two nodes cut off from the other two leaves neither side with three of four,
/// so neither side can finalize. That the run produces nothing is the point.
#[test]
fn a_split_that_denies_both_sides_a_quorum_finalizes_nothing() {
    let mut blocked = BTreeSet::new();
    for from in [0usize, 1] {
        for to in [2usize, 3] {
            blocked.insert((from, to));
            blocked.insert((to, from));
        }
    }
    let mut devnet = Devnet::start(
        4,
        7,
        Adversary {
            max_delay_ms: 3,
            blocked,
            ..Adversary::default()
        },
    );
    devnet.run(4_000);
    devnet.assert_no_conflicting_finality();
    assert!(
        devnet.finalized.is_empty(),
        "a two-two split has no quorum on either side, yet something was finalized: {:?}",
        devnet.finalized.keys().collect::<Vec<_>>()
    );
    println!("--- partition without a quorum ---");
    println!(
        "4000 events, virtual clock {} ms, node rounds {:?}, nothing finalized",
        devnet.now_ms(),
        devnet
            .nodes
            .iter()
            .map(|node| node.engine.round())
            .collect::<Vec<_>>()
    );
}

// ----------------------------------------------------------- the liveness ---

/// **Liveness after a mute proposer**: with the proposer of round 0 saying
/// nothing at all, the height finalizes at a later round.
///
/// This is the case [ADR-018] calls fatal for the one-phase alternative, and it
/// is therefore the case that justifies the whole architecture. The mute node is
/// otherwise entirely correct — it prevotes, it precommits, it is counted in
/// every quorum — it simply never emits its proposal for that round.
#[test]
fn a_height_survives_a_proposer_that_says_nothing() {
    let mut devnet = Devnet::start(4, 99, Adversary::default());
    let silent = devnet.proposer_of(1, 0);
    devnet
        .adversary
        .silenced_proposals
        .insert((silent.clone(), 0));

    let processed = devnet.run_until_chain_length(1, 50_000);

    for node in &devnet.nodes {
        assert!(
            !node.chain.is_empty(),
            "node {} finalized nothing after a mute proposer at round 0",
            node.validator_id
        );
    }
    devnet.assert_chains_agree();
    devnet.assert_no_conflicting_finality();

    let block = &devnet.nodes[0].chain[0];
    assert_eq!(block.header.height, 1);
    assert!(
        block.quorum_certificate.round > 0,
        "the height finalized at round 0, so the proposer was not actually silenced"
    );
    let winner = devnet.proposer_of(1, block.quorum_certificate.round);
    assert_ne!(
        winner, silent,
        "the round that succeeded must have had a different proposer"
    );

    println!("--- GATE-LIVENESS-AFTER-SILENCE ---");
    println!(
        "proposer of (height 1, round 0) is {silent}, silenced. \
         Height 1 finalized at round {} proposed by {winner}, \
         after {processed} scheduled events and {} ms of virtual clock.",
        block.quorum_certificate.round,
        devnet.now_ms(),
    );
    for line in devnet.transcript(0) {
        println!("  {line}");
    }
}

/// The stronger form: the proposers of the first **two** rounds are mute, and
/// the height still finalizes at the third.
#[test]
fn a_height_survives_two_consecutive_mute_proposers() {
    let mut devnet = Devnet::start(4, 1_234, Adversary::default());
    let first = devnet.proposer_of(1, 0);
    let second = devnet.proposer_of(1, 1);
    assert_ne!(first, second);
    devnet.adversary.silenced_proposals.insert((first, 0));
    devnet.adversary.silenced_proposals.insert((second, 1));

    devnet.run_until_chain_length(1, 50_000);
    for node in &devnet.nodes {
        assert!(!node.chain.is_empty(), "node {} stalled", node.validator_id);
    }
    assert!(devnet.nodes[0].chain[0].quorum_certificate.round >= 2);
    devnet.assert_no_conflicting_finality();
}

/// A node that goes entirely silent — proposals, prevotes, precommits — does not
/// stop the other three, because three of four is a quorum.
#[test]
fn one_validator_that_never_speaks_does_not_stop_the_chain() {
    let mut devnet = Devnet::start(4, 4_242, Adversary::default());
    devnet.adversary.silenced_nodes.insert("val-002".to_owned());
    devnet.run_until_chain_length(5, 200_000);
    for node in &devnet.nodes {
        if node.validator_id == "val-002" {
            continue;
        }
        assert!(
            node.chain.len() >= 5,
            "node {} finalized only {}",
            node.validator_id,
            node.chain.len()
        );
    }
    devnet.assert_chains_agree();
    devnet.assert_no_conflicting_finality();
}

// -------------------------------------------------------- the equivocation ---

/// **Equivocation refused**: a validator that emits two different, correctly
/// signed precommits in one round does not cause two blocks to finalize.
///
/// The forged votes are signed with the equivocator's **real key**, so they pass
/// the boundary: this test is about what the engine does with two valid
/// signatures, not about signature checking.
#[test]
fn a_validator_that_precommits_twice_finalizes_only_one_block() {
    let mut devnet = Devnet::start(
        4,
        555,
        Adversary {
            max_delay_ms: 3,
            ..Adversary::default()
        },
    );
    // Let the network reach a round in which precommits are being cast.
    devnet.run(400);

    // Then have val-000 precommit two different, non-existent blocks in every
    // one of the first four rounds of height 1, on top of whatever it really
    // precommitted.
    let mut injected = 0;
    for round in 0..4u64 {
        for byte in [0xAAu8, 0xBB] {
            let vote =
                devnet.sign_vote(0, VotePhase::Precommit, 1, round, Digest32::repeated(byte));
            devnet.inject(&Wire::Vote {
                phase: VotePhase::Precommit,
                vote,
            });
            injected += 1;
        }
    }
    devnet.run_until_chain_length(3, 200_000);

    devnet.assert_no_conflicting_finality();
    devnet.assert_chains_agree();
    devnet.assert_all_certificates_verify();

    // The injected IDs are not block IDs of anything, so nothing may carry them.
    for ids in devnet.finalized.values() {
        assert!(!ids.contains(&Digest32::repeated(0xAA)));
        assert!(!ids.contains(&Digest32::repeated(0xBB)));
    }
    for node in &devnet.nodes {
        for block in &node.chain {
            let mut seen = BTreeSet::new();
            for signature in &block.quorum_certificate.signatures {
                assert!(
                    seen.insert(signature.validator_id.clone()),
                    "a certificate counted {} twice",
                    signature.validator_id
                );
            }
        }
    }
    println!("--- equivocation ---");
    println!(
        "{injected} conflicting precommits injected under val-000's real key across 4 rounds; \
         chain length {}, no height with two block IDs, no certificate with a repeated signer",
        devnet.nodes[0].chain.len()
    );
}

/// The same, at the tally: two conflicting precommits from one validator are
/// counted once, so a set of two honest validators plus one equivocator never
/// reaches a quorum.
///
/// Four validators, three of which are silenced entirely. The one that speaks
/// precommits everything it can. Nothing finalizes, because one power is not a
/// quorum however many messages it sends.
#[test]
fn one_validator_cannot_reach_a_quorum_by_voting_many_times() {
    let mut devnet = Devnet::start(4, 31_337, Adversary::default());
    for index in 1..4 {
        devnet
            .adversary
            .silenced_nodes
            .insert(format!("val-{index:03}"));
    }
    devnet.run(600);
    for round in 0..8u64 {
        for byte in 0u8..8 {
            let vote =
                devnet.sign_vote(0, VotePhase::Precommit, 1, round, Digest32::repeated(byte));
            devnet.inject(&Wire::Vote {
                phase: VotePhase::Precommit,
                vote,
            });
            let prevote =
                devnet.sign_vote(0, VotePhase::Prevote, 1, round, Digest32::repeated(byte));
            devnet.inject(&Wire::Vote {
                phase: VotePhase::Prevote,
                vote: prevote,
            });
        }
    }
    devnet.run(4_000);
    assert!(devnet.finalized.is_empty());
    devnet.assert_no_conflicting_finality();
}

// --------------------------------------------------------- the determinism ---

/// **Determinism**: the same seed and the same adversary produce the same chain,
/// byte for byte, including every certificate.
#[test]
fn the_same_schedule_produces_the_same_chain_byte_for_byte() {
    let adversary = || Adversary {
        max_delay_ms: 40,
        duplicate: true,
        ..Adversary::default()
    };
    let mut first = Devnet::start(4, 8_675_309, adversary());
    let mut second = Devnet::start(4, 8_675_309, adversary());
    first.run_until_chain_length(6, 200_000);
    second.run_until_chain_length(6, 200_000);

    for index in 0..4 {
        let left = first.chain_bytes(index);
        let right = second.chain_bytes(index);
        assert!(!left.is_empty(), "node {index} produced no chain");
        assert_eq!(
            left, right,
            "node {index} produced different chain bytes on two runs of one schedule"
        );
    }
    println!("--- determinism ---");
    println!(
        "two runs of seed 8675309: {} chain bytes per node, identical on all four nodes",
        first.chain_bytes(0).len()
    );

    // And a different seed is a different schedule, so the assertion above is
    // not true of everything.
    let mut other = Devnet::start(4, 8_675_310, adversary());
    other.run_until_chain_length(6, 200_000);
    assert_ne!(
        first.now_ms(),
        other.now_ms(),
        "two seeds produced the identical virtual timeline, so the seed is not being used"
    );
}
