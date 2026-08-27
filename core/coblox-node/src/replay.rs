//! The anti-replay cache of the wire envelope boundary.
//!
//! `wire.md` §*Signed envelope*: receivers *«cache message IDs and
//! `(sender_node_id, nonce)` until expiry. The cache has protocol caps
//! `replay_cache_entries_global` and `replay_cache_entries_per_peer`; an
//! insertion that would exceed either cap rejects the new envelope as
//! `rate_limited` and MUST NOT evict a still-live entry.»*
//!
//! Both halves are enforced here, and the non-eviction rule is the reason this
//! is not an LRU: evicting a live entry would silently reopen the replay window
//! the cache exists to close, so a full cache rejects instead.
//!
//! **The caps are local devnet constants, not the signed parameters.**
//! `replay_cache_entries_global`, `replay_cache_entries_per_peer` and
//! `max_envelope_validity_ms` are fields of `ConsensusParametersBody`, and no
//! signed consensus-parameters document reaches a devnet node yet — its
//! `consensus_parameters_hash` is a placeholder. The values below carry the
//! protocol names so the substitution is a lookup and not a search. See
//! [REVIEW-049] RF-001.

use std::collections::HashMap;

use coblox_core::hash::Digest32;

/// Local stand-in for the signed `replay_cache_entries_global`.
pub const REPLAY_CACHE_ENTRIES_GLOBAL: usize = 65_536;

/// Local stand-in for the signed `replay_cache_entries_per_peer`.
pub const REPLAY_CACHE_ENTRIES_PER_PEER: usize = 8_192;

/// Why an envelope was not admitted by the cache.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReplayVerdict {
    /// First time this `message_id` and this `(sender_node_id, nonce)` are seen.
    Fresh,
    /// The `message_id` is already cached and still live.
    DuplicateMessageId,
    /// The `(sender_node_id, nonce)` pair is already cached and still live.
    DuplicateNonce,
    /// Admitting it would exceed a cap, and no live entry may be evicted.
    RateLimited,
}

#[derive(Debug)]
struct Entry {
    sender_node_id: String,
    expires_at_ms: u64,
}

/// Bounded replay cache over `message_id` and `(sender_node_id, nonce)`.
#[derive(Debug)]
pub struct ReplayCache {
    global_cap: usize,
    per_peer_cap: usize,
    by_message_id: HashMap<Digest32, Entry>,
    by_sender_nonce: HashMap<(String, [u8; 16]), u64>,
    per_peer: HashMap<String, usize>,
}

impl Default for ReplayCache {
    fn default() -> Self {
        Self::new(REPLAY_CACHE_ENTRIES_GLOBAL, REPLAY_CACHE_ENTRIES_PER_PEER)
    }
}

impl ReplayCache {
    /// A cache with explicit caps.
    #[must_use]
    pub fn new(global_cap: usize, per_peer_cap: usize) -> Self {
        Self {
            global_cap,
            per_peer_cap,
            by_message_id: HashMap::new(),
            by_sender_nonce: HashMap::new(),
            per_peer: HashMap::new(),
        }
    }

    /// Drops every entry whose envelope has expired at `now_ms`.
    ///
    /// Expiry is the only way an entry leaves the cache: see the module note on
    /// the non-eviction rule.
    pub fn expire(&mut self, now_ms: u64) {
        let per_peer = &mut self.per_peer;
        self.by_message_id.retain(|_, entry| {
            if entry.expires_at_ms >= now_ms {
                return true;
            }
            if let Some(count) = per_peer.get_mut(&entry.sender_node_id) {
                *count = count.saturating_sub(1);
            }
            false
        });
        self.per_peer.retain(|_, count| *count > 0);
        self.by_sender_nonce.retain(|_, expires| *expires >= now_ms);
    }

    /// Admits `message_id` and `(sender_node_id, nonce)`, or says why not.
    ///
    /// Call [`Self::expire`] first: this method never drops a live entry, and a
    /// cache that is never expired fills up and starts rejecting.
    pub fn admit(
        &mut self,
        message_id: Digest32,
        sender_node_id: &str,
        nonce: [u8; 16],
        expires_at_ms: u64,
    ) -> ReplayVerdict {
        if self.by_message_id.contains_key(&message_id) {
            return ReplayVerdict::DuplicateMessageId;
        }
        let pair_key = (sender_node_id.to_owned(), nonce);
        if self.by_sender_nonce.contains_key(&pair_key) {
            return ReplayVerdict::DuplicateNonce;
        }
        if self.by_message_id.len() >= self.global_cap {
            return ReplayVerdict::RateLimited;
        }
        if self
            .per_peer
            .get(sender_node_id)
            .is_some_and(|count| *count >= self.per_peer_cap)
        {
            return ReplayVerdict::RateLimited;
        }

        self.by_message_id.insert(
            message_id,
            Entry {
                sender_node_id: sender_node_id.to_owned(),
                expires_at_ms,
            },
        );
        self.by_sender_nonce.insert(pair_key, expires_at_ms);
        *self.per_peer.entry(sender_node_id.to_owned()).or_insert(0) += 1;
        ReplayVerdict::Fresh
    }

    /// Number of cached message IDs.
    #[must_use]
    pub fn len(&self) -> usize {
        self.by_message_id.len()
    }

    /// Whether the cache holds nothing.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.by_message_id.is_empty()
    }
}
