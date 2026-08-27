//! Node runner driving the consensus engine pump and local services.

use std::collections::BTreeMap;
use std::time::{Duration, SystemTime, UNIX_EPOCH};

use tokio::sync::mpsc;

use coblox_core::block::BlockHeader;
use coblox_core::consensus::{
    Action, BlockProposal, Engine, EngineConfig, Event, FinalizedBlock, Outbound, SignedVote,
    TimeoutKind, Validity, VotePhase, verify_proposal, verify_vote,
};
use coblox_core::hash::Digest32;
use coblox_core::json::JsonObject;
use coblox_core::merkle;
use coblox_core::validator_set::ValidatorEntry;
use coblox_core::verifier::ConsensusVerifier;

use crate::buffer::FutureHeightBuffer;
use crate::config::NodeConfig;
use crate::envelope::{SignedEnvelope, fresh_nonce};
use crate::error::{NodeError, Result};
use crate::network::NetworkService;
use crate::replay::{ReplayCache, ReplayVerdict};
use crate::store::BlockStore;
use crate::wal::Wal;

/// Upper bound on the finalized blocks one `block_request` may cause this node
/// to emit.
///
/// `wire.md` puts synchronization on a request/response stream; the devnet
/// answers on the gossip topic, so a single unauthenticated request with
/// `from_height = 1` would otherwise make every validator re-broadcast the whole
/// chain to everyone. The response is capped instead: a peer that is further
/// behind asks again from where it got to. See [REVIEW-049] RF-006.
pub const MAX_BLOCKS_PER_SYNC_RESPONSE: u64 = 8;

/// Minimum interval between two answers this node gives to the same requester.
///
/// The bound above limits one answer; this limits how often one peer can ask
/// for another. Both are needed, and the second was learned by running the
/// runbook: with the envelope expiry check of [REVIEW-049] RF-001 finally
/// enforced, an unthrottled catch-up burst — three peers each answering every
/// request, on a topic every node receives — delayed live consensus messages
/// past their own expiry and stalled the chain for the duration of the sync.
/// The amplification was always there; the boundary is what made it visible.
pub const MIN_MS_BETWEEN_SYNC_ANSWERS: u64 = 1_000;

/// Environment variable that aborts the process between the vote's `fsync` and
/// its transmission.
///
/// It exists for `GATE-DURABLE-BEFORE-SEND`, which asks for the window to be
/// **observed** and not argued: the kill point is an instruction, not a `sleep`.
/// Unset — which is every case but that test — the check is a single string
/// lookup per vote and the branch is never taken. See [REVIEW-049] RF-003.
pub const ABORT_AFTER_WAL_SYNC_ENV: &str = "COBLOX_NODE_ABORT_AFTER_WAL_SYNC";

/// Environment variable that prints one `VOTE_SENT` line per vote handed to the
/// network, immediately after the send.
///
/// Off by default because it is one line per vote per height on every node, and
/// a node's normal output is one line per finalized block. The durability test
/// turns it **on** in both of its runs: the absence of the line in the run that
/// aborts means something only if the line would otherwise have been there.
pub const TRACE_VOTES_ENV: &str = "COBLOX_NODE_TRACE_VOTES";

/// Runs a single validator node.
pub struct NodeRunner {
    config: NodeConfig,
    wal: Wal,
    store: BlockStore,
    engine: Engine,
    buffer: FutureHeightBuffer,
    replay: ReplayCache,
    /// The highest height any peer has announced as finalized. The periodic
    /// `block_request` goes out only when this is ahead of us.
    observed_peer_height: u64,
    /// When this node last answered a `block_request` from each requester.
    last_sync_answer_ms: BTreeMap<String, u64>,
    /// Envelopes handed to the outbound channel since start, whether or not the
    /// channel accepted them. It is what the bound of
    /// [`MAX_BLOCKS_PER_SYNC_RESPONSE`] is observed through.
    outbound_attempts: u64,
    inbound_rx: mpsc::Receiver<SignedEnvelope>,
    outbound_tx: mpsc::Sender<SignedEnvelope>,
    timer_tx: mpsc::Sender<(u64, u64, TimeoutKind)>,
    timer_rx: mpsc::Receiver<(u64, u64, TimeoutKind)>,
}

/// Wall-clock milliseconds since the Unix epoch.
///
/// Every one of this function's callers passes the result as an envelope's
/// `created_at_ms`, or compares it against an envelope's `expires_at_ms`. It
/// arms no consensus timeout: [`Action::ScheduleTimeout`] carries a `delay_ms`
/// that goes to a sleep, and never touches a wall clock. The earlier note here
/// said otherwise and was wrong ([REVIEW-049] RF-013).
///
/// It returns a `Result` because both ways of not having a clock have to fail in
/// the same direction, and neither of them can be made up. A saturating
/// `u64::MAX` used to fail **open**: `expires_at_ms` saturates with it and the
/// envelope never expires, which is exactly the check the boundary was added to
/// perform. A clock this node cannot read is a failure of the node.
///
/// # Errors
///
/// Restituisce errore se l'orologio di sistema e' prima dell'epoca Unix o se i
/// millisecondi trascorsi non entrano in `u64`.
fn now_ms() -> Result<u64> {
    let elapsed = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| NodeError::Protocol("system clock is before the Unix epoch".into()))?;
    u64::try_from(elapsed.as_millis())
        .map_err(|_| NodeError::Protocol("system clock does not fit in u64 milliseconds".into()))
}

impl NodeRunner {
    /// Creates and initializes the node runner and its persistent state.
    ///
    /// # Errors
    ///
    /// Restituisce errore se il write-ahead log o il magazzino dei blocchi non si aprono, se lo stato riletto e' incoerente, o se il motore non parte.
    pub fn new(config: NodeConfig) -> Result<(Self, Option<NetworkService>)> {
        let wal_path = config.data_dir.join("wal.jsonl");
        let store_path = config.data_dir.join("blocks.jsonl");

        let wal = Wal::open(&wal_path)?;
        let store = BlockStore::open(&store_path, config.chain_id, config.genesis_block_id)?;

        let start_height = if store.latest_height() > 0 {
            store.latest_height() + 1
        } else {
            1
        };
        let start_prev_block_id = store.latest_block_id();

        // The lock survives the restart, and the information it is rebuilt from
        // was already on disk: the engine locks and precommits in the same step
        // (Algorithm 1 lines 38-40), so the highest round this node precommitted
        // in at `start_height` is the round it was locked at when it died.
        // Without this, a validator killed while locked comes back unlocked and
        // may prevote a different value in a later round of the same height —
        // no double signature, no WAL entry to see it, and the
        // quorum-intersection argument of [ADR-018] no longer holds.
        // [REVIEW-049] RF-002.
        let restored_lock = wal.locked_at_height(start_height);
        if let Some((round, block_id)) = restored_lock {
            println!(
                "LOCK_RESTORED node={} height={start_height} round={round} block_id={block_id:?}",
                config.validator_id
            );
        }

        let (engine, initial_actions) = Engine::start(EngineConfig {
            chain_id: config.chain_id,
            set: config.validator_set.clone(),
            validator_id: config.validator_id.clone(),
            timeouts: config.timeouts,
            height: start_height,
            previous_block_id: start_prev_block_id,
            locked_round: restored_lock.map(|(round, _)| round),
            locked_block_id: restored_lock.map(|(_, block_id)| block_id),
        })?;

        let (inbound_tx, inbound_rx) = mpsc::channel(1000);
        let (outbound_tx, outbound_rx) = mpsc::channel(1000);
        let (timer_tx, timer_rx) = mpsc::channel(100);

        let network = NetworkService::new(
            &config.network_id,
            &config.listen_addr,
            &config.seed_peers,
            inbound_tx,
            outbound_rx,
        )?;

        let mut runner = Self {
            config,
            wal,
            store,
            engine,
            buffer: FutureHeightBuffer::with_defaults(),
            replay: ReplayCache::default(),
            observed_peer_height: 0,
            last_sync_answer_ms: BTreeMap::new(),
            outbound_attempts: 0,
            inbound_rx,
            outbound_tx,
            timer_tx,
            timer_rx,
        };

        // Process initial actions from engine start
        runner.process_actions(initial_actions)?;

        Ok((runner, Some(network)))
    }

    /// Signs an envelope of `message_type` around `payload`.
    ///
    /// The single place a `nonce` and a `created_at_ms` are chosen, so that both
    /// are chosen the same way everywhere: a fresh nonce from the system CSPRNG
    /// ([REVIEW-049] RF-007(c)) and a clock reading that propagates its failure
    /// instead of saturating (RF-013).
    fn sign_envelope(
        &self,
        message_type: &str,
        validity_ms: u64,
        payload: JsonObject,
    ) -> Result<SignedEnvelope> {
        SignedEnvelope::build_and_sign(
            &self.config.chain_id,
            &self.config.network_id,
            message_type,
            &self.config.validator_id,
            now_ms()?,
            validity_ms,
            fresh_nonce()?,
            payload,
            &self.config.signing_key,
        )
    }

    /// Hands `envelope` to the network service.
    ///
    /// The outbound channel is bounded, and a full channel used to drop a vote
    /// that had already been made durable without a line anywhere. It is not
    /// promoted to a fatal error — the node is still correct, it has simply
    /// failed to be heard — but it is no longer silent. [REVIEW-049] RF-016.
    fn send_envelope(&mut self, envelope: SignedEnvelope) {
        self.outbound_attempts = self.outbound_attempts.saturating_add(1);
        let message_type = envelope.message_type.clone();
        if let Err(e) = self.outbound_tx.try_send(envelope) {
            eprintln!(
                "SEND_DROPPED node={} message_type={message_type}: {e}",
                self.config.validator_id
            );
        }
    }

    /// Broadcasts a block request for height `from_height`.
    fn request_blocks_from(&mut self, from_height: u64) {
        let built = JsonObject::builder()
            .uint("from_height", from_height)
            .build()
            .map_err(NodeError::from)
            .and_then(|payload| self.sign_envelope("block_request", 30_000, payload));
        match built {
            Ok(env) => self.send_envelope(env),
            Err(e) => eprintln!("SYNC_REQUEST_FAILED node={}: {e}", self.config.validator_id),
        }
    }

    // Il ciclo che consuma le Action del motore. Spezzarlo dentro una passata
    // dichiarata meccanica sarebbe il modo di introdurre un difetto invisibile in un
    // percorso di consenso: la divisione va fatta con la reviewer davanti, non da un
    // Lead che sta chiudendo dei lint. Dichiarato nella presa in carico del 2026-08-27.
    #[allow(clippy::too_many_lines)]
    /// Helper to process engine actions.
    fn process_actions(&mut self, actions: Vec<Action>) -> Result<bool> {
        let mut target_reached = false;
        let mut self_deliveries: Vec<SignedEnvelope> = Vec::new();

        for action in actions {
            match action {
                Action::Broadcast(Outbound::Proposal(proposal)) => {
                    let json = proposal.to_json()?;
                    let envelope = self.sign_envelope("block_proposal", 30_000, json)?;
                    self.send_envelope(envelope.clone());
                    self_deliveries.push(envelope);
                }
                Action::Broadcast(Outbound::Vote {
                    phase,
                    height,
                    round,
                    block_id,
                }) => {
                    if !self.wal.can_vote(height, round, phase, &block_id) {
                        eprintln!(
                            "SAFETY: refusing to sign conflicting vote for height={height} round={round} phase={phase:?}"
                        );
                        continue;
                    }

                    let signature = match phase {
                        VotePhase::Prevote => self.config.signing_key.sign_prevote(
                            &self.config.chain_id,
                            height,
                            round,
                            &block_id,
                        ),
                        VotePhase::Precommit => self.config.signing_key.sign_precommit(
                            &self.config.chain_id,
                            height,
                            round,
                            &block_id,
                        ),
                    };

                    // Invariant: write to WAL with fsync before sending
                    self.wal.record_vote(
                        height,
                        round,
                        phase,
                        &block_id,
                        &self.config.validator_id,
                        &signature,
                    )?;

                    // GATE-DURABLE-BEFORE-SEND's observation point. The vote is
                    // on disk and `sync_all` has returned; nothing has been
                    // handed to the outbound channel yet. Killing the process
                    // here is what makes the window a fact instead of an
                    // argument. [REVIEW-049] RF-003.
                    if std::env::var_os(ABORT_AFTER_WAL_SYNC_ENV).is_some() {
                        eprintln!(
                            "ABORT_AFTER_WAL_SYNC node={} height={height} round={round} phase={phase:?}",
                            self.config.validator_id
                        );
                        std::process::abort();
                    }

                    let signed_vote = SignedVote {
                        height,
                        round,
                        block_id,
                        validator_id: self.config.validator_id.clone(),
                        signature,
                    };

                    let phase_str = match phase {
                        VotePhase::Prevote => "prevote",
                        VotePhase::Precommit => "precommit",
                    };

                    let json = signed_vote.to_json()?;
                    let envelope = self.sign_envelope(phase_str, 30_000, json)?;
                    self.send_envelope(envelope.clone());
                    // Printed after the send and nowhere else: the durability
                    // test asserts on its absence to show the vote never left.
                    if std::env::var_os(TRACE_VOTES_ENV).is_some() {
                        println!(
                            "VOTE_SENT node={} height={height} round={round} phase={phase_str}",
                            self.config.validator_id
                        );
                    }
                    self_deliveries.push(envelope);
                }
                Action::ScheduleTimeout {
                    kind,
                    height,
                    round,
                    delay_ms,
                } => {
                    let tx = self.timer_tx.clone();
                    tokio::spawn(async move {
                        tokio::time::sleep(Duration::from_millis(delay_ms)).await;
                        let _ = tx.send((height, round, kind)).await;
                    });
                }
                Action::RequestValue { height, round } => {
                    // The action list being drained here was produced by an
                    // engine that may no longer be the one this node holds: a
                    // `finalized_block` delivered from the future-height buffer,
                    // in a nested `dispatch_envelope`, replaces `self.engine`
                    // with a fresh one at the next height. Answering the old
                    // engine's `getValue()` request into the new engine is an
                    // `UnsolicitedValue`, which is a fatal error and stopped the
                    // node — observed running the runbook, at
                    // `UnsolicitedValue { height: 123, round: 0 }`, on a
                    // validator catching up after a restart. A value for a
                    // height already decided is obsolete, and dropping it is the
                    // whole of the repair.
                    if height != self.engine.height() || round != self.engine.round() {
                        eprintln!(
                            "STALE_VALUE_REQUEST node={} requested=({height},{round}) now=({},{})",
                            self.config.validator_id,
                            self.engine.height(),
                            self.engine.round()
                        );
                        continue;
                    }
                    let set_hash = self.config.validator_set.hash()?;
                    // No `unwrap_or(0)`: a node absent from its own set has no
                    // index, and behaving like index 0 would say the opposite of
                    // what `Engine::start` guarantees. [REVIEW-049] RF-019.
                    let proposer_idx = self
                        .config
                        .validator_set
                        .validators
                        .iter()
                        .position(|v| v.validator_id == self.config.validator_id)
                        .ok_or_else(|| {
                            NodeError::Protocol(
                                "this node is not a member of its own validator set".into(),
                            )
                        })
                        .and_then(|index| {
                            u64::try_from(index).map_err(|_| {
                                NodeError::Protocol("validator index does not fit in u64".into())
                            })
                        })?;

                    let prev_id = self.store.latest_block_id();
                    let empty_txs: Vec<JsonObject> = Vec::new();
                    let empty_root = merkle::transactions_root(&[])?;

                    let header = BlockHeader {
                        schema_version: "0.1".to_owned(),
                        protocol_version: "0.1".to_owned(),
                        network_id: self.config.network_id.clone(),
                        height,
                        round,
                        timestamp_ms: 1_787_654_400_000
                            + height * 5_000
                            + round * 1_000
                            + proposer_idx,
                        previous_block_id: prev_id,
                        transactions_root: empty_root,
                        state_root: Digest32::repeated(0x33),
                        validator_set_hash: set_hash,
                        next_validator_set_hash: set_hash,
                        consensus_parameters_hash: Digest32::repeated(0x44),
                    };

                    let next_actions = self.engine.step_event(Event::Value {
                        height,
                        round,
                        header: Box::new(header),
                        transactions: empty_txs,
                    })?;
                    if self.process_actions(next_actions)? {
                        target_reached = true;
                    }
                }
                Action::Finalize(finalized) => {
                    // Verify finalized block integrity
                    finalized.verify(
                        &self.config.chain_id,
                        &self.config.validator_set,
                        &ConsensusVerifier,
                    )?;

                    let h = finalized.header.height;
                    let bid = finalized.block_id(&self.config.chain_id)?;
                    self.store.append_block(&finalized)?;
                    println!(
                        "FINALIZED node={} height={} round={} block_id={:?}",
                        self.config.validator_id, h, finalized.quorum_certificate.round, bid
                    );

                    // Broadcast finalized block to announce to peers and support catch-up
                    let announced = finalized
                        .to_json()
                        .map_err(NodeError::from)
                        .and_then(|json| self.sign_envelope("finalized_block", 60_000, json));
                    match announced {
                        Ok(env) => self.send_envelope(env),
                        Err(e) => eprintln!(
                            "ANNOUNCE_FAILED node={} height={h}: {e}",
                            self.config.validator_id
                        ),
                    }

                    if let Some(target) = self.config.target_height
                        && h >= target
                    {
                        target_reached = true;
                    }

                    // The buffer keeps a `BTreeMap` entry per height it holds,
                    // and heights get skipped whenever a `finalized_block`
                    // carries the node forward. Without this the entries for the
                    // skipped heights stay for the life of the process.
                    // [REVIEW-049] RF-010.
                    self.buffer.prune_before(self.engine.height());

                    // Drain future buffer for the new height
                    let buffered = self.buffer.drain_height(self.engine.height());
                    for env in buffered {
                        if self.dispatch_envelope(env)? {
                            target_reached = true;
                        }
                    }
                }
            }
        }

        // Deliver self broadcasts
        for env in self_deliveries {
            if self.dispatch_envelope(env)? {
                target_reached = true;
            }
        }

        Ok(target_reached)
    }

    /// Resolves `sender_node_id` to the member of the active set that owns it.
    ///
    /// A sender that is not a member has no consensus key to verify against, and
    /// that is the end of the matter: `wire.md` line 516 makes a proposal's
    /// authenticity the envelope's, so an unresolvable sender is an envelope with
    /// no authenticity at all.
    fn resolve_sender(&self, sender_node_id: &str) -> Result<&ValidatorEntry> {
        self.config
            .validator_set
            .validators
            .iter()
            .find(|entry| entry.node_id == sender_node_id)
            .ok_or_else(|| {
                NodeError::Rejected(format!(
                    "sender_node_id {sender_node_id:?} is not a member of the active validator set"
                ))
            })
    }

    /// The wire boundary: verifies an envelope, then routes it.
    ///
    /// **Everything a peer sends enters here and nowhere else**, and the order
    /// below is the order the checks have to happen in:
    ///
    /// 1. `network_id` — a message for another network is not this node's
    ///    business, and is dropped rather than rejected.
    /// 2. `sender_node_id` resolves to a member of the active set. A non-member
    ///    has no key.
    /// 3. [`SignedEnvelope::verify`] under the local `chain_id` and that
    ///    member's `consensus_public_key`. This is at once the signature check,
    ///    the chain check — the chain is bound into `message_id` and into the
    ///    signature domain, so an envelope of another chain fails here — the
    ///    expiry check, and the validity-window check.
    /// 4. The replay cache over `message_id` and `(sender_node_id, nonce)`.
    ///
    /// Only then does the payload get looked at. Before [REVIEW-049] RF-001 none
    /// of steps 2 to 4 existed and `SignedEnvelope::verify` had no caller in the
    /// workspace: a stranger's `block_proposal`, in an envelope expired since
    /// 1970, made honest nodes sign a block whose `state_root` it had chosen.
    ///
    /// # Errors
    ///
    /// Restituisce [`NodeError::Rejected`] se la busta non supera il confine —
    /// mittente non membro, firma o catena sbagliate, busta scaduta o troppo
    /// longeva, replay. Restituisce gli altri errori se una scrittura durevole o
    /// il motore falliscono, e quelli sono fatali. Una busta di un'altra rete non
    /// e' un errore: viene scartata.
    pub fn handle_envelope(&mut self, envelope: SignedEnvelope) -> Result<bool> {
        if envelope.network_id != self.config.network_id {
            return Ok(false);
        }

        let now = now_ms()?;
        let sender_key = self
            .resolve_sender(&envelope.sender_node_id)?
            .consensus_public_key;
        envelope
            .verify(&self.config.chain_id, &sender_key, now, &ConsensusVerifier)
            .map_err(|e| NodeError::Rejected(e.to_string()))?;

        self.replay.expire(now);
        match self.replay.admit(
            envelope.message_id,
            &envelope.sender_node_id,
            envelope.nonce,
            envelope.expires_at_ms,
        ) {
            ReplayVerdict::Fresh => {}
            verdict => {
                return Err(NodeError::Rejected(format!(
                    "replay cache refused envelope from {}: {verdict:?}",
                    envelope.sender_node_id
                )));
            }
        }

        self.dispatch_envelope(envelope)
    }

    // Come sopra: il ramo che classifica e instrada le buste in arrivo. Non
    // ristrutturato in questa passata, e per la stessa ragione.
    #[allow(clippy::too_many_lines)]
    /// Routes an envelope that has already passed the boundary.
    ///
    /// Separate from [`Self::handle_envelope`] because two callers hold
    /// envelopes that were verified once already and must not be verified again:
    /// the future-height buffer, whose contents were admitted when they arrived,
    /// and this node's own broadcasts, which it delivers to itself the way a
    /// gossip topic would. Re-entering the boundary would reject both as
    /// replays of themselves.
    ///
    /// # Errors
    ///
    /// Restituisce errore se il motore rifiuta l'evento, se una scrittura
    /// durevole fallisce, o se il mittente non e' piu' risolvibile nel set.
    fn dispatch_envelope(&mut self, envelope: SignedEnvelope) -> Result<bool> {
        let curr_height = self.engine.height();

        match envelope.message_type.as_str() {
            "block_request" => {
                if let Ok(from_height) = envelope.payload.uint("from_height") {
                    let now = now_ms()?;
                    let requester = envelope.sender_node_id.clone();
                    let too_soon = self
                        .last_sync_answer_ms
                        .get(&requester)
                        .is_some_and(|last| {
                            now.saturating_sub(*last) < MIN_MS_BETWEEN_SYNC_ANSWERS
                        });
                    if too_soon {
                        return Ok(false);
                    }
                    self.last_sync_answer_ms.insert(requester, now);

                    let latest = self.store.latest_height();
                    // Bounded by a declared constant. The response still goes to
                    // the topic rather than to the requester, so an unbounded
                    // one would let a single `from_height = 1` make every
                    // validator re-broadcast the whole chain to everyone.
                    // [REVIEW-049] RF-006.
                    let last = latest.min(
                        from_height.saturating_add(MAX_BLOCKS_PER_SYNC_RESPONSE.saturating_sub(1)),
                    );
                    for h in from_height..=last {
                        let Some(blk) = self.store.get_block(h) else {
                            continue;
                        };
                        let answer = blk
                            .to_json()
                            .map_err(NodeError::from)
                            .and_then(|json| self.sign_envelope("finalized_block", 60_000, json));
                        match answer {
                            Ok(env) => self.send_envelope(env),
                            Err(e) => eprintln!(
                                "SYNC_RESPONSE_FAILED node={} height={h}: {e}",
                                self.config.validator_id
                            ),
                        }
                    }
                }
            }
            "finalized_block" => {
                if let Ok(finalized) = FinalizedBlock::from_json(&envelope.payload) {
                    let blk_height = finalized.header.height;
                    self.observed_peer_height = self.observed_peer_height.max(blk_height);
                    // The certificate is checked **before** the block is
                    // buffered, not only on the branch that consumes it: a
                    // future-height block held in memory on nobody's authority
                    // is an unverified object waiting to be trusted later.
                    // [REVIEW-049] RF-010.
                    if let Err(e) = finalized.verify(
                        &self.config.chain_id,
                        &self.config.validator_set,
                        &ConsensusVerifier,
                    ) {
                        // Silently dropping this used to be the one rejection
                        // nobody could see. [REVIEW-049] RF-018.
                        eprintln!(
                            "Invalid finalized_block received: node={} height={blk_height}: {e}",
                            self.config.validator_id
                        );
                        return Ok(false);
                    }
                    if blk_height > curr_height {
                        self.buffer.insert(curr_height, blk_height, envelope);
                    } else if blk_height == curr_height {
                        let bid = finalized.block_id(&self.config.chain_id)?;
                        self.store.append_block(&finalized)?;
                        println!(
                            "SYNC_FINALIZED node={} height={} block_id={:?}",
                            self.config.validator_id, blk_height, bid
                        );

                        let next_height = blk_height + 1;
                        let (new_engine, initial_actions) = Engine::start(EngineConfig {
                            chain_id: self.config.chain_id,
                            set: self.config.validator_set.clone(),
                            validator_id: self.config.validator_id.clone(),
                            timeouts: self.config.timeouts,
                            height: next_height,
                            previous_block_id: bid,
                            locked_round: self
                                .wal
                                .locked_at_height(next_height)
                                .map(|(round, _)| round),
                            locked_block_id: self
                                .wal
                                .locked_at_height(next_height)
                                .map(|(_, block_id)| block_id),
                        })?;
                        self.engine = new_engine;
                        // Heights are skipped here by construction, and every
                        // skipped one would otherwise keep its buffer entry for
                        // the life of the process. [REVIEW-049] RF-010.
                        self.buffer.prune_before(next_height);

                        if let Some(target) = self.config.target_height
                            && blk_height >= target
                        {
                            return Ok(true);
                        }

                        if self.process_actions(initial_actions)? {
                            return Ok(true);
                        }

                        let buffered = self.buffer.drain_height(next_height);
                        for env in buffered {
                            if self.dispatch_envelope(env)? {
                                return Ok(true);
                            }
                        }
                    }
                }
            }
            "block_proposal" => {
                let proposal = BlockProposal::from_json(&envelope.payload)?;
                let msg_height = proposal.height;
                if msg_height > curr_height {
                    self.buffer.insert(curr_height, msg_height, envelope);
                    self.request_blocks_from(curr_height);
                    return Ok(false);
                }
                if msg_height < curr_height {
                    return Ok(false);
                }

                // validity check valid(v)
                let prev_match = proposal.header.previous_block_id == self.store.latest_block_id();
                let validity = if prev_match && proposal.header.height == curr_height {
                    Validity::Valid
                } else {
                    Validity::Invalid
                };

                // `sender_node_id` is a node ID and `verify_proposal` wants a
                // validator ID; the boundary has already resolved the one to the
                // member that owns it, so this reads the member's own ID rather
                // than trusting a string the payload's sender chose.
                let proposer = self
                    .resolve_sender(&envelope.sender_node_id)?
                    .validator_id
                    .clone();
                match verify_proposal(
                    &self.config.chain_id,
                    &self.config.validator_set,
                    &proposer,
                    proposal,
                    validity,
                ) {
                    Ok(verified) => {
                        let actions = self.engine.step_event(Event::Message(verified))?;
                        return self.process_actions(actions);
                    }
                    Err(e) => {
                        eprintln!("Invalid proposal received: {e:?}");
                    }
                }
            }
            "prevote" => {
                let vote = SignedVote::from_json(&envelope.payload)?;
                let msg_height = vote.height;
                if msg_height > curr_height {
                    self.buffer.insert(curr_height, msg_height, envelope);
                    self.request_blocks_from(curr_height);
                    return Ok(false);
                }
                if msg_height < curr_height {
                    return Ok(false);
                }

                match verify_vote(
                    &self.config.chain_id,
                    &self.config.validator_set,
                    VotePhase::Prevote,
                    vote,
                    &ConsensusVerifier,
                ) {
                    Ok(verified) => {
                        let actions = self.engine.step_event(Event::Message(verified))?;
                        return self.process_actions(actions);
                    }
                    Err(e) => {
                        eprintln!("Invalid prevote received: {e:?}");
                    }
                }
            }
            "precommit" => {
                let vote = SignedVote::from_json(&envelope.payload)?;
                let msg_height = vote.height;
                if msg_height > curr_height {
                    self.buffer.insert(curr_height, msg_height, envelope);
                    self.request_blocks_from(curr_height);
                    return Ok(false);
                }
                if msg_height < curr_height {
                    return Ok(false);
                }

                match verify_vote(
                    &self.config.chain_id,
                    &self.config.validator_set,
                    VotePhase::Precommit,
                    vote,
                    &ConsensusVerifier,
                ) {
                    Ok(verified) => {
                        let actions = self.engine.step_event(Event::Message(verified))?;
                        return self.process_actions(actions);
                    }
                    Err(e) => {
                        eprintln!("Invalid precommit received: {e:?}");
                    }
                }
            }
            _ => {}
        }
        Ok(false)
    }

    /// Runs the node event loop until completion or target height.
    ///
    /// # Errors
    ///
    /// Restituisce errore se un'azione del motore non e' eseguibile — una
    /// scrittura durevole che fallisce, un orologio di sistema illeggibile, il
    /// motore che rifiuta un evento — o se il nodo non e' membro del proprio
    /// set. **Una trasmissione che non parte non e' fra questi**: il canale in
    /// uscita e' limitato e un `try_send` respinto viene registrato come
    /// `SEND_DROPPED` e non propagato, perche' un nodo che non e' stato sentito
    /// e' ancora corretto. La formulazione precedente diceva il contrario e
    /// nessun `try_send` la attuava ([REVIEW-049] RF-011(b) e RF-016). Una busta
    /// rifiutata al confine viene registrata come `REJECTED` e il ciclo prosegue.
    pub async fn run(&mut self) -> Result<()> {
        let mut sync_interval = tokio::time::interval(Duration::from_millis(500));
        loop {
            tokio::select! {
                _ = sync_interval.tick() => {
                    // Only when a peer has announced a height this node does not
                    // have. The unconditional request used to make four healthy
                    // nodes re-broadcast blocks at eight per second in steady
                    // state, for nothing. [REVIEW-049] RF-006.
                    if self.observed_peer_height > self.store.latest_height() {
                        self.request_blocks_from(self.store.latest_height() + 1);
                    }
                }
                Some(envelope) = self.inbound_rx.recv() => {
                    // A rejection at the wire boundary is not a reason for this
                    // node to stop: the boundary exists to be hit by whatever a
                    // peer sends, and a node that exited on the first bad
                    // envelope could be stopped by any stranger. Every other
                    // error is still fatal. [REVIEW-049] RF-001.
                    match self.handle_envelope(envelope) {
                        Ok(true) => break,
                        Ok(false) => {}
                        Err(e) if !e.is_fatal() => {
                            eprintln!("REJECTED node={}: {e}", self.config.validator_id);
                        }
                        Err(e) => return Err(e),
                    }
                }
                Some((h, r, kind)) = self.timer_rx.recv() => {
                    if h == self.engine.height() && r == self.engine.round() {
                        let actions = self.engine.step_event(Event::Timeout {
                            kind,
                            height: h,
                            round: r,
                        })?;
                        if self.process_actions(actions)? {
                            break;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    /// Returns the latest finalized height in store.
    #[must_use]
    pub fn latest_height(&self) -> u64 {
        self.store.latest_height()
    }

    /// Returns the latest block in store.
    #[must_use]
    pub fn latest_block(&self) -> Option<&FinalizedBlock> {
        self.store.latest_block()
    }

    /// Returns the total number of votes in WAL.
    #[must_use]
    pub fn wal_vote_count(&self) -> usize {
        self.wal.count()
    }

    /// The round and block this node's engine is locked at, if it is locked.
    #[must_use]
    pub fn locked(&self) -> Option<(u64, Digest32)> {
        match (self.engine.locked_round(), self.engine.locked_block_id()) {
            (Some(round), Some(block_id)) => Some((round, block_id)),
            _ => None,
        }
    }

    /// Envelopes handed to the outbound channel since start.
    #[must_use]
    pub const fn outbound_attempts(&self) -> u64 {
        self.outbound_attempts
    }

    /// Number of envelopes still held for future heights.
    #[must_use]
    pub fn buffered_message_count(&self) -> usize {
        self.buffer.len()
    }
}
