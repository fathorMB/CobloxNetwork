//! The three consensus messages, and the boundary that admits them.
//!
//! `wire.md#block_proposal`, `wire.md#prevote` and `wire.md#precommit` publish
//! the payload schemas; this module is their Rust form. [ADR-018] §4 fixes that
//! there are exactly three and that they travel on the `SignedEnvelope` already
//! specified, which is why nothing here carries a `network_id`, a `nonce` or an
//! expiry: those are the envelope's fields and duplicating them would give a
//! receiver two answers to the same question.
//!
//! # Two signatures, and only one of them is in this file
//!
//! A **prevote** and a **precommit** each carry their own 64-byte signature over
//! their own domain-separated preimage, because those signatures outlive the
//! connection that carried them: a precommit becomes an entry of a
//! [`QuorumCertificate`](super::QuorumCertificate) that a light client verifies
//! years later, and a prevote is what a validator points at to justify a lock.
//!
//! A **proposal** carries none. Its authenticity is the envelope's
//! `coblox-wire-envelope-v0` signature over `sender_node_id`, and
//! [`verify_proposal`] therefore takes the authenticated sender as a parameter
//! instead of checking bytes. This is a deliberate limit and not an oversight:
//! [ADR-018] authorizes **one** new signature domain, and a fourth would be
//! published surface the decision does not carry. The residual is named where a
//! reader can act on it — a proposer that sends two different proposals in one
//! round is detectable by anyone who receives both, but is **not attributable
//! from a payload alone**, so proposal equivocation cannot be turned into
//! on-chain evidence the way a double precommit could. Safety does not depend on
//! it: a forged or doubled proposal can only make a round fail, because every
//! rule that can finalize anything counts signed votes.

use crate::block::BlockHeader;
use crate::error::{ConsensusError, JsonError, Result};
use crate::hash::{ChainId, Digest32, Domain};
use crate::json::{Json, JsonObject};
use crate::registry::{block_prevote_preimage, block_vote_preimage};
use crate::validator_set::ValidatorSet;
use crate::verifier::verify_in_context;
use crate::{SignatureVerifier, encoding};

use super::proposer::proposer_at;

/// Which of the two vote phases a signed vote belongs to.
///
/// The two phases are the same six fields under two domain separators, so the
/// phase is not recoverable from the payload: it is the domain a verifier
/// chooses, and this enum is how a caller says which one it means.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VotePhase {
    /// The first phase, `coblox-block-prevote-v0`. Added by [ADR-018].
    Prevote,
    /// The second phase, `coblox-block-vote-v0`. It predates the protocol that
    /// gives it a name.
    Precommit,
}

impl VotePhase {
    /// The signature domain of this phase.
    #[must_use]
    pub const fn domain(self) -> Domain {
        match self {
            Self::Prevote => Domain::SIG_BLOCK_PREVOTE,
            Self::Precommit => Domain::SIG_BLOCK_VOTE,
        }
    }

    /// The `message_type` this phase travels under.
    #[must_use]
    pub const fn message_type(self) -> &'static str {
        match self {
            Self::Prevote => "prevote",
            Self::Precommit => "precommit",
        }
    }
}

/// A signed vote: a prevote or a precommit, depending on the domain it was
/// verified under.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SignedVote {
    /// The height voted on.
    pub height: u64,
    /// The round voted in.
    pub round: u64,
    /// The block voted for. There is no nil vote in this protocol; see
    /// [`super`] §*The votes this protocol does not have*.
    pub block_id: Digest32,
    /// The voting member's `validator_id`.
    pub validator_id: String,
    /// The signature over the phase's preimage of `(height, round, block_id)`.
    pub signature: [u8; 64],
}

/// The field names of a vote payload.
const VOTE_FIELDS: [&str; 5] = ["block_id", "height", "round", "signature", "validator_id"];

impl SignedVote {
    /// The canonical payload object.
    pub fn to_json(&self) -> Result<JsonObject> {
        JsonObject::builder()
            .digest("block_id", &self.block_id)
            .uint("height", self.height)
            .uint("round", self.round)
            .bytes("signature", &self.signature)
            .str("validator_id", &self.validator_id)
            .build()
    }

    /// Reads a vote payload, rejecting unknown fields.
    pub fn from_json(object: &JsonObject) -> Result<Self> {
        object.reject_unknown_fields(&VOTE_FIELDS)?;
        Ok(Self {
            height: object.uint("height")?,
            round: object.uint("round")?,
            block_id: object.digest("block_id")?,
            validator_id: object.string("validator_id")?.to_owned(),
            signature: encoding::base64url_decode_fixed::<64>(
                object.string("signature")?,
                "consensus vote signature",
            )?,
        })
    }
}

/// A block proposal: the value, plus the round whose prevotes justify it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BlockProposal {
    /// The height proposed at.
    pub height: u64,
    /// The round proposed in.
    pub round: u64,
    /// `validRound`: the round at which more than two thirds of prevotes for
    /// this value were seen, when the value is being carried across rounds.
    ///
    /// `None` is Algorithm 1's `-1`, spelled as an absent field because the
    /// protocol's JSON has no `null` — see [`crate::json::Json`], which cannot
    /// represent one. `ValidatorSet::election` is the same idiom.
    pub valid_round: Option<u64>,
    /// The proposed header.
    pub header: BlockHeader,
    /// The proposed transactions, in canonical execution order.
    ///
    /// Opaque here. The engine neither executes nor orders them; the value it
    /// agrees on is `block_id`, and `block_id` covers them through
    /// `transactions_root`.
    pub transactions: Vec<JsonObject>,
}

/// The field names of a proposal payload, with `valid_round` optional.
const PROPOSAL_FIELDS: [&str; 5] = ["header", "height", "round", "transactions", "valid_round"];

impl BlockProposal {
    /// `id(v)`: the ID of the proposed block.
    pub fn block_id(&self, chain_id: &ChainId) -> Result<Digest32> {
        self.header.block_id(chain_id)
    }

    /// The canonical payload object.
    pub fn to_json(&self) -> Result<JsonObject> {
        let mut builder = JsonObject::builder()
            .object("header", self.header.to_json()?)
            .uint("height", self.height)
            .uint("round", self.round)
            .array(
                "transactions",
                self.transactions
                    .iter()
                    .cloned()
                    .map(Json::Object)
                    .collect(),
            );
        if let Some(valid_round) = self.valid_round {
            builder = builder.uint("valid_round", valid_round);
        }
        builder.build()
    }

    /// Reads a proposal payload, rejecting unknown fields.
    pub fn from_json(object: &JsonObject) -> Result<Self> {
        object.reject_unknown_fields(&PROPOSAL_FIELDS)?;
        let mut transactions = Vec::new();
        for entry in object.array("transactions")? {
            let Json::Object(entry) = entry else {
                return Err(JsonError::NotAnObject.into());
            };
            transactions.push(entry.clone());
        }
        let valid_round = match object.get("valid_round") {
            Some(_) => Some(object.uint("valid_round")?),
            None => None,
        };
        Ok(Self {
            height: object.uint("height")?,
            round: object.uint("round")?,
            valid_round,
            header: BlockHeader::from_json(object.object("header")?)?,
            transactions,
        })
    }
}

/// The caller's verdict on a proposed value, Algorithm 1's `valid(v)`.
///
/// The engine cannot answer this: deciding whether `state_root` is the result of
/// executing `transactions` needs an executor and the account state, neither of
/// which is a consensus concern and neither of which is in this crate. Passing
/// it in at verification time is also *where* Algorithm 1 asks the question —
/// `valid(v)` is evaluated at the moment a proposal is received — so nothing is
/// deferred by taking it here.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Validity {
    /// The caller has executed the block and it is valid.
    Valid,
    /// The caller rejects the block. The engine will not prevote it, and will
    /// not lock on it.
    Invalid,
}

/// One of the three messages, after it has passed the boundary.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConsensusMessage {
    /// A proposal from the round's proposer, with the caller's `valid(v)`.
    Proposal {
        /// The proposal itself.
        proposal: Box<BlockProposal>,
        /// `id(v)`, recomputed from the header at verification time.
        block_id: Digest32,
        /// The caller's verdict on the value.
        validity: Validity,
    },
    /// A prevote whose signature verified under `coblox-block-prevote-v0`.
    Prevote(SignedVote),
    /// A precommit whose signature verified under `coblox-block-vote-v0`.
    Precommit(SignedVote),
}

/// A message the engine is willing to consume.
///
/// The inner value is private and the only constructors are [`verify_proposal`],
/// [`verify_vote`] and their siblings, so an engine cannot be fed a message
/// whose signature nobody checked. It is the same shape as
/// [`crate::params::ValidatedConsensusParameters`]: a type whose existence is
/// the evidence.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct VerifiedMessage(ConsensusMessage);

impl VerifiedMessage {
    /// The message inside.
    #[must_use]
    pub const fn get(&self) -> &ConsensusMessage {
        &self.0
    }

    /// Consumes the wrapper.
    #[must_use]
    pub fn into_inner(self) -> ConsensusMessage {
        self.0
    }
}

/// Admits a proposal from `sender_validator_id`.
///
/// The checks, in order, and every one of them is a rejection a receiver must be
/// able to make **before** the value reaches any rule that can lock:
///
/// 1. the sender is a member of `set`;
/// 2. the sender is the proposer of `(height, round)` under
///    [`proposer_at`]. This is the check that makes the proposer rule a rule and
///    not a convention;
/// 3. the header's `height` and `round` are the message's. A header that
///    disagreed with its envelope would make `block_id` a value nobody could
///    recompute from what they thought they had agreed on;
/// 4. `valid_round`, when present, is strictly below `round` — Algorithm 1's
///    `vr >= 0 ∧ vr < round_p`, checked here so that the engine never has to
///    consider a proposal that justifies itself with its own round.
///
/// `validity` is the caller's `valid(v)` and is carried, not checked.
pub fn verify_proposal(
    chain_id: &ChainId,
    set: &ValidatorSet,
    sender_validator_id: &str,
    proposal: BlockProposal,
    validity: Validity,
) -> Result<VerifiedMessage> {
    if !set
        .validators
        .iter()
        .any(|entry| entry.validator_id == sender_validator_id)
    {
        return Err(ConsensusError::SenderNotAMember {
            validator_id: sender_validator_id.to_owned(),
        }
        .into());
    }
    let expected = proposer_at(set, proposal.height, proposal.round)?;
    if expected.validator_id != sender_validator_id {
        return Err(ConsensusError::NotTheProposer {
            height: proposal.height,
            round: proposal.round,
            expected: expected.validator_id.clone(),
            actual: sender_validator_id.to_owned(),
        }
        .into());
    }
    if proposal.header.height != proposal.height {
        return Err(ConsensusError::ProposalHeaderMismatch { field: "height" }.into());
    }
    if let Some(valid_round) = proposal.valid_round
        && valid_round >= proposal.round
    {
        return Err(ConsensusError::ProposalValidRoundNotBelowRound {
            round: proposal.round,
            valid_round,
        }
        .into());
    }
    let block_id = proposal.block_id(chain_id)?;
    Ok(VerifiedMessage(ConsensusMessage::Proposal {
        proposal: Box::new(proposal),
        block_id,
        validity,
    }))
}

/// Admits a prevote or a precommit.
///
/// The signature is verified through [`verify_in_context`] under the phase's own
/// domain, so a prevote presented as a precommit — the one confusion that would
/// let a single signature both lock a validator and finalize a block — fails on
/// the domain before it fails on anything else.
pub fn verify_vote<V: SignatureVerifier + ?Sized>(
    chain_id: &ChainId,
    set: &ValidatorSet,
    phase: VotePhase,
    vote: SignedVote,
    verifier: &V,
) -> Result<VerifiedMessage> {
    let member = set
        .validators
        .iter()
        .find(|entry| entry.validator_id == vote.validator_id)
        .ok_or_else(|| ConsensusError::SenderNotAMember {
            validator_id: vote.validator_id.clone(),
        })?;
    let preimage = match phase {
        VotePhase::Prevote => {
            block_prevote_preimage(chain_id, vote.height, vote.round, &vote.block_id)
        }
        VotePhase::Precommit => {
            block_vote_preimage(chain_id, vote.height, vote.round, &vote.block_id)
        }
    };
    if !verify_in_context(
        verifier,
        phase.domain(),
        chain_id,
        &member.consensus_public_key,
        &preimage,
        &vote.signature,
    ) {
        return Err(ConsensusError::InvalidSignature {
            validator_id: vote.validator_id,
        }
        .into());
    }
    Ok(VerifiedMessage(match phase {
        VotePhase::Prevote => ConsensusMessage::Prevote(vote),
        VotePhase::Precommit => ConsensusMessage::Precommit(vote),
    }))
}
