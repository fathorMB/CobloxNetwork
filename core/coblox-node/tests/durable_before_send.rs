//! `GATE-DURABLE-BEFORE-SEND`, executed.
//!
//! The gate asks that a vote be observed **not leaving the process before it is
//! durable**, by killing the process in the window and checking the restart.
//! [REVIEW-049] RF-003 found the gate marked satisfied with no such test:
//! `wal_safety.rs` reopens the `Wal` in the same process, and the multi-process
//! devnet kills a node at an instant unrelated to the window. There was no
//! window at all — not even a probabilistic one built with a `sleep`.
//!
//! The kill point here is an **instruction**. `NodeRunner::process_actions`
//! calls `std::process::abort()` between `Wal::record_vote` (which has returned
//! from `sync_all`) and the `try_send` that hands the vote to the network,
//! when `COBLOX_NODE_ABORT_AFTER_WAL_SYNC` is set.

use std::process::{Command, Stdio};
use std::time::{Duration, Instant};

use tempfile::TempDir;

use coblox_core::consensus::{VotePhase, proposer_at};
use coblox_node::config::devnet_4_validator_set;
use coblox_node::node::{ABORT_AFTER_WAL_SYNC_ENV, TRACE_VOTES_ENV};
use coblox_node::wal::Wal;

fn node_binary() -> std::path::PathBuf {
    let mut path = std::env::current_exe().expect("current exe");
    path.pop();
    if path.ends_with("deps") {
        path.pop();
    }
    path.push("coblox-node");
    if cfg!(windows) {
        path.set_extension("exe");
    }
    path
}

/// The validator that proposes at `(1, 0)`, and its seed index: alone on a
/// network, it is the only one that reaches a vote at all.
fn proposer_of_the_first_round() -> (String, usize) {
    let (set, _) = devnet_4_validator_set();
    let id = proposer_at(&set, 1, 0)
        .expect("a proposer for (1, 0)")
        .validator_id
        .clone();
    let index = set
        .validators
        .iter()
        .position(|v| v.validator_id == id)
        .expect("the proposer is a member");
    (id, index)
}

#[test]
fn a_vote_is_durable_before_it_is_sent() {
    let dir = TempDir::new().expect("tempdir");
    let (validator_id, seed_index) = proposer_of_the_first_round();
    let log_path = dir.path().join("node.stdout");
    let log = std::fs::File::create(&log_path).expect("create log");

    let mut child = Command::new(node_binary())
        .arg("start")
        .arg("--validator-id")
        .arg(&validator_id)
        .arg("--seed-index")
        .arg(seed_index.to_string())
        .arg("--data-dir")
        .arg(dir.path())
        .arg("--listen-addr")
        .arg("/ip4/127.0.0.1/tcp/0")
        .env(ABORT_AFTER_WAL_SYNC_ENV, "1")
        .env(TRACE_VOTES_ENV, "1")
        .stdout(Stdio::from(log))
        .stderr(Stdio::null())
        .spawn()
        .expect("the node binary must start");

    let deadline = Instant::now() + Duration::from_secs(30);
    let status = loop {
        if let Some(status) = child.try_wait().expect("try_wait") {
            break status;
        }
        assert!(
            Instant::now() < deadline,
            "the node did not reach the abort point within 30 s"
        );
        std::thread::sleep(Duration::from_millis(50));
    };

    assert!(
        !status.success(),
        "the process must die at the abort point, not exit cleanly: {status:?}"
    );

    // The vote is on disk, in a log written by a process that no longer exists.
    let wal = Wal::open(dir.path().join("wal.jsonl")).expect("the WAL must reopen after the abort");
    assert_eq!(
        wal.count(),
        1,
        "exactly the vote the process was killed on must be durable"
    );
    assert!(
        wal.vote_of(1, 0, VotePhase::Prevote).is_some(),
        "the prevote of (1, 0) must be readable after the restart"
    );

    // And it never left: `VOTE_SENT` is printed on the line after the send, and
    // the abort is before it.
    let printed = std::fs::read_to_string(&log_path).expect("read log");
    assert!(
        !printed.contains("VOTE_SENT"),
        "no vote may have been transmitted before the kill:\n{printed}"
    );
}

#[test]
fn without_the_abort_point_the_same_node_does_send_the_vote() {
    // The twin, so that the assertion above cannot be satisfied by a node that
    // simply never votes. Same command line, same data directory shape, one
    // environment variable fewer.
    let dir = TempDir::new().expect("tempdir");
    let (validator_id, seed_index) = proposer_of_the_first_round();
    let log_path = dir.path().join("node.stdout");
    let log = std::fs::File::create(&log_path).expect("create log");

    let mut child = Command::new(node_binary())
        .arg("start")
        .arg("--validator-id")
        .arg(&validator_id)
        .arg("--seed-index")
        .arg(seed_index.to_string())
        .arg("--data-dir")
        .arg(dir.path())
        .arg("--listen-addr")
        .arg("/ip4/127.0.0.1/tcp/0")
        .env(TRACE_VOTES_ENV, "1")
        .stdout(Stdio::from(log))
        .stderr(Stdio::null())
        .spawn()
        .expect("the node binary must start");

    // It cannot finalize alone — three of four are missing — so it is stopped
    // once it has had time to vote.
    let deadline = Instant::now() + Duration::from_secs(30);
    loop {
        let printed = std::fs::read_to_string(&log_path).unwrap_or_default();
        if printed.contains("VOTE_SENT") {
            break;
        }
        assert!(
            Instant::now() < deadline,
            "the node never reported sending a vote:\n{printed}"
        );
        std::thread::sleep(Duration::from_millis(50));
    }
    let _ = child.kill();
    let _ = child.wait();

    let wal = Wal::open(dir.path().join("wal.jsonl")).expect("open wal");
    assert!(wal.vote_of(1, 0, VotePhase::Prevote).is_some());
}
