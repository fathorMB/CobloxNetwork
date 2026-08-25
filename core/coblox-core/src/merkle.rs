//! The protocol's Merkle constructions.
//!
//! Coblox v0 defines six trees. Five of them share one shape — sorted leaves,
//! padded to a power of two, with a distinct empty-list root — and differ only
//! in their four tag bytes and in how a leaf preimage is built. That shared
//! shape is [`TaggedTree`]; the leaf preimages are the free functions below.
//! The sixth, the account state tree, is a depth-256 sparse tree and is
//! [`SparseAccountTree`].
//!
//! Tag bytes are held in [`TaggedTree`] constants rather than passed at call
//! sites, for the same reason [`crate::hash::Domain`] holds domain strings: a
//! tag mix-up yields a plausible, wrong root.

use std::sync::OnceLock;

use crate::error::{MerkleError, Result};
use crate::hash::{AccountKey, Digest32, tagged_hash};

/// A binary Merkle tree over sorted leaves, padded to a power of two.
///
/// The four tags are, in order: the leaf tag (used by the leaf constructors,
/// recorded here for documentation), the internal-node tag, the padding-leaf
/// tag, and the tag whose bare hash is the root of an empty list.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct TaggedTree {
    leaf: u8,
    node: u8,
    padding: u8,
    empty: u8,
}

impl TaggedTree {
    /// The block transaction tree (`ledger.md#hashing-primitives`).
    pub const TRANSACTIONS: Self = Self::new(0x00, 0x01, 0x02, 0x03);
    /// The publisher-reward active subscription tree.
    pub const SUBSCRIPTIONS: Self = Self::new(0x20, 0x21, 0x22, 0x23);
    /// The existence-income eligible set tree.
    pub const ELIGIBLE_NODES: Self = Self::new(0x24, 0x25, 0x26, 0x27);
    /// The weak-subjectivity checkpoint revocation tree.
    pub const REVOCATIONS: Self = Self::new(0x30, 0x31, 0x32, 0x33);
    /// The validator-election committed candidate tree.
    pub const CANDIDATES: Self = Self::new(0x40, 0x41, 0x42, 0x43);

    /// The maximum number of transactions in a block.
    pub const MAX_TRANSACTIONS: usize = 16_384;

    const fn new(leaf: u8, node: u8, padding: u8, empty: u8) -> Self {
        Self {
            leaf,
            node,
            padding,
            empty,
        }
    }

    /// The leaf tag this tree's leaf preimages start with.
    #[must_use]
    pub const fn leaf_tag(self) -> u8 {
        self.leaf
    }

    /// The padding leaf, `H(empty_leaf_tag)`.
    #[must_use]
    pub fn empty_leaf(self) -> Digest32 {
        tagged_hash(self.padding, &[])
    }

    /// The root of an empty list, `H(empty_root_tag)`.
    ///
    /// This is deliberately a different tag from the padding leaf, so an empty
    /// tree cannot be confused with a one-leaf tree of padding.
    #[must_use]
    pub fn empty_root(self) -> Digest32 {
        tagged_hash(self.empty, &[])
    }

    /// Combines two children: `H(node_tag || left || right)`.
    #[must_use]
    pub fn combine(self, left: &Digest32, right: &Digest32) -> Digest32 {
        tagged_hash(self.node, &[left.as_bytes(), right.as_bytes()])
    }

    /// The root over already-computed leaf digests, in the order given.
    ///
    /// Callers pass leaves in the protocol's sort order; see
    /// [`sorted_unique_leaves`], which is how every caller in this crate
    /// obtains them and which rejects duplicates.
    #[must_use]
    pub fn root(self, leaves: &[Digest32]) -> Digest32 {
        if leaves.is_empty() {
            return self.empty_root();
        }
        let levels = self.levels(leaves);
        levels
            .last()
            .and_then(|level| level.first())
            .copied()
            .unwrap_or_else(|| self.empty_root())
    }

    /// Verifies an inclusion proof for the leaf at `leaf_index`.
    ///
    /// `siblings` runs leaf-to-root, one per level of the padded tree. The
    /// wire form of such a proof is not defined in protocol v0 — "Serving those
    /// proofs is M-02 work" — so this function takes the siblings directly and
    /// makes no claim about how they are transported.
    #[must_use]
    pub fn verify_inclusion(
        self,
        root: &Digest32,
        leaf: &Digest32,
        leaf_index: usize,
        siblings: &[Digest32],
    ) -> bool {
        let mut current = *leaf;
        for (level, sibling) in siblings.iter().enumerate() {
            current = if (leaf_index >> level) & 1 == 0 {
                self.combine(&current, sibling)
            } else {
                self.combine(sibling, &current)
            };
        }
        current == *root
    }

    /// Every level of the tree, from the padded leaves up to the root.
    ///
    /// Exposed because `ledger.md`'s worked example publishes the internal
    /// nodes as well as the root, and a conformance suite must be able to
    /// compare them individually rather than only the root they produce.
    #[must_use]
    pub fn levels(self, leaves: &[Digest32]) -> Vec<Vec<Digest32>> {
        if leaves.is_empty() {
            return Vec::new();
        }
        let mut current: Vec<Digest32> = leaves.to_vec();
        let padded_width = current.len().next_power_of_two();
        current.resize(padded_width, self.empty_leaf());
        let mut levels = vec![current.clone()];
        while current.len() > 1 {
            let next: Vec<Digest32> = current
                .chunks(2)
                .map(|pair| self.combine(&pair[0], &pair[1]))
                .collect();
            levels.push(next.clone());
            current = next;
        }
        levels
    }
}

/// Sorts `entries` bytewise by their 32-byte sort key and rejects duplicates.
///
/// Every protocol tree says "unique and sorted bytewise". Sorting here rather
/// than trusting the caller is what makes the committed root a function of the
/// *set* and not of the order a proposer happened to serialize it in.
pub fn sorted_unique_leaves<T: Copy>(
    mut entries: Vec<([u8; 32], T)>,
) -> Result<Vec<([u8; 32], T)>> {
    entries.sort_unstable_by_key(|entry| entry.0);
    if entries.windows(2).any(|pair| pair[0].0 == pair[1].0) {
        return Err(MerkleError::DuplicateKey.into());
    }
    Ok(entries)
}

/// `tx_leaf = H(0x00 || raw_32_bytes(tx_id))`.
#[must_use]
pub fn transaction_leaf(tx_id: &Digest32) -> Digest32 {
    tagged_hash(TaggedTree::TRANSACTIONS.leaf_tag(), &[tx_id.as_bytes()])
}

/// The transaction Merkle root of a block, in block order.
///
/// "The transaction Merkle tree preserves block order" — so unlike the other
/// five trees, this one is not sorted, and a block may legitimately be empty.
pub fn transactions_root(tx_ids: &[Digest32]) -> Result<Digest32> {
    if tx_ids.len() > TaggedTree::MAX_TRANSACTIONS {
        return Err(MerkleError::TooManyLeaves {
            limit: TaggedTree::MAX_TRANSACTIONS,
            actual: tx_ids.len(),
        }
        .into());
    }
    let leaves: Vec<Digest32> = tx_ids.iter().map(transaction_leaf).collect();
    Ok(TaggedTree::TRANSACTIONS.root(&leaves))
}

/// `eligible_leaf = H(0x24 || u64be(reward_epoch) || account_key_32)`.
#[must_use]
pub fn eligible_node_leaf(reward_epoch: u64, account_key: &AccountKey) -> Digest32 {
    tagged_hash(
        TaggedTree::ELIGIBLE_NODES.leaf_tag(),
        &[&reward_epoch.to_be_bytes(), account_key.as_bytes()],
    )
}

/// The `eligible_set_root` of an `existence_income` mint.
pub fn eligible_set_root(reward_epoch: u64, account_keys: &[AccountKey]) -> Result<Digest32> {
    let entries = sorted_unique_leaves(
        account_keys
            .iter()
            .map(|key| (*key.as_bytes(), ()))
            .collect(),
    )?;
    let leaves: Vec<Digest32> = entries
        .iter()
        .map(|(key, ())| eligible_node_leaf(reward_epoch, &AccountKey::from_bytes(*key)))
        .collect();
    Ok(TaggedTree::ELIGIBLE_NODES.root(&leaves))
}

/// `candidate_leaf = H(0x40 || u64be(election_epoch) || account_key_32)`.
#[must_use]
pub fn candidate_leaf(election_epoch: u64, account_key: &AccountKey) -> Digest32 {
    tagged_hash(
        TaggedTree::CANDIDATES.leaf_tag(),
        &[&election_epoch.to_be_bytes(), account_key.as_bytes()],
    )
}

/// The `candidate_root` of an [`crate::election::ElectionRecord`].
pub fn candidate_root(election_epoch: u64, account_keys: &[AccountKey]) -> Result<Digest32> {
    let entries = sorted_unique_leaves(
        account_keys
            .iter()
            .map(|key| (*key.as_bytes(), ()))
            .collect(),
    )?;
    let leaves: Vec<Digest32> = entries
        .iter()
        .map(|(key, ())| candidate_leaf(election_epoch, &AccountKey::from_bytes(*key)))
        .collect();
    Ok(TaggedTree::CANDIDATES.root(&leaves))
}

/// `subscription_leaf = H(0x20 || app_id_32 || u64be(reward_epoch) ||
/// account_key_32 || subscription_burn_tx_id_32)`.
#[must_use]
pub fn subscription_leaf(
    app_id: &Digest32,
    reward_epoch: u64,
    account_key: &AccountKey,
    subscription_burn_tx_id: &Digest32,
) -> Digest32 {
    tagged_hash(
        TaggedTree::SUBSCRIPTIONS.leaf_tag(),
        &[
            app_id.as_bytes(),
            &reward_epoch.to_be_bytes(),
            account_key.as_bytes(),
            subscription_burn_tx_id.as_bytes(),
        ],
    )
}

/// `revocation_leaf = H(0x30 || u32be(len(node_id_utf8)) || node_id_utf8 ||
/// u64be(effective_height))`.
pub fn revocation_leaf(node_id: &str, effective_height: u64) -> Result<Digest32> {
    let bytes = node_id.as_bytes();
    let length = u32::try_from(bytes.len())
        .map_err(|_| crate::error::Error::Arithmetic("revocation leaf node_id length"))?;
    Ok(tagged_hash(
        TaggedTree::REVOCATIONS.leaf_tag(),
        &[
            &length.to_be_bytes(),
            bytes,
            &effective_height.to_be_bytes(),
        ],
    ))
}

/// The `revocation_root` of a weak subjectivity checkpoint.
///
/// Entries are unique and sorted bytewise by `node_id`. An empty list uses the
/// dedicated empty-root tag rather than a root over zero padding leaves.
pub fn revocation_root(entries: &[(String, u64)]) -> Result<Digest32> {
    if entries.is_empty() {
        return Ok(TaggedTree::REVOCATIONS.empty_root());
    }
    let mut sorted: Vec<&(String, u64)> = entries.iter().collect();
    sorted.sort_by(|left, right| left.0.as_bytes().cmp(right.0.as_bytes()));
    if sorted
        .windows(2)
        .any(|pair| pair[0].0.as_bytes() == pair[1].0.as_bytes())
    {
        return Err(MerkleError::DuplicateKey.into());
    }
    let leaves = sorted
        .iter()
        .map(|(node_id, height)| revocation_leaf(node_id, *height))
        .collect::<Result<Vec<_>>>()?;
    Ok(TaggedTree::REVOCATIONS.root(&leaves))
}

// --------------------------------------------------------------------------
// Sparse account state tree
// --------------------------------------------------------------------------

/// An app account's lifecycle state.
///
/// `ledger.md` writes `app_leaf` as
/// `H(0x13 || account_key || u64be(balance) || u64be(nonce) || lifecycle_u8 ||
/// u64be(suspension_effective_epoch))`, and since 2026-08-25 it also fixes
/// `lifecycle_u8`: `active` is `0x01`, `grace` is `0x02`, `suspended` is
/// `0x03`, and every other value — `0x00` included — is invalid. The reserved
/// zero is deliberate and is the reason [`AppLifecycle::from_u8`] exists as a
/// fallible constructor rather than as a `From<u8>` with a fallback arm: a
/// zero-filled or truncated record must be rejected where it is read, not
/// silently become the permissive state.
///
/// Until that encoding was published this crate carried a provisional `0/1/2`
/// mapping, recorded as [DEBT-012]: two conformant implementations could
/// compute different `app_leaf` values for the same state and split the chain
/// at the first app account that was not `active`. `APP-0` in the conformance
/// registry now pins the published byte.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppLifecycle {
    /// Serving normally.
    Active,
    /// The next charge was unavailable; suspension is pending.
    Grace,
    /// Suspended, still in state and in the catalog.
    Suspended,
}

impl AppLifecycle {
    /// The reserved `lifecycle_u8` value. Never assigned, always invalid.
    pub const RESERVED_U8: u8 = 0x00;

    /// The normative `lifecycle_u8` encoding of `ledger.md`.
    #[must_use]
    pub const fn as_u8(self) -> u8 {
        match self {
            Self::Active => 0x01,
            Self::Grace => 0x02,
            Self::Suspended => 0x03,
        }
    }

    /// Decodes a `lifecycle_u8`, rejecting every unassigned value.
    ///
    /// There is no default arm on purpose. `0x00` is the value an
    /// uninitialized or truncated record yields for free, and the whole point
    /// of reserving it is that reading one is an error rather than a silently
    /// permissive `Active`.
    pub fn from_u8(byte: u8) -> Result<Self> {
        match byte {
            0x01 => Ok(Self::Active),
            0x02 => Ok(Self::Grace),
            0x03 => Ok(Self::Suspended),
            _ => Err(crate::error::JsonError::Field("lifecycle_u8".to_owned()).into()),
        }
    }

    /// Parses the protocol's textual spelling.
    pub fn parse(text: &str) -> Result<Self> {
        match text {
            "active" => Ok(Self::Active),
            "grace" => Ok(Self::Grace),
            "suspended" => Ok(Self::Suspended),
            _ => Err(crate::error::JsonError::Field("lifecycle".to_owned()).into()),
        }
    }
}

/// The account state a proof asserts.
///
/// `Absent` carries no balance and no nonce, so "an absent account with a
/// non-zero balance" is not a value this type can hold. Step 9 of the
/// light-client algorithm requires that check; making it structural means it
/// cannot be forgotten at one call site.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AccountState {
    /// No leaf exists; balance and nonce are implicitly zero.
    Absent,
    /// A node account leaf.
    Node {
        /// Balance in microtokens.
        balance_microtokens: u64,
        /// Strictly consecutive spend nonce.
        account_nonce: u64,
    },
    /// An app account leaf.
    App {
        /// Balance in microtokens.
        balance_microtokens: u64,
        /// Strictly consecutive spend nonce.
        account_nonce: u64,
        /// Lifecycle state.
        lifecycle: AppLifecycle,
        /// Epoch at which a pending suspension becomes effective.
        suspension_effective_epoch: u64,
    },
}

impl AccountState {
    /// The leaf digest at depth 256, or the depth-256 default when absent.
    #[must_use]
    pub fn leaf(&self, account_key: &AccountKey) -> Digest32 {
        match self {
            Self::Absent => SparseAccountTree::empty_at(256),
            Self::Node {
                balance_microtokens,
                account_nonce,
            } => tagged_hash(
                SparseAccountTree::NODE_LEAF_TAG,
                &[
                    account_key.as_bytes(),
                    &balance_microtokens.to_be_bytes(),
                    &account_nonce.to_be_bytes(),
                ],
            ),
            Self::App {
                balance_microtokens,
                account_nonce,
                lifecycle,
                suspension_effective_epoch,
            } => tagged_hash(
                SparseAccountTree::APP_LEAF_TAG,
                &[
                    account_key.as_bytes(),
                    &balance_microtokens.to_be_bytes(),
                    &account_nonce.to_be_bytes(),
                    &[lifecycle.as_u8()],
                    &suspension_effective_epoch.to_be_bytes(),
                ],
            ),
        }
    }
}

/// The depth-256 binary sparse Merkle tree over account keys.
#[derive(Debug, Clone, Copy)]
pub struct SparseAccountTree;

impl SparseAccountTree {
    /// `node_leaf` tag.
    pub const NODE_LEAF_TAG: u8 = 0x10;
    /// `branch` tag, also the tag of every default subtree above depth 256.
    pub const BRANCH_TAG: u8 = 0x11;
    /// `empty[256]` tag.
    pub const EMPTY_LEAF_TAG: u8 = 0x12;
    /// `app_leaf` tag.
    pub const APP_LEAF_TAG: u8 = 0x13;
    /// Tree depth.
    pub const DEPTH: usize = 256;

    /// `empty[depth]` for `depth` in `0..=256`.
    ///
    /// # Panics
    ///
    /// Panics if `depth > 256`, which is a programming error rather than an
    /// input condition: the depth is always a loop index in this crate.
    #[must_use]
    pub fn empty_at(depth: usize) -> Digest32 {
        static DEFAULTS: OnceLock<Vec<Digest32>> = OnceLock::new();
        let defaults = DEFAULTS.get_or_init(|| {
            let mut levels = vec![Digest32::default(); Self::DEPTH + 1];
            levels[Self::DEPTH] = tagged_hash(Self::EMPTY_LEAF_TAG, &[]);
            for depth in (0..Self::DEPTH).rev() {
                let child = levels[depth + 1];
                levels[depth] =
                    tagged_hash(Self::BRANCH_TAG, &[child.as_bytes(), child.as_bytes()]);
            }
            levels
        });
        defaults[depth]
    }

    /// `branch = H(0x11 || left || right)`.
    #[must_use]
    pub fn branch(left: &Digest32, right: &Digest32) -> Digest32 {
        tagged_hash(Self::BRANCH_TAG, &[left.as_bytes(), right.as_bytes()])
    }
}

/// One account proof against a header's `state_root`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AccountProof {
    /// The 32-byte account key. A verifier recomputes it from the requested
    /// account kind and subject ID before using the proof.
    pub account_key: AccountKey,
    /// The state the proof asserts.
    pub state: AccountState,
    /// 256 bits, root-to-leaf, most significant bit first in each byte.
    pub sibling_bitmap: [u8; 32],
    /// Non-default siblings, root-to-leaf.
    pub siblings: Vec<Digest32>,
}

impl AccountProof {
    /// Recomputes the root this proof implies, rejecting non-canonical proofs.
    ///
    /// "An explicitly supplied default hash (bit 1 with sibling equal to
    /// `empty[d+1]`) is non-canonical and MUST be rejected even if it
    /// reconstructs the root."
    pub fn compute_root(&self) -> Result<Digest32> {
        let population = self
            .sibling_bitmap
            .iter()
            .map(|byte| byte.count_ones() as usize)
            .sum::<usize>();
        if population != self.siblings.len() {
            return Err(MerkleError::SiblingCountMismatch {
                expected: population,
                actual: self.siblings.len(),
            }
            .into());
        }

        // Siblings are ordered root-to-leaf; the rebuild walks leaf-to-root, so
        // it consumes them from the back.
        let mut next_sibling = self.siblings.len();
        let mut current = self.state.leaf(&self.account_key);
        for depth in (0..SparseAccountTree::DEPTH).rev() {
            let default = SparseAccountTree::empty_at(depth + 1);
            let sibling = if bit_at(&self.sibling_bitmap, depth) {
                next_sibling -= 1;
                let supplied = self.siblings[next_sibling];
                if supplied == default {
                    return Err(MerkleError::NonCanonicalDefaultSibling { depth }.into());
                }
                supplied
            } else {
                default
            };
            current = if bit_at(self.account_key.as_bytes(), depth) {
                SparseAccountTree::branch(&sibling, &current)
            } else {
                SparseAccountTree::branch(&current, &sibling)
            };
        }
        Ok(current)
    }

    /// Verifies the proof against a trusted `state_root`.
    #[must_use]
    pub fn verify(&self, state_root: &Digest32) -> bool {
        self.compute_root()
            .is_ok_and(|root| constant_time_eq(root.as_bytes(), state_root.as_bytes()))
    }
}

/// Reads bit `index` of a big-endian bit string, most significant bit first.
fn bit_at(bytes: &[u8; 32], index: usize) -> bool {
    let byte = bytes[index / 8];
    (byte >> (7 - (index % 8))) & 1 == 1
}

/// Compares two digests without an early exit.
///
/// "Compare the final 32-byte value to `state_root` in constant time."
#[must_use]
pub fn constant_time_eq(left: &[u8; 32], right: &[u8; 32]) -> bool {
    let mut difference = 0u8;
    for index in 0..32 {
        difference |= left[index] ^ right[index];
    }
    difference == 0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty_root_and_padding_leaf_use_different_tags() {
        assert_ne!(
            TaggedTree::CANDIDATES.empty_root(),
            TaggedTree::CANDIDATES.empty_leaf()
        );
    }

    #[test]
    fn duplicate_leaves_are_rejected_before_hashing() {
        let key = AccountKey::from_bytes([7u8; 32]);
        assert!(candidate_root(1, &[key, key]).is_err());
    }

    #[test]
    fn transaction_tree_rejects_more_than_the_block_limit() {
        let ids = vec![Digest32::repeated(1); TaggedTree::MAX_TRANSACTIONS + 1];
        assert!(transactions_root(&ids).is_err());
    }
}
