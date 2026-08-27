//! How much one `block_request` can make a node emit.
//!
//! [REVIEW-049] RF-006: the response goes to the gossip topic rather than to the
//! requester, and used to carry every block from `from_height` to the tip. One
//! message with `from_height = 1` made every validator re-broadcast the whole
//! chain to everyone. The response is now bounded by a declared constant, and
//! the periodic request is emitted only when a peer has announced a height this
//! node does not have.
//!
//! What this does **not** claim: the response is still a topic publication and
//! not the request/response stream `wire.md` specifies for `ledger-sync`. That
//! is recorded as remaining work in the implementation evidence of [SPEC-029],
//! not as done.

use tempfile::TempDir;

use coblox_core::block::BlockHeader;
use coblox_core::consensus::{CertificateSignature, FinalizedBlock, QuorumCertificate};
use coblox_core::hash::{ChainId, Digest32};
use coblox_core::json::JsonObject;
use coblox_core::merkle;
use coblox_node::config::{NodeConfig, devnet_4_validator_set, devnet_timeouts};
use coblox_node::envelope::{SignedEnvelope, fresh_nonce};
use coblox_node::node::{MAX_BLOCKS_PER_SYNC_RESPONSE, NodeRunner};
use coblox_node::store::BlockStore;

const NETWORK_ID: &str = "coblox-devnet-0";
const CHAIN_LENGTH: u64 = 20;

fn chain_id() -> ChainId {
    ChainId::from_digest(Digest32::repeated(0x7a))
}

fn now_ms() -> u64 {
    u64::try_from(
        std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("clock at or after the Unix epoch")
            .as_millis(),
    )
    .expect("fits")
}

/// Writes a chain of `CHAIN_LENGTH` blocks into `dir`.
///
/// The certificates are not genuine — building twenty real ones needs a quorum
/// this test does not run. That is sound here and only here: the branch under
/// test reads a block out of the store and serializes it, and never verifies a
/// certificate. The certificate rules have their own tests in `coblox-core`.
fn seed_chain(dir: &std::path::Path) {
    let (set, _) = devnet_4_validator_set();
    let set_hash = set.hash().expect("set hash");
    let mut store = BlockStore::open(
        dir.join("blocks.jsonl"),
        chain_id(),
        Digest32::repeated(0x01),
    )
    .expect("store opens");

    let mut previous = Digest32::repeated(0x01);
    for height in 1..=CHAIN_LENGTH {
        let header = BlockHeader {
            schema_version: "0.1".to_owned(),
            protocol_version: "0.1".to_owned(),
            network_id: NETWORK_ID.to_owned(),
            height,
            round: 0,
            timestamp_ms: 1_787_654_400_000 + height * 5_000,
            previous_block_id: previous,
            transactions_root: merkle::transactions_root(&[]).expect("empty root"),
            state_root: Digest32::repeated(0x33),
            validator_set_hash: set_hash,
            next_validator_set_hash: set_hash,
            consensus_parameters_hash: Digest32::repeated(0x44),
        };
        let block = FinalizedBlock {
            header,
            transactions: Vec::new(),
            quorum_certificate: QuorumCertificate {
                height,
                round: 0,
                block_id: Digest32::repeated(0x00),
                validator_set_hash: set_hash,
                signatures: vec![CertificateSignature {
                    validator_id: "val-000".to_owned(),
                    signature: [0x11; 64],
                }],
            },
        };
        let block_id = block.block_id(&chain_id()).expect("block id");
        let mut block = block;
        block.quorum_certificate.block_id = block_id;
        store.append_block(&block).expect("append");
        previous = block_id;
    }
    assert_eq!(store.latest_height(), CHAIN_LENGTH);
}

fn runner_with_chain(dir: &TempDir) -> NodeRunner {
    seed_chain(dir.path());
    let (set, keys) = devnet_4_validator_set();
    let config = NodeConfig {
        validator_id: "val-000".to_owned(),
        node_id: "val-000".to_owned(),
        signing_key: keys[0].clone(),
        network_id: NETWORK_ID.to_owned(),
        chain_id: chain_id(),
        genesis_block_id: Digest32::repeated(0x01),
        listen_addr: "/ip4/127.0.0.1/tcp/0".to_owned(),
        seed_peers: Vec::new(),
        data_dir: dir.path().to_path_buf(),
        validator_set: set,
        timeouts: devnet_timeouts(),
        target_height: None,
    };
    let (runner, _network) = NodeRunner::new(config).expect("runner starts");
    runner
}

#[test]
fn the_bound_is_a_declared_constant() {
    assert_eq!(
        MAX_BLOCKS_PER_SYNC_RESPONSE, 8,
        "the bound is a named constant a reader can find; if it moves, the \
         evidence that cites it moves with it"
    );
}

/// The chain this file builds must be longer than the bound, or the test above
/// observes nothing. Checked at compile time so it cannot be edited apart.
const _: () = assert!(MAX_BLOCKS_PER_SYNC_RESPONSE < CHAIN_LENGTH);

#[tokio::test]
async fn a_block_request_from_height_one_emits_no_more_than_the_bound() {
    let dir = TempDir::new().expect("tempdir");
    let mut runner = runner_with_chain(&dir);
    let before = runner.outbound_attempts();

    let (_, keys) = devnet_4_validator_set();
    let payload = JsonObject::builder()
        .uint("from_height", 1)
        .build()
        .expect("payload");
    let request = SignedEnvelope::build_and_sign(
        &chain_id(),
        NETWORK_ID,
        "block_request",
        "val-001",
        now_ms(),
        30_000,
        fresh_nonce().expect("entropy"),
        payload,
        &keys[1],
    )
    .expect("request builds");

    runner
        .handle_envelope(request)
        .expect("a well-signed block_request from a member is admitted");

    let emitted = runner.outbound_attempts() - before;
    assert_eq!(
        emitted, MAX_BLOCKS_PER_SYNC_RESPONSE,
        "a request for a {CHAIN_LENGTH}-block chain must answer with the bound, \
         not with the chain"
    );
    assert!(emitted < CHAIN_LENGTH);

    // And a second request from the same peer, immediately after, is not
    // answered at all: `MIN_MS_BETWEEN_SYNC_ANSWERS`. Running the runbook showed
    // that the per-answer bound alone is not enough — an unthrottled catch-up
    // burst delayed live consensus messages past their own expiry.
    let after_first = runner.outbound_attempts();
    let payload = JsonObject::builder()
        .uint("from_height", 1)
        .build()
        .expect("payload");
    let again = SignedEnvelope::build_and_sign(
        &chain_id(),
        NETWORK_ID,
        "block_request",
        "val-001",
        now_ms(),
        30_000,
        fresh_nonce().expect("entropy"),
        payload,
        &keys[1],
    )
    .expect("request builds");
    runner.handle_envelope(again).expect("admitted");
    assert_eq!(
        runner.outbound_attempts(),
        after_first,
        "a second request from the same peer within \
         MIN_MS_BETWEEN_SYNC_ANSWERS emits nothing"
    );
}
