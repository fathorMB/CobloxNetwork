//! The support code the two consensus suites of [SPEC-025] share.
//!
//! It is a directory of its own and **not** an addition to `tests/common/`, for a
//! mechanical reason worth writing down: `tests/common/mod.rs` is declared by
//! nine test crates, so anything added there is compiled into all nine, and a
//! four-validator devnet compiled into the Merkle suite is nine copies of a
//! build cost and nine crates' worth of lint surface for one crate's benefit.
//! Only `consensus_rules.rs` and `consensus_devnet.rs` declare this one.

#![allow(dead_code)]

pub mod devnet;
pub mod ed25519_signer;
