//! Durable Write-Ahead Log (WAL) for consensus votes.
//!
//! Enforces safety invariant: a validator persists every signed vote to disk
//! with `fsync` (`sync_all`) before transmission over the network, and on restart
//! refuses to emit any conflicting vote for a `(height, round, phase)` it previously voted.

use std::collections::BTreeMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

use coblox_core::consensus::VotePhase;
use coblox_core::hash::Digest32;
use coblox_core::json::JsonObject;

use crate::error::{NodeError, Result};

fn phase_to_u8(phase: VotePhase) -> u8 {
    match phase {
        VotePhase::Prevote => 0,
        VotePhase::Precommit => 1,
    }
}

/// Durable WAL manager.
#[derive(Debug)]
pub struct Wal {
    path: PathBuf,
    file: File,
    /// Maps `(height, round, phase_u8)` -> recorded `block_id`.
    recorded_votes: BTreeMap<(u64, u64, u8), Digest32>,
}

impl Wal {
    /// Opens or creates a WAL log at `path`.
    ///
    /// # Errors
    ///
    /// Restituisce errore se il file non e' apribile o se una riga gia' scritta non e' interpretabile: un log illeggibile non viene ignorato, perche' ignorarlo significherebbe ripartire senza sapere cosa si e' gia' firmato.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let mut recorded_votes = BTreeMap::new();

        if path.exists() {
            let read_file = File::open(&path)?;
            let reader = BufReader::new(read_file);
            for line in reader.lines() {
                let line = line?;
                let line = line.trim();
                if line.is_empty() {
                    continue;
                }
                let obj = JsonObject::parse_canonical(line.as_bytes())?;
                let height = obj.uint("height")?;
                let round = obj.uint("round")?;
                let phase_str = obj.string("phase")?;
                let phase_u8 = match phase_str {
                    "prevote" => 0u8,
                    "precommit" => 1u8,
                    _ => return Err(NodeError::Protocol("unknown WAL vote phase".into())),
                };
                let block_id = obj.digest("block_id")?;
                if let Some(existing) = recorded_votes.insert((height, round, phase_u8), block_id)
                    && existing != block_id
                {
                    return Err(NodeError::Protocol(
                        "WAL contains conflicting votes at same height/round/phase".into(),
                    ));
                }
            }
        }

        let file = OpenOptions::new().create(true).append(true).open(&path)?;

        Ok(Self {
            path,
            file,
            recorded_votes,
        })
    }

    /// Checks if a vote for `(height, round, phase)` can be emitted for `block_id`.
    ///
    /// Returns `true` if not voted yet, or if already voted for exact same `block_id`.
    /// Returns `false` if already voted for a *different* `block_id` (preventing equivocation).
    #[must_use]
    pub fn can_vote(&self, height: u64, round: u64, phase: VotePhase, block_id: &Digest32) -> bool {
        match self
            .recorded_votes
            .get(&(height, round, phase_to_u8(phase)))
        {
            Some(existing) => existing == block_id,
            None => true,
        }
    }

    /// Returns the recorded `block_id` for `(height, round, phase)` if any.
    #[must_use]
    pub fn vote_of(&self, height: u64, round: u64, phase: VotePhase) -> Option<&Digest32> {
        self.recorded_votes
            .get(&(height, round, phase_to_u8(phase)))
    }

    /// Appends a signed vote to WAL and durably synchronizes with disk (`fsync`).
    ///
    /// # Errors
    ///
    /// Restituisce errore se la scrittura o il `fsync` falliscono. **L'errore non e' recuperabile dal chiamante trasmettendo lo stesso**: un voto che non e' durevole non deve lasciare il processo.
    pub fn record_vote(
        &mut self,
        height: u64,
        round: u64,
        phase: VotePhase,
        block_id: &Digest32,
        validator_id: &str,
        signature: &[u8; 64],
    ) -> Result<()> {
        let phase_u8 = phase_to_u8(phase);
        if let Some(existing) = self.recorded_votes.get(&(height, round, phase_u8)) {
            if existing != block_id {
                return Err(NodeError::Protocol(
                    "equivocation forbidden: contradictory vote already recorded in WAL".into(),
                ));
            }
            return Ok(()); // idempotent write
        }

        let phase_str = match phase {
            VotePhase::Prevote => "prevote",
            VotePhase::Precommit => "precommit",
        };

        let obj = JsonObject::builder()
            .digest("block_id", block_id)
            .uint("height", height)
            .str("phase", phase_str)
            .bytes("signature", signature)
            .str("validator_id", validator_id)
            .uint("round", round)
            .build()?;

        let mut line = obj.to_jcs();
        line.push(b'\n');

        self.file.write_all(&line)?;
        self.file.flush()?;
        self.file.sync_all()?;

        self.recorded_votes
            .insert((height, round, phase_u8), *block_id);
        Ok(())
    }

    /// Number of distinct recorded votes.
    #[must_use]
    pub fn count(&self) -> usize {
        self.recorded_votes.len()
    }

    /// The path to the WAL file on disk.
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }
}
