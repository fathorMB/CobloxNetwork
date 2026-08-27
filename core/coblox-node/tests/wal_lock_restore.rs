//! What a restarted validator recovers from its own write-ahead log.
//!
//! [REVIEW-049] RF-002 and RF-008.

use std::fs::OpenOptions;

use tempfile::TempDir;

use coblox_core::consensus::VotePhase;
use coblox_core::hash::{ChainId, Digest32};
use coblox_node::config::{NodeConfig, devnet_4_validator_set, devnet_timeouts};
use coblox_node::node::NodeRunner;
use coblox_node::wal::Wal;

fn sig() -> [u8; 64] {
    [0x77; 64]
}

#[test]
fn the_lock_is_the_highest_round_precommit_of_the_height() {
    let dir = TempDir::new().expect("tempdir");
    let path = dir.path().join("wal.jsonl");
    let mut wal = Wal::open(&path).expect("open");

    let block_b = Digest32::repeated(0xB0);
    let block_c = Digest32::repeated(0xC0);

    assert_eq!(wal.locked_at_height(5), None, "no precommit is no lock");

    // A prevote is not a lock: Algorithm 1 lines 38-40 lock and precommit
    // together, and only the precommit is the act.
    wal.record_vote(5, 0, VotePhase::Prevote, &block_c, "val-000", &sig())
        .expect("prevote");
    assert_eq!(wal.locked_at_height(5), None, "a prevote is not a lock");

    wal.record_vote(5, 0, VotePhase::Precommit, &block_b, "val-000", &sig())
        .expect("precommit r0");
    assert_eq!(wal.locked_at_height(5), Some((0, block_b)));

    wal.record_vote(5, 1, VotePhase::Precommit, &block_c, "val-000", &sig())
        .expect("precommit r1");
    assert_eq!(
        wal.locked_at_height(5),
        Some((1, block_c)),
        "the highest round wins"
    );

    // A precommit at another height is another height's lock.
    wal.record_vote(6, 0, VotePhase::Precommit, &block_b, "val-000", &sig())
        .expect("precommit h6");
    assert_eq!(wal.locked_at_height(5), Some((1, block_c)));
    assert_eq!(wal.locked_at_height(6), Some((0, block_b)));
    assert_eq!(wal.locked_at_height(7), None);
}

#[test]
fn the_lock_survives_reopening_the_log() {
    let dir = TempDir::new().expect("tempdir");
    let path = dir.path().join("wal.jsonl");
    let block_b = Digest32::repeated(0xB0);
    {
        let mut wal = Wal::open(&path).expect("open");
        wal.record_vote(5, 1, VotePhase::Precommit, &block_b, "val-000", &sig())
            .expect("precommit");
    }
    let reopened = Wal::open(&path).expect("reopen");
    assert_eq!(reopened.locked_at_height(5), Some((1, block_b)));
}

#[tokio::test]
async fn a_node_restarted_at_a_height_it_precommitted_comes_back_locked() {
    let dir = TempDir::new().expect("tempdir");
    let block_b = Digest32::repeated(0xB0);
    {
        let mut wal = Wal::open(dir.path().join("wal.jsonl")).expect("open");
        wal.record_vote(1, 1, VotePhase::Precommit, &block_b, "val-000", &sig())
            .expect("precommit at height 1 round 1");
    }

    let (set, keys) = devnet_4_validator_set();
    let config = NodeConfig {
        validator_id: "val-000".to_owned(),
        node_id: "val-000".to_owned(),
        signing_key: keys[0].clone(),
        network_id: "coblox-devnet-0".to_owned(),
        chain_id: ChainId::from_digest(Digest32::repeated(0x7a)),
        genesis_block_id: Digest32::repeated(0x01),
        listen_addr: "/ip4/127.0.0.1/tcp/0".to_owned(),
        seed_peers: Vec::new(),
        data_dir: dir.path().to_path_buf(),
        validator_set: set,
        timeouts: devnet_timeouts(),
        target_height: None,
    };
    let (runner, _network) = NodeRunner::new(config).expect("the runner must start");

    assert_eq!(
        runner.locked(),
        Some((1, block_b)),
        "a node that precommitted at (1, 1) and died must come back locked on it"
    );
}

#[test]
fn an_incomplete_trailing_record_is_discarded_and_the_file_truncated() {
    // [REVIEW-049] RF-008, inverted: the executed proof of concept truncated the
    // last record by 40 bytes and observed `Wal::open` failing on every
    // subsequent start, which turns a power loss into a validator lost for good.
    let dir = TempDir::new().expect("tempdir");
    let path = dir.path().join("wal.jsonl");
    let block_b = Digest32::repeated(0xB0);
    {
        let mut wal = Wal::open(&path).expect("open");
        wal.record_vote(1, 0, VotePhase::Precommit, &block_b, "val-000", &sig())
            .expect("one complete record");
    }
    let complete_len = std::fs::metadata(&path).expect("metadata").len();

    // Append the first 40 bytes of a second record, with no newline: exactly
    // what a `kill -9` between `write_all` and `sync_all` can leave behind.
    let partial = std::fs::read(&path).expect("read")[..40].to_vec();
    {
        use std::io::Write;
        let mut file = OpenOptions::new().append(true).open(&path).expect("append");
        file.write_all(&partial).expect("write partial");
    }
    assert!(std::fs::metadata(&path).expect("metadata").len() > complete_len);

    let wal = Wal::open(&path).expect("an interrupted append must not brick the validator");
    assert_eq!(wal.count(), 1, "the one complete record survives");
    assert_eq!(wal.vote_of(1, 0, VotePhase::Precommit), Some(&block_b));
    assert_eq!(
        std::fs::metadata(&path).expect("metadata").len(),
        complete_len,
        "the file is truncated back to the last complete record"
    );
}

#[test]
fn a_malformed_record_that_is_not_the_tail_is_still_fatal() {
    // The other half of RF-008: only an unterminated tail has a benign
    // explanation. A corrupt line in the middle must still stop the node.
    let dir = TempDir::new().expect("tempdir");
    let path = dir.path().join("wal.jsonl");
    {
        let mut wal = Wal::open(&path).expect("open");
        wal.record_vote(
            1,
            0,
            VotePhase::Precommit,
            &Digest32::repeated(0xB0),
            "val-000",
            &sig(),
        )
        .expect("record");
    }
    {
        use std::io::Write;
        let mut file = OpenOptions::new().append(true).open(&path).expect("append");
        file.write_all(b"{not canonical json}\n").expect("write");
        file.write_all(b"{\"also\":\"garbage\"}\n").expect("write");
    }
    assert!(
        Wal::open(&path).is_err(),
        "an unreadable line that is not an interrupted tail must fail the start"
    );
}
