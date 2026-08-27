//! Bounded message buffer for future consensus heights.
//!
//! Holds consensus messages (proposals, prevotes, precommits) that arrive
//! ahead of the local node's current height, re-injecting them when the node
//! advances to that height.

use std::collections::BTreeMap;

use crate::envelope::SignedEnvelope;

/// Bounded buffer for future consensus heights.
#[derive(Debug)]
pub struct FutureHeightBuffer {
    max_lookahead: u64,
    max_messages_per_height: usize,
    buffer: BTreeMap<u64, Vec<SignedEnvelope>>,
}

impl FutureHeightBuffer {
    /// Creates a new buffer with lookahead and per-height limits.
    #[must_use]
    pub fn new(max_lookahead: u64, max_messages_per_height: usize) -> Self {
        Self {
            max_lookahead,
            max_messages_per_height,
            buffer: BTreeMap::new(),
        }
    }

    /// Default buffer configuration (lookahead of 20 heights, 500 messages per height).
    #[must_use]
    pub fn with_defaults() -> Self {
        Self::new(20, 500)
    }

    /// Buffers `envelope` for `height` if within lookahead window.
    pub fn insert(&mut self, current_height: u64, message_height: u64, envelope: SignedEnvelope) {
        if message_height <= current_height {
            return;
        }
        if message_height > current_height + self.max_lookahead {
            return; // drop beyond lookahead window
        }

        let entry = self.buffer.entry(message_height).or_default();
        if entry.len() < self.max_messages_per_height {
            entry.push(envelope);
        }
    }

    /// Drains and returns all buffered envelopes for `height`.
    pub fn drain_height(&mut self, height: u64) -> Vec<SignedEnvelope> {
        self.buffer.remove(&height).unwrap_or_default()
    }

    /// Prunes any stale messages buffered for heights `< current_height`.
    pub fn prune_before(&mut self, current_height: u64) {
        self.buffer.retain(|&h, _| h >= current_height);
    }
}
