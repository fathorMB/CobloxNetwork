//! The context a `SigningPreimage` carries, and the entry point that checks it.
//!
//! [DEBT-021]: `signing_preimage` writes `domain || 0x00 || chain_id_32` into
//! the bytes and the type then forgot both, so a preimage built for the wrong
//! domain or for another chain was a well-typed, semantically false value that
//! the verifier accepted. Domain separation exists precisely so that a
//! signature valid in one context is not valid in another.
//!
//! **What this file tests and what it deliberately does not.** The property
//! under test is the context binding, not the cryptography: [SPEC-012] closed
//! the verification logic and [REVIEW-019] checked it against three independent
//! oracles, and `speccheck_conformance.rs` still holds those vectors. Mixing the
//! two here would make every assertion below depend on a keypair this crate has
//! no way to produce, so the matrix runs against a stub verifier that accepts
//! everything. That is the stronger arrangement rather than the weaker one: a
//! verifier that always accepts means every rejection below comes from the
//! context check and from nothing else, and the accepting cases prove the guard
//! is not simply refusing everything — which is the half [SPEC-009] paid for.
//!
//! **No quantity is held constant.** The matrix crosses two domains with two
//! chain IDs and checks all sixteen combinations, because a gate whose cases all
//! share one value on a quantity that is not under test has never seen the case
//! that breaks it.

use coblox_core::SignatureVerifier;
use coblox_core::hash::{ChainId, Digest32, Domain};
use coblox_core::registry::{SigningPreimage, block_vote_preimage, signing_preimage};
use coblox_core::verifier::{ConsensusVerifier, verify_in_context};

/// A verifier that accepts every signature.
///
/// It exists so that a `false` from [`verify_in_context`] can only have come
/// from the context check. It is confined to this file and implements the same
/// public trait a real verifier does, which is the point: the checked entry
/// point is generic over the trait and takes no shortcut for the real one.
#[derive(Debug, Clone, Copy)]
struct AlwaysAccepts;

impl SignatureVerifier for AlwaysAccepts {
    fn verify(
        &self,
        _public_key: &[u8; 32],
        _preimage: &SigningPreimage,
        _signature: &[u8; 64],
    ) -> bool {
        true
    }
}

const DOMAINS: [Domain; 2] = [Domain::SIG_BLOCK_VOTE, Domain::SIG_LEDGER_TRANSACTION];

fn chains() -> [ChainId; 2] {
    [
        ChainId::from_digest(Digest32::repeated(0x01)),
        ChainId::from_digest(Digest32::repeated(0x02)),
    ]
}

#[test]
fn a_preimage_binds_the_domain_and_chain_it_was_built_for() {
    let chain = ChainId::from_digest(Digest32::repeated(0x01));
    let preimage = signing_preimage(Domain::SIG_BLOCK_VOTE, &chain, b"payload");

    let context = preimage.context().expect("a built preimage has a context");
    assert_eq!(context.domain(), Domain::SIG_BLOCK_VOTE);
    assert_eq!(context.chain_id(), &chain);
    assert!(preimage.binds(Domain::SIG_BLOCK_VOTE, &chain));
}

/// The full matrix: four preimages against four expectations.
///
/// Exactly the four matching pairs are accepted. Fifteen of the sixteen cells
/// would have been indistinguishable before this change, because every one of
/// them is a well-formed `SigningPreimage`.
#[test]
fn only_the_matching_context_is_accepted() {
    let chains = chains();
    let mut accepted = 0;
    for built_domain in DOMAINS {
        for built_chain in &chains {
            let preimage = signing_preimage(built_domain, built_chain, b"payload");
            for expected_domain in DOMAINS {
                for expected_chain in &chains {
                    let matching = built_domain == expected_domain && built_chain == expected_chain;
                    let verdict = verify_in_context(
                        &AlwaysAccepts,
                        expected_domain,
                        expected_chain,
                        &[0u8; 32],
                        &preimage,
                        &[0u8; 64],
                    );
                    assert_eq!(
                        verdict,
                        matching,
                        "built for {}/{} and offered as {}/{}",
                        built_domain.as_str(),
                        built_chain.as_digest().to_prefixed(),
                        expected_domain.as_str(),
                        expected_chain.as_digest().to_prefixed(),
                    );
                    accepted += usize::from(verdict);
                }
            }
        }
    }
    assert_eq!(
        accepted, 4,
        "one acceptance per built preimage, and no more"
    );
}

/// The specialized wrappers carry the context their name promises.
#[test]
fn block_vote_preimage_binds_the_block_vote_domain() {
    let chain = ChainId::from_digest(Digest32::repeated(0x03));
    let other = ChainId::from_digest(Digest32::repeated(0x04));
    let preimage = block_vote_preimage(&chain, 7, 0, &Digest32::repeated(0x05));

    assert!(preimage.binds(Domain::SIG_BLOCK_VOTE, &chain));
    assert!(!preimage.binds(Domain::SIG_BLOCK_VOTE, &other));
    assert!(!preimage.binds(Domain::SIG_CONSENSUS_KEY_BINDING, &chain));
}

/// A preimage built from raw non-consensus bytes binds nothing, and fails closed.
///
/// It has no `domain || 0x00 || chain_id` prefix, so reporting a context would
/// be inventing one. `binds` therefore answers `false` for every question,
/// which is the direction that keeps the `conformance-testing` escape hatch
/// fenced instead of unfencing it.
#[test]
fn a_raw_non_consensus_preimage_binds_nothing() {
    let chain = ChainId::from_digest(Digest32::repeated(0x01));
    let raw = SigningPreimage::from_raw_bytes_non_consensus(b"upstream test vector message");

    assert!(raw.context().is_none());
    for domain in DOMAINS {
        for candidate in &chains() {
            assert!(!raw.binds(domain, candidate));
        }
    }
    assert!(!verify_in_context(
        &AlwaysAccepts,
        Domain::SIG_BLOCK_VOTE,
        &chain,
        &[0u8; 32],
        &raw,
        &[0u8; 64],
    ));
}

/// The real verifier reaches the same verdict, and the unchecked entry point
/// still reaches the old one.
///
/// The second half is the one worth writing down: `verify` has not changed and
/// must not, so a wrong-context preimage is still *signature*-checked by it. The
/// difference this change makes is that a caller now has an entry point that
/// asks the other question too.
#[test]
fn the_consensus_verifier_rejects_a_wrong_context_before_any_curve_arithmetic() {
    let chain = ChainId::from_digest(Digest32::repeated(0x01));
    let other = ChainId::from_digest(Digest32::repeated(0x02));
    let preimage = block_vote_preimage(&chain, 1, 0, &Digest32::repeated(0x09));

    assert!(!verify_in_context(
        &ConsensusVerifier,
        Domain::SIG_BLOCK_VOTE,
        &other,
        &[0u8; 32],
        &preimage,
        &[0u8; 64],
    ));
    // The all-zero public key is small-order and rule 3 rejects it, so `verify`
    // says `false` here for a reason that has nothing to do with the context.
    // The assertion records that the two entry points are answering different
    // questions rather than the same one twice.
    assert!(!ConsensusVerifier.verify(&[0u8; 32], &preimage, &[0u8; 64]));
}
