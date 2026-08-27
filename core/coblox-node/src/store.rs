//! Persistent chain storage for finalized blocks.

use std::collections::BTreeMap;
use std::fs::{File, OpenOptions};
use std::io::{BufRead, BufReader, Write};
use std::path::{Path, PathBuf};

use coblox_core::consensus::FinalizedBlock;
use coblox_core::hash::{ChainId, Digest32};
use coblox_core::json::JsonObject;

use crate::error::{NodeError, Result};

/// Persistent block storage manager.
#[derive(Debug)]
pub struct BlockStore {
    path: PathBuf,
    file: File,
    chain_id: ChainId,
    genesis_block_id: Digest32,
    blocks: BTreeMap<u64, FinalizedBlock>,
    latest_height: u64,
    latest_block_id: Digest32,
}

impl BlockStore {
    /// Opens or initializes a block store at `path`.
    ///
    /// # Errors
    ///
    /// Restituisce errore se la directory non e' creabile o se un blocco gia' scritto non e' rileggibile.
    pub fn open(
        path: impl AsRef<Path>,
        chain_id: ChainId,
        genesis_block_id: Digest32,
    ) -> Result<Self> {
        let path = path.as_ref().to_path_buf();
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let mut blocks = BTreeMap::new();
        let mut latest_height = 0;
        let mut latest_block_id = genesis_block_id;

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
                let block = FinalizedBlock::from_json(&obj)?;
                let height = block.header.height;
                let block_id = block.block_id(&chain_id)?;

                if height != latest_height + 1 {
                    return Err(NodeError::Protocol(
                        "block store height discontinuity".into(),
                    ));
                }
                if block.header.previous_block_id != latest_block_id {
                    return Err(NodeError::Protocol(
                        "block store chain discontinuity".into(),
                    ));
                }

                latest_height = height;
                latest_block_id = block_id;
                blocks.insert(height, block);
            }
        }

        let file = OpenOptions::new().create(true).append(true).open(&path)?;

        Ok(Self {
            path,
            file,
            chain_id,
            genesis_block_id,
            blocks,
            latest_height,
            latest_block_id,
        })
    }

    /// Appends a finalized block and flushes/syncs to disk.
    ///
    /// # Errors
    ///
    /// Restituisce errore se il blocco non e' serializzabile o se la scrittura su disco fallisce.
    pub fn append_block(&mut self, block: &FinalizedBlock) -> Result<()> {
        let height = block.header.height;
        let block_id = block.block_id(&self.chain_id)?;

        if height != self.latest_height + 1 {
            return Err(NodeError::Protocol(
                "cannot append block: height not sequential".into(),
            ));
        }
        if block.header.previous_block_id != self.latest_block_id {
            return Err(NodeError::Protocol(
                "cannot append block: previous_block_id mismatch".into(),
            ));
        }

        let json = block.to_json()?;
        let mut line = json.to_jcs();
        line.push(b'\n');

        self.file.write_all(&line)?;
        self.file.flush()?;
        self.file.sync_all()?;

        self.latest_height = height;
        self.latest_block_id = block_id;
        self.blocks.insert(height, block.clone());

        Ok(())
    }

    /// Returns the highest finalized height.
    #[must_use]
    pub const fn latest_height(&self) -> u64 {
        self.latest_height
    }

    /// Returns the highest finalized block ID (or `genesis_block_id` if height 0).
    #[must_use]
    pub const fn latest_block_id(&self) -> Digest32 {
        self.latest_block_id
    }

    /// Returns the finalized block at `height`, if present.
    #[must_use]
    pub fn get_block(&self, height: u64) -> Option<&FinalizedBlock> {
        self.blocks.get(&height)
    }

    /// Returns the latest finalized block, if any.
    #[must_use]
    pub fn latest_block(&self) -> Option<&FinalizedBlock> {
        self.blocks.get(&self.latest_height)
    }

    /// Number of finalized blocks.
    #[must_use]
    pub fn count(&self) -> usize {
        self.blocks.len()
    }

    /// The path to the store file on disk.
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// The genesis block ID this store was initialized with.
    #[must_use]
    pub const fn genesis_block_id(&self) -> &Digest32 {
        &self.genesis_block_id
    }
}
