//! `GEN-0`, the genesis derivation, and the rule that breaks its circularity.
//!
//! **Provenance rule for this file**, the same one `conformance_registry.rs`
//! carries. Every `EXPECTED_*` constant is copied character by character from
//! the table in `docs/protocol/README.md#hash-conformance-fixtures`. None is
//! produced by running this crate, because a test that generates its own
//! expectation passes whatever the implementation does.
//!
//! **Why this file is not enough on its own, and what stands beside it.**
//! `chain_id` is derived from `genesis_block_id`, which is the `block_id` of the
//! height-0 header, whose preimage carries `chain_id_32`. A rule that breaks a
//! circular derivation cannot be verified by one implementation: one
//! implementation is internally consistent *by construction*, which is exactly
//! why [DEBT-012] stayed invisible until [SPEC-010]. The second road is
//! `sim/tools/genesis_chain_id.py`, written from the document text, reaching
//! for `hashlib` and `json.dumps` where this file reaches for `PreimageWriter`
//! and `JsonObject`. The two share no code. What makes the published values
//! evidence is that both arrive at them.
//!
//! **`GEN-1` is the second genesis, and it is published.** A gate whose cases all
//! hold one quantity constant has never seen the case that breaks it, and here
//! the quantity that must move is the byte length of `network_id`. `GEN-1` was
//! first derived on both roads and only *printed*, with the two outputs compared
//! by eye: that showed the values move and asserted nothing about the two roads
//! moving together, so a divergence tomorrow would have failed no test
//! ([REVIEW-028] RF-001). Hardcoding the other road's answer here would have
//! been worse — a road that copies the other has stopped being a second road —
//! so the values were published in the registry table instead, and both roads
//! now meet the same third party, which is the arrangement `GEN-0` already had.

mod common;

use coblox_core::hash::{ChainId, Digest32, Domain, PreimageWriter};
use coblox_core::json::JsonObject;
use coblox_core::merkle::TaggedTree;
use coblox_core::registry::{self, DocumentKind};

use common::{consensus_body, consensus_parameters_pd0};

// --- README.md#hash-conformance-fixtures, the published table ---------------

const EXPECTED_EMPTY_TRANSACTIONS_ROOT: &str =
    "sha256:084fed08b978af4d7d196a7446a86b58009e636b611db16211b65a9aadff29c5";
const EXPECTED_GENESIS_CONSENSUS_PARAMETERS_HASH: &str =
    "sha256:bec637279b6dceb786a0758c8a48de508d6d08bff5878c0b71f844e48da0f275";
const EXPECTED_GENESIS_BLOCK_ID: &str =
    "sha256:1334f5368141f78f23528624bf91973cb4cdf316c1e3452cb0e5470ff7145f92";
const EXPECTED_CHAIN_ID: &str =
    "sha256:3004d71cffe8ea2cc07b254abcc65494c112c13b20a305910476860b6cc62847";
const EXPECTED_DHT_NAMESPACE_KEY: &str =
    "sha256:80c13c86cb480fe927e4aafe885b687d5fd2900a2d53e46de0460ee48f943b26";

const EXPECTED_GEN1_CONSENSUS_PARAMETERS_HASH: &str =
    "sha256:6ba582b42339763c4b79e7a41ff7d75f6283800a5a4b4d97176f318cb5f63c0d";
const EXPECTED_GEN1_GENESIS_BLOCK_ID: &str =
    "sha256:6b62539240dcbc9aedf3e47e32edef91d302cf0687865dad8904326d8f49c53d";
const EXPECTED_GEN1_CHAIN_ID: &str =
    "sha256:172fd2e8bbdffefecc8952c1e0b97b69275af0de9bc637c6735a09b872d5e033";
const EXPECTED_GEN1_DHT_NAMESPACE_KEY: &str =
    "sha256:e8ceaa4c9095078ae2347bb111484ed532e5c494e49341aba2f5b57312d72c7b";

// The consensus `PD-0` hash, which [SPEC-017] does not touch. It is asserted
// first, so that the method is shown to reproduce an unchanged published value
// before it is trusted on the five new ones.
const EXPECTED_CONSENSUS_PARAMETERS_HASH_PD0: &str =
    "sha256:87dc1d92edcd94d5efe3837af9157a4bda604dbd7a658f509bd6fb864f86ada5";

const GEN0_NETWORK_ID: &str = "genesis-fixture";
/// `GEN-1`, deliberately a different byte length from `GEN-0`'s.
const GEN1_NETWORK_ID: &str = "genesis-fixture-b";

/// The genesis `consensus_parameters` document of `GEN-0`.
///
/// "`schema_version:"0.1"`, `document_kind:"consensus_parameters"`,
/// `network_id:"genesis-fixture"`, `chain_id` the 32 zero bytes of the
/// placeholder, `sequence:"1"`, `activation_height:"0"`, and the body of the
/// consensus `PD-0` unchanged."
fn gen0_consensus_document(network_id: &str) -> JsonObject {
    JsonObject::builder()
        .str("schema_version", "0.1")
        .str("document_kind", "consensus_parameters")
        .str("network_id", network_id)
        .digest("chain_id", ChainId::GENESIS_PLACEHOLDER.as_digest())
        .uint("sequence", 1)
        .uint("activation_height", 0)
        .object("body", consensus_body(&consensus_parameters_pd0()))
        .build()
        .expect("the GEN-0 genesis consensus_parameters document")
}

/// The height-0 header of `GEN-0`.
fn gen0_header(network_id: &str, consensus_parameters_hash: &Digest32) -> JsonObject {
    JsonObject::builder()
        .str("schema_version", "0.1")
        .str("protocol_version", "0.1")
        .str("network_id", network_id)
        .uint("height", 0)
        .uint("round", 0)
        .uint("timestamp_ms", 1)
        .digest("previous_block_id", &Digest32::repeated(0x00))
        .digest("transactions_root", &TaggedTree::TRANSACTIONS.empty_root())
        .digest("state_root", &Digest32::repeated(0xEE))
        .digest("validator_set_hash", &Digest32::repeated(0xDD))
        .digest("next_validator_set_hash", &Digest32::repeated(0xDD))
        .digest("consensus_parameters_hash", consensus_parameters_hash)
        .build()
        .expect("the GEN-0 genesis header")
}

/// The whole derivation, for one network name.
fn derive(network_id: &str) -> (Digest32, Digest32, ChainId, Digest32) {
    let document = gen0_consensus_document(network_id);
    let consensus_parameters_hash = registry::protocol_document_hash(
        DocumentKind::ConsensusParameters,
        &ChainId::GENESIS_PLACEHOLDER,
        &document,
    )
    .expect("the document declares its own kind");
    let header = gen0_header(network_id, &consensus_parameters_hash);
    let (genesis_block_id, chain_id) =
        registry::genesis_derivation(network_id, &header).expect("a 15- or 17-byte network name");
    let dht = registry::dht_namespace_key(&genesis_block_id);
    (consensus_parameters_hash, genesis_block_id, chain_id, dht)
}

fn expect(text: &str) -> Digest32 {
    Digest32::parse_prefixed(text).expect("a published value is a canonical sha256 string")
}

#[test]
fn the_method_reproduces_a_value_this_pass_did_not_change() {
    let pd0 = common::protocol_document_pd0(common::Pd0Kind::Consensus);
    let hash = registry::protocol_document_hash(
        DocumentKind::ConsensusParameters,
        &ChainId::from_digest(Digest32::repeated(0x00)),
        &pd0,
    )
    .expect("PD-0 declares its own kind");
    assert_eq!(hash, expect(EXPECTED_CONSENSUS_PARAMETERS_HASH_PD0));
}

#[test]
fn the_empty_transactions_root_is_the_one_gen0_publishes() {
    assert_eq!(
        TaggedTree::TRANSACTIONS.empty_root(),
        expect(EXPECTED_EMPTY_TRANSACTIONS_ROOT)
    );
}

#[test]
fn gen0_derives_the_published_genesis_values() {
    let (consensus_parameters_hash, genesis_block_id, chain_id, dht) = derive(GEN0_NETWORK_ID);
    assert_eq!(
        consensus_parameters_hash,
        expect(EXPECTED_GENESIS_CONSENSUS_PARAMETERS_HASH)
    );
    assert_eq!(genesis_block_id, expect(EXPECTED_GENESIS_BLOCK_ID));
    assert_eq!(chain_id.as_digest(), &expect(EXPECTED_CHAIN_ID));
    assert_eq!(dht, expect(EXPECTED_DHT_NAMESPACE_KEY));
}

#[test]
fn the_placeholder_is_thirty_two_zero_bytes() {
    assert_eq!(
        ChainId::GENESIS_PLACEHOLDER.as_digest(),
        &Digest32::repeated(0x00)
    );
}

/// `GEN-1` derives the values the registry table publishes for it.
///
/// The expectations are the document's, not the Python road's. That distinction
/// is the whole of [REVIEW-028] RF-001: copying the other road's output here
/// would make this test confirm that road instead of meeting it, while printing
/// the values and comparing them by hand asserted nothing at all. Both roads
/// meet the table.
#[test]
fn gen1_derives_the_published_genesis_values() {
    let (document, block_id, chain_id, dht) = derive(GEN1_NETWORK_ID);
    assert_eq!(document, expect(EXPECTED_GEN1_CONSENSUS_PARAMETERS_HASH));
    assert_eq!(block_id, expect(EXPECTED_GEN1_GENESIS_BLOCK_ID));
    assert_eq!(chain_id.as_digest(), &expect(EXPECTED_GEN1_CHAIN_ID));
    assert_eq!(dht, expect(EXPECTED_GEN1_DHT_NAMESPACE_KEY));
}

/// And every one of them differs from its `GEN-0` counterpart.
///
/// This is the property `GEN-1` exists for, kept as its own assertion: two
/// published tables of digests that happened to coincide would satisfy the test
/// above and prove nothing about the network name entering the derivation.
#[test]
fn gen1_moves_every_derived_value_away_from_gen0() {
    let (gen0_doc, gen0_block, gen0_chain, gen0_dht) = derive(GEN0_NETWORK_ID);
    let (gen1_doc, gen1_block, gen1_chain, gen1_dht) = derive(GEN1_NETWORK_ID);

    assert_ne!(gen0_doc, gen1_doc);
    assert_ne!(gen0_block, gen1_block);
    assert_ne!(gen0_chain.as_digest(), gen1_chain.as_digest());
    assert_ne!(gen0_dht, gen1_dht);
}

/// The placeholder clause, watched failing.
///
/// A rule nobody has seen bind is arithmetic. Hashing the same header under a
/// chain ID of 32 `ff` bytes must move both derived values; if it did not, the
/// placeholder would not be load-bearing and the rule would say nothing.
#[test]
fn a_different_placeholder_moves_the_genesis_block_id_and_the_chain_id() {
    let document = gen0_consensus_document(GEN0_NETWORK_ID);
    let consensus_parameters_hash = registry::protocol_document_hash(
        DocumentKind::ConsensusParameters,
        &ChainId::GENESIS_PLACEHOLDER,
        &document,
    )
    .expect("the document declares its own kind");
    let header = gen0_header(GEN0_NETWORK_ID, &consensus_parameters_hash);

    let wrong = ChainId::from_digest(Digest32::repeated(0xFF));
    let wrong_block_id = registry::block_id(&wrong, &header);
    let wrong_chain_id =
        ChainId::derive(GEN0_NETWORK_ID, &wrong_block_id).expect("a 15-byte network name");

    assert_ne!(wrong_block_id, expect(EXPECTED_GENESIS_BLOCK_ID));
    assert_ne!(wrong_chain_id.as_digest(), &expect(EXPECTED_CHAIN_ID));
}

/// The length-prefix clause, watched failing.
///
/// `chain_id` hashes `u32be(len(network_id_utf8)) || network_id_utf8`. Dropping
/// the prefix is the mistake an implementation makes silently, so it is made
/// here on purpose and required to produce a different value.
#[test]
fn dropping_the_network_length_prefix_moves_the_chain_id() {
    let (_, genesis_block_id, chain_id, _) = derive(GEN0_NETWORK_ID);
    let without_prefix = PreimageWriter::new(Domain::CHAIN_ID)
        .raw(GEN0_NETWORK_ID.as_bytes())
        .raw32(&genesis_block_id)
        .finish();
    assert_ne!(&without_prefix, chain_id.as_digest());
}

// --- The `key_binding_signature` clause, which no published value exercises --

/// A genesis binding is taken under the placeholder, and moves with `network_id`.
///
/// This clause is the one the fixtures cannot reach: publishing a `ValidatorSet`
/// would publish a genesis cohort, whose size, stagger and term limits the
/// election constraint block governs. So it is asserted here, in memory, against
/// nothing published — which closes the gap [REVIEW-029] RF-004 named, that the
/// clause was expressed neither in a fixture nor anywhere in the code.
///
/// The second assertion is the one that matters. Before `network_id` entered the
/// object, these two preimages were **byte-identical**: the placeholder is the
/// same 32 zero bytes on every network, so a genesis entry signed on one network
/// carried a signature that verified unchanged on another, and a distribution
/// could seat a validator in a genesis it never consented to ([REVIEW-029]
/// RF-002).
#[test]
fn a_genesis_key_binding_is_placeholder_bound_and_moves_with_the_network() {
    let consensus_public_key = [0x22u8; 32];
    let build = |network_id: &str| {
        coblox_core::validator_set::consensus_key_binding_preimage(
            &ChainId::GENESIS_PLACEHOLDER,
            network_id,
            0,
            &consensus_public_key,
            "cblx1genesisfixture",
            "cblx1genesisfixture",
        )
        .expect("a genesis key binding")
    };

    let on_gen0 = build(GEN0_NETWORK_ID);
    let on_gen1 = build(GEN1_NETWORK_ID);

    assert!(on_gen0.binds(
        Domain::SIG_CONSENSUS_KEY_BINDING,
        &ChainId::GENESIS_PLACEHOLDER
    ));
    assert_ne!(
        on_gen0.as_bytes(),
        on_gen1.as_bytes(),
        "two networks must not share a genesis key-binding payload"
    );

    // And `binds()` alone cannot tell them apart, which is the residual the
    // document states: inside the genesis window the carried context is the
    // domain and a constant, so the separation lives in the payload.
    assert!(on_gen1.binds(
        Domain::SIG_CONSENSUS_KEY_BINDING,
        &ChainId::GENESIS_PLACEHOLDER
    ));
}
