//! The rules of the consensus protocol, one at a time.
//!
//! The chain, the adversarial schedule and the mute proposer are in
//! `consensus_devnet.rs`. This file holds the parts that can be asserted without
//! running a network: the new signature domain, the proposer rule, the
//! certificate rules, and the message boundary.

mod consensus_support;

use coblox_core::consensus::{
    BlockProposal, CertificateSignature, ConsensusMessage, FinalizedBlock, QuorumCertificate,
    SignedVote, Validity, VotePhase, is_proposer, proposer_at, verify_proposal, verify_vote,
};
use coblox_core::error::{ConsensusError, Error};
use coblox_core::hash::{ChainId, Digest32, Domain};
use coblox_core::json::JsonObject;
use coblox_core::registry::{block_prevote_preimage, block_vote_preimage};
use coblox_core::validator_set::ValidatorSet;
use coblox_core::{ConsensusVerifier, verify_in_context};

use consensus_support::devnet::{Adversary, Devnet, devnet_set, harness_header};
use consensus_support::ed25519_signer::{SigningKey, rfc8032_vectors_reproduce};

fn chain_id() -> ChainId {
    ChainId::from_digest(Digest32::repeated(0x7a))
}

// -------------------------------------------------------------- the signer ---

/// The first oracle of the test signer: RFC 8032 §7.1, reproduced.
///
/// Every consensus test in this crate is built on this signer, so this test runs
/// before any of them means anything.
#[test]
fn the_test_signer_reproduces_rfc_8032_section_7_1() {
    let transcript = rfc8032_vectors_reproduce();
    assert_eq!(transcript.len(), 3);
    for line in &transcript {
        println!("{line}");
    }
}

/// The second oracle: the shipped ZIP-215 verifier accepts what the RFC 8032
/// signer produces, and rejects one bit-flip of it.
#[test]
fn the_shipped_verifier_accepts_this_signer_and_rejects_a_flipped_bit() {
    let key = SigningKey::from_seed(&[0x11; 32]);
    let preimage = block_vote_preimage(&chain_id(), 42, 3, &Digest32::repeated(0x55));
    let signature = key.sign(preimage.as_bytes());
    assert!(verify_in_context(
        &ConsensusVerifier,
        Domain::SIG_BLOCK_VOTE,
        &chain_id(),
        &key.public_key(),
        &preimage,
        &signature,
    ));
    let mut flipped = signature;
    flipped[0] ^= 0x01;
    assert!(!verify_in_context(
        &ConsensusVerifier,
        Domain::SIG_BLOCK_VOTE,
        &chain_id(),
        &key.public_key(),
        &preimage,
        &flipped,
    ));
}

// --------------------------------------------------------- the new domain ---

/// `coblox-block-prevote-v0` exists, carries the preimage [ADR-018] §1
/// publishes, and is a different preimage from the precommit's over the same
/// fields.
#[test]
fn the_prevote_domain_is_separate_from_the_precommit_domain() {
    assert_eq!(
        Domain::SIG_BLOCK_PREVOTE.as_str(),
        "coblox-block-prevote-v0"
    );
    assert_eq!(Domain::SIG_BLOCK_VOTE.as_str(), "coblox-block-vote-v0");

    let block_id = Digest32::repeated(0x66);
    let prevote = block_prevote_preimage(&chain_id(), 7, 2, &block_id);
    let precommit = block_vote_preimage(&chain_id(), 7, 2, &block_id);
    assert_ne!(prevote.as_bytes(), precommit.as_bytes());

    // The whole difference is the separator: after it, the two preimages are
    // byte-identical. This is the assertion that would fail if a future edit
    // changed one payload and not the other.
    let prevote_tail = &prevote.as_bytes()[Domain::SIG_BLOCK_PREVOTE.as_str().len() + 1..];
    let precommit_tail = &precommit.as_bytes()[Domain::SIG_BLOCK_VOTE.as_str().len() + 1..];
    assert_eq!(prevote_tail, precommit_tail);

    // And the payload is exactly what the document publishes:
    // chain_id_32 || u64be(height) || u64be(round) || raw_32_bytes(block_id).
    let mut expected = Vec::new();
    expected.extend_from_slice(chain_id().as_digest().as_bytes());
    expected.extend_from_slice(&7u64.to_be_bytes());
    expected.extend_from_slice(&2u64.to_be_bytes());
    expected.extend_from_slice(block_id.as_bytes());
    assert_eq!(prevote_tail, expected.as_slice());
}

/// A prevote signature does not verify as a precommit, and the reverse.
///
/// This is the confusion that would let one message both lock a validator and
/// count towards finalizing a block.
#[test]
fn a_prevote_signature_is_not_a_precommit_signature() {
    let key = SigningKey::from_seed(&[0x21; 32]);
    let block_id = Digest32::repeated(0x66);
    let prevote = block_prevote_preimage(&chain_id(), 7, 2, &block_id);
    let precommit = block_vote_preimage(&chain_id(), 7, 2, &block_id);
    let signature = key.sign(prevote.as_bytes());

    assert!(verify_in_context(
        &ConsensusVerifier,
        Domain::SIG_BLOCK_PREVOTE,
        &chain_id(),
        &key.public_key(),
        &prevote,
        &signature,
    ));
    assert!(!verify_in_context(
        &ConsensusVerifier,
        Domain::SIG_BLOCK_VOTE,
        &chain_id(),
        &key.public_key(),
        &precommit,
        &signature,
    ));
}

// ------------------------------------------------------- the proposer rule ---

/// Two nodes holding the same set — built independently, agreeing only on
/// `validator_set_hash` — name the same proposer at every pair, without
/// exchanging anything.
#[test]
fn two_nodes_with_the_same_set_hash_compute_the_same_proposer() {
    let (left, _) = devnet_set(4);
    let (right, _) = devnet_set(4);
    assert_eq!(left.hash().unwrap(), right.hash().unwrap());

    let mut names = Vec::new();
    for height in 0..40u64 {
        for round in 0..6u64 {
            let a = proposer_at(&left, height, round)
                .unwrap()
                .validator_id
                .clone();
            let b = proposer_at(&right, height, round)
                .unwrap()
                .validator_id
                .clone();
            assert_eq!(a, b, "disagreement at ({height}, {round})");
            names.push(a);
        }
    }
    // And it is not a constant: a "deterministic" rule that named one validator
    // forever would pass the paragraph above.
    let distinct: std::collections::BTreeSet<&String> = names.iter().collect();
    assert_eq!(distinct.len(), 4, "every member must propose sometimes");
}

/// Consecutive rounds at one height never repeat a proposer while an unvisited
/// member remains.
///
/// This is the obligation the liveness criterion rests on: a proposer that says
/// nothing at round `r` has to be replaced at `r+1` by somebody else.
#[test]
fn consecutive_rounds_visit_every_member_before_repeating() {
    let (set, _) = devnet_set(4);
    for height in 0..17u64 {
        let names: Vec<String> = (0..4u64)
            .map(|round| {
                proposer_at(&set, height, round)
                    .unwrap()
                    .validator_id
                    .clone()
            })
            .collect();
        let distinct: std::collections::BTreeSet<&String> = names.iter().collect();
        assert_eq!(
            distinct.len(),
            4,
            "height {height} repeats a proposer within four rounds: {names:?}"
        );
    }
}

/// The index runs over power, so a member with more power proposes more often.
#[test]
fn the_proposer_index_is_weighted_by_voting_power() {
    let (mut set, _) = devnet_set(3);
    set.validators[0].voting_power = 3;
    let total = set.total_voting_power().unwrap();
    assert_eq!(total, 5);

    let mut counts = std::collections::BTreeMap::new();
    for round in 0..total * 4 {
        *counts
            .entry(proposer_at(&set, 0, round).unwrap().validator_id.clone())
            .or_insert(0u64) += 1;
    }
    assert_eq!(counts["val-000"], 12);
    assert_eq!(counts["val-001"], 4);
    assert_eq!(counts["val-002"], 4);
}

/// `(height, round)` is the whole index: nothing a participant supplies enters
/// it.
///
/// Checked by changing everything else about a set that a participant could
/// influence — the node IDs, the keys, the stamps — and requiring the proposer
/// of every pair to be unchanged as long as the `validator_id` order and the
/// powers are.
#[test]
fn the_proposer_does_not_depend_on_anything_a_participant_supplies() {
    let (reference, _) = devnet_set(4);
    let mut mutated = reference.clone();
    for (index, entry) in mutated.validators.iter_mut().enumerate() {
        entry.node_id = format!("cblx1ground{index}");
        entry.consensus_public_key = [u8::try_from(index).unwrap() + 200; 32];
        entry.key_binding_signature = [0xff; 64];
        entry.seated_since_epoch = 5;
        entry.term_expiry_epoch = 99;
    }
    assert_ne!(reference.hash().unwrap(), mutated.hash().unwrap());
    for height in 0..25u64 {
        for round in 0..4u64 {
            assert_eq!(
                proposer_at(&reference, height, round).unwrap().validator_id,
                proposer_at(&mutated, height, round).unwrap().validator_id,
            );
        }
    }
}

/// A structurally invalid set has no proposer, rather than an arbitrary one.
#[test]
fn a_structurally_invalid_set_has_no_proposer() {
    let (mut set, _) = devnet_set(4);
    set.validators.swap(0, 1);
    assert!(proposer_at(&set, 1, 0).is_err());

    let empty = ValidatorSet {
        schema_version: "0.1".to_owned(),
        activation_height: 0,
        election: None,
        validators: Vec::new(),
    };
    assert!(proposer_at(&empty, 1, 0).is_err());
}

// ------------------------------------------------------- the message boundary ---

/// A proposal from a node that is not the round's proposer is refused.
#[test]
fn a_proposal_from_the_wrong_sender_is_refused() {
    let (set, _) = devnet_set(4);
    let set_hash = set.hash().unwrap();
    let proposer = proposer_at(&set, 1, 0).unwrap().validator_id.clone();
    let impostor = set
        .validators
        .iter()
        .find(|entry| entry.validator_id != proposer)
        .unwrap()
        .validator_id
        .clone();
    let proposal = BlockProposal {
        height: 1,
        round: 0,
        valid_round: None,
        header: harness_header(&set_hash, 1, 0, 0, &Digest32::repeated(0x01)),
        transactions: Vec::new(),
    };

    assert!(
        verify_proposal(
            &chain_id(),
            &set,
            &proposer,
            proposal.clone(),
            Validity::Valid
        )
        .is_ok()
    );
    assert!(matches!(
        verify_proposal(
            &chain_id(),
            &set,
            &impostor,
            proposal.clone(),
            Validity::Valid
        ),
        Err(Error::Consensus(ConsensusError::NotTheProposer { .. }))
    ));
    assert!(matches!(
        verify_proposal(&chain_id(), &set, "val-999", proposal, Validity::Valid),
        Err(Error::Consensus(ConsensusError::SenderNotAMember { .. }))
    ));
}

/// A proposal whose `valid_round` is not below its own round is refused.
#[test]
fn a_proposal_cannot_justify_itself_with_its_own_round() {
    let (set, _) = devnet_set(4);
    let set_hash = set.hash().unwrap();
    let proposer = proposer_at(&set, 1, 2).unwrap().validator_id.clone();
    for valid_round in [2u64, 3u64] {
        let proposal = BlockProposal {
            height: 1,
            round: 2,
            valid_round: Some(valid_round),
            header: harness_header(&set_hash, 1, 2, 0, &Digest32::repeated(0x01)),
            transactions: Vec::new(),
        };
        assert!(matches!(
            verify_proposal(&chain_id(), &set, &proposer, proposal, Validity::Valid),
            Err(Error::Consensus(
                ConsensusError::ProposalValidRoundNotBelowRound { .. }
            ))
        ));
    }
}

/// A proposal whose header carries a different height from the message is
/// refused.
#[test]
fn a_proposal_header_must_carry_the_height_the_message_claims() {
    let (set, _) = devnet_set(4);
    let set_hash = set.hash().unwrap();
    let proposer = proposer_at(&set, 1, 0).unwrap().validator_id.clone();
    let proposal = BlockProposal {
        height: 1,
        round: 0,
        valid_round: None,
        header: harness_header(&set_hash, 2, 0, 0, &Digest32::repeated(0x01)),
        transactions: Vec::new(),
    };
    assert!(matches!(
        verify_proposal(&chain_id(), &set, &proposer, proposal, Validity::Valid),
        Err(Error::Consensus(ConsensusError::ProposalHeaderMismatch {
            field: "height"
        }))
    ));
}

/// A vote with a signature under the other phase's domain is refused, and so is
/// a vote from a non-member.
#[test]
fn the_vote_boundary_refuses_the_wrong_domain_and_the_wrong_sender() {
    let (set, keys) = devnet_set(4);
    let block_id = Digest32::repeated(0x66);
    let prevote_preimage = block_prevote_preimage(&chain_id(), 1, 0, &block_id);
    let signature = keys[0].sign(prevote_preimage.as_bytes());
    let vote = SignedVote {
        height: 1,
        round: 0,
        block_id,
        validator_id: "val-000".to_owned(),
        signature,
    };

    assert!(
        verify_vote(
            &chain_id(),
            &set,
            VotePhase::Prevote,
            vote.clone(),
            &ConsensusVerifier
        )
        .is_ok()
    );
    assert!(matches!(
        verify_vote(
            &chain_id(),
            &set,
            VotePhase::Precommit,
            vote.clone(),
            &ConsensusVerifier
        ),
        Err(Error::Consensus(ConsensusError::InvalidSignature { .. }))
    ));

    let stranger = SignedVote {
        validator_id: "val-999".to_owned(),
        ..vote
    };
    assert!(matches!(
        verify_vote(
            &chain_id(),
            &set,
            VotePhase::Prevote,
            stranger,
            &ConsensusVerifier
        ),
        Err(Error::Consensus(ConsensusError::SenderNotAMember { .. }))
    ));
}

/// A vote signed for another chain does not verify on this one.
#[test]
fn a_vote_is_bound_to_its_chain() {
    let (set, keys) = devnet_set(4);
    let other = ChainId::from_digest(Digest32::repeated(0x7b));
    let block_id = Digest32::repeated(0x66);
    let signature = keys[0].sign(block_prevote_preimage(&other, 1, 0, &block_id).as_bytes());
    let vote = SignedVote {
        height: 1,
        round: 0,
        block_id,
        validator_id: "val-000".to_owned(),
        signature,
    };
    assert!(
        verify_vote(
            &chain_id(),
            &set,
            VotePhase::Prevote,
            vote,
            &ConsensusVerifier
        )
        .is_err()
    );
}

// --------------------------------------------------------- the certificate ---

fn certificate_over(
    set: &ValidatorSet,
    keys: &[SigningKey],
    signers: &[usize],
    height: u64,
    round: u64,
    block_id: Digest32,
) -> QuorumCertificate {
    let preimage = block_vote_preimage(&chain_id(), height, round, &block_id);
    let mut signatures: Vec<CertificateSignature> = signers
        .iter()
        .map(|&index| CertificateSignature {
            validator_id: set.validators[index].validator_id.clone(),
            signature: keys[index].sign(preimage.as_bytes()),
        })
        .collect();
    signatures.sort_by(|a, b| a.validator_id.cmp(&b.validator_id));
    QuorumCertificate {
        height,
        round,
        block_id,
        validator_set_hash: set.hash().unwrap(),
        signatures,
    }
}

/// The four published rules of `ledger.md#what-validators-sign`, each observed
/// rejecting.
#[test]
fn the_certificate_rules_reject_what_the_document_says_they_reject() {
    let (set, keys) = devnet_set(4);
    let block_id = Digest32::repeated(0x66);

    // Three of four is a quorum: 3*3 > 4*2.
    let good = certificate_over(&set, &keys, &[0, 1, 2], 1, 0, block_id);
    assert!(good.verify(&chain_id(), &set, &ConsensusVerifier).is_ok());

    // Two of four is not: 2*3 > 4*2 is false. The predicate is strict and this
    // is the boundary the document publishes.
    let short = certificate_over(&set, &keys, &[0, 1], 1, 0, block_id);
    assert!(matches!(
        short.verify(&chain_id(), &set, &ConsensusVerifier),
        Err(Error::Consensus(ConsensusError::BelowQuorum {
            signed_power: 2,
            total_power: 4
        }))
    ));

    // Empty.
    let mut empty = good.clone();
    empty.signatures.clear();
    assert!(matches!(
        empty.verify(&chain_id(), &set, &ConsensusVerifier),
        Err(Error::Consensus(ConsensusError::CertificateEmpty))
    ));

    // Duplicated: the same entry twice, which is also unsorted.
    let mut duplicated = good.clone();
    duplicated.signatures.push(good.signatures[2].clone());
    duplicated.signatures.swap(2, 3);
    assert!(matches!(
        duplicated.verify(&chain_id(), &set, &ConsensusVerifier),
        Err(Error::Consensus(
            ConsensusError::CertificateNotSortedOrUnique
        ))
    ));

    // Unsorted.
    let mut unsorted = good.clone();
    unsorted.signatures.swap(0, 1);
    assert!(matches!(
        unsorted.verify(&chain_id(), &set, &ConsensusVerifier),
        Err(Error::Consensus(
            ConsensusError::CertificateNotSortedOrUnique
        ))
    ));

    // A certificate that names another set.
    let mut other_set = good.clone();
    other_set.validator_set_hash = Digest32::repeated(0xaa);
    assert!(matches!(
        other_set.verify(&chain_id(), &set, &ConsensusVerifier),
        Err(Error::Consensus(
            ConsensusError::CertificateNamesAnotherSet { .. }
        ))
    ));

    // Signatures taken over another round do not verify at this one.
    let wrong_round = QuorumCertificate {
        round: 1,
        ..good.clone()
    };
    assert!(matches!(
        wrong_round.verify(&chain_id(), &set, &ConsensusVerifier),
        Err(Error::Consensus(ConsensusError::InvalidSignature { .. }))
    ));

    // A non-member signer.
    let mut stranger = good;
    stranger.signatures[2].validator_id = "val-999".to_owned();
    assert!(matches!(
        stranger.verify(&chain_id(), &set, &ConsensusVerifier),
        Err(Error::Consensus(ConsensusError::NotAMember { .. }))
    ));
}

/// A valid certificate for a different block does not finalize this one.
#[test]
fn a_block_cannot_borrow_another_blocks_certificate() {
    let (set, keys) = devnet_set(4);
    let set_hash = set.hash().unwrap();
    let header = harness_header(&set_hash, 1, 0, 0, &Digest32::repeated(0x01));
    let block_id = header.block_id(&chain_id()).unwrap();
    let other_id = Digest32::repeated(0x99);

    let mine = FinalizedBlock {
        header: header.clone(),
        transactions: Vec::new(),
        quorum_certificate: certificate_over(&set, &keys, &[0, 1, 2], 1, 0, block_id),
    };
    assert!(mine.verify(&chain_id(), &set, &ConsensusVerifier).is_ok());

    let borrowed = FinalizedBlock {
        header,
        transactions: Vec::new(),
        quorum_certificate: certificate_over(&set, &keys, &[0, 1, 2], 1, 0, other_id),
    };
    assert!(matches!(
        borrowed.verify(&chain_id(), &set, &ConsensusVerifier),
        Err(Error::Consensus(
            ConsensusError::CertificateForAnotherBlock { .. }
        ))
    ));
}

/// Every consensus object round-trips through canonical JSON.
#[test]
fn the_three_messages_and_the_block_round_trip_through_canonical_json() {
    let (set, keys) = devnet_set(4);
    let set_hash = set.hash().unwrap();
    let header = harness_header(&set_hash, 1, 2, 0, &Digest32::repeated(0x01));
    let block_id = header.block_id(&chain_id()).unwrap();

    for valid_round in [None, Some(1u64)] {
        let proposal = BlockProposal {
            height: 1,
            round: 2,
            valid_round,
            header: header.clone(),
            transactions: Vec::new(),
        };
        let bytes = proposal.to_json().unwrap().to_jcs();
        let parsed = JsonObject::parse_canonical(&bytes).unwrap();
        assert_eq!(BlockProposal::from_json(&parsed).unwrap(), proposal);
    }

    let vote = SignedVote {
        height: 1,
        round: 2,
        block_id,
        validator_id: "val-000".to_owned(),
        signature: keys[0].sign(b"anything"),
    };
    let bytes = vote.to_json().unwrap().to_jcs();
    let parsed = JsonObject::parse_canonical(&bytes).unwrap();
    assert_eq!(SignedVote::from_json(&parsed).unwrap(), vote);

    let block = FinalizedBlock {
        header,
        transactions: Vec::new(),
        quorum_certificate: certificate_over(&set, &keys, &[0, 1, 2], 1, 2, block_id),
    };
    let bytes = block.to_json().unwrap().to_jcs();
    let parsed = JsonObject::parse_canonical(&bytes).unwrap();
    assert_eq!(FinalizedBlock::from_json(&parsed).unwrap(), block);

    // The published field set, checked against `ledger.md#block-format`.
    let object = block.to_json().unwrap();
    let keys_present: Vec<&String> = object.iter().map(|(key, _)| key).collect();
    assert_eq!(
        keys_present,
        ["header", "quorum_certificate", "transactions"]
    );
}

/// `is_proposer` and `proposer_at` agree, so a caller cannot get two answers.
#[test]
fn is_proposer_agrees_with_proposer_at() {
    let (set, _) = devnet_set(4);
    for height in 0..12u64 {
        for round in 0..4u64 {
            let named = proposer_at(&set, height, round)
                .unwrap()
                .validator_id
                .clone();
            for entry in &set.validators {
                assert_eq!(
                    is_proposer(&set, height, round, &entry.validator_id).unwrap(),
                    entry.validator_id == named,
                );
            }
        }
    }
}

/// A verified message keeps what the boundary established, so the engine reads
/// the same `block_id` the boundary computed.
#[test]
fn the_boundary_recomputes_the_block_id_it_hands_on() {
    let devnet = Devnet::start(4, 1, Adversary::default());
    let set_hash = devnet.set.hash().unwrap();
    let header = harness_header(&set_hash, 1, 0, 0, &Digest32::repeated(0x01));
    let expected = header.block_id(devnet.chain_id()).unwrap();
    let proposer = devnet.proposer_of(1, 0);
    let verified = verify_proposal(
        devnet.chain_id(),
        &devnet.set,
        &proposer,
        BlockProposal {
            height: 1,
            round: 0,
            valid_round: None,
            header,
            transactions: Vec::new(),
        },
        Validity::Valid,
    )
    .unwrap();
    match verified.get() {
        ConsensusMessage::Proposal { block_id, .. } => assert_eq!(*block_id, expected),
        other => panic!("expected a proposal, got {other:?}"),
    }
}

// ------------------------------------- the published preimage is unchanged ---

/// `block_vote_preimage` still produces exactly the bytes a fixture committed
/// **before this spec** already carries.
///
/// This is `GATE-NOTHING-PUBLISHED-CHANGED` at the level that matters. The diff
/// of [SPEC-025] touches `registry.rs`, because the prevote preimage was added
/// next to the precommit one and both now build their shared payload in one
/// place. A reviewer reading that diff is entitled to ask whether the shared
/// payload is the payload that was there before.
///
/// The answer is not this test's opinion. `tests/fixtures/ed25519_coblox_extension.json`
/// ships seven Ed25519 vectors whose `message` field is a **whole finality-vote
/// preimage**, generated by `sim/tools/ed25519_coblox_extension_vectors.py`,
/// committed by [SPEC-012] and unmodified by this spec. This test parses those
/// bytes back into `(chain_id, height, round, block_id)` and requires
/// `block_vote_preimage` to rebuild them byte for byte. It is the second road:
/// the fixture came from a Python generator that shares no code with this crate.
#[test]
fn the_finality_vote_preimage_still_reproduces_a_fixture_older_than_this_spec() {
    const FIXTURE: &str = include_str!("fixtures/ed25519_coblox_extension.json");
    let vectors: serde_json::Value =
        serde_json::from_str(FIXTURE).expect("the extension fixture is JSON");
    let vectors = vectors.as_array().expect("the fixture is an array");
    assert_eq!(vectors.len(), 7, "the fixture ships seven vectors");

    let prefix = b"coblox-block-vote-v0\0";
    let mut checked = 0;
    for vector in vectors {
        let hex = vector["message"].as_str().expect("message");
        let bytes: Vec<u8> = (0..hex.len() / 2)
            .map(|index| u8::from_str_radix(&hex[index * 2..index * 2 + 2], 16).expect("hex"))
            .collect();
        assert!(
            bytes.starts_with(prefix),
            "vector {checked} is not a finality vote"
        );
        // domain || 0x00 || chain_id_32 || u64be(height) || u64be(round) || block_id_32
        assert_eq!(bytes.len(), prefix.len() + 32 + 8 + 8 + 32);
        let body = &bytes[prefix.len()..];
        let mut chain = [0u8; 32];
        chain.copy_from_slice(&body[..32]);
        let height = u64::from_be_bytes(body[32..40].try_into().unwrap());
        let round = u64::from_be_bytes(body[40..48].try_into().unwrap());
        let mut block = [0u8; 32];
        block.copy_from_slice(&body[48..80]);

        let rebuilt = block_vote_preimage(
            &ChainId::from_digest(Digest32::from_bytes(chain)),
            height,
            round,
            &Digest32::from_bytes(block),
        );
        assert_eq!(
            rebuilt.as_bytes(),
            bytes.as_slice(),
            "vector {checked}: block_vote_preimage no longer reproduces the committed fixture"
        );
        checked += 1;
    }
    println!(
        "GATE-NOTHING-PUBLISHED-CHANGED: {checked} pre-existing finality-vote preimages \
         reproduced byte for byte by the refactored block_vote_preimage"
    );
}
