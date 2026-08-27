//! A four-validator devnet in one process, on a transport the test controls.
//!
//! # Why in memory, and what that buys
//!
//! [SPEC-025] cuts the network out on purpose. The cases that decide whether a
//! BFT protocol is correct — a partition, a proposer that says nothing, a
//! validator that votes twice — are three lines here and days of work on a real
//! network, and on a real network they do not **reproduce**. Every execution this
//! harness performs is a total function of its seed and its schedule, so a
//! failure is a seed and not a story.
//!
//! # The clock is a number, and that is the point
//!
//! There is no thread, no sleep and no wall clock anywhere below. Time is
//! `now_ms`, a `u64` that advances only to the timestamp of the next scheduled
//! event, and the engine never sees it: the engine asks for a timeout with
//! [`Action::ScheduleTimeout`] and is told about it with [`Event::Timeout`]. That
//! is the same property `GATE-NO-IO` asks of the engine, seen from the caller's
//! side — the harness could not inject a real clock even if it wanted to.
//!
//! # What is deliberately *not* modelled
//!
//! * **Byzantine engines.** Every node here runs the honest state machine. The
//!   adversary is the *scheduler* — it reorders, delays, duplicates and
//!   partitions — plus the two explicitly injected misbehaviours, a silenced
//!   proposer and a forged double precommit. A node whose *rules* are wrong is
//!   not exercised, and the safety criterion of [SPEC-025] is about a correct
//!   majority under an adversarial network, which is what this is.
//! * **Cross-height buffering inside the engine.** The engine drops messages for
//!   a height it is not executing; this harness re-delivers them when the node
//!   advances, which is what `wire.md`'s ledger sync does in production. Without
//!   it a node that fell one height behind would never catch up, and the test
//!   would be measuring the harness.

#![allow(dead_code)]

use std::collections::{BTreeMap, BTreeSet, BinaryHeap};

use coblox_core::block::BlockHeader;
use coblox_core::consensus::{
    Action, ConsensusTimeouts, Engine, EngineConfig, Event, FinalizedBlock, Outbound, SignedVote,
    Validity, VotePhase, proposer_at, verify_proposal, verify_vote,
};
use coblox_core::hash::{ChainId, Digest32};
use coblox_core::json::JsonObject;
use coblox_core::merkle::transactions_root;
use coblox_core::registry::{block_prevote_preimage, block_vote_preimage, tx_id};
use coblox_core::validator_set::{ValidatorEntry, ValidatorSet};
use coblox_core::{ConsensusVerifier, SignatureVerifier};

use super::ed25519_signer::SigningKey;

/// The fixture network's name. Nothing derives a chain ID from it here; the
/// devnet works under an explicit `chain_id` so that no test depends on the
/// genesis derivation, which is another spec's subject.
pub const NETWORK_ID: &str = "coblox-devnet-0";

/// A message on the wire, after the sender signed whatever needed signing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Wire {
    /// A proposal, attributed to its sender by the transport.
    Proposal {
        from: String,
        proposal: Box<coblox_core::consensus::BlockProposal>,
    },
    /// A signed prevote or precommit.
    Vote { phase: VotePhase, vote: SignedVote },
}

impl Wire {
    fn height(&self) -> u64 {
        match self {
            Self::Proposal { proposal, .. } => proposal.height,
            Self::Vote { vote, .. } => vote.height,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Delivery {
    Message {
        to: usize,
        wire: Wire,
    },
    Timer {
        to: usize,
        kind: coblox_core::consensus::TimeoutKind,
        height: u64,
        round: u64,
    },
    Value {
        to: usize,
        height: u64,
        round: u64,
    },
}

/// One scheduled event. `seq` breaks ties, so the heap order is total and the
/// whole execution is a function of the inputs.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Scheduled {
    at: u64,
    seq: u64,
    delivery: Delivery,
}

impl Ord for Scheduled {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        // Reversed, because `BinaryHeap` is a max-heap and the earliest event
        // must come out first.
        other.at.cmp(&self.at).then(other.seq.cmp(&self.seq))
    }
}

impl PartialOrd for Scheduled {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// One validator.
#[derive(Debug)]
pub struct Node {
    pub engine: Engine,
    pub key: SigningKey,
    pub validator_id: String,
    /// Every block this node finalized, in order.
    pub chain: Vec<FinalizedBlock>,
    /// Messages received for a height above the one being executed, kept until
    /// the node reaches that height.
    held: Vec<Wire>,
}

/// A tiny deterministic generator, so an execution is a function of its seed.
///
/// `xorshift64*`. It is not a cryptographic generator and nothing here needs one:
/// it chooses message delays.
#[derive(Debug, Clone, Copy)]
pub struct Prng(u64);

impl Prng {
    #[must_use]
    pub const fn new(seed: u64) -> Self {
        // A zero state is a fixed point of xorshift, so it is moved off zero.
        Self(seed ^ 0x9E37_79B9_7F4A_7C15)
    }

    pub fn next_u64(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x >> 12;
        x ^= x << 25;
        x ^= x >> 27;
        self.0 = x;
        x.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    /// A value in `0..bound`.
    pub fn below(&mut self, bound: u64) -> u64 {
        assert!(bound > 0, "an empty range has no member");
        self.next_u64() % bound
    }
}

/// How the scheduler is allowed to misbehave.
#[derive(Debug, Clone, Default)]
pub struct Adversary {
    /// Delivery delay is drawn from `1..=max_delay_ms`.
    pub max_delay_ms: u64,
    /// Directed pairs `(from, to)` whose messages are dropped entirely.
    pub blocked: BTreeSet<(usize, usize)>,
    /// `(validator_id, round)` pairs whose **proposal** never leaves the node.
    ///
    /// This is the mute proposer: the node is otherwise fully correct, it simply
    /// does not speak in that round. Modelling it as a dropped message rather
    /// than as a modified engine keeps the engine honest.
    pub silenced_proposals: BTreeSet<(String, u64)>,
    /// Validators whose every outbound message is dropped.
    pub silenced_nodes: BTreeSet<String>,
    /// Deliver every message twice. A duplicate must change nothing.
    pub duplicate: bool,
}

/// The devnet.
#[derive(Debug)]
pub struct Devnet {
    pub chain_id: ChainId,
    pub set: ValidatorSet,
    pub nodes: Vec<Node>,
    pub adversary: Adversary,
    pub verifier: ConsensusVerifier,
    /// Every `(height, block_id)` any node finalized. The safety criterion is a
    /// statement about this map.
    pub finalized: BTreeMap<u64, BTreeSet<Digest32>>,
    /// Extra precommits injected by a test, to model equivocation.
    genesis_block_id: Digest32,
    queue: BinaryHeap<Scheduled>,
    now_ms: u64,
    seq: u64,
    prng: Prng,
    /// Messages the harness has admitted, for a test to inspect.
    pub delivered: u64,
    /// Messages the boundary refused, for a test that needs to observe a
    /// rejection rather than infer it from an absence.
    pub rejected: u64,
}

/// The default timeouts of the harness. Round 0 is deliberately short and the
/// increment deliberately large, so that a test that needs several rounds does
/// not need a large virtual clock and one that needs a round to fail does not
/// wait long.
#[must_use]
pub fn harness_timeouts() -> ConsensusTimeouts {
    ConsensusTimeouts {
        propose_ms: 100,
        prevote_ms: 100,
        precommit_ms: 100,
        round_increment_ms: 50,
    }
}

/// Builds the four-validator set of the harness, with real consensus keys.
///
/// `voting_power` is 1 for every member, which is what an elected Coblox set
/// looks like (`ValidatorSet::check_elected_shape` requires exactly that), so a
/// quorum here is 3 of 4: `3 * 3 > 4 * 2`, and `2 * 3 > 4 * 2` is false.
#[must_use]
pub fn devnet_set(count: usize) -> (ValidatorSet, Vec<SigningKey>) {
    let mut validators = Vec::with_capacity(count);
    let mut keys = Vec::with_capacity(count);
    for index in 0..count {
        let seed = [u8::try_from(index).expect("fewer than 256 validators") + 1; 32];
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
    set.check_structure()
        .expect("the harness set must be structurally valid");
    (set, keys)
}

/// The header the harness proposes for `(height, round)` on top of
/// `previous_block_id`, as seen by `proposer_index`.
///
/// `timestamp_ms` carries the proposer's index **on purpose**. Every node builds
/// headers the same way, so without it two different proposers at the same
/// height would produce the same `block_id` and the safety criterion would be
/// satisfied by arithmetic rather than by the protocol: there would be no two
/// different blocks for a height to finalize. With it, every `(height, round,
/// proposer)` triple is a distinct block, which is also what a real network looks
/// like.
#[must_use]
pub fn harness_header(
    set_hash: &Digest32,
    height: u64,
    round: u64,
    proposer_index: u64,
    previous_block_id: &Digest32,
) -> BlockHeader {
    BlockHeader {
        schema_version: "0.1".to_owned(),
        protocol_version: "0.1".to_owned(),
        network_id: NETWORK_ID.to_owned(),
        height,
        round,
        timestamp_ms: 1_787_654_400_000 + height * 5_000 + round * 1_000 + proposer_index,
        previous_block_id: *previous_block_id,
        transactions_root: transactions_root(&[]).expect("the empty transactions root"),
        state_root: Digest32::repeated(0x33),
        validator_set_hash: *set_hash,
        next_validator_set_hash: *set_hash,
        consensus_parameters_hash: Digest32::repeated(0x44),
    }
}

/// A transaction object of the shape `ledger.md#unsigned-transaction-and-authorization`
/// publishes, carrying an `authorization` the ID rule has to remove.
///
/// The `authorization` is not decoration. `tx_id` is taken over the object
/// **with `authorization` removed**, so a payload whose members carried none
/// would let a boundary that forgot the removal pass this test by accident.
#[must_use]
pub fn harness_transaction(pay_to: &str, amount_microtokens: u64) -> JsonObject {
    JsonObject::builder()
        .object(
            "authorization",
            JsonObject::builder()
                .str("public_key", "11qYAYdk9J0L5Z-6hB4qMTPBSAE5nK1G0IU2n6z1V9g")
                .bytes("signature", &[0u8; 64])
                .build()
                .expect("the harness authorization is well formed"),
        )
        .object(
            "body",
            JsonObject::builder()
                .uint("amount_microtokens", amount_microtokens)
                .str("pay_to", pay_to)
                .build()
                .expect("the harness body is well formed"),
        )
        .uint("created_at_ms", 1_787_654_505_000)
        .uint("expires_at_ms", 1_787_654_805_000)
        .str("kind", "fund_app")
        .str("network_id", NETWORK_ID)
        .str("schema_version", "0.1")
        .build()
        .expect("the harness transaction is well formed")
}

/// The `transactions_root` of a payload, computed the way `ledger.md` defines it.
///
/// This is deliberately a **second** implementation of the rule the boundary
/// applies: it calls the two published primitives directly, so a test that uses
/// it is not asking `verify_proposal` whether it agrees with itself.
#[must_use]
pub fn harness_transactions_root(chain_id: &ChainId, transactions: &[JsonObject]) -> Digest32 {
    let ids: Vec<Digest32> = transactions
        .iter()
        .map(|transaction| {
            let mut unsigned = JsonObject::new();
            for (key, value) in transaction.iter() {
                if key != "authorization" {
                    unsigned
                        .insert(key, value.clone())
                        .expect("a copied field is a valid field");
                }
            }
            tx_id(chain_id, &unsigned)
        })
        .collect();
    transactions_root(&ids).expect("the harness payload is within the tree limit")
}

impl Devnet {
    /// Starts a devnet of `count` validators at `height`.
    pub fn start(count: usize, seed: u64, adversary: Adversary) -> Self {
        let chain_id = ChainId::from_digest(Digest32::repeated(0x7a));
        let (set, keys) = devnet_set(count);
        let genesis_block_id = Digest32::repeated(0x01);
        let mut devnet = Self {
            chain_id,
            set: set.clone(),
            nodes: Vec::with_capacity(count),
            adversary,
            verifier: ConsensusVerifier,
            finalized: BTreeMap::new(),
            genesis_block_id,
            queue: BinaryHeap::new(),
            now_ms: 0,
            seq: 0,
            prng: Prng::new(seed),
            delivered: 0,
            rejected: 0,
        };
        let mut pending = Vec::new();
        for (index, key) in keys.into_iter().enumerate() {
            let validator_id = format!("val-{index:03}");
            let (engine, actions) = Engine::start(EngineConfig {
                chain_id,
                set: set.clone(),
                validator_id: validator_id.clone(),
                timeouts: harness_timeouts(),
                height: 1,
                previous_block_id: genesis_block_id,
                locked_round: None,
                locked_block_id: None,
            })
            .expect("the harness engine must start");
            devnet.nodes.push(Node {
                engine,
                key,
                validator_id,
                chain: Vec::new(),
                held: Vec::new(),
            });
            pending.push((index, actions));
        }
        for (index, actions) in pending {
            devnet.apply(index, actions);
        }
        devnet
    }

    /// The virtual clock.
    #[must_use]
    pub const fn now_ms(&self) -> u64 {
        self.now_ms
    }

    /// Runs until every node has finalized `target` blocks, or until `budget`
    /// events have been processed.
    ///
    /// Returns the number of events processed. It is a budget and not a timeout:
    /// nothing here waits, so exhausting it means the protocol stopped making
    /// progress, which is a finding and not a flake.
    pub fn run_until_chain_length(&mut self, target: usize, budget: u64) -> u64 {
        let mut processed = 0;
        while processed < budget {
            if self.nodes.iter().all(|node| node.chain.len() >= target) {
                return processed;
            }
            if !self.tick() {
                return processed;
            }
            processed += 1;
        }
        processed
    }

    /// Runs `budget` events, or until the queue drains.
    pub fn run(&mut self, budget: u64) -> u64 {
        let mut processed = 0;
        while processed < budget && self.tick() {
            processed += 1;
        }
        processed
    }

    /// Processes the earliest scheduled event. Returns `false` when the queue is
    /// empty, which for a live protocol never happens: a round always has a
    /// timeout outstanding.
    pub fn tick(&mut self) -> bool {
        let Some(event) = self.queue.pop() else {
            return false;
        };
        self.now_ms = self.now_ms.max(event.at);
        match event.delivery {
            Delivery::Message { to, wire } => self.deliver(to, wire),
            Delivery::Timer {
                to,
                kind,
                height,
                round,
            } => {
                let actions = self.nodes[to]
                    .engine
                    .step_event(Event::Timeout {
                        kind,
                        height,
                        round,
                    })
                    .expect("a timeout is never a malformed event");
                self.apply(to, actions);
            }
            Delivery::Value { to, height, round } => {
                let previous = self.nodes[to]
                    .chain
                    .last()
                    .map_or(self.genesis_block_id, |block| {
                        block.quorum_certificate.block_id
                    });
                let header = harness_header(
                    &self.set.hash().expect("set hash"),
                    height,
                    round,
                    u64::try_from(to).expect("fewer than 2^64 nodes"),
                    &previous,
                );
                // A value offered for a round the node has already left is
                // ordinary: the request was made, the round timed out, the reply
                // arrived late. The engine rejects it and the harness drops it.
                if let Ok(actions) = self.nodes[to].engine.step_event(Event::Value {
                    height,
                    round,
                    header: Box::new(header),
                    transactions: Vec::new(),
                }) {
                    self.apply(to, actions);
                }
            }
        }
        true
    }

    /// Hands one wire message to one node, through the real boundary.
    ///
    /// Every message goes through [`verify_proposal`] or [`verify_vote`], with
    /// the shipped [`ConsensusVerifier`]. Nothing reaches an engine without its
    /// signature having been checked, which is the property the whole test rests
    /// on: a chain built from messages the harness waved through would say
    /// nothing about certificates.
    fn deliver(&mut self, to: usize, wire: Wire) {
        if wire.height() > self.nodes[to].engine.height() {
            self.nodes[to].held.push(wire);
            return;
        }
        if wire.height() < self.nodes[to].engine.height() {
            return;
        }
        let verified = match &wire {
            Wire::Proposal { from, proposal } => verify_proposal(
                &self.chain_id,
                &self.set,
                from,
                (**proposal).clone(),
                Validity::Valid,
            ),
            Wire::Vote { phase, vote } => verify_vote(
                &self.chain_id,
                &self.set,
                *phase,
                vote.clone(),
                &self.verifier,
            ),
        };
        let Ok(verified) = verified else {
            // A message the boundary rejects never reaches the engine. That is
            // what a conformant node does, and it is where the equivocation test
            // observes a forged signature being refused.
            self.rejected += 1;
            return;
        };
        self.delivered += 1;
        let actions = self.nodes[to]
            .engine
            .step_event(Event::Message(verified))
            .expect("a verified message is never a malformed event");
        self.apply(to, actions);
    }

    /// Carries out the engine's actions, without ever performing one inside it.
    fn apply(&mut self, from: usize, actions: Vec<Action>) {
        for action in actions {
            match action {
                Action::ScheduleTimeout {
                    kind,
                    height,
                    round,
                    delay_ms,
                } => {
                    let at = self.now_ms + delay_ms;
                    self.push(
                        at,
                        Delivery::Timer {
                            to: from,
                            kind,
                            height,
                            round,
                        },
                    );
                }
                Action::RequestValue { height, round } => {
                    // Same instant, next sequence number: building a block is
                    // local work, and giving it a delay would only make the
                    // clock arithmetic of a test harder to read.
                    self.push(
                        self.now_ms,
                        Delivery::Value {
                            to: from,
                            height,
                            round,
                        },
                    );
                }
                Action::Broadcast(outbound) => self.broadcast(from, outbound),
                Action::Finalize(block) => self.finalize(from, *block),
            }
        }
        self.release_held(from);
    }

    /// Signs a vote and puts the message on the wire to every node, including
    /// the sender.
    fn broadcast(&mut self, from: usize, outbound: Outbound) {
        let sender = self.nodes[from].validator_id.clone();
        if self.adversary.silenced_nodes.contains(&sender) {
            return;
        }
        let wire = match outbound {
            Outbound::Proposal(proposal) => {
                if self
                    .adversary
                    .silenced_proposals
                    .contains(&(sender.clone(), proposal.round))
                {
                    return;
                }
                Wire::Proposal {
                    from: sender,
                    proposal,
                }
            }
            Outbound::Vote {
                phase,
                height,
                round,
                block_id,
            } => {
                let preimage = match phase {
                    VotePhase::Prevote => {
                        block_prevote_preimage(&self.chain_id, height, round, &block_id)
                    }
                    VotePhase::Precommit => {
                        block_vote_preimage(&self.chain_id, height, round, &block_id)
                    }
                };
                let signature = self.nodes[from].key.sign(preimage.as_bytes());
                Wire::Vote {
                    phase,
                    vote: SignedVote {
                        height,
                        round,
                        block_id,
                        validator_id: sender,
                        signature,
                    },
                }
            }
        };
        self.send_to_all(from, &wire);
    }

    fn send_to_all(&mut self, from: usize, wire: &Wire) {
        let copies = if self.adversary.duplicate { 2 } else { 1 };
        for to in 0..self.nodes.len() {
            if self.adversary.blocked.contains(&(from, to)) {
                continue;
            }
            for _ in 0..copies {
                let delay = if self.adversary.max_delay_ms == 0 {
                    0
                } else {
                    self.prng.below(self.adversary.max_delay_ms) + 1
                };
                let at = self.now_ms + delay;
                self.push(
                    at,
                    Delivery::Message {
                        to,
                        wire: wire.clone(),
                    },
                );
            }
        }
    }

    /// Injects an arbitrary wire message, for a test that models a Byzantine
    /// sender rather than a Byzantine schedule.
    pub fn inject(&mut self, wire: &Wire) {
        for to in 0..self.nodes.len() {
            self.push(
                self.now_ms,
                Delivery::Message {
                    to,
                    wire: wire.clone(),
                },
            );
        }
    }

    /// Injects a wire message at **one** node.
    ///
    /// [`Devnet::inject`] models a Byzantine sender that says one thing to
    /// everybody. This models the other shape: a proposer that tells two honest
    /// nodes two different things, which is what a gossip transport cannot
    /// prevent and what the receiver's own checks have to survive.
    pub fn inject_to(&mut self, to: usize, wire: &Wire) {
        self.push(
            self.now_ms,
            Delivery::Message {
                to,
                wire: wire.clone(),
            },
        );
    }

    /// Signs a vote with a member's real key, for the equivocation test.
    pub fn sign_vote(
        &self,
        index: usize,
        phase: VotePhase,
        height: u64,
        round: u64,
        block_id: Digest32,
    ) -> SignedVote {
        let preimage = match phase {
            VotePhase::Prevote => block_prevote_preimage(&self.chain_id, height, round, &block_id),
            VotePhase::Precommit => block_vote_preimage(&self.chain_id, height, round, &block_id),
        };
        SignedVote {
            height,
            round,
            block_id,
            validator_id: self.nodes[index].validator_id.clone(),
            signature: self.nodes[index].key.sign(preimage.as_bytes()),
        }
    }

    /// Accepts a finalized block, **after verifying it with the shipped
    /// verifier**.
    ///
    /// This is the only place a block enters a chain in this harness, and the
    /// verification is not optional here: `SPEC-025` asks for "certificati veri
    /// che il verificatore esistente accetta", and a harness that recorded blocks
    /// without checking them would be a harness that declares finality.
    fn finalize(&mut self, from: usize, block: FinalizedBlock) {
        block
            .verify(&self.chain_id, &self.set, &self.verifier)
            .expect("every finalized block must carry a certificate the shipped verifier accepts");
        let block_id = block.quorum_certificate.block_id;
        let height = block.header.height;
        self.finalized.entry(height).or_default().insert(block_id);
        let node = &mut self.nodes[from];
        if let Some(previous) = node.chain.last() {
            assert_eq!(
                block.header.previous_block_id, previous.quorum_certificate.block_id,
                "node {} finalized a block that does not extend its own chain",
                node.validator_id
            );
        }
        node.chain.push(block);
    }

    /// Re-offers messages a node held for a height it had not reached.
    fn release_held(&mut self, index: usize) {
        let height = self.nodes[index].engine.height();
        let ready: Vec<Wire> = {
            let node = &mut self.nodes[index];
            let (ready, keep) = node
                .held
                .drain(..)
                .partition::<Vec<_>, _>(|wire| wire.height() <= height);
            node.held = keep;
            ready
        };
        for wire in ready {
            self.push(self.now_ms, Delivery::Message { to: index, wire });
        }
    }

    fn push(&mut self, at: u64, delivery: Delivery) {
        self.seq += 1;
        self.queue.push(Scheduled {
            at,
            seq: self.seq,
            delivery,
        });
    }

    /// The safety property, as a check rather than as a claim: no height carries
    /// two different finalized block IDs.
    ///
    /// # Panics
    ///
    /// Panics naming the height and both IDs. A safety violation is not a test
    /// failure to be summarized.
    pub fn assert_no_conflicting_finality(&self) {
        for (height, ids) in &self.finalized {
            assert!(
                ids.len() <= 1,
                "SAFETY VIOLATION at height {height}: {} distinct block IDs finalized: {:?}",
                ids.len(),
                ids.iter().map(Digest32::to_prefixed).collect::<Vec<_>>()
            );
        }
    }

    /// The chains of all nodes agree wherever they overlap.
    pub fn assert_chains_agree(&self) {
        let shortest = self
            .nodes
            .iter()
            .map(|node| node.chain.len())
            .min()
            .unwrap_or(0);
        for index in 0..shortest {
            let reference = self.nodes[0].chain[index].quorum_certificate.block_id;
            for node in &self.nodes {
                assert_eq!(
                    node.chain[index].quorum_certificate.block_id, reference,
                    "node {} disagrees at chain position {index}",
                    node.validator_id
                );
            }
        }
    }

    /// The canonical `Block` bytes of one node's chain, for the determinism
    /// criterion.
    #[must_use]
    pub fn chain_bytes(&self, index: usize) -> Vec<u8> {
        let mut out = Vec::new();
        for block in &self.nodes[index].chain {
            out.extend_from_slice(
                &block
                    .to_json()
                    .expect("a finalized block serializes")
                    .to_jcs(),
            );
            out.push(b'\n');
        }
        out
    }

    /// One transcript line per finalized block of `index`, for the evidence.
    #[must_use]
    pub fn transcript(&self, index: usize) -> Vec<String> {
        self.nodes[index]
            .chain
            .iter()
            .map(|block| {
                format!(
                    "height {:>2} header.round {} qc.round {} signatures {} block_id {} verified {}",
                    block.header.height,
                    block.header.round,
                    block.quorum_certificate.round,
                    block.quorum_certificate.signatures.len(),
                    &block.quorum_certificate.block_id.to_prefixed()[..23],
                    block
                        .verify(&self.chain_id, &self.set, &self.verifier)
                        .is_ok(),
                )
            })
            .collect()
    }

    /// The proposer of `(height, round)`, for a test that needs to silence it.
    #[must_use]
    pub fn proposer_of(&self, height: u64, round: u64) -> String {
        proposer_at(&self.set, height, round)
            .expect("the harness set has a proposer at every pair")
            .validator_id
            .clone()
    }

    /// The block ID every height-1 proposal must name as its parent.
    #[must_use]
    pub const fn genesis_block_id(&self) -> Digest32 {
        self.genesis_block_id
    }

    /// The index of a member in `nodes`, by `validator_id`.
    #[must_use]
    pub fn index_of(&self, validator_id: &str) -> usize {
        self.nodes
            .iter()
            .position(|node| node.validator_id == validator_id)
            .expect("the harness only names its own members")
    }

    /// The chain ID every signature in this devnet is bound to.
    #[must_use]
    pub const fn chain_id(&self) -> &ChainId {
        &self.chain_id
    }

    /// An empty transaction list, for a test that builds a header by hand.
    #[must_use]
    pub fn no_transactions() -> Vec<JsonObject> {
        Vec::new()
    }

    /// Asserts that the shipped verifier is the one that accepted every block.
    ///
    /// A restatement of what [`Devnet::finalize`] already enforced, kept as a
    /// separate call so a test can name it and so the count appears in a
    /// transcript.
    pub fn assert_all_certificates_verify(&self) -> u64 {
        let mut checked = 0;
        for node in &self.nodes {
            for block in &node.chain {
                block
                    .verify(&self.chain_id, &self.set, &self.verifier)
                    .expect("certificate must verify");
                checked += 1;
            }
        }
        checked
    }

    /// Confirms the verifier in use is the consensus-critical one, by asking it
    /// a question only that rule answers the same way.
    pub fn verifier_is_the_shipped_one(&self) -> bool {
        // A small-order public key must be rejected: it is rule 3 of
        // `README.md#consensus-critical-ed25519-verification` and the one a
        // library default is most likely to differ on.
        let preimage = block_vote_preimage(&self.chain_id, 1, 0, &Digest32::repeated(0x11));
        !self.verifier.verify(&[0u8; 32], &preimage, &[0u8; 64])
    }
}
