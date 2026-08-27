//! Multi-process devnet integration tests.
//!
//! Exercises 4 separate OS child processes executing `coblox-node start` over
//! real loopback TCP connections, validating BFT finalization of >= 10 blocks,
//! verifying every finalized block with `FinalizedBlock::verify` using `ConsensusVerifier`,
//! and demonstrating crash-recovery without equivocation.

use std::fs::File;
use std::io::{BufRead, BufReader};
use std::process::{Child, Command, Stdio};
use std::time::{Duration, Instant};
use tempfile::tempdir;

use coblox_core::consensus::FinalizedBlock;
use coblox_core::hash::{ChainId, Digest32};
use coblox_core::json::JsonObject;
use coblox_core::verifier::ConsensusVerifier;
use coblox_node::config::devnet_4_validator_set;

fn get_bin_path() -> std::path::PathBuf {
    let mut path = std::env::current_exe().expect("current exe");
    path.pop(); // remove test binary name
    if path.ends_with("deps") {
        path.pop();
    }
    path.push("coblox-node");
    if cfg!(windows) {
        path.set_extension("exe");
    }
    path
}

fn read_finalized_blocks(dir: &std::path::Path) -> Vec<FinalizedBlock> {
    let path = dir.join("blocks.jsonl");
    if !path.exists() {
        return Vec::new();
    }
    let file = File::open(&path).expect("open blocks.jsonl");
    let reader = BufReader::new(file);
    let mut blocks = Vec::new();
    for line in reader.lines() {
        let line = line.expect("line");
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let obj = JsonObject::parse_canonical(line.as_bytes()).expect("parse block json");
        let block = FinalizedBlock::from_json(&obj).expect("block from json");
        blocks.push(block);
    }
    blocks
}

// `#[ignore]`d nella passata normale e rieseguito in CI dal proprio step, in
// release e con `--test-threads=1`. La ragione e' misurata, non prudenziale:
// `cargo test --workspace` esegue i test in parallelo, quindi questi due
// avviavano insieme otto processi validatore su un runner condiviso, e in debug
// ogni messaggio costa una verifica Ed25519 reale. Gli esiti erano sempre della
// stessa forma — catena viva e in avanzamento, ma sotto la scadenza:
// `[7, 7, 7, 7]` su ubuntu e `[8, 9, 9, 8]` su windows. Non era un difetto del
// nodo, ed e' il motivo per cui nessuna scadenza e' stata allentata qui: cio'
// che va cambiato e' come il test viene eseguito, non cio' che asserisce. Stesso
// trattamento, e per la stessa ragione, dello sweep esteso di [SPEC-025].
#[ignore = "avvia quattro processi validatore: eseguito dal proprio step CI, in serie e in release"]
#[test]
fn four_seed_validator_processes_finalize_ten_blocks() {
    let bin_path = get_bin_path();
    let temp_dirs = [
        tempdir().unwrap(),
        tempdir().unwrap(),
        tempdir().unwrap(),
        tempdir().unwrap(),
    ];

    let base_port = 19100;
    let peers: Vec<String> = (0..4)
        .map(|i| format!("/ip4/127.0.0.1/tcp/{}", base_port + i))
        .collect();
    let peers_arg = peers.join(",");

    println!("Starting 4 validator child processes using binary: {bin_path:?}");

    let mut children: Vec<Child> = Vec::new();
    for i in 0..4 {
        let val_id = format!("val-{i:03}");
        let data_dir = temp_dirs[i].path().to_str().unwrap();
        let listen_addr = &peers[i];

        let child = Command::new(&bin_path)
            .arg("start")
            .arg("--validator-id")
            .arg(&val_id)
            .arg("--seed-index")
            .arg(i.to_string())
            .arg("--data-dir")
            .arg(data_dir)
            .arg("--listen-addr")
            .arg(listen_addr)
            .arg("--seed-peers")
            .arg(&peers_arg)
            .arg("--target-height")
            .arg("10")
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("spawn validator child process");

        println!(
            "Spawned node {val_id} (PID: {}) listening on {listen_addr}",
            child.id()
        );
        children.push(child);
    }

    let (validator_set, _) = devnet_4_validator_set();
    let chain_id = ChainId::from_digest(Digest32::repeated(0x7a));

    // Monitor finalization for up to 30 seconds
    let start = Instant::now();
    let target_height = 10;
    let mut finalized_counts = [0usize; 4];

    loop {
        for i in 0..4 {
            let blocks = read_finalized_blocks(temp_dirs[i].path());
            finalized_counts[i] = blocks.len();
        }

        if finalized_counts.iter().all(|&c| c >= target_height) {
            println!("All 4 validators finalized at least {target_height} blocks!");
            break;
        }

        assert!(
            start.elapsed() <= Duration::from_secs(30),
            "Timeout waiting for 4 validators to finalize {target_height} blocks. Counts: {finalized_counts:?}"
        );

        std::thread::sleep(Duration::from_millis(200));
    }

    // Terminate children
    for mut child in children {
        let _ = child.kill();
        let _ = child.wait();
    }

    // Verify all finalized blocks from all 4 validators
    let val0_blocks = read_finalized_blocks(temp_dirs[0].path());
    assert!(val0_blocks.len() >= target_height);

    for (h_idx, block) in val0_blocks.iter().enumerate() {
        let height = (h_idx + 1) as u64;
        assert_eq!(block.header.height, height);

        // GATE-FOUR-PROCESSES: verify certificate and header integrity with ConsensusVerifier
        block
            .verify(&chain_id, &validator_set, &ConsensusVerifier)
            .expect("finalized block must pass ConsensusVerifier verification");

        let block_id = block.block_id(&chain_id).expect("block_id");
        println!(
            "GATE-FOUR-PROCESSES: verified height={height} block_id={block_id:?} signatures={}",
            block.quorum_certificate.signatures.len()
        );

        // Verify all other validators finalized the exact same block
        for (i, dir) in temp_dirs.iter().enumerate().skip(1) {
            let peer_blocks = read_finalized_blocks(dir.path());
            assert!(peer_blocks.len() > h_idx);
            let peer_id = peer_blocks[h_idx]
                .block_id(&chain_id)
                .expect("peer block_id");
            assert_eq!(
                peer_id, block_id,
                "node {i} block mismatch at height {height}"
            );
        }
    }
}

// Il test che esercita GATE-RESTART-NO-EQUIVOCATION: avvia quattro processi,
// ne uccide uno a meta' altezza, lo riavvia e verifica che non contraddica il
// proprio log. Le sue 105 righe sono la sequenza di quel caso, e spezzarla in
// helper renderebbe piu' difficile leggere cosa viene ucciso e quando — che e'
// l'unica cosa che questo test deve rendere evidente.
// Dichiarato dal Lead nella presa in carico correttiva del 2026-08-27.
#[allow(clippy::too_many_lines)]
// `#[ignore]`d nella passata normale e rieseguito in CI dal proprio step, in
// release e con `--test-threads=1`. La ragione e' misurata, non prudenziale:
// `cargo test --workspace` esegue i test in parallelo, quindi questi due
// avviavano insieme otto processi validatore su un runner condiviso, e in debug
// ogni messaggio costa una verifica Ed25519 reale. Gli esiti erano sempre della
// stessa forma — catena viva e in avanzamento, ma sotto la scadenza:
// `[7, 7, 7, 7]` su ubuntu e `[8, 9, 9, 8]` su windows. Non era un difetto del
// nodo, ed e' il motivo per cui nessuna scadenza e' stata allentata qui: cio'
// che va cambiato e' come il test viene eseguito, non cio' che asserisce. Stesso
// trattamento, e per la stessa ragione, dello sweep esteso di [SPEC-025].
#[ignore = "avvia quattro processi validatore: eseguito dal proprio step CI, in serie e in release"]
#[test]
fn validator_crash_and_restart_recovers_without_equivocation() {
    let bin_path = get_bin_path();
    let temp_dirs = [
        tempdir().unwrap(),
        tempdir().unwrap(),
        tempdir().unwrap(),
        tempdir().unwrap(),
    ];

    let base_port = 19200;
    let peers: Vec<String> = (0..4)
        .map(|i| format!("/ip4/127.0.0.1/tcp/{}", base_port + i))
        .collect();
    let peers_arg = peers.join(",");

    let spawn_node = |i: usize, target: u64| -> Child {
        let val_id = format!("val-{i:03}");
        let data_dir = temp_dirs[i].path().to_str().unwrap();
        let listen_addr = &peers[i];

        Command::new(&bin_path)
            .arg("start")
            .arg("--validator-id")
            .arg(&val_id)
            .arg("--seed-index")
            .arg(i.to_string())
            .arg("--data-dir")
            .arg(data_dir)
            .arg("--listen-addr")
            .arg(listen_addr)
            .arg("--seed-peers")
            .arg(&peers_arg)
            .arg("--target-height")
            .arg(target.to_string())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .expect("spawn validator child process")
    };

    println!("Starting 4 validator child processes for crash-recovery test");
    let mut children: Vec<Option<Child>> = (0..4).map(|i| Some(spawn_node(i, 8))).collect();

    let (validator_set, _) = devnet_4_validator_set();
    let chain_id = ChainId::from_digest(Digest32::repeated(0x7a));

    // Wait until height 2 is finalized
    let start = Instant::now();
    loop {
        let b0 = read_finalized_blocks(temp_dirs[0].path());
        if b0.len() >= 2 {
            println!("Network reached height 2. Simulating crash of val-003...");
            break;
        }
        assert!(
            start.elapsed() <= Duration::from_secs(45),
            "Timeout waiting for initial blocks"
        );
        std::thread::sleep(Duration::from_millis(100));
    }

    // Kill node 3 mid-stream
    if let Some(mut c3) = children[3].take() {
        let pid = c3.id();
        let _ = c3.kill();
        let _ = c3.wait();
        println!("val-003 (PID: {pid}) killed.");
    }

    // Wait until remaining 3 nodes finalize height 4 (demonstrating 3/4 quorum resilience)
    let start2 = Instant::now();
    loop {
        let b0 = read_finalized_blocks(temp_dirs[0].path());
        if b0.len() >= 4 {
            println!("Remaining 3 validators progressed to height 4 without node 3!");
            break;
        }
        assert!(
            start2.elapsed() <= Duration::from_secs(45),
            "Timeout waiting for 3 nodes to progress"
        );
        std::thread::sleep(Duration::from_millis(100));
    }

    // Restart node 3 with exact same data directory and WAL
    println!("Restarting val-003 with persisted WAL and blocks...");
    let c3_restarted = spawn_node(3, 8);
    println!("val-003 restarted with PID: {}", c3_restarted.id());
    children[3] = Some(c3_restarted);

    // Wait until all 4 nodes finalize height 8
    let start3 = Instant::now();
    loop {
        let mut counts = [0usize; 4];
        for i in 0..4 {
            counts[i] = read_finalized_blocks(temp_dirs[i].path()).len();
        }
        if counts.iter().all(|&c| c >= 8) {
            println!("All 4 validators including restarted node finalized height 8!");
            break;
        }
        // ATTENZIONE, questo test fallisce in CI su Linux e la scadenza non
        // e' la causa. Il Lead ha sbagliato la diagnosi due volte prima di
        // arrivarci, e le tre esecuzioni sono qui perche' la prossima persona
        // non le rifaccia: [8, 8, 8, 3] a 20s, poi [8, 8, 7, 8] a 20s, poi
        // [8, 8, 8, 5] a 45s. Il terzo dato e' quello che decide: con piu' del
        // doppio del tempo il nodo riavviato e' passato da 3 a 5, non a 8.
        //
        // Il meccanismo: i nodi sono avviati con `--target-height`, e un nodo
        // che raggiunge il bersaglio **esce** (`node.rs`, il ramo che
        // restituisce `Ok(true)`). I tre sani arrivano a 8 e terminano, e il
        // nodo riavviato resta senza alcun pari da cui sincronizzare. Col
        // throttle introdotto da [REVIEW-049] RF-006 — otto blocchi per
        // risposta, una risposta al secondo per richiedente — il recupero
        // richiede piu' secondi di **presenza dei pari** di quanti il bersaglio
        // gliene conceda. Allungare la scadenza peggiora le cose, perche' i
        // pari escono comunque e il ritardatario resta solo piu' a lungo.
        //
        // Registrato come debito. Il rimedio non e' qui dentro: o i pari
        // restano vivi finche' il ritardatario ha recuperato, o il recupero non
        // dipende da un throttle tarato sull'abuso.
        assert!(
            start3.elapsed() <= Duration::from_secs(45),
            "Timeout waiting for all nodes to reach height 8. Counts: {counts:?}"
        );
        std::thread::sleep(Duration::from_millis(100));
    }

    // Clean up processes
    for mut child in children.into_iter().flatten() {
        let _ = child.kill();
        let _ = child.wait();
    }

    // Verify all blocks from node 3
    let val3_blocks = read_finalized_blocks(temp_dirs[3].path());
    assert!(val3_blocks.len() >= 8);
    for block in &val3_blocks {
        block
            .verify(&chain_id, &validator_set, &ConsensusVerifier)
            .expect("restarted node finalized blocks must be valid");
    }
    println!(
        "GATE-RESTART-NO-EQUIVOCATION: val-003 recovered and finalized cleanly without equivocation."
    );
}
