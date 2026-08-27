//! Node runner driving the consensus engine pump and local services.

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
use coblox_core::verifier::ConsensusVerifier;

use crate::buffer::FutureHeightBuffer;
use crate::config::NodeConfig;
use crate::envelope::SignedEnvelope;
use crate::error::Result;
use crate::network::NetworkService;
use crate::store::BlockStore;
use crate::wal::Wal;

/// Runs a single validator node.
pub struct NodeRunner {
    config: NodeConfig,
    wal: Wal,
    store: BlockStore,
    engine: Engine,
    buffer: FutureHeightBuffer,
    inbound_rx: mpsc::Receiver<SignedEnvelope>,
    outbound_tx: mpsc::Sender<SignedEnvelope>,
    timer_tx: mpsc::Sender<(u64, u64, TimeoutKind)>,
    timer_rx: mpsc::Receiver<(u64, u64, TimeoutKind)>,
}

/// Wall-clock milliseconds since the Unix epoch, saturating instead of wrapping.
///
/// `as_millis` returns `u128` and the engine's clock is `u64`. The conversion is
/// written explicitly rather than as a bare `as` cast: the truncation point is
/// past the year 584 million, so it cannot be reached, but a silent cast on the
/// value that arms every consensus timeout is the kind of narrowing that is
/// noticed only once it has already happened.
fn now_ms() -> u64 {
    u64::try_from(
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap_or(Duration::ZERO)
            .as_millis(),
    )
    .unwrap_or(u64::MAX)
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

        let (engine, initial_actions) = Engine::start(EngineConfig {
            chain_id: config.chain_id,
            set: config.validator_set.clone(),
            validator_id: config.validator_id.clone(),
            timeouts: config.timeouts,
            height: start_height,
            previous_block_id: start_prev_block_id,
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
            inbound_rx,
            outbound_tx,
            timer_tx,
            timer_rx,
        };

        // Process initial actions from engine start
        runner.process_actions(initial_actions)?;

        Ok((runner, Some(network)))
    }

    /// Broadcasts a block request for height `from_height`.
    fn request_blocks_from(&self, from_height: u64) {
        if let Ok(payload) = JsonObject::builder()
            .uint("from_height", from_height)
            .build()
            && let Ok(env) = SignedEnvelope::build_and_sign(
                &self.config.chain_id,
                &self.config.network_id,
                "block_request",
                &self.config.validator_id,
                now_ms(),
                30_000,
                [0u8; 16],
                payload,
                &self.config.signing_key,
            )
        {
            let _ = self.outbound_tx.try_send(env);
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
                    let envelope = SignedEnvelope::build_and_sign(
                        &self.config.chain_id,
                        &self.config.network_id,
                        "block_proposal",
                        &self.config.validator_id,
                        now_ms(),
                        30_000,
                        [0u8; 16],
                        json,
                        &self.config.signing_key,
                    )?;
                    let _ = self.outbound_tx.try_send(envelope.clone());
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
                    let envelope = SignedEnvelope::build_and_sign(
                        &self.config.chain_id,
                        &self.config.network_id,
                        phase_str,
                        &self.config.validator_id,
                        now_ms(),
                        30_000,
                        [0u8; 16],
                        json,
                        &self.config.signing_key,
                    )?;
                    let _ = self.outbound_tx.try_send(envelope.clone());
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
                    let set_hash = self.config.validator_set.hash()?;
                    let proposer_idx = self
                        .config
                        .validator_set
                        .validators
                        .iter()
                        .position(|v| v.validator_id == self.config.validator_id)
                        .unwrap_or(0) as u64;

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
                    if let Ok(block_json) = finalized.to_json()
                        && let Ok(env) = SignedEnvelope::build_and_sign(
                            &self.config.chain_id,
                            &self.config.network_id,
                            "finalized_block",
                            &self.config.validator_id,
                            now_ms(),
                            60_000,
                            [0u8; 16],
                            block_json,
                            &self.config.signing_key,
                        )
                    {
                        let _ = self.outbound_tx.try_send(env);
                    }

                    if let Some(target) = self.config.target_height
                        && h >= target
                    {
                        target_reached = true;
                    }

                    // Drain future buffer for the new height
                    let buffered = self.buffer.drain_height(self.engine.height());
                    for env in buffered {
                        if self.handle_envelope(env)? {
                            target_reached = true;
                        }
                    }
                }
            }
        }

        // Deliver self broadcasts
        for env in self_deliveries {
            if self.handle_envelope(env)? {
                target_reached = true;
            }
        }

        Ok(target_reached)
    }

    // Come sopra: il ramo che classifica e instrada le buste in arrivo. Non
    // ristrutturato in questa passata, e per la stessa ragione.
    #[allow(clippy::too_many_lines)]
    /// Handles an incoming wire envelope.
    ///
    /// # Errors
    ///
    /// Restituisce errore se la busta non verifica o se il motore rifiuta l'evento. Una busta di un'altra rete o di un'altra catena non e' un errore: viene scartata.
    pub fn handle_envelope(&mut self, envelope: SignedEnvelope) -> Result<bool> {
        if envelope.network_id != self.config.network_id {
            return Ok(false);
        }

        let curr_height = self.engine.height();

        match envelope.message_type.as_str() {
            "block_request" => {
                if let Ok(from_height) = envelope.payload.uint("from_height") {
                    let latest = self.store.latest_height();
                    if latest >= from_height {
                        for h in from_height..=latest {
                            if let Some(blk) = self.store.get_block(h)
                                && let Ok(blk_json) = blk.to_json()
                                && let Ok(env) = SignedEnvelope::build_and_sign(
                                    &self.config.chain_id,
                                    &self.config.network_id,
                                    "finalized_block",
                                    &self.config.validator_id,
                                    now_ms(),
                                    60_000,
                                    [0u8; 16],
                                    blk_json,
                                    &self.config.signing_key,
                                )
                            {
                                let _ = self.outbound_tx.try_send(env);
                            }
                        }
                    }
                }
            }
            "finalized_block" => {
                if let Ok(finalized) = FinalizedBlock::from_json(&envelope.payload) {
                    let blk_height = finalized.header.height;
                    if blk_height > curr_height {
                        self.buffer.insert(curr_height, blk_height, envelope);
                    } else if blk_height == curr_height
                        && finalized
                            .verify(
                                &self.config.chain_id,
                                &self.config.validator_set,
                                &ConsensusVerifier,
                            )
                            .is_ok()
                    {
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
                        })?;
                        self.engine = new_engine;

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
                            if self.handle_envelope(env)? {
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

                let proposer = envelope.sender_node_id;
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
    /// Restituisce errore se un'azione del motore non e' eseguibile: una scrittura durevole che fallisce, una trasmissione che non parte, un timer che non si arma.
    pub async fn run(&mut self) -> Result<()> {
        let mut sync_interval = tokio::time::interval(Duration::from_millis(500));
        loop {
            tokio::select! {
                _ = sync_interval.tick() => {
                    self.request_blocks_from(self.engine.height());
                }
                Some(envelope) = self.inbound_rx.recv() => {
                    if self.handle_envelope(envelope)? {
                        break;
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
}
