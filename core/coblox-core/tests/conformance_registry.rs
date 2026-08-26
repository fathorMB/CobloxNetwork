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

use coblox_core::hash::{AccountKey, ChainId, Digest32, NodeId};
use coblox_core::merkle;
use coblox_core::registry::{self, DocumentKind};

use common::{
    IDENTITY_FIXTURE_NODE_ID, Pd0Kind, challenge_request_req0, challenge_response_resp0,
    enrollment_request_er0, identity_fixture_public_key, protocol_document_pd0,
    weak_subjectivity_checkpoint_wsc0, zero_chain_id,
};

// --- README.md#hash-conformance-fixtures, the published table ---------------

const EXPECTED_ENROLLMENT_REQUEST_HASH: &str =
    "sha256:52118f65908736ec7fd837a4d6c1b8c2b3ba28e2f0127cea6e282b311e401e58";
const EXPECTED_PARAMETER_SET_HASH: &str =
    "sha256:a2553f36f496d30a7773b9f6424c3ffd5ef22e3f8620bf0cca88a9bcdccd4f63";
const EXPECTED_POLICY_HASH: &str =
    "sha256:89da35fbb8f0ba3c9ebffc0e3c5987045a005aaa7414356ef16a978a92025c48";
const EXPECTED_HOSTING_RATE_CARD_HASH: &str =
    "sha256:9b10204164f4197fb368f0f6ad6c186ae7af1a85b7b6383eeac412a10b8b3ae8";
const EXPECTED_CONSENSUS_PARAMETERS_HASH: &str =
    "sha256:87dc1d92edcd94d5efe3837af9157a4bda604dbd7a658f509bd6fb864f86ada5";
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
const EXPECTED_APP0_ACCOUNT_KEY: &str =
    "sha256:a881e2e0907aa86b225aaa2a2e1898afda1ce4733bd6d9cb390475ded4737e9d";
const EXPECTED_APP0_APP_LEAF: &str =
    "sha256:2eac8b0a7955a70543eddf975843fb8e4ddf377daef08b61c7b8cde469515697";

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
const REGISTRY_ROW_COUNT: usize = 18;

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

/// `APP-0`, the app account in state `suspended`.
///
/// This is the row [DEBT-012] existed for. `app_leaf` commits `lifecycle_u8`,
/// which no document assigned until 2026-08-25, so two conformant
/// implementations could produce different `state_root` values for the same
/// state and nothing published would have caught it. The fixture is
/// deliberately **not** `active`: the state whose byte an implementer would
/// guess correctly proves nothing about the encoding.
#[test]
fn app0_account_key_and_app_leaf_match_the_registry() {
    // "`APP-0` is an **app** account in state `suspended`, for `app_id` `99`
    // repeated 32 bytes, with `balance_microtokens` 1, `account_nonce` 1 and
    // `suspension_effective_epoch` 1".
    let account_key = AccountKey::for_app(&Digest32::repeated(0x99));
    assert_eq!(
        Digest32::from_bytes(*account_key.as_bytes()),
        expect(EXPECTED_APP0_ACCOUNT_KEY),
    );

    let leaf = merkle::AccountState::App {
        balance_microtokens: 1,
        account_nonce: 1,
        lifecycle: merkle::AppLifecycle::Suspended,
        suspension_effective_epoch: 1,
    }
    .leaf(&account_key);
    assert_eq!(leaf, expect(EXPECTED_APP0_APP_LEAF));

    // The published value is the one the specification's own encoding table
    // produces, and only that one. Under the provisional `0/1/2` mapping this
    // crate carried before [DEBT-012] was closed, `suspended` was `2` and the
    // leaf below is what it would have committed — a different `state_root`
    // for the same account.
    assert_ne!(
        merkle::AppLifecycle::Suspended.as_u8(),
        2,
        "the provisional encoding must not survive as the normative one"
    );
    assert_eq!(merkle::AppLifecycle::Suspended.as_u8(), 0x03);
}

/// Guards the count in the evidence: eighteen table rows, each with its own
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
        "account_key (app) / APP-0",
        "app_leaf / APP-0",
    ];
    assert_eq!(covered.len(), REGISTRY_ROW_COUNT);
}

fn sign_test_attestation(
    seed: &[u8; 32],
    chain_id: &ChainId,
    attestation: &mut coblox_core::identity::TransportKeyAttestation,
) -> [u8; 32] {
    use curve25519_dalek::constants::ED25519_BASEPOINT_POINT;
    use curve25519_dalek::scalar::Scalar;
    use sha2::{Digest, Sha512};

    let mut h = Sha512::new();
    h.update(seed);
    let az = h.finalize();

    let mut a_bytes = [0u8; 32];
    a_bytes.copy_from_slice(&az[..32]);
    a_bytes[0] &= 0b1111_1000;
    a_bytes[31] &= 0b0111_1111;
    a_bytes[31] |= 0b0100_0000;

    let mut a_wide = [0u8; 64];
    a_wide[..32].copy_from_slice(&a_bytes);
    let a_scalar = Scalar::from_bytes_mod_order_wide(&a_wide);
    let a_point = ED25519_BASEPOINT_POINT * a_scalar;
    let identity_public_key = a_point.compress().to_bytes();

    let unsigned = attestation.to_unsigned_json().unwrap();
    let preimage = registry::transport_key_attestation_signing_preimage(chain_id, &unsigned);

    let mut h = Sha512::new();
    h.update(&az[32..]);
    h.update(preimage.as_bytes());
    let r_wide = h.finalize();
    let r_scalar = Scalar::from_bytes_mod_order_wide(&r_wide.into());
    let r_point = ED25519_BASEPOINT_POINT * r_scalar;
    let r_bytes = r_point.compress().to_bytes();

    let mut h = Sha512::new();
    h.update(r_bytes);
    h.update(identity_public_key);
    h.update(preimage.as_bytes());
    let k_wide = h.finalize();
    let k_scalar = Scalar::from_bytes_mod_order_wide(&k_wide.into());

    let s_scalar = r_scalar + (k_scalar * a_scalar);
    let s_bytes = s_scalar.to_bytes();

    let mut sig = [0u8; 64];
    sig[..32].copy_from_slice(&r_bytes);
    sig[32..].copy_from_slice(&s_bytes);
    attestation.signature = sig;

    identity_public_key
}

/// The term no parameter bounds, and what an anchor does to it.
///
/// `identity.md` declared, before [SPEC-020], that *a receiver whose clock is
/// far behind accepts attestations that expired hours ago, and no certificate
/// attests a clock*. That term — not the `D_max + S_max` window the debt's
/// original title named — is the dominant one in the exposure, and it is the
/// only one with no bound at all.
///
/// This test asserts three things that must hold together, and the third is the
/// one a reader should check first:
///
/// 1. a receiver an hour behind **still** accepts an attestation that expired
///    fifty minutes ago when it has no checkpoint — the declared limit,
///    reproduced rather than assumed;
/// 2. the same receiver, holding a checkpoint issued at real time, **rejects**
///    it — the reduction. Not the closure: the residue becomes the age of the
///    checkpoint held, which no rule of this protocol bounds;
/// 3. the floor moves the comparison in the rejecting direction **only**. It
///    never revives an attestation the local clock had already expired, and it
///    never expires one that is genuinely live at the external reading.
///
/// The quantity varied here is the one every case of `gate_no_attestation_rejected`
/// holds constant: the *form* of the clock. That gate exercises nine rejection
/// paths and two skew edges, and all eleven with a bare local clock — which is
/// correct for what they test and is why this case exists separately
/// ([SKILL-001] step 4).
///
/// A quantity this case in turn holds constant is *which half of rule 5 is
/// exercised*: `created_at_ms` is in the past throughout, so all six sub-cases
/// test the expiry half. The admission half is
/// `the_floor_is_spent_only_where_it_rejects`, and it was written after
/// [REVIEW] RF-002 found the cell empty.
#[test]
fn the_external_clock_floor_reduces_the_term_no_parameter_bounds() {
    use coblox_core::ConsensusVerifier;
    use coblox_core::cadence::AttestationClock;
    use coblox_core::error::AttestationError;
    use coblox_core::identity::{AttestationBounds, TransportKeyAttestation};

    let chain_id = zero_chain_id();
    let network_id = "coblox-devnet-0";
    let transport_pk = [0x55u8; 32];
    let verifier = ConsensusVerifier;
    let seed = [0x42u8; 32];

    // Real time is 5 000 000. The attestation was created at 1 000 000 and
    // expired at 2 000 000: fifty minutes dead, on a window well inside the cap.
    let created_at_ms = 1_000_000;
    let expires_at_ms = 2_000_000;
    let real_time_ms = 5_000_000;
    let bounds = AttestationBounds {
        max_validity_ms: 1_000_000,
        max_future_skew_ms: 5_000,
    };

    let mut attestation = TransportKeyAttestation::new(
        network_id.to_owned(),
        NodeId::from_string("placeholder".to_owned()),
        transport_pk,
        created_at_ms,
        expires_at_ms,
        [0u8; 64],
    );
    let identity_public_key = sign_test_attestation(&seed, &chain_id, &mut attestation);
    attestation.node_id = NodeId::derive(&identity_public_key);
    sign_test_attestation(&seed, &chain_id, &mut attestation);

    let verify = |clock: AttestationClock| {
        attestation.verify(
            &chain_id,
            network_id,
            &identity_public_key,
            &transport_pk,
            clock,
            &bounds,
            &verifier,
        )
    };

    // A receiver whose clock reads 1 500 000 while real time is 5 000 000 — an
    // hour behind — and which holds no checkpoint.
    let behind_ms = 1_500_000;
    let bootstrap = AttestationClock::local_only(behind_ms);
    assert_eq!(bootstrap.floor_ms(), 0);
    assert!(
        verify(bootstrap).is_ok(),
        "the declared limit of identity.md is reproduced, not assumed: a \
         bootstrap receiver an hour behind accepts an attestation that expired \
         fifty minutes ago"
    );

    // The same receiver, now holding a weak subjectivity checkpoint issued at
    // real time. Nothing about the receiver's own clock changed.
    let anchored = AttestationClock::with_checkpoint_floor(behind_ms, real_time_ms);
    assert_eq!(anchored.now_ms(), real_time_ms);
    assert_eq!(anchored.floor_ms(), real_time_ms - behind_ms);
    assert!(matches!(
        verify(anchored).unwrap_err(),
        coblox_core::Error::Attestation(AttestationError::Expired { .. })
    ));

    // Direction, asserted rather than argued. A checkpoint *older* than the
    // local clock cannot revive an attestation the local clock has expired:
    // the floor is a lower bound and takes the larger of the two.
    let stale_checkpoint = AttestationClock::with_checkpoint_floor(real_time_ms, 1_500_000);
    assert_eq!(stale_checkpoint.now_ms(), real_time_ms);
    assert!(matches!(
        verify(stale_checkpoint).unwrap_err(),
        coblox_core::Error::Attestation(AttestationError::Expired { .. })
    ));

    // And the floor does not expire an attestation that is live at the external
    // reading: a checkpoint issued inside the window is accepted by a behind
    // receiver exactly as the external clock says it should be.
    assert!(
        verify(AttestationClock::with_checkpoint_floor(
            behind_ms, 1_999_999
        ))
        .is_ok()
    );
    assert!(matches!(
        verify(AttestationClock::with_checkpoint_floor(
            behind_ms, 2_000_001
        ))
        .unwrap_err(),
        coblox_core::Error::Attestation(AttestationError::Expired { .. })
    ));

    // The boundary itself: `now_ms > expires_at_ms` is the rejection, so the
    // last accepting external reading is `expires_at_ms` exactly.
    assert!(
        verify(AttestationClock::with_checkpoint_floor(
            behind_ms,
            expires_at_ms
        ))
        .is_ok()
    );
}

/// The half of rule 5 the floor must **not** touch, and the cell that was empty.
///
/// Rule 5 rejects when `now_ms > expires_at_ms` *or* when
/// `created_at_ms > now_ms + max_future_skew_ms`. A floor under `now_ms` rejects
/// more on the first clause and **admits more on the second**. Calling a floor
/// "fail-closed" is therefore true of one half of the rule and false of the
/// other, and this test is the false half.
///
/// The attack it excludes: an anchor ahead of real time by `Δ` lets a receiver
/// whose own clock is **exact** accept an attestation postdated further than the
/// signed tolerance allows, making the real acceptance window
/// `max_validity_ms + Δ` with `Δ` chosen by whoever signs the checkpoint. The
/// release key already holds capabilities of denial and of anchoring; this one
/// is **admission on the transport**, and denial does not subsume admission.
///
/// Every other attestation case in this file — the eleven of
/// `gate_no_attestation_rejected` and the six of the floor case — holds
/// `created_at_ms` in the past, so all seventeen exercise the expiry half and
/// none exercise this one ([SKILL-001] step 4: the constant quantity was *which
/// half of rule 5 is under test*, and the cell "non-zero floor × admission half"
/// was empty).
#[test]
fn the_floor_is_spent_only_where_it_rejects() {
    use coblox_core::ConsensusVerifier;
    use coblox_core::cadence::AttestationClock;
    use coblox_core::error::AttestationError;
    use coblox_core::identity::{AttestationBounds, TransportKeyAttestation};

    let chain_id = zero_chain_id();
    let network_id = "coblox-devnet-0";
    let transport_pk = [0x55u8; 32];
    let verifier = ConsensusVerifier;
    let seed = [0x42u8; 32];

    // Real time is 1 000 000 and the receiver's own clock is exact. The
    // attestation is postdated by 50 000 ms, ten times the 5 000 ms tolerance
    // the signed parameters grant. Its declared duration is well inside the cap,
    // so rule 4 has nothing to say about it: only rule 5 stands between this
    // attestation and acceptance.
    let real_time_ms = 1_000_000;
    let created_at_ms = 1_050_000;
    let expires_at_ms = 1_500_000;
    let bounds = AttestationBounds {
        max_validity_ms: 1_000_000,
        max_future_skew_ms: 5_000,
    };
    // The anchor runs 100 000 ms ahead of real time.
    let anchor_ahead_ms = 1_100_000;

    let mut attestation = TransportKeyAttestation::new(
        network_id.to_owned(),
        NodeId::from_string("placeholder".to_owned()),
        transport_pk,
        created_at_ms,
        expires_at_ms,
        [0u8; 64],
    );
    let identity_public_key = sign_test_attestation(&seed, &chain_id, &mut attestation);
    attestation.node_id = NodeId::derive(&identity_public_key);
    sign_test_attestation(&seed, &chain_id, &mut attestation);

    let verify = |clock: AttestationClock| {
        attestation.verify(
            &chain_id,
            network_id,
            &identity_public_key,
            &transport_pk,
            clock,
            &bounds,
            &verifier,
        )
    };

    // The postdating is refused against the receiver's own clock, which is the
    // behaviour that existed before the floor and must survive it.
    assert!(matches!(
        verify(AttestationClock::local_only(real_time_ms)).unwrap_err(),
        coblox_core::Error::Attestation(AttestationError::Expired { .. })
    ));

    // The admission the defect would have granted, demonstrated rather than
    // described: a receiver whose clock genuinely reads the anchor's value
    // accepts this attestation. Nothing is wrong with that — its clock says the
    // attestation is live — and it is precisely why routing the admission half
    // through a floored clock would be an attacker-chosen acceptance.
    assert!(verify(AttestationClock::local_only(anchor_ahead_ms)).is_ok());

    // The rule as written: the receiver's clock is exact, the anchor is ahead,
    // the floor is non-zero — and the postdated attestation is still refused,
    // because the admission half never reads the floored value.
    let anchored = AttestationClock::with_checkpoint_floor(real_time_ms, anchor_ahead_ms);
    assert_eq!(anchored.now_ms(), anchor_ahead_ms);
    assert_eq!(anchored.local_clock_ms(), real_time_ms);
    assert!(
        anchored.floor_ms() > 0,
        "the floor must be live in this case"
    );
    assert!(matches!(
        verify(anchored).unwrap_err(),
        coblox_core::Error::Attestation(AttestationError::Expired { .. })
    ));

    // The expiry half still spends the floor, in the same call and on the same
    // value: an attestation that the anchor says is dead is refused even though
    // the receiver's exact clock would have accepted it.
    let mut dead = attestation.clone();
    dead.created_at_ms = 900_000;
    dead.expires_at_ms = 1_050_000;
    sign_test_attestation(&seed, &chain_id, &mut dead);
    let verify_dead = |clock: AttestationClock| {
        dead.verify(
            &chain_id,
            network_id,
            &identity_public_key,
            &transport_pk,
            clock,
            &bounds,
            &verifier,
        )
    };
    assert!(verify_dead(AttestationClock::local_only(real_time_ms)).is_ok());
    assert!(matches!(
        verify_dead(AttestationClock::with_checkpoint_floor(
            real_time_ms,
            anchor_ahead_ms
        ))
        .unwrap_err(),
        coblox_core::Error::Attestation(AttestationError::Expired { .. })
    ));

    // And the argument's own boundary: `max` is symmetric, so swapping the two
    // arguments of `with_checkpoint_floor` leaves `now_ms` identical. Before the
    // two halves were split that swap was invisible in behaviour and the type
    // was pure convention. It is not invisible now — the admission half reads
    // the argument the swap moves — and this assertion is what keeps it so.
    let swapped = AttestationClock::with_checkpoint_floor(anchor_ahead_ms, real_time_ms);
    assert_eq!(swapped.now_ms(), anchored.now_ms());
    assert_ne!(swapped.local_clock_ms(), anchored.local_clock_ms());
    assert!(
        verify(swapped).is_ok(),
        "the swap must be observable in behaviour, not merely in the accessors"
    );
}

/// GATE-NO-ATTESTATION-REJECTED:
/// A peer presenting a transport key without a valid attestation is rejected.
/// Exercising all failure paths: missing attestation, invalid signature,
/// mismatched node ID, mismatched transport key, expired/inactive time window,
/// and wrong network ID.
#[test]
#[allow(clippy::too_many_lines)]
fn gate_no_attestation_rejected() {
    use coblox_core::ConsensusVerifier;
    use coblox_core::cadence::AttestationClock;
    use coblox_core::error::AttestationError;
    use coblox_core::identity::{AttestationBounds, TransportKeyAttestation};

    let chain_id = zero_chain_id();
    let network_id = "coblox-devnet-0";
    let transport_pk = [0x55u8; 32];
    let other_transport_pk = [0x66u8; 32];
    let verifier = ConsensusVerifier;

    let seed = [0x42u8; 32];
    let created_at_ms = 1_000_000;
    let expires_at_ms = 2_000_000;
    // The bootstrap form throughout this gate: a receiver holding no weak
    // subjectivity checkpoint. Every assertion below therefore also asserts
    // that [SPEC-020] changed nothing for such a receiver — `local_only` has
    // `floor_ms() == 0` by construction.
    let now_ms = AttestationClock::local_only(1_500_000);
    // The two signed network parameters of `identity.md#bounded-validity-in-time`.
    // The window of this attestation is exactly `max_validity_ms`, so the
    // over-long case below is one millisecond away and not an order of
    // magnitude away.
    let bounds = AttestationBounds {
        max_validity_ms: 1_000_000,
        max_future_skew_ms: 5_000,
    };

    let mut attestation = TransportKeyAttestation::new(
        network_id.to_owned(),
        NodeId::from_string("placeholder".to_owned()),
        transport_pk,
        created_at_ms,
        expires_at_ms,
        [0u8; 64],
    );

    // Compute real identity public key and sign
    let identity_public_key = sign_test_attestation(&seed, &chain_id, &mut attestation);
    let real_node_id = NodeId::derive(&identity_public_key);
    attestation.node_id = real_node_id;
    // Re-sign with matching node_id
    sign_test_attestation(&seed, &chain_id, &mut attestation);

    // Valid attestation passes
    assert!(
        attestation
            .verify(
                &chain_id,
                network_id,
                &identity_public_key,
                &transport_pk,
                now_ms,
                &bounds,
                &verifier
            )
            .is_ok()
    );

    // Failure Path 1: Mismatched transport key
    let err = attestation
        .verify(
            &chain_id,
            network_id,
            &identity_public_key,
            &other_transport_pk,
            now_ms,
            &bounds,
            &verifier,
        )
        .unwrap_err();
    assert_eq!(
        err,
        coblox_core::Error::Attestation(AttestationError::TransportKeyMismatch)
    );

    // Failure Path 2: Mismatched network ID
    let err = attestation
        .verify(
            &chain_id,
            "coblox-mainnet",
            &identity_public_key,
            &transport_pk,
            now_ms,
            &bounds,
            &verifier,
        )
        .unwrap_err();
    assert!(matches!(
        err,
        coblox_core::Error::Attestation(AttestationError::NetworkIdMismatch { .. })
    ));

    // Failure Path 3: Expired attestation (now_ms > expires_at_ms)
    let err = attestation
        .verify(
            &chain_id,
            network_id,
            &identity_public_key,
            &transport_pk,
            AttestationClock::local_only(2_500_000),
            &bounds,
            &verifier,
        )
        .unwrap_err();
    assert!(matches!(
        err,
        coblox_core::Error::Attestation(AttestationError::Expired { .. })
    ));

    // Failure Path 4: Attestation not yet active (now_ms < created_at_ms)
    let err = attestation
        .verify(
            &chain_id,
            network_id,
            &identity_public_key,
            &transport_pk,
            AttestationClock::local_only(500_000),
            &bounds,
            &verifier,
        )
        .unwrap_err();
    assert!(matches!(
        err,
        coblox_core::Error::Attestation(AttestationError::Expired { .. })
    ));

    // Failure Path 5: Mismatched node ID
    let other_node_id = NodeId::from_string("cblx1otheridentity".to_owned());
    let mut bad_node_att = attestation.clone();
    bad_node_att.node_id = other_node_id;
    let err = bad_node_att
        .verify(
            &chain_id,
            network_id,
            &identity_public_key,
            &transport_pk,
            now_ms,
            &bounds,
            &verifier,
        )
        .unwrap_err();
    assert!(matches!(
        err,
        coblox_core::Error::Attestation(AttestationError::NodeIdMismatch { .. })
    ));

    // Failure Path 6: Invalid signature
    let mut bad_sig_att = attestation.clone();
    bad_sig_att.signature[0] ^= 0xff;
    let err = bad_sig_att
        .verify(
            &chain_id,
            network_id,
            &identity_public_key,
            &transport_pk,
            now_ms,
            &bounds,
            &verifier,
        )
        .unwrap_err();
    assert_eq!(
        err,
        coblox_core::Error::Attestation(AttestationError::InvalidSignature)
    );

    // Failure Path 7: the attested transport key IS the enrolled identity key.
    //
    // This is the path that carries the privacy property of [ADR-015]. Without
    // it a fully conformant node can present its identity key as its transport
    // key, and any offline reader of the ledger recomputes its Peer ID from the
    // published certificate — TM-28 in its original form, with every gate still
    // green. Remove the check in `identity.rs` and this assertion fails.
    let mut reused_key_att = attestation.clone();
    reused_key_att.transport_public_key = identity_public_key;
    sign_test_attestation(&seed, &chain_id, &mut reused_key_att);
    let err = reused_key_att
        .verify(
            &chain_id,
            network_id,
            &identity_public_key,
            // The handshake proves possession of exactly this key, so the
            // rejection cannot come from the transport-key comparison: it must
            // come from the distinctness rule.
            &identity_public_key,
            now_ms,
            &bounds,
            &verifier,
        )
        .unwrap_err();
    assert_eq!(
        err,
        coblox_core::Error::Attestation(AttestationError::TransportKeyEqualsIdentityKey)
    );

    // Failure Path 8: inverted validity window, `expires_at_ms < created_at_ms`.
    //
    // The first clause of the MUST in `identity.md#bounded-validity-in-time`.
    // It was implemented before this gate existed and exercised by nothing.
    let mut inverted_att = attestation.clone();
    inverted_att.created_at_ms = 2_000_000;
    inverted_att.expires_at_ms = 1_000_000;
    sign_test_attestation(&seed, &chain_id, &mut inverted_att);
    let err = inverted_att
        .verify(
            &chain_id,
            network_id,
            &identity_public_key,
            &transport_pk,
            AttestationClock::local_only(1_500_000),
            &bounds,
            &verifier,
        )
        .unwrap_err();
    assert!(matches!(
        err,
        coblox_core::Error::Attestation(AttestationError::InvalidValidityWindow { .. })
    ));

    // Failure Path 9: a window one millisecond longer than the signed cap.
    //
    // The second clause of the same MUST. Without it an attestation with
    // `expires_at_ms = u64::MAX` verifies, the binding is permanent, and the
    // stated reason for choosing timestamps — that a compromised transport key
    // expires on its own — is false.
    let mut long_att = attestation.clone();
    long_att.expires_at_ms = created_at_ms + bounds.max_validity_ms + 1;
    sign_test_attestation(&seed, &chain_id, &mut long_att);
    let err = long_att
        .verify(
            &chain_id,
            network_id,
            &identity_public_key,
            &transport_pk,
            now_ms,
            &bounds,
            &verifier,
        )
        .unwrap_err();
    assert_eq!(
        err,
        coblox_core::Error::Attestation(AttestationError::ValidityWindowTooLong {
            duration_ms: bounds.max_validity_ms + 1,
            maximum_ms: bounds.max_validity_ms,
        })
    );

    let mut unbounded_att = attestation.clone();
    unbounded_att.expires_at_ms = u64::MAX;
    sign_test_attestation(&seed, &chain_id, &mut unbounded_att);
    assert!(matches!(
        unbounded_att
            .verify(
                &chain_id,
                network_id,
                &identity_public_key,
                &transport_pk,
                now_ms,
                &bounds,
                &verifier,
            )
            .unwrap_err(),
        coblox_core::Error::Attestation(AttestationError::ValidityWindowTooLong { .. })
    ));

    // The clock-skew tolerance, at both of its edges, and its asymmetry.
    //
    // A receiver whose clock is behind by no more than `max_future_skew_ms`
    // still accepts a freshly issued attestation. If it did not, it would lose
    // `ledger-sync` — the only source from which it could correct its clock —
    // and the isolation would be self-sustaining.
    let earliest_accepting_clock = created_at_ms - bounds.max_future_skew_ms;
    assert!(
        attestation
            .verify(
                &chain_id,
                network_id,
                &identity_public_key,
                &transport_pk,
                AttestationClock::local_only(earliest_accepting_clock),
                &bounds,
                &verifier,
            )
            .is_ok()
    );
    assert!(matches!(
        attestation
            .verify(
                &chain_id,
                network_id,
                &identity_public_key,
                &transport_pk,
                AttestationClock::local_only(earliest_accepting_clock - 1),
                &bounds,
                &verifier,
            )
            .unwrap_err(),
        coblox_core::Error::Attestation(AttestationError::Expired { .. })
    ));
    // No slack is granted past `expires_at_ms`: one millisecond after expiry is
    // a rejection whatever the future-skew tolerance is.
    assert!(matches!(
        attestation
            .verify(
                &chain_id,
                network_id,
                &identity_public_key,
                &transport_pk,
                AttestationClock::local_only(expires_at_ms + 1),
                &bounds,
                &verifier,
            )
            .unwrap_err(),
        coblox_core::Error::Attestation(AttestationError::Expired { .. })
    ));
}
