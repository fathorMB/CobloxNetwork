//! The consensus engine: the part of the protocol that *reaches* agreement.
//!
//! Everything else in this crate **verifies**. This module is the first that
//! produces: it turns messages and elapsed timers into a chain of finalized
//! blocks with certificates the rest of the crate already knew how to check.
//! [ADR-018] is the decision it implements and this documentation does not
//! repeat it; what is here is the comparison with the source the safety rules
//! were taken from, and the four places where this implementation is not that
//! source.
//!
//! # The source, named
//!
//! > Ethan Buchman, Jae Kwon, Zarko Milosevic, **"The latest gossip on BFT
//! > consensus"**, arXiv:1807.04938, **Algorithm 1 — Tendermint consensus
//! > algorithm**.
//!
//! The comparison below was made against the LaTeX of the algorithm in the
//! paper's own e-print, not against a description of it and not from memory:
//!
//! ```text
//! https://arxiv.org/e-print/1807.04938
//!   sha256(tarball)       138b688f2c8e4dee0ee89b7574aafa7cd99d43bbb8fdca3fc4cba9ee17bbc29f
//!   sha256(consensus.tex) 7fa4253844ac93c4ef23a3ffeaf4c1fd36c6e2f5e04aec8fbfbbbca4c09f8d3f
//! ```
//!
//! `consensus.tex` holds Algorithm 1 as an `algorithmic` environment; the line
//! numbers used throughout this module are that environment's, counting `\STATE`,
//! `\IF`, `\UPON` and `\FUNCTION` from 1, which is what the typeset paper prints
//! in its margin. [ADR-018] §2 states the same rules in prose and the two agree;
//! where the prose and the algorithm could be read differently, the algorithm
//! decides, and the one place that happened is *Divergence 4* below.
//!
//! # The locking rule, line by line
//!
//! | Algorithm 1 | Here |
//! | --- | --- |
//! | 11-21 `StartRound` | [`Engine::start_round`] |
//! | 14-19 proposer re-proposes `validValue_p`, else `getValue()` | same, with `getValue()` inverted into [`Action::RequestValue`] |
//! | 22-27 prevote on a proposal with `vr = -1` | [`Engine::try_prevote_on_proposal`], `valid_round: None` arm |
//! | 23 `valid(v) ∧ (lockedRound_p = -1 ∨ lockedValue_p = v)` | the `None` arm's `unlocked`, with `acceptable` carrying `valid(v)` |
//! | 28-33 prevote on a proposal with `0 ≤ vr < round_p` | the `Some(valid_round)` arm |
//! | 28 `AND 2f+1 ⟨PREVOTE, h_p, vr, id(v)⟩` | `prevote_quorum_for(valid_round, block_id)` |
//! | 29 `valid(v) ∧ (lockedRound_p ≤ vr ∨ lockedValue_p = v)` | `*locked_round < valid_round ‖ locked.block_id == block_id` — **Divergence 4** |
//! | 34-35 `2f+1` prevotes of any kind schedule `OnTimeoutPrevote` | **Divergence 1**: armed unconditionally at the step change |
//! | 36 proposal at `round_p` **AND** `2f+1` prevotes for `id(v)`, `valid(v)`, `step_p ≥ prevote`, first time | [`Engine::try_lock_and_precommit`] |
//! | 37-39 `if step_p = prevote` then `lockedValue_p := v`, `lockedRound_p := round_p` | same |
//! | 40-41 broadcast `PRECOMMIT id(v)`, `step_p := precommit` | same |
//! | 42-43 `validValue_p := v`, `validRound_p := round_p`, unconditionally | same |
//! | 44-46 `2f+1` prevotes for nil, precommit nil | **Divergence 1** |
//! | 47-48 `2f+1` precommits of any kind schedule `OnTimeoutPrecommit` | **Divergence 1** |
//! | 49-51 proposal at any `r` **AND** `2f+1` precommits for `id(v)`, `valid(v)` | [`Engine::try_decide`] |
//! | 52-54 `h_p := h_p + 1`, reset locks and log, `StartRound(0)` | same, plus [`Action::Finalize`] |
//! | 55-56 `f+1` messages at `round > round_p` | [`Engine::try_skip_round`], with `f+1` read as power — **Divergence 3** |
//! | 57-60 `OnTimeoutPropose` | [`Engine::on_timeout`], `Propose` arm |
//! | 61-64 `OnTimeoutPrevote` | `Prevote` arm |
//! | 65-67 `OnTimeoutPrecommit` → `StartRound(round_p + 1)` | `Precommit` arm |
//!
//! ## Divergence 1 — there are no nil votes, so the timeouts chain themselves
//!
//! Algorithm 1 has four broadcasts of a **nil** vote (lines 26, 32, 45, 59, 63)
//! and two rules that count them (34, 44, 47). Coblox has no nil vote and cannot
//! grow one inside this spec: a vote signs
//! `... || raw_32_bytes(block_id)` and `ledger.md#what-validators-sign` is a
//! published preimage that [ADR-018] declares unchanged. Spelling nil as 32 zero
//! bytes would be a new meaning for a published preimage, which is the premise
//! the decision rests on, so it was not done.
//!
//! What nil votes are *for* in Algorithm 1 is not safety — no nil vote ever
//! locks anything or decides anything — it is **arming the next timer**. Lines
//! 35 and 48 schedule `OnTimeoutPrevote` and `OnTimeoutPrecommit` only once a
//! quorum of votes of any kind has been seen, and without nils those quorums may
//! never form, so the chain of timers would break at its first link and a round
//! would never end.
//!
//! Here each timer is armed at the step transition instead:
//! `Propose → Prevote` arms `OnTimeoutPrevote`, `Prevote → Precommit` arms
//! `OnTimeoutPrecommit`, and `OnTimeoutPrecommit` starts the next round. The
//! substitution is a **superset**: every state in which Algorithm 1 would have
//! armed a timer is a state this engine has already armed it in, and there are
//! states — a round nobody votes in at all — where this engine arms one and
//! Algorithm 1 does not. It cannot affect safety, because no rule that locks,
//! precommits or decides reads a timer; a timer can only abandon a round, and
//! abandoning a round is what a locked validator's lock survives.
//!
//! The cost is real and is latency: a round that has visibly failed is abandoned
//! when its timer says so rather than when two thirds have said nil, so a failed
//! round costs `propose + prevote + precommit` of local timeout instead of a
//! round trip. The gain is that nothing published changes.
//!
//! ## Divergence 2 — the proposer arms `OnTimeoutPropose` too
//!
//! Algorithm 1 line 21 schedules `OnTimeoutPropose` in the `else` of "am I the
//! proposer". [`Engine::start_round`] schedules it in both branches. The guard
//! inside `OnTimeoutPropose` is `step_p = propose`, and a proposer that has
//! proposed is past `propose` by the time it hears its own proposal — so the
//! extra timer fires only for a proposer whose own value never arrived, where
//! Algorithm 1 would have hung. It is again a superset, for the same reason.
//!
//! ## Divergence 3 — `f+1` is a share of power, not a count of processes
//!
//! Algorithm 1 counts processes because its model gives each one vote. Coblox
//! weights by `voting_power` — the quorum predicate does, the certificate does —
//! so `f+1` here is [`engine::one_correct_threshold`], `signed_power * 3 >
//! total_power`. It is the same statement in the protocol's own units: more than
//! a third of the power cannot be entirely faulty when less than a third is.
//!
//! It is deliberately **not** in [`crate::quorum`], whose documentation says
//! there is one predicate there and no variants. This one is not a quorum and
//! authorizes only the round skip, which decides nothing.
//!
//! ## Divergence 4 — the unlock comparison is strict, and [ADR-018] asked for it
//!
//! Algorithm 1 line 29 is `lockedRound_p ≤ vr ∨ lockedValue_p = v`. [ADR-018] §2
//! says a locked validator unlocks on seeing more than two thirds of prevotes
//! for a different block at a round **maggiore** — greater — than its lock's.
//! This engine implements the ADR: `*locked_round < valid_round`.
//!
//! The difference is the case `lockedRound_p = vr` with `v ≠ lockedValue_p`, and
//! that case cannot occur while fewer than a third of the power is faulty: two
//! prevote quorums at the same round for different blocks would have to overlap
//! in more than a third of the power, and every process in the overlap would
//! have prevoted twice in one round. So under the fault assumption the two
//! spellings agree, and outside it the strict one is the more restrictive of the
//! two — it unlocks in strictly fewer situations, and unlocking is the only move
//! in this rule that can cost safety.
//!
//! **This is a divergence from the source and it is written down as one**, with
//! the direction of the difference named, because a locking rule that quietly
//! differs from the paper it claims to implement is exactly the defect
//! `GATE-LOCKING-FROM-SOURCE` exists to catch. It is reported to the Lead as a
//! discrepancy between [ADR-018] §2 and Algorithm 1 line 29 rather than resolved
//! by the implementer.
//!
//! # Two rounds, and why they are allowed to differ
//!
//! A `BlockHeader` carries a `round` and so does a [`QuorumCertificate`], and
//! they are **not** always equal.
//!
//! Algorithm 1 line 16 re-proposes `validValue_p` **unchanged**. It has to:
//! prevotes at `vr` are for `id(v)`, so a proposer that rewrote a field of the
//! header before re-proposing would change `block_id` and the carried-over
//! prevotes would justify nothing — the lock would be unreachable and the height
//! would stall exactly as [ADR-018] describes for the one-phase alternative. So
//! `header.round` is the round the block was **first proposed** at, and
//! `quorum_certificate.round` is the round it was **finalized** at. They are
//! equal whenever a height succeeds at the first attempt that produced a value,
//! which is the ordinary case.
//!
//! [ADR-018]'s consequence section says of `round` that "il suo valore nel blocco
//! finalizzato dice a quale tentativo quell'altezza è riuscita". That is true of
//! the certificate's round and only sometimes true of the header's, and no
//! implementation can make it true of the header's without breaking the lock.
//! It is reported as a finding rather than papered over here.
//!
//! # The votes this protocol does not have
//!
//! There is no nil vote, and there is no proposal signature. Both absences are
//! consequences of [ADR-018]'s premise that one domain is added and none is
//! changed, both are named where a reader meets them —
//! [`messages`] for the second, *Divergence 1* for the first — and neither
//! weakens safety, because every rule that can finalize anything counts signed
//! votes for a specific `block_id`.
//!
//! # No I/O, and how that is shown
//!
//! The claim is not that this module refrains from I/O. It is that its interface
//! gives it nowhere to put any, and that is checkable rather than assertable:
//!
//! * [`Engine`] is not generic, has no lifetime parameter, and holds no trait
//!   object, no closure and no handle. Its fields are `ChainId`, `ValidatorSet`,
//!   `String`, `ConsensusTimeouts`, `u64`, `Step`, `Digest32`, ordered maps and
//!   `Option`s of those — every one a plain value.
//! * [`Engine::step_event`] takes an [`Event`] and returns `Vec<Action>`. Both
//!   are plain-data enums. There is no callback to hand it, so there is no seam
//!   an implementor could route a socket through — the shape
//!   [REVIEW-022] found in `pub(crate)`, a guarantee held by a name, is not the
//!   shape here.
//! * The three things a consensus engine would otherwise reach out for are
//!   inverted: **time** arrives as [`Event::Timeout`] and leaves as
//!   [`Action::ScheduleTimeout`]; **the value to propose** arrives as
//!   [`Event::Value`] after [`Action::RequestValue`]; **the signing key** never
//!   arrives at all, because [`Action::Broadcast`] emits votes unsigned and the
//!   caller signs them.
//! * Signature *verification* is at the boundary, in [`messages::verify_vote`],
//!   not inside the engine, so the engine is not even generic over
//!   [`crate::SignatureVerifier`].
//!
//! The second fence is `sim/tools/consensus_no_io.py`, versioned and run in CI,
//! which fails if any file under this module names a clock, a socket, a file, a
//! thread or a randomness source, or if a public signature here grows a generic
//! parameter, a `dyn`, or an `impl Fn`. It is a lint and not a boundary, and it
//! is the half that would catch the property being undone by one line.
//!
//! # What is the caller's, and is not hidden
//!
//! * **Buffering across heights.** [`Engine`] keeps the log of the height it is
//!   executing and drops messages for any other, which is Algorithm 1's `h_p`
//!   scoping. A node that finishes a height before its peers will therefore
//!   broadcast into engines that discard, and the peers catch up through the
//!   caller — `wire.md`'s ledger sync in production, redelivery in the test
//!   harness. Holding an unbounded number of future heights inside the engine
//!   would be a memory a remote peer could grow.
//! * **`valid(v)`** beyond height and parent linkage, which is
//!   [`messages::Validity`].
//! * **Signing, sending, and delivering a node its own broadcast.**

pub mod certificate;
pub mod engine;
pub mod messages;
pub mod proposer;

pub use certificate::{CertificateSignature, FinalizedBlock, QuorumCertificate};
pub use engine::{
    Action, ConsensusTimeouts, Engine, EngineConfig, Event, Outbound, Step, TimeoutKind,
};
pub use messages::{
    BlockProposal, ConsensusMessage, SignedVote, Validity, VerifiedMessage, VotePhase,
    verify_proposal, verify_vote,
};
pub use proposer::{is_proposer, proposer_at};
