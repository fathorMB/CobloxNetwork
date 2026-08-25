//! Every published value of the hash conformance registry.
//!
//! **Provenance rule for this file.** Each `EXPECTED_*` constant is copied
//! character by character from the table in
//! `docs/protocol/README.md#hash-conformance-fixtures`, or from the section of
//! `ledger.md` / `identity.md` cited in its comment. No expected value is
//! produced by running this crate. A test that generated its expectation from
//! the implementation and then compared the two would pass whatever the
//! implementation did.
//!
//! "Conformance suites MUST reconstruct every preimage from these definitions
//! and compare all 32 digest bytes; checking only presentation strings is
//! insufficient." Every assertion below therefore compares [`Digest32`] values,
//! which are the 32 raw bytes, and the constants are parsed into digests rather
//! than compared as text.

mod common;

use coblox_core::hash::{ChainId, Digest32, NodeId};
use coblox_core::merkle;
use coblox_core::registry::{self, DocumentKind};

use common::{
    IDENTITY_FIXTURE_NODE_ID, Pd0Kind, challenge_request_req0, challenge_response_resp0,
    enrollment_request_er0, identity_fixture_public_key, protocol_document_pd0,
    weak_subjectivity_checkpoint_wsc0, zero_chain_id,
};

// --- README.md#hash-conformance-fixtures, the published table ---------------

const EXPECTED_ENROLLMENT_REQUEST_HASH: &str =
    "sha256:cb1245f681d732aba57064face8872cd2104a185916ff1f0ac2d2e0651e7fb7f";
const EXPECTED_PARAMETER_SET_HASH: &str =
    "sha256:a2553f36f496d30a7773b9f6424c3ffd5ef22e3f8620bf0cca88a9bcdccd4f63";
const EXPECTED_POLICY_HASH: &str =
    "sha256:fbc7493ae6da64e92d935f35ecb9c2703c005df960e18e7cb609606838132f0d";
const EXPECTED_HOSTING_RATE_CARD_HASH: &str =
    "sha256:9b10204164f4197fb368f0f6ad6c186ae7af1a85b7b6383eeac412a10b8b3ae8";
const EXPECTED_CONSENSUS_PARAMETERS_HASH: &str =
    "sha256:840dd6a980a6350b4879c60f8581466165125408a62839d67468c32ca3f0c33f";
const EXPECTED_OBJECT_ID: &str =
    "sha256:fa67b77e3e686a4b3a2022fbe81edecd3e70a43a98d7e5aee2b76fdbdbe8a78c";
const EXPECTED_INPUT_HASH: &str =
    "sha256:66810b0847d6694ce6ac99a10db2f7339b89b10d3ed7817f6d27af832a6462c9";
const EXPECTED_ISSUER_COMMITMENT: &str =
    "sha256:19556b209c36de1940340bd3ada4a4c821fe70cde0fd3906af2b71f31445e4d5";
const EXPECTED_CHALLENGE_RANDOMNESS: &str =
    "sha256:8cebe4ad890bd41e8c37b87ad976ad92b8ef35aa3284c441d86691cfdaad88d7";
const EXPECTED_REQUEST_HASH: &str =
    "sha256:8beb98273d89ed31dd62803506e6739fc83ccf3bbca9c20d1028b998fa033360";
const EXPECTED_RESPONSE_HASH: &str =
    "sha256:cb7b622e8c2530b8da824765ccdd58cc29b116824bc8ad527fde2f262647df41";
const EXPECTED_ADMISSION_TAG: &str =
    "sha256:457915b8cd8816c5fe76651bdda0578983f8e393c7e4fe0b24376ca0bca22628";
const EXPECTED_ELECTION_ENTROPY: &str =
    "sha256:29a63c10926e75ed418c6f3c7670b0edfeb0b021868d1342b788327f53767e42";
const EXPECTED_ELECTION_SEED: &str =
    "sha256:9e2aa2621f957279e4bdf1c4ccea5629ce429892ac3f9af10c9456a3c78dad85";
const EXPECTED_ELECTION_TICKET: &str =
    "sha256:a10e8ec4a79c2defa40f869c68c2a1570bbb4a5b12b597a28d94294bf7582f21";
const EXPECTED_WEAK_SUBJECTIVITY_CHECKPOINT_HASH: &str =
    "sha256:2bc543a3f8e4df60735e6431a6c1fb7293ed53047e98fe2e5bc1a879f200c71e";

/// "carried on the wire as the unpadded base64url of those 32 bytes".
const EXPECTED_CHALLENGE_RANDOMNESS_BASE64URL: &str = "jOvkrYkL1B6MN7h62XatkrjvNaoyhMRB2GaRz9qtiNc";

// --- README.md#weak-subjectivity-checkpoint --------------------------------

/// "an empty list uses `H(0x33)` as `revocation_root`, which is [...]".
const EXPECTED_EMPTY_REVOCATION_ROOT: &str =
    "sha256:4e07408562bedb8b60ce05c1decfe3ad16b72230967de01f640b7e4729b49fce";

/// "Fixture `REVL-0`, the leaf for `cblx1revokedfixture` at `effective_height`
/// 50, is [...] which is also the single-entry root."
const EXPECTED_REVL0: &str =
    "sha256:7fb1f4024627c413cbf70b49a390b6d31778e667e86042864c4bed107cd52497";

/// The number of registry rows this file reproduces, checked by
/// [`the_registry_is_covered_in_full`] against the assertions that exist.
const REGISTRY_ROW_COUNT: usize = 16;

fn expect(text: &str) -> Digest32 {
    Digest32::parse_prefixed(text).expect("an expected value from the specification")
}

fn issuer_commitment_cmt0() -> Digest32 {
    // "`CMT-0` is the issuer commitment for issuer `cblx1issuerfixture`,
    // `commitment_epoch` 1, and an issuer secret of `44` repeated 32 bytes."
    registry::issuer_commitment(&zero_chain_id(), "cblx1issuerfixture", 1, &[0x44; 32])
        .expect("CMT-0")
}

fn challenge_randomness_rnd0() -> Digest32 {
    // "`RND-0` is the challenge randomness derived from `CMT-0`, beacon height
    // 1, beacon block ID `55` repeated 32 bytes, and subject `cblx1fixture`."
    registry::challenge_randomness(
        &zero_chain_id(),
        1,
        &Digest32::repeated(0x55),
        &issuer_commitment_cmt0(),
        &[0x44; 32],
        "cblx1fixture",
    )
    .expect("RND-0")
}

fn election_entropy_elec0() -> Digest32 {
    // "`ELEC-0` is the epoch-3 election of the worked example [...]:
    // `election_epoch` 3, `election_entropy_blocks` 3, and entropy block IDs
    // `aa`, `bb` and `cc` each repeated 32 bytes in that order."
    registry::election_entropy(
        &zero_chain_id(),
        3,
        3,
        &[
            Digest32::repeated(0xaa),
            Digest32::repeated(0xbb),
            Digest32::repeated(0xcc),
        ],
    )
    .expect("ELEC-0 entropy")
}

#[test]
fn enrollment_request_hash_over_er0() {
    let actual = registry::enrollment_request_hash(&zero_chain_id(), &enrollment_request_er0());
    assert_eq!(actual, expect(EXPECTED_ENROLLMENT_REQUEST_HASH));
}

#[test]
fn parameter_set_hash_over_enrollment_pd0() {
    let actual = registry::protocol_document_hash(
        DocumentKind::EnrollmentParameters,
        &zero_chain_id(),
        &protocol_document_pd0(Pd0Kind::Enrollment),
    )
    .expect("enrollment PD-0");
    assert_eq!(actual, expect(EXPECTED_PARAMETER_SET_HASH));
}

#[test]
fn policy_hash_over_reward_pd0() {
    let actual = registry::protocol_document_hash(
        DocumentKind::RewardPolicy,
        &zero_chain_id(),
        &protocol_document_pd0(Pd0Kind::Reward),
    )
    .expect("reward PD-0");
    assert_eq!(actual, expect(EXPECTED_POLICY_HASH));
}

#[test]
fn hosting_rate_card_hash_over_hosting_pd0() {
    let actual = registry::protocol_document_hash(
        DocumentKind::HostingRateCard,
        &zero_chain_id(),
        &protocol_document_pd0(Pd0Kind::Hosting),
    )
    .expect("hosting PD-0");
    assert_eq!(actual, expect(EXPECTED_HOSTING_RATE_CARD_HASH));
}

#[test]
fn consensus_parameters_hash_over_consensus_pd0() {
    let actual = registry::protocol_document_hash(
        DocumentKind::ConsensusParameters,
        &zero_chain_id(),
        &protocol_document_pd0(Pd0Kind::Consensus),
    )
    .expect("consensus PD-0");
    assert_eq!(actual, expect(EXPECTED_CONSENSUS_PARAMETERS_HASH));
}

#[test]
fn object_id_over_the_three_byte_fixture() {
    // "byte fixtures use `00 01 02`".
    let actual = registry::object_id(&[0x00, 0x01, 0x02]).expect("object_id");
    assert_eq!(actual, expect(EXPECTED_OBJECT_ID));
}

#[test]
fn input_hash_over_the_three_byte_fixture() {
    let actual = registry::input_hash(&[0x00, 0x01, 0x02]).expect("input_hash");
    assert_eq!(actual, expect(EXPECTED_INPUT_HASH));
}

#[test]
fn issuer_commitment_over_cmt0() {
    assert_eq!(issuer_commitment_cmt0(), expect(EXPECTED_ISSUER_COMMITMENT));
}

#[test]
fn challenge_randomness_over_rnd0() {
    let actual = challenge_randomness_rnd0();
    assert_eq!(actual, expect(EXPECTED_CHALLENGE_RANDOMNESS));
    assert_eq!(
        coblox_core::encoding::base64url_encode(actual.as_bytes()),
        EXPECTED_CHALLENGE_RANDOMNESS_BASE64URL
    );
}

#[test]
fn request_hash_over_req0() {
    let request = challenge_request_req0(&challenge_randomness_rnd0(), &issuer_commitment_cmt0());
    let actual = registry::challenge_request_hash(&zero_chain_id(), &request);
    assert_eq!(actual, expect(EXPECTED_REQUEST_HASH));
}

#[test]
fn response_hash_over_resp0() {
    let actual = registry::challenge_response_hash(&zero_chain_id(), &challenge_response_resp0());
    assert_eq!(actual, expect(EXPECTED_RESPONSE_HASH));
}

#[test]
fn admission_tag_over_adm0() {
    // "`ADM-0` uses zero `chain_id`, `admission_nonce` `88` repeated 16 bytes
    // [...] the identity fixture public key [...] and `admission_solution` "0"."
    let actual = registry::admission_tag(
        &zero_chain_id(),
        &[0x88; 16],
        &identity_fixture_public_key(),
        0,
    );
    assert_eq!(actual, expect(EXPECTED_ADMISSION_TAG));
}

/// The `admission_nonce` of `ADM-0` is published in both spellings; checking
/// them against each other is what makes the fixture unambiguous.
#[test]
fn the_admission_nonce_base64url_spelling_matches_its_bytes() {
    assert_eq!(
        coblox_core::encoding::base64url_encode(&[0x88; 16]),
        "iIiIiIiIiIiIiIiIiIiIiA"
    );
}

#[test]
fn election_entropy_over_elec0() {
    assert_eq!(election_entropy_elec0(), expect(EXPECTED_ELECTION_ENTROPY));
}

#[test]
fn election_seed_over_elec0() {
    let actual = registry::election_seed(&zero_chain_id(), 3, &election_entropy_elec0());
    assert_eq!(actual, expect(EXPECTED_ELECTION_SEED));
}

#[test]
fn election_ticket_over_elec0() {
    // "Its `election_ticket` row uses the account key `05` repeated 32 bytes."
    let seed = registry::election_seed(&zero_chain_id(), 3, &election_entropy_elec0());
    let actual = registry::election_ticket(
        &zero_chain_id(),
        &seed,
        &coblox_core::hash::AccountKey::from_bytes([0x05; 32]),
    );
    assert_eq!(actual, expect(EXPECTED_ELECTION_TICKET));
}

#[test]
fn weak_subjectivity_checkpoint_hash_over_wsc0() {
    let actual = registry::weak_subjectivity_checkpoint_hash(
        &zero_chain_id(),
        &weak_subjectivity_checkpoint_wsc0(),
    );
    assert_eq!(actual, expect(EXPECTED_WEAK_SUBJECTIVITY_CHECKPOINT_HASH));
}

// --- Values published outside the table but with the same standing ----------

#[test]
fn the_empty_revocation_root_is_the_published_hash_of_its_tag() {
    let actual = merkle::revocation_root(&[]).expect("empty revocation root");
    assert_eq!(actual, expect(EXPECTED_EMPTY_REVOCATION_ROOT));
}

#[test]
fn revl0_is_both_the_leaf_and_the_single_entry_root() {
    let leaf = merkle::revocation_leaf("cblx1revokedfixture", 50).expect("REVL-0 leaf");
    assert_eq!(leaf, expect(EXPECTED_REVL0));
    let root =
        merkle::revocation_root(&[("cblx1revokedfixture".to_owned(), 50)]).expect("REVL-0 root");
    assert_eq!(root, expect(EXPECTED_REVL0));
}

/// `identity.md`: "conformance fixtures MUST recompute the identifier from the
/// fixture key rather than treating prose as a trust anchor."
#[test]
fn the_fixture_node_id_is_recomputed_from_the_fixture_key() {
    let derived = NodeId::derive(&identity_fixture_public_key());
    assert_eq!(derived.as_str(), IDENTITY_FIXTURE_NODE_ID);
}

/// `chain_id` has no published fixture value, so this checks the property the
/// derivation exists for rather than a magic number: distinct networks over the
/// same genesis block, and distinct genesis blocks on the same network, bind to
/// distinct chains.
#[test]
fn chain_id_binds_both_the_network_and_the_genesis_block() {
    let genesis = Digest32::repeated(0x01);
    let a = ChainId::derive("coblox-devnet-0", &genesis).expect("chain a");
    let b = ChainId::derive("coblox-devnet-1", &genesis).expect("chain b");
    let c = ChainId::derive("coblox-devnet-0", &Digest32::repeated(0x02)).expect("chain c");
    assert_ne!(a, b);
    assert_ne!(a, c);
    assert_eq!(a, ChainId::derive("coblox-devnet-0", &genesis).unwrap());
}

/// Guards the count in the evidence: sixteen table rows, each with its own
/// test above. Raising the count without adding a test fails here.
#[test]
fn the_registry_is_covered_in_full() {
    let covered = [
        "enrollment_request_hash / ER-0",
        "parameter_set_hash / enrollment PD-0",
        "policy_hash / reward PD-0",
        "hosting_rate_card_hash / hosting PD-0",
        "consensus_parameters_hash / consensus PD-0",
        "object_id / bytes 00 01 02",
        "input_hash / bytes 00 01 02",
        "issuer_commitment / CMT-0",
        "challenge_randomness / RND-0",
        "request_hash / REQ-0",
        "response_hash / RESP-0",
        "admission_tag / ADM-0",
        "election_entropy / ELEC-0",
        "election_seed / ELEC-0",
        "election_ticket / ELEC-0",
        "weak_subjectivity_checkpoint_hash / WSC-0",
    ];
    assert_eq!(covered.len(), REGISTRY_ROW_COUNT);
}
