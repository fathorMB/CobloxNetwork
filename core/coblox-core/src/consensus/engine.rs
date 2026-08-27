//! The consensus state machine.
//!
//! Everything normative in this file is annotated with the line of Algorithm 1
//! it implements. The comparison with the source, line by line and including the
//! four places where this implementation deliberately differs, is in [`super`].

use std::collections::btree_map::Entry;
use std::collections::{BTreeMap, BTreeSet};

use crate::block::BlockHeader;
use crate::error::{ConsensusError, Error, Result};
use crate::hash::{ChainId, Digest32};
use crate::json::JsonObject;
use crate::quorum::quorum;
use crate::validator_set::ValidatorSet;

use super::certificate::{CertificateSignature, FinalizedBlock, QuorumCertificate};
use super::messages::{
    BlockProposal, ConsensusMessage, SignedVote, Validity, VerifiedMessage, VotePhase,
};
use super::proposer::proposer_at;

/// The three steps of a round.
///
/// The ordering is Algorithm 1's `step_p >= prevote` and is what makes line 36's
/// guard expressible; `PartialOrd` is derived from the declaration order and the
/// declaration order is the protocol's.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Step {
    /// Waiting for the round's proposal.
    Propose,
    /// Prevote sent (or the propose timeout elapsed); waiting for prevotes.
    Prevote,
    /// Precommit sent (or the prevote timeout elapsed); waiting for precommits.
    Precommit,
}

/// Which timeout elapsed.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TimeoutKind {
    /// `OnTimeoutPropose`.
    Propose,
    /// `OnTimeoutPrevote`.
    Prevote,
    /// `OnTimeoutPrecommit`.
    Precommit,
}

/// The three consensus timeouts, and the amount each one grows per round.
///
/// **These are local parameters.** They are not carried by any signed document
/// and they are not genesis constants, so — by the criterion of
/// [[predicato-di-accettazione]] — **no validity rule of this protocol can ever
/// compare them**, on any network, at any acceptance point. Two nodes with
/// different values here are both conformant. What they buy is the speed at
/// which a failed round is abandoned, which is a liveness property of a
/// deployment and not a property of the chain.
///
/// [ADR-018] §5 nominates this at the decision level rather than leaving it to
/// the implementation, and says why: the same question about ten operational
/// parameters cost three review passes precisely because it was answered late.
///
/// Growth is `base + round * increment`, saturating. It has to grow, or a
/// network whose real message delay exceeds the timeout never finishes a round
/// no matter how many it tries; it does not have to grow *exponentially*, and
/// this shape is chosen because it is the one whose behaviour at `u64` extremes
/// is obvious.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ConsensusTimeouts {
    /// Time to wait for the round's proposal, at round 0.
    pub propose_ms: u64,
    /// Time to wait for prevotes, at round 0.
    pub prevote_ms: u64,
    /// Time to wait for precommits, at round 0.
    pub precommit_ms: u64,
    /// Added to each of the three per round.
    pub round_increment_ms: u64,
}

impl ConsensusTimeouts {
    /// The delay for `kind` at `round`.
    #[must_use]
    pub const fn delay_ms(&self, kind: TimeoutKind, round: u64) -> u64 {
        let base = match kind {
            TimeoutKind::Propose => self.propose_ms,
            TimeoutKind::Prevote => self.prevote_ms,
            TimeoutKind::Precommit => self.precommit_ms,
        };
        base.saturating_add(round.saturating_mul(self.round_increment_ms))
    }
}

/// What the engine was told, from outside.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Event {
    /// A message that passed [`super::messages::verify_proposal`] or
    /// [`super::messages::verify_vote`].
    Message(VerifiedMessage),
    /// A timeout the caller had been asked to schedule has elapsed.
    ///
    /// `height` and `round` are the ones the [`Action::ScheduleTimeout`] carried.
    /// The engine checks them: a timeout for a round it has left is discarded,
    /// which is Algorithm 1's guard `height = h_p ∧ round = round_p` and the
    /// reason a caller may deliver late timeouts without harm.
    Timeout {
        /// Which timeout.
        kind: TimeoutKind,
        /// The height it was scheduled for.
        height: u64,
        /// The round it was scheduled for.
        round: u64,
    },
    /// The value the caller was asked for by [`Action::RequestValue`].
    ///
    /// This is Algorithm 1's `getValue()`, turned inside out. It is a request
    /// and a reply rather than a call, because `getValue()` is where a real node
    /// reaches a mempool and an executor, and a callback would have put that
    /// reach *inside* the engine — which is exactly the property
    /// `GATE-NO-IO` exists to keep out.
    Value {
        /// The height the value is for.
        height: u64,
        /// The round the value is for.
        round: u64,
        /// The proposed header. Its `height` and `round` must be the pair above.
        header: Box<BlockHeader>,
        /// The proposed transactions, in canonical execution order.
        transactions: Vec<JsonObject>,
    },
}

/// A message the caller must sign and broadcast.
///
/// Votes leave unsigned. The engine holds no key and performs no signature: a
/// key is a thing a process must read from somewhere, and "somewhere" is I/O. The
/// caller signs with the phase's preimage — [`crate::registry::block_prevote_preimage`]
/// or [`crate::registry::block_vote_preimage`] — and delivers the result to
/// every member of the set **including this engine**, exactly as a gossip topic
/// does. An engine that never receives its own precommit back cannot put its own
/// signature into a certificate it assembles.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outbound {
    /// A proposal. It carries no signature of its own; see [`super::messages`].
    Proposal(Box<BlockProposal>),
    /// A vote to sign under `phase`'s domain and broadcast.
    Vote {
        /// Which phase, and therefore which signature domain.
        phase: VotePhase,
        /// The height voted on.
        height: u64,
        /// The round voted in.
        round: u64,
        /// The block voted for.
        block_id: Digest32,
    },
}

/// What the engine asks the caller to do.
///
/// The four variants are the complete list of ways this engine affects anything
/// outside itself, and none of them performs the effect.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    /// Sign if it is a vote, then send to every member including this one.
    Broadcast(Outbound),
    /// Deliver [`Event::Timeout`] with these fields after `delay_ms`.
    ScheduleTimeout {
        /// Which timeout.
        kind: TimeoutKind,
        /// The height to echo back.
        height: u64,
        /// The round to echo back.
        round: u64,
        /// How long to wait, in milliseconds of the caller's clock.
        delay_ms: u64,
    },
    /// Build a block for this height and round and return it as [`Event::Value`].
    RequestValue {
        /// The height to build for.
        height: u64,
        /// The round to build for.
        round: u64,
    },
    /// This block is final. It carries a certificate the existing verifier
    /// accepts.
    Finalize(Box<FinalizedBlock>),
}

/// A value under consideration: a proposal's payload plus its ID.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Value {
    header: BlockHeader,
    transactions: Vec<JsonObject>,
    block_id: Digest32,
}

/// Everything the engine remembers about the height it is executing.
///
/// Discarded wholesale on decision — Algorithm 1 line 53's "empty message log".
#[derive(Debug, Default)]
struct HeightLog {
    /// The first proposal seen per round. A second one from the same proposer is
    /// dropped: a proposer that equivocates gets one proposal considered, and
    /// which one is a race the protocol does not need to resolve, because a
    /// proposal decides nothing on its own.
    proposals: BTreeMap<u64, (BlockProposal, Digest32, Validity)>,
    /// `(round, block_id) -> validator_id -> signature`, for prevotes.
    prevotes: BTreeMap<(u64, Digest32), BTreeMap<String, [u8; 64]>>,
    /// `(round, block_id) -> validator_id -> signature`, for precommits.
    precommits: BTreeMap<(u64, Digest32), BTreeMap<String, [u8; 64]>>,
    /// The first block each validator prevoted per round.
    prevote_of: BTreeMap<(u64, String), Digest32>,
    /// The first block each validator precommitted per round.
    ///
    /// This map is the whole of the equivocation defence, and it is a defence
    /// that needs no detection: a second, different precommit from the same
    /// validator in the same round finds an entry here and is dropped, so that
    /// validator's power is counted towards **one** block per round and can
    /// never appear on both sides of a split.
    precommit_of: BTreeMap<(u64, String), Digest32>,
    /// Which validators have been heard from at each round, for the round-skip
    /// rule.
    participants: BTreeMap<u64, BTreeSet<String>>,
}

/// The consensus engine: `(state, event) -> (state', actions)`.
///
/// # What this type cannot do, by its shape
///
/// It holds no socket, no file, no clock and no key; it is generic over nothing,
/// stores no closure and no trait object, and every one of its public methods
/// takes and returns plain data. See [`super`] §*No I/O, and how that is shown*.
#[derive(Debug)]
pub struct Engine {
    chain_id: ChainId,
    set: ValidatorSet,
    me: String,
    timeouts: ConsensusTimeouts,

    height: u64,
    round: u64,
    step: Step,
    previous_block_id: Digest32,

    /// Algorithm 1 lines 6-7, `lockedValue_p` and `lockedRound_p`. `None` is
    /// `nil` / `-1`.
    ///
    /// The lock holds the value's **ID** and not the value: every rule that
    /// reads it — lines 23 and 29 — compares `lockedValue_p` to `id(v)`, and
    /// nothing re-proposes it (line 16 re-proposes `validValue_p`, which does
    /// keep the whole value). Holding only the ID is what makes the lock
    /// restorable from a write-ahead log that records `(height, round, phase) ->
    /// block_id` and nothing else, which is [REVIEW-049] RF-002.
    locked: Option<(u64, Digest32)>,
    /// Algorithm 1 lines 8-9, `validValue_p` and `validRound_p`.
    valid: Option<(u64, Value)>,

    log: HeightLog,
    /// Rounds whose line 36 rule has already fired ("for the first time").
    value_quorum_handled: BTreeSet<u64>,
    /// The `(height, round)` a [`Action::RequestValue`] is outstanding for.
    awaiting_value: Option<(u64, u64)>,
}

/// Everything the engine needs to exist.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EngineConfig {
    /// The chain every signature is bound to.
    pub chain_id: ChainId,
    /// The active validator set.
    pub set: ValidatorSet,
    /// This node's `validator_id`. Must be a member of `set`.
    pub validator_id: String,
    /// The local timeouts.
    pub timeouts: ConsensusTimeouts,
    /// The height to start executing.
    pub height: u64,
    /// The ID of the block at `height - 1`, which every proposal at `height`
    /// must name as its `previous_block_id`.
    pub previous_block_id: Digest32,
    /// `lockedRound_p` to start with, for a node resuming `height` after a
    /// restart. `None` is Algorithm 1's `-1`.
    ///
    /// A node that precommitted a block at this height and then died is locked,
    /// and an engine that starts unlocked will happily prevote a different value
    /// in a later round of the same height. That is not double-signing — the
    /// write-ahead log keys votes by round and sees nothing wrong — but it
    /// breaks the locking rule that the quorum-intersection argument of
    /// [ADR-018] rests on. See [REVIEW-049] RF-002.
    ///
    /// Must be `Some` exactly when `locked_block_id` is; a half-specified lock
    /// is rejected at construction rather than silently dropped.
    pub locked_round: Option<u64>,
    /// `lockedValue_p` to start with, as the block ID the lock is on. See
    /// [`Self::locked_round`].
    pub locked_block_id: Option<Digest32>,
}

impl Engine {
    /// Creates an engine and executes `StartRound(0)` — Algorithm 1 line 10.
    ///
    /// # Errors
    ///
    /// Rejects a structurally invalid set, and a `validator_id` that is not a
    /// member of it. A non-member cannot run this engine: it would prevote and
    /// precommit into a set that will discard every one of its signatures, and
    /// failing at construction says so once instead of once per round.
    ///
    /// Also rejects a half-specified restored lock — one of `locked_round` and
    /// `locked_block_id` without the other.
    pub fn start(config: EngineConfig) -> Result<(Self, Vec<Action>)> {
        config.set.check_structure()?;
        let locked = match (config.locked_round, config.locked_block_id) {
            (None, None) => None,
            (Some(round), Some(block_id)) => Some((round, block_id)),
            (round, block_id) => {
                return Err(ConsensusError::IncompleteRestoredLock {
                    has_round: round.is_some(),
                    has_block_id: block_id.is_some(),
                }
                .into());
            }
        };
        if !config
            .set
            .validators
            .iter()
            .any(|entry| entry.validator_id == config.validator_id)
        {
            return Err(ConsensusError::SenderNotAMember {
                validator_id: config.validator_id,
            }
            .into());
        }
        let mut engine = Self {
            chain_id: config.chain_id,
            set: config.set,
            me: config.validator_id,
            timeouts: config.timeouts,
            height: config.height,
            round: 0,
            step: Step::Propose,
            previous_block_id: config.previous_block_id,
            locked,
            valid: None,
            log: HeightLog::default(),
            value_quorum_handled: BTreeSet::new(),
            awaiting_value: None,
        };
        let mut actions = Vec::new();
        engine.start_round(0, &mut actions)?;
        Ok((engine, actions))
    }

    /// The chain every vote this engine emits must be signed under.
    ///
    /// A caller needs it to build the preimage of an [`Outbound::Vote`], and it
    /// is exposed rather than left to the caller's own copy so that a node
    /// cannot sign for one chain while its engine reasons about another.
    #[must_use]
    pub const fn chain_id(&self) -> &ChainId {
        &self.chain_id
    }

    /// The height being executed.
    #[must_use]
    pub const fn height(&self) -> u64 {
        self.height
    }

    /// The current round.
    #[must_use]
    pub const fn round(&self) -> u64 {
        self.round
    }

    /// The current step.
    #[must_use]
    pub const fn step(&self) -> Step {
        self.step
    }

    /// The round this engine is locked at, if it is locked.
    #[must_use]
    pub const fn locked_round(&self) -> Option<u64> {
        match self.locked {
            Some((round, _)) => Some(round),
            None => None,
        }
    }

    /// The block this engine is locked on, if it is locked.
    #[must_use]
    pub const fn locked_block_id(&self) -> Option<Digest32> {
        match self.locked {
            Some((_, block_id)) => Some(block_id),
            None => None,
        }
    }

    /// This node's `validator_id`.
    #[must_use]
    pub fn validator_id(&self) -> &str {
        &self.me
    }

    /// Applies one event and returns the actions it produces.
    ///
    /// # Errors
    ///
    /// Returns an error only for an event that is malformed rather than merely
    /// late or irrelevant: a value offered for a `(height, round)` this engine is
    /// not proposing at, a header that does not carry the pair it is offered
    /// for, or an arithmetic rejection from the set. A message for an old height,
    /// an old round, or a block this engine will not vote for is **not** an
    /// error: those are the ordinary weather of an asynchronous network and the
    /// engine drops them silently, because a caller that had to distinguish them
    /// from real failures would end up ignoring both.
    pub fn step_event(&mut self, event: Event) -> Result<Vec<Action>> {
        let mut actions = Vec::new();
        match event {
            Event::Message(message) => self.record(message.into_inner())?,
            Event::Timeout {
                kind,
                height,
                round,
            } => self.on_timeout(kind, height, round, &mut actions)?,
            Event::Value {
                height,
                round,
                header,
                transactions,
            } => self.on_value(height, round, *header, transactions, &mut actions)?,
        }
        self.drive(&mut actions)?;
        Ok(actions)
    }

    // ---------------------------------------------------------------- log ---

    /// Files a verified message into the height log, dropping what does not
    /// belong to the height being executed.
    fn record(&mut self, message: ConsensusMessage) -> Result<()> {
        match message {
            ConsensusMessage::Proposal {
                proposal,
                block_id,
                validity,
            } => {
                if proposal.height != self.height {
                    return Ok(());
                }
                self.log
                    .participants
                    .entry(proposal.round)
                    .or_default()
                    .insert(
                        proposer_at(&self.set, proposal.height, proposal.round)?
                            .validator_id
                            .clone(),
                    );
                self.log
                    .proposals
                    .entry(proposal.round)
                    .or_insert((*proposal, block_id, validity));
            }
            ConsensusMessage::Prevote(vote) => self.record_vote(VotePhase::Prevote, vote),
            ConsensusMessage::Precommit(vote) => self.record_vote(VotePhase::Precommit, vote),
        }
        Ok(())
    }

    fn record_vote(&mut self, phase: VotePhase, vote: SignedVote) {
        if vote.height != self.height {
            return;
        }
        self.log
            .participants
            .entry(vote.round)
            .or_default()
            .insert(vote.validator_id.clone());
        let (first_of, tally) = match phase {
            VotePhase::Prevote => (&mut self.log.prevote_of, &mut self.log.prevotes),
            VotePhase::Precommit => (&mut self.log.precommit_of, &mut self.log.precommits),
        };
        // Equivocation: the first vote of a `(round, validator)` pair stands and
        // any later different one is dropped. A repeat of the *same* vote is not
        // equivocation and is idempotent here.
        match first_of.entry((vote.round, vote.validator_id.clone())) {
            Entry::Occupied(entry) => {
                if *entry.get() != vote.block_id {
                    return;
                }
            }
            Entry::Vacant(entry) => {
                entry.insert(vote.block_id);
            }
        }
        tally
            .entry((vote.round, vote.block_id))
            .or_default()
            .insert(vote.validator_id, vote.signature);
    }

    // ------------------------------------------------------------- drivers ---

    /// Re-evaluates every `upon` rule until none fires.
    ///
    /// Algorithm 1's rules are conditions on the message log, not handlers for
    /// message arrivals: a rule can become true because of a message that
    /// arrived three events ago and only now completes a quorum. Re-evaluating
    /// the whole set after every event is the direct reading of that, and the
    /// loop terminates because each rule that fires either advances `step`,
    /// advances `round`, advances `height`, or marks a round handled — all
    /// monotone.
    fn drive(&mut self, actions: &mut Vec<Action>) -> Result<()> {
        loop {
            if self.try_decide(actions)? {
                continue;
            }
            if self.try_skip_round(actions)? {
                continue;
            }
            if self.try_prevote_on_proposal(actions)? {
                continue;
            }
            if self.try_lock_and_precommit(actions)? {
                continue;
            }
            return Ok(());
        }
    }

    /// Algorithm 1 lines 11-21, `StartRound(round)`.
    fn start_round(&mut self, round: u64, actions: &mut Vec<Action>) -> Result<()> {
        self.round = round;
        self.step = Step::Propose;
        self.awaiting_value = None;
        // Line 21 schedules this only for a non-proposer. Here it is scheduled
        // for everyone; see [`super`] §*Divergence 2*. It is a superset: the
        // guard on `OnTimeoutPropose` is `step_p = propose`, and a proposer that
        // has proposed is past it.
        actions.push(Action::ScheduleTimeout {
            kind: TimeoutKind::Propose,
            height: self.height,
            round,
            delay_ms: self.timeouts.delay_ms(TimeoutKind::Propose, round),
        });
        if proposer_at(&self.set, self.height, round)?.validator_id != self.me {
            return Ok(());
        }
        // Lines 15-19. `validValue_p` is re-proposed unchanged, which is why the
        // header it carries keeps the round it was first proposed at.
        if let Some((valid_round, value)) = self.valid.clone() {
            let proposal = BlockProposal {
                height: self.height,
                round,
                valid_round: Some(valid_round),
                header: value.header,
                transactions: value.transactions,
            };
            actions.push(Action::Broadcast(Outbound::Proposal(Box::new(proposal))));
        } else {
            self.awaiting_value = Some((self.height, round));
            actions.push(Action::RequestValue {
                height: self.height,
                round,
            });
        }
        Ok(())
    }

    /// Algorithm 1 lines 57-67, the three timeout functions, with the nil
    /// broadcasts replaced by the step transition alone. See [`super`]
    /// §*Divergence 1*.
    fn on_timeout(
        &mut self,
        kind: TimeoutKind,
        height: u64,
        round: u64,
        actions: &mut Vec<Action>,
    ) -> Result<()> {
        if height != self.height || round != self.round {
            return Ok(());
        }
        match kind {
            TimeoutKind::Propose if self.step == Step::Propose => {
                self.enter_prevote(actions);
            }
            TimeoutKind::Prevote if self.step == Step::Prevote => {
                self.enter_precommit(actions);
            }
            TimeoutKind::Precommit => {
                let next = round
                    .checked_add(1)
                    .ok_or(Error::Arithmetic("consensus round overflows u64"))?;
                self.start_round(next, actions)?;
            }
            _ => {}
        }
        Ok(())
    }

    /// Sets `step_p := prevote` and arms the next timeout in the chain.
    fn enter_prevote(&mut self, actions: &mut Vec<Action>) {
        self.step = Step::Prevote;
        actions.push(Action::ScheduleTimeout {
            kind: TimeoutKind::Prevote,
            height: self.height,
            round: self.round,
            delay_ms: self.timeouts.delay_ms(TimeoutKind::Prevote, self.round),
        });
    }

    /// Sets `step_p := precommit` and arms the last timeout in the chain.
    fn enter_precommit(&mut self, actions: &mut Vec<Action>) {
        self.step = Step::Precommit;
        actions.push(Action::ScheduleTimeout {
            kind: TimeoutKind::Precommit,
            height: self.height,
            round: self.round,
            delay_ms: self.timeouts.delay_ms(TimeoutKind::Precommit, self.round),
        });
    }

    /// Algorithm 1 line 65-67 reached from a `getValue()` reply.
    fn on_value(
        &mut self,
        height: u64,
        round: u64,
        header: BlockHeader,
        transactions: Vec<JsonObject>,
        actions: &mut Vec<Action>,
    ) -> Result<()> {
        if self.awaiting_value != Some((height, round)) {
            return Err(ConsensusError::UnsolicitedValue { height, round }.into());
        }
        if header.height != height {
            return Err(ConsensusError::ProposalHeaderMismatch { field: "height" }.into());
        }
        if header.round != round {
            return Err(ConsensusError::ProposalHeaderMismatch { field: "round" }.into());
        }
        self.awaiting_value = None;
        actions.push(Action::Broadcast(Outbound::Proposal(Box::new(
            BlockProposal {
                height,
                round,
                valid_round: None,
                header,
                transactions,
            },
        ))));
        Ok(())
    }

    // --------------------------------------------------------------- rules ---

    /// Algorithm 1 lines 22-33: the two prevote rules.
    ///
    /// They are one function because they differ in exactly two places — which
    /// `valid_round` the proposal carries, and which unlocking condition applies
    /// — and because writing them twice is how the two conditions drift apart.
    fn try_prevote_on_proposal(&mut self, actions: &mut Vec<Action>) -> Result<bool> {
        if self.step != Step::Propose {
            return Ok(false);
        }
        let Some((proposal, block_id, validity)) = self.log.proposals.get(&self.round) else {
            return Ok(false);
        };
        let block_id = *block_id;
        let proposal_valid_round = proposal.valid_round;
        let acceptable = *validity == Validity::Valid && self.links_to_the_chain(&proposal.header);

        let unlocked = match proposal_valid_round {
            // Line 23: `lockedRound_p = -1 ∨ lockedValue_p = v`.
            None => match self.locked {
                None => true,
                Some((_, locked_block_id)) => locked_block_id == block_id,
            },
            // Line 28's guard needs `2f+1` prevotes for `id(v)` at `vr`, and
            // line 29 is `lockedRound_p <= vr ∨ lockedValue_p = v`. See
            // [`super`] §*Divergence 4* for why the comparison below is strict.
            Some(valid_round) => {
                if !self.prevote_quorum_for(valid_round, block_id)? {
                    return Ok(false);
                }
                match self.locked {
                    None => true,
                    Some((locked_round, locked_block_id)) => {
                        locked_round < valid_round || locked_block_id == block_id
                    }
                }
            }
        };

        if acceptable && unlocked {
            actions.push(Action::Broadcast(Outbound::Vote {
                phase: VotePhase::Prevote,
                height: self.height,
                round: self.round,
                block_id,
            }));
        }
        // Lines 27 and 33: `step_p := prevote`, whether or not a prevote went
        // out. Without a nil vote, "prevote nothing" is the else branch's whole
        // effect, and the step still moves so the round can time out.
        self.enter_prevote(actions);
        Ok(true)
    }

    /// Algorithm 1 lines 36-43: lock, precommit, and record the valid value.
    fn try_lock_and_precommit(&mut self, actions: &mut Vec<Action>) -> Result<bool> {
        if self.step < Step::Prevote || self.value_quorum_handled.contains(&self.round) {
            return Ok(false);
        }
        let round = self.round;
        let Some((proposal, block_id, validity)) = self.log.proposals.get(&round) else {
            return Ok(false);
        };
        if *validity != Validity::Valid || !self.links_to_the_chain(&proposal.header) {
            return Ok(false);
        }
        let block_id = *block_id;
        let value = Value {
            header: proposal.header.clone(),
            transactions: proposal.transactions.clone(),
            block_id,
        };
        if !self.prevote_quorum_for(round, block_id)? {
            return Ok(false);
        }
        self.value_quorum_handled.insert(round);
        // Line 37: the precommit and the lock happen only from `prevote`. A node
        // that has already precommitted this round does not precommit again, and
        // that — not a detection rule — is what makes a second, different
        // precommit from this node impossible.
        if self.step == Step::Prevote {
            self.locked = Some((round, block_id)); // lines 38-39
            actions.push(Action::Broadcast(Outbound::Vote {
                phase: VotePhase::Precommit,
                height: self.height,
                round,
                block_id,
            })); // line 40
            self.enter_precommit(actions); // line 41
        }
        self.valid = Some((round, value)); // lines 42-43
        Ok(true)
    }

    /// Algorithm 1 lines 49-54: decide, and move to the next height.
    fn try_decide(&mut self, actions: &mut Vec<Action>) -> Result<bool> {
        let total_power = self.set.total_voting_power()?;
        let mut decision: Option<(u64, Digest32)> = None;
        for (&(round, block_id), voters) in &self.log.precommits {
            let Some((_, proposed_id, validity)) = self.log.proposals.get(&round) else {
                continue;
            };
            if *proposed_id != block_id || *validity != Validity::Valid {
                continue;
            }
            if quorum(self.power_of_voters(voters)?, total_power)? {
                decision = Some((round, block_id));
                break;
            }
        }
        let Some((round, block_id)) = decision else {
            return Ok(false);
        };

        let (header, transactions) = self
            .log
            .proposals
            .get(&round)
            .map(|(proposal, _, _)| (proposal.header.clone(), proposal.transactions.clone()))
            .expect("the decision loop only selects rounds whose proposal it read");
        let mut signatures: Vec<CertificateSignature> = self
            .log
            .precommits
            .get(&(round, block_id))
            .expect("the decision loop only selects tallies it read")
            .iter()
            .map(|(validator_id, signature)| CertificateSignature {
                validator_id: validator_id.clone(),
                signature: *signature,
            })
            .collect();
        // The tally is a `BTreeMap` keyed by `validator_id`, so this is already
        // in order; sorting is here because "unique and sorted by validator ID"
        // is a rule of the document and a rule a reader should be able to find
        // being applied, not a property to infer from a container choice.
        signatures.sort_by(|left, right| left.validator_id.cmp(&right.validator_id));

        let finalized = FinalizedBlock {
            header,
            transactions,
            quorum_certificate: QuorumCertificate {
                height: self.height,
                round,
                block_id,
                validator_set_hash: self.set.hash()?,
                signatures,
            },
        };
        actions.push(Action::Finalize(Box::new(finalized)));

        // Lines 52-54.
        self.previous_block_id = block_id;
        self.height = self
            .height
            .checked_add(1)
            .ok_or(Error::Arithmetic("consensus height overflows u64"))?;
        self.locked = None;
        self.valid = None;
        self.log = HeightLog::default();
        self.value_quorum_handled.clear();
        self.start_round(0, actions)?;
        Ok(true)
    }

    /// Algorithm 1 lines 55-56: skip to a round others have already reached.
    fn try_skip_round(&mut self, actions: &mut Vec<Action>) -> Result<bool> {
        let total_power = self.set.total_voting_power()?;
        let mut target = None;
        for (&round, participants) in &self.log.participants {
            if round <= self.round {
                continue;
            }
            if one_correct_threshold(self.power_of_participants(participants)?, total_power)? {
                target = Some(round);
                break;
            }
        }
        let Some(round) = target else {
            return Ok(false);
        };
        self.start_round(round, actions)?;
        Ok(true)
    }

    // ------------------------------------------------------------- helpers ---

    /// Whether more than two thirds of the set's power has prevoted `block_id`
    /// at `round`.
    fn prevote_quorum_for(&self, round: u64, block_id: Digest32) -> Result<bool> {
        let Some(voters) = self.log.prevotes.get(&(round, block_id)) else {
            return Ok(false);
        };
        quorum(
            self.power_of_voters(voters)?,
            self.set.total_voting_power()?,
        )
    }

    /// The engine's own share of `valid(v)`: the part it can answer without an
    /// executor.
    ///
    /// A header at the wrong height, or one that does not extend the block this
    /// engine has already finalized, is not a value this engine may prevote at
    /// any price — and unlike `state_root`, both are decidable here.
    fn links_to_the_chain(&self, header: &BlockHeader) -> bool {
        header.height == self.height && header.previous_block_id == self.previous_block_id
    }

    /// Sums the voting power of a vote tally's signers.
    fn power_of_voters(&self, voters: &BTreeMap<String, [u8; 64]>) -> Result<u64> {
        let mut total: u64 = 0;
        for validator_id in voters.keys() {
            total = total
                .checked_add(self.power_of_one(validator_id))
                .ok_or(Error::Arithmetic("summed consensus power overflows u64"))?;
        }
        Ok(total)
    }

    /// Sums the voting power of the validators heard from at a round.
    fn power_of_participants(&self, participants: &BTreeSet<String>) -> Result<u64> {
        let mut total: u64 = 0;
        for validator_id in participants {
            total = total
                .checked_add(self.power_of_one(validator_id))
                .ok_or(Error::Arithmetic("summed consensus power overflows u64"))?;
        }
        Ok(total)
    }

    /// One member's voting power, or zero if the name is not a member.
    ///
    /// Zero rather than an error: the boundary already rejects a non-member's
    /// message, so reaching this with an unknown name would mean the set changed
    /// underneath the log, and counting an unknown name as zero is the direction
    /// that fails closed — it can only make a quorum harder to reach.
    fn power_of_one(&self, validator_id: &str) -> u64 {
        self.set
            .validators
            .iter()
            .find(|entry| entry.validator_id == validator_id)
            .map_or(0, |entry| entry.voting_power)
    }
}

/// `signed_power * 3 > total_power`: more than one third of the power.
///
/// **This is not a quorum and must never be used as one.** It is Algorithm 1's
/// `f+1`, expressed in power because this protocol weights by power, and its
/// meaning is the opposite of a quorum's: a quorum is "enough that no other
/// quorum can disagree", this is "enough that at least one of them is honest".
/// It authorizes exactly one thing — following the crowd to a higher round,
/// which decides nothing — and it is deliberately **not** in
/// [`crate::quorum`], whose module documentation states that there is one
/// predicate there and no variants. A second spelling living next to the first
/// is precisely how the two would eventually be confused.
///
/// No validity rule of this protocol compares anything to this threshold, and
/// nothing signed carries it.
fn one_correct_threshold(signed_power: u64, total_power: u64) -> Result<bool> {
    if total_power == 0 {
        return Err(Error::Arithmetic("round-skip over zero total power"));
    }
    let signed = u128::from(signed_power)
        .checked_mul(3)
        .ok_or(Error::Arithmetic("round-skip signed_power * 3"))?;
    Ok(signed > u128::from(total_power))
}
