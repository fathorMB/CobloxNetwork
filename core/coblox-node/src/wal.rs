//! Durable Write-Ahead Log (WAL) for consensus votes.
//!
//! Enforces safety invariant: a validator persists every signed vote to disk
//! with `fsync` (`sync_all`) before transmission over the network, and on restart
//! refuses to emit any conflicting vote for a `(height, round, phase)` it previously voted.

use std::collections::BTreeMap;
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};

use coblox_core::consensus::VotePhase;
use coblox_core::hash::Digest32;
use coblox_core::json::JsonObject;

use crate::error::{NodeError, Result};

/// Drops ASCII whitespace from both ends of `bytes`.
fn trim_ascii(bytes: &[u8]) -> &[u8] {
    let start = bytes
        .iter()
        .position(|b| !b.is_ascii_whitespace())
        .unwrap_or(bytes.len());
    let end = bytes
        .iter()
        .rposition(|b| !b.is_ascii_whitespace())
        .map_or(start, |i| i + 1);
    &bytes[start..end]
}

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
    /// Restituisce errore se il file non e' apribile o se una riga **completa**
    /// gia' scritta non e' interpretabile: un log illeggibile non viene
    /// ignorato, perche' ignorarlo significherebbe ripartire senza sapere cosa
    /// si e' gia' firmato.
    ///
    /// Fa eccezione, e una sola, l'**ultima** riga quando non termina con `\n`:
    /// e' una scrittura interrotta a meta' — la finestra fra `write_all` e
    /// `sync_all` che questo log esiste per proteggere — e non un record
    /// corrotto. Un voto la cui riga non e' completa non e' mai stato
    /// trasmesso, perche' `record_vote` propaga l'errore prima del `try_send`:
    /// scartarlo e troncare il file all'ultimo record completo non perde nulla
    /// che sia uscito dal processo. Una riga malformata che **non** e' in coda
    /// resta un errore fatale, perche' quella non ha spiegazione benigna.
    /// Vedi [REVIEW-049] RF-008.
    pub fn open(path: impl AsRef<Path>) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let mut recorded_votes = BTreeMap::new();

        if path.exists() {
            let raw = std::fs::read(&path)?;
            // A trailing byte sequence with no final `\n` is an interrupted
            // append and nothing else: every completed record ends with one.
            let (complete, truncated_tail_len) = match raw.iter().rposition(|&b| b == b'\n') {
                Some(last_newline) => (&raw[..=last_newline], raw.len() - last_newline - 1),
                None => (&raw[..0], raw.len()),
            };

            for line in complete.split(|&b| b == b'\n') {
                let line = trim_ascii(line);
                if line.is_empty() {
                    continue;
                }
                let obj = JsonObject::parse_canonical(line)?;
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

            if truncated_tail_len > 0 {
                let keep = u64::try_from(complete.len())
                    .map_err(|_| NodeError::Protocol("WAL length does not fit in u64".into()))?;
                OpenOptions::new().write(true).open(&path)?.set_len(keep)?;
                eprintln!(
                    "WAL: discarded {truncated_tail_len} trailing byte(s) of an incomplete record in {} \
                     and truncated to the last complete one; a record without its newline never left this process",
                    path.display()
                );
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

    /// The lock this node must resume `height` with, as `(round, block_id)`.
    ///
    /// A precommit is exactly the act that sets `lockedValue_p`/`lockedRound_p`
    /// in Algorithm 1 lines 38-39: the engine locks and precommits in the same
    /// step and never one without the other, so the highest round at `height`
    /// this node precommitted in **is** the round it was locked at when it died.
    /// The lock is therefore already in this log; nothing new has to be written
    /// to recover it. See [REVIEW-049] RF-002.
    ///
    /// Returns `None` for a height this node never precommitted in, which is the
    /// unlocked case and Algorithm 1's `lockedRound_p = -1`.
    #[must_use]
    pub fn locked_at_height(&self, height: u64) -> Option<(u64, Digest32)> {
        let precommit = phase_to_u8(VotePhase::Precommit);
        self.recorded_votes
            .range((height, 0, 0)..=(height, u64::MAX, u8::MAX))
            .rfind(|((_, _, phase), _)| *phase == precommit)
            .map(|((_, round, _), block_id)| (*round, *block_id))
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
