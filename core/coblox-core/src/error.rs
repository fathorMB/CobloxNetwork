//! Recoverable error types for the deterministic protocol layer.
//!
//! Every fallible operation in this crate returns [`Error`]. Nothing in the
//! deterministic layer panics on untrusted input: a governed document that a
//! quorum signed, a peer-supplied object, and a hand-written test fixture all
//! reach the same validation code and all fail the same recoverable way.

use core::fmt;

/// Result alias used throughout the crate.
pub type Result<T> = core::result::Result<T, Error>;

/// Everything that can go wrong in the deterministic protocol layer.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Error {
    /// A byte string was not valid unpadded RFC 4648 base64url, or decoded to
    /// the wrong length for the field it belongs to.
    Base64Url(&'static str),
    /// A byte string was not valid lowercase unpadded RFC 4648 base32.
    Base32(&'static str),
    /// A hash was not `sha256:` followed by exactly 64 lowercase hex digits.
    DigestString,
    /// An integer field was not the shortest unsigned base-10 form.
    NonCanonicalUint,
    /// A JSON document violated the I-JSON/JCS subset the protocol defines.
    Json(JsonError),
    /// A Merkle tree input was rejected before any hashing happened.
    Merkle(MerkleError),
    /// A governed document or a genesis trust anchor failed validation.
    Parameter(ParameterError),
    /// A validator-set document or transition was rejected.
    ValidatorSet(SetError),
    /// The election derivation produced no valid set, or its inputs were
    /// inconsistent with the finalized data they claim to summarize.
    Election(ElectionError),
    /// A checked `u128` intermediate overflowed, or a total power was zero.
    Arithmetic(&'static str),
}

/// Reasons a JSON document is not an acceptable Coblox object.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum JsonError {
    /// The byte stream was not valid UTF-8.
    NotUtf8,
    /// The document parsed, but its bytes are not the JCS serialization of the
    /// value they encode. Non-canonical bytes are rejected, never normalized.
    NonCanonical,
    /// Trailing bytes followed the top-level value.
    TrailingBytes,
    /// The input ended in the middle of a value.
    UnexpectedEnd,
    /// A byte was not legal at that position.
    UnexpectedByte(u8),
    /// JSON numbers are forbidden: every protocol integer is a `u64` string.
    NumberForbidden,
    /// `null` is forbidden.
    NullForbidden,
    /// An object key repeated.
    DuplicateKey(String),
    /// An object key was not lower `snake_case` ASCII.
    InvalidKey(String),
    /// A `\u` escape was malformed or formed an unpaired surrogate.
    InvalidEscape,
    /// A raw control character appeared inside a string.
    ControlCharacter(u8),
    /// The top-level value was not an object.
    NotAnObject,
    /// A field was absent, or had the wrong JSON type.
    Field(String),
}

/// Reasons a Merkle input is rejected.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum MerkleError {
    /// Two leaves carried the same sort key; protocol trees require uniqueness.
    DuplicateKey,
    /// More leaves than the protocol permits for that tree.
    TooManyLeaves { limit: usize, actual: usize },
    /// A sparse-tree proof declared a sibling count that disagrees with its
    /// bitmap population count.
    SiblingCountMismatch { expected: usize, actual: usize },
    /// A sparse-tree proof set a bitmap bit but supplied the depth default as
    /// the sibling. Such a proof reconstructs the root and is still invalid.
    NonCanonicalDefaultSibling { depth: usize },
    /// An absent account carried a non-zero balance, non-zero nonce, or
    /// account-kind-specific fields.
    AbsentAccountNotEmpty,
}

/// Reasons a governed document or genesis trust anchor is rejected.
///
/// These are acceptance-time validity rules, not advice: a document that
/// violates one of them is invalid, and the network must not adopt it.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ParameterError {
    /// A relational or magnitude constraint of the election constraint block
    /// failed. `rule` is the constraint as it is written in `ledger.md`.
    Constraint {
        /// The constraint text, verbatim from the specification.
        rule: &'static str,
    },
    /// An `ElectionBounds` object is not a usable trust anchor.
    Bounds {
        /// The bound that failed.
        rule: &'static str,
    },
    /// The chain ID of a trust anchor or document differs from the configured
    /// chain ID. A client must fail closed rather than adopt it.
    ChainIdMismatch,
    /// An election parameter moved further than the genesis change ratio
    /// permits, against the currently active document.
    ChangeRatio {
        /// The parameter that moved too far.
        parameter: &'static str,
    },
    /// Consecutive election-parameter changes were not spaced by at least
    /// `election_parameter_min_activation_gap_blocks`.
    ActivationGap,
    /// `validator_max_consecutive_terms` decreased. On a live chain the term
    /// limit never decreases.
    TermLimitDecreased,
    /// `sequence` did not strictly increase for that `document_kind`.
    SequenceNotIncreasing,
    /// The Argon2id cost floor of the enrollment parameter document failed.
    EnrollmentCostFloor {
        /// The floor that failed.
        rule: &'static str,
    },
    /// A reward-policy body violated one of its acceptance-time rules.
    RewardPolicy {
        /// The rule that failed.
        rule: &'static str,
    },
}

/// Reasons a validator-set document or transition is rejected.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SetError {
    /// Members are not sorted by `validator_id`, or an ID repeats.
    NotSortedOrUnique,
    /// The set is empty, or its size leaves `[min_set_size, max_set_size]`.
    Size { member_count: u64 },
    /// An entry of an elected set has `validator_id != node_id`, or
    /// `voting_power != 1`.
    NotUniformElectedEntry { validator_id: String },
    /// Summed voting power overflowed `u64`, or an entry had zero power.
    VotingPower,
    /// `election_epoch >= term_expiry_epoch` for some entry.
    TermExpired { validator_id: String },
    /// The set activates at a height that is not `election_epoch * L`.
    ActivationHeight { expected: u64, actual: u64 },
    /// `previous_validator_set_hash` does not equal the hash of the set being
    /// replaced.
    PreviousHashMismatch,
    /// `retained_count`, `filled_count` or `member_count` disagrees with the
    /// array they describe.
    CountMismatch { field: &'static str },
    /// `filled_count` exceeds `validator_churn_cap_seats`.
    ChurnCapExceeded { filled: u64, cap: u64 },
    /// `3 * member_count(new) > 2 * member_count(old)` failed.
    ContractionFloor { new: u64, old: u64 },
    /// A member present in both sets changed `seated_since_epoch` or
    /// `term_expiry_epoch`, or a newly seated member carries the wrong stamps.
    StampInconsistent { validator_id: String },
    /// The committed `election_seed` is not the hash of the committed entropy
    /// block IDs.
    SeedMismatch,
    /// `entropy_block_ids` has the wrong length, or `entropy_first_height`
    /// disagrees with the boundary.
    EntropyWindow,
    /// An off-boundary transition admitted a member, changed an entry, or
    /// altered the copied election record.
    RemovalOnlyViolated { reason: &'static str },
    /// A removed member has no revocation covering that activation height, or
    /// a retained member is revoked.
    Revocation { node_id: String },
    /// The genesis set is not a valid trust anchor.
    Genesis { rule: &'static str },
    /// The header changed the committed successor set outside an election
    /// boundary and outside a revocation-forced transition.
    OffScheduleChange { height: u64 },
}

/// Reasons the election derivation yields no set.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ElectionError {
    /// Epoch 0 is the genesis set and is never derived.
    NotAnElectionEpoch,
    /// The derived set does not satisfy the contraction floor. No valid set
    /// exists for this epoch and the chain stalls at the boundary.
    ContractionFloor { new: u64, previous: u64 },
    /// The derived set holds fewer than `validator_min_set_size` members. No
    /// valid set exists for this epoch and the chain stalls at the boundary.
    BelowMinimumSetSize { new: u64, minimum: u64 },
    /// A caller-supplied eligible set contradicts the derivation: a retained
    /// member absent from `C`, or a member of `P` that failed retention still
    /// present in `C`.
    InconsistentCandidateSet { node_id: String },
    /// Two candidates declared the same `account_key`.
    DuplicateAccountKey,
    /// The entropy window does not hold exactly `election_entropy_blocks` IDs.
    EntropyWindow { expected: u64, actual: usize },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Base64Url(ctx) => write!(f, "invalid unpadded base64url in {ctx}"),
            Self::Base32(ctx) => write!(f, "invalid lowercase base32 in {ctx}"),
            Self::DigestString => {
                f.write_str("expected sha256: followed by 64 lowercase hex digits")
            }
            Self::NonCanonicalUint => {
                f.write_str("integer is not the shortest unsigned base-10 form")
            }
            Self::Json(e) => write!(f, "canonical JSON rejected: {e:?}"),
            Self::Merkle(e) => write!(f, "merkle input rejected: {e:?}"),
            Self::Parameter(e) => write!(f, "parameter validation failed: {e:?}"),
            Self::ValidatorSet(e) => write!(f, "validator set rejected: {e:?}"),
            Self::Election(e) => write!(f, "no valid election result: {e:?}"),
            Self::Arithmetic(ctx) => write!(f, "checked arithmetic failed in {ctx}"),
        }
    }
}

impl core::error::Error for Error {}

impl From<JsonError> for Error {
    fn from(value: JsonError) -> Self {
        Self::Json(value)
    }
}

impl From<MerkleError> for Error {
    fn from(value: MerkleError) -> Self {
        Self::Merkle(value)
    }
}

impl From<ParameterError> for Error {
    fn from(value: ParameterError) -> Self {
        Self::Parameter(value)
    }
}

impl From<SetError> for Error {
    fn from(value: SetError) -> Self {
        Self::ValidatorSet(value)
    }
}

impl From<ElectionError> for Error {
    fn from(value: ElectionError) -> Self {
        Self::Election(value)
    }
}
