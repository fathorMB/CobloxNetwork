//! Shared, platform-neutral foundation for Coblox services and native shells.
//!
//! This crate is the **deterministic layer** of protocol family `coblox/0`: the
//! part of the protocol that is a total function of its inputs. Canonical
//! serialization, the hash preimage registry, the Merkle constructions, the
//! validator set with its election record and derivation, the quorum predicate,
//! the header-only continuity rules, the light client's normative composition
//! checks, and the acceptance-time validity rules on governed parameters all
//! live here. Networking, BFT consensus, storage and the WASM runtime do not.
//!
//! # How to read this crate
//!
//! Modules follow the shape of `docs/protocol/`, bottom-up, because that is the
//! dependency order and because a reviewer holding the specification should be
//! able to find each rule where they expect it:
//!
//! | Module | Specification section |
//! | --- | --- |
//! | [`encoding`] | common representation, byte-string encodings |
//! | [`json`] | common representation, JCS |
//! | [`hash`] | identifiers and cryptographic conventions |
//! | [`registry`] | hash preimage registry |
//! | [`merkle`] | hashing primitives, the five sorted trees, sparse account state |
//! | [`params`] | signed protocol documents, election bounds, the constraint block |
//! | [`quorum`] | quorum predicate |
//! | [`authorization`] | unsigned transaction and authorization |
//! | [`block`] | block format, validator-set continuity |
//! | [`cadence`] | the measured cadence, the reward-epoch derivation |
//! | [`validator_set`] | validator-set continuity, revocation transitions |
//! | [`election`] | validator election and rotation |
//! | [`light_client`] | light-client balance verification, set composition |
//! | [`verifier`] | consensus-critical Ed25519 verification |
//! | [`consensus`] | the two-phase consensus protocol of [ADR-018], `wire.md`'s three consensus messages, and the `QuorumCertificate` |
//!
//! # Three conventions the rest of the project inherits
//!
//! **Canonicalization is the only path.** [`json::Json`] cannot represent a
//! JSON number or `null`, [`json::JsonObject`] cannot hold an invalid or
//! duplicate key and cannot be serialized in a non-canonical order, and
//! [`json::ObjectBuilder`] emits the canonical spelling of each typed field.
//! The single decode entry point,
//! [`json::JsonObject::parse_canonical`], rejects non-canonical bytes instead
//! of normalizing them. The costliest defect class in a ledger is a value with
//! two serializations, and the countermeasure here is that the second one is
//! not constructible.
//!
//! **Domain separation is structural.** Every preimage domain is a constant of
//! [`hash::Domain`] and the only way to start a preimage is
//! [`hash::PreimageWriter::new`], which writes `domain || 0x00` and cannot be
//! told not to. Merkle tag bytes live in [`merkle::TaggedTree`] constants for
//! the same reason. A domain mistake produces a plausible, wrong hash, which is
//! the worst defect to diagnose.
//!
//! **Parameters are validated configuration, never compiled constants.** No
//! launch value appears in this crate. The objects that carry launch values
//! into it fall into **three** classes and not one, and the classes are named
//! separately because what a rejection *means* is different in each. A list
//! that merged them would describe at least one of its members backwards.
//!
//! **Governed documents** — [`params::ConsensusParameters`],
//! [`params::RewardPolicy`] and [`params::EnrollmentParameters`] — arrive
//! inside a document a validator quorum signed. The election derivation and the
//! creator-share cap accept only [`params::ValidatedConsensusParameters`] and
//! [`params::ValidatedRewardPolicy`], which have no constructors other than
//! validation against the constraint block and the genesis bounds. Validation
//! failure is a recoverable error rather than a panic, because rejecting such a
//! document is ordinary protocol operation.
//!
//! **Genesis bounds** — [`params::ElectionBounds`] and
//! [`params::RewardBounds`] — ship inside the signed network distribution and
//! in no other channel. They bound what a governed document may carry, so a
//! document outside them is rejected on acceptance, and nothing on-chain
//! constrains the bounds themselves.
//!
//! **[`params::CadenceBand`] is in neither class, and saying so is the point of
//! listing three.** It ships in the signed distribution like a genesis bound,
//! but it bounds **nothing any document carries**: it is the tolerance applied
//! to a measurement whose two endpoints are outside the chain, and no validity
//! rule of this protocol compares anything to it. So the sentence that closes
//! the first class — that in production these values arrive inside a document a
//! validator quorum signed — is **false for this one, and deliberately**. Its
//! values reach a deployment through the release channel alone, never through a
//! quorum-signed document and never learned from a peer or a header, because a
//! band a sitting quorum could widen would be a tolerance underneath the only
//! measurement the protocol has of that quorum's own behaviour
//! (`README.md#cadence-band`, [ADR-013], [ADR-016]). A new signed release may
//! narrow it; nothing on-chain may widen it.
//!
//! **A trust anchor is checked before it is trusted.** Anchors are
//! configuration, so nothing on-chain constrains them, and a degenerate one
//! would disable the rule it is supposed to carry rather than fail. Every
//! composed entry point therefore validates its anchor as its first act:
//! [`light_client::authenticate_consensus_parameters`] for
//! [`params::ElectionBounds`], [`light_client::authenticate_reward_policy`] for
//! [`params::RewardBounds`], and both cadence measurements —
//! [`cadence::measure_cadence_from_checkpoint`] and
//! [`cadence::measure_cadence_between_checkpoints`] — for
//! [`params::CadenceBand`], which is why a caller cannot reach the arithmetic
//! with a band that admits every rate. The rate-of-change rule additionally
//! refuses a degenerate ratio in [`params::RewardPolicy::validate`] itself, so
//! the rule cannot become vacuous on any path.
//!
//! # Consensus-critical Ed25519 verification
//!
//! `docs/protocol/README.md#consensus-critical-ed25519-verification` requires one
//! identical ZIP-215-derived rule with a fifth condition of its own —
//! `[8]A != identity` — and forbids unproven substitutions.
//!
//! This crate ships the [`verifier::ConsensusVerifier`] implementation of
//! [`SignatureVerifier`], verified vector-by-vector against vectors 0–11 of
//! `novifinancial/ed25519-speccheck` as its oracle.

#![forbid(unsafe_code)]
// Justified deviations from `clippy::pedantic`, which is `-D warnings` in CI.
// Each is a documentation- or ergonomics-level lint, never a correctness one.
#![allow(
    // Every fallible function returns the crate-wide `Error`, whose variants
    // are documented once in `error`. Repeating that per function would add
    // volume without adding information.
    clippy::missing_errors_doc,
    // The protocol's own vocabulary repeats the module name (`ValidatorSet` in
    // `validator_set`, `ElectionRecord` in `election`), and renaming types away
    // from the specification would cost more than the lint saves.
    clippy::module_name_repetitions,
    // Specification quotes in doc comments contain bare identifiers and
    // all-caps terms that this lint reads as un-backticked items.
    clippy::doc_markdown,
    // The protocol uses closely related names deliberately: `candidate_root`
    // and `candidate_count`, `retained_count` and `filled_count`.
    clippy::similar_names,
    // Applied to nearly every accessor in a data-modelling crate.
    clippy::must_use_candidate
)]

pub mod authorization;
pub mod block;
pub mod cadence;
pub mod consensus;
pub mod election;
pub mod encoding;
pub mod error;
pub mod hash;
pub mod identity;
pub mod json;
pub mod light_client;
pub mod merkle;
pub mod params;
pub mod quorum;
pub mod registry;
pub mod validator_set;
pub mod verifier;

pub use error::{Error, Result};
pub use identity::{RevocationReason, RevokeIdentityBody, TransportKeyAttestation};
pub use registry::{PreimageContext, SigningPreimage};
pub use verifier::{ConsensusVerifier, verify_in_context};

/// Returns the semantic version exposed by every native shell.
#[must_use]
pub const fn core_version() -> &'static str {
    env!("CARGO_PKG_VERSION")
}

/// The consensus-critical Ed25519 verification rule, as a seam.
///
/// An implementation of this trait MUST satisfy all four rules of
/// `README.md#consensus-critical-ed25519-verification` plus the small-order
/// public-key rejection, and MUST reproduce the published outcome table for
/// vectors 0–11 of `novifinancial/ed25519-speccheck` before it is used on a
/// Coblox network. [`ConsensusVerifier`] provides the canonical implementation.
pub trait SignatureVerifier {
    /// Verifies `signature` over `preimage` under `public_key`.
    ///
    /// `preimage` is the chain-bound preimage produced by
    /// [`registry::signing_preimage`], not a digest of it.
    fn verify(
        &self,
        public_key: &[u8; 32],
        preimage: &SigningPreimage,
        signature: &[u8; 64],
    ) -> bool;
}

#[cfg(test)]
mod tests {
    #[test]
    fn exposes_the_package_version() {
        assert_eq!(super::core_version(), env!("CARGO_PKG_VERSION"));
    }
}
