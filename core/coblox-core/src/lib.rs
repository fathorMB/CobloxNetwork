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
//! | [`block`] | block format, validator-set continuity |
//! | [`validator_set`] | validator-set continuity, revocation transitions |
//! | [`election`] | validator election and rotation |
//! | [`light_client`] | light-client balance verification, set composition |
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
//! launch value appears in this crate. Values arrive as
//! [`params::ConsensusParameters`], [`params::ElectionBounds`],
//! [`params::RewardPolicy`], [`params::RewardBounds`] and
//! [`params::EnrollmentParameters`], and the election derivation and the
//! creator-share cap accept only [`params::ValidatedConsensusParameters`] and
//! [`params::ValidatedRewardPolicy`], which have no constructors other
//! than validation against the constraint block and genesis bounds.
//! Validation failure is a recoverable error, because in production these
//! values arrive inside a document a validator quorum signed and rejecting
//! one is ordinary operation.
//!
//! **A trust anchor is checked before it is trusted.** Bounds objects are
//! configuration, so nothing on-chain constrains them, and a degenerate one
//! would disable the rule it is supposed to carry rather than fail. Both
//! composed entry points therefore validate the anchor as their first act:
//! [`light_client::authenticate_consensus_parameters`] for
//! [`params::ElectionBounds`] and [`light_client::authenticate_reward_policy`]
//! for [`params::RewardBounds`]. The rate-of-change rule additionally refuses a
//! degenerate ratio in [`params::RewardPolicy::validate`] itself, so the rule
//! cannot become vacuous on any path.
//!
//! # Declared limit: no signature verifier ships here
//!
//! `README.md#consensus-critical-ed25519-verification` requires one identical
//! ZIP-215-derived rule with a fifth condition of its own — `[8]A != identity`
//! — and states that an implementation "MUST NOT substitute
//! `ed25519-dalek::verify_strict`, legacy-compatibility modes, or a library
//! default whose edge-case acceptance has not been shown equivalent to these
//! four rules", with conformance measured against vectors 0–11 of
//! `novifinancial/ed25519-speccheck`.
//!
//! This crate therefore ships the **signature preimages** — which are
//! deterministic and are tested — and the [`SignatureVerifier`] seam, and does
//! **not** ship a verifier. Shipping one without the speccheck vectors as its
//! oracle would be precisely the unvalidated edge-case behaviour the
//! specification forbids, and it would be indistinguishable from a correct one
//! until a chain split. The verifier, its vectors and its conformance table
//! belong together in their own change.

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

pub mod block;
pub mod election;
pub mod encoding;
pub mod error;
pub mod hash;
pub mod json;
pub mod light_client;
pub mod merkle;
pub mod params;
pub mod quorum;
pub mod registry;
pub mod validator_set;

pub use error::{Error, Result};

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
/// Coblox network. Nothing in this crate implements it; see the crate
/// documentation for why.
pub trait SignatureVerifier {
    /// Verifies `signature` over `message` under `public_key`.
    ///
    /// `message` is the chain-bound preimage produced by
    /// [`registry::signing_preimage`], not a digest of it.
    fn verify(&self, public_key: &[u8; 32], message: &[u8], signature: &[u8; 64]) -> bool;
}

#[cfg(test)]
mod tests {
    #[test]
    fn exposes_the_package_version() {
        assert_eq!(super::core_version(), env!("CARGO_PKG_VERSION"));
    }
}
