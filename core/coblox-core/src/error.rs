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
    /// A transport key attestation failed verification.
    Attestation(AttestationError),
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
    /// A measured cadence left the genesis band, or a reward-epoch index ran
    /// ahead of the chain that has to pay for it.
    Cadence(CadenceError),
    /// A single-key transaction authorization failed the *enrolled, unrevoked*
    /// qualification, or the key did not derive the node ID it claims.
    Authorization(AuthorizationError),
    /// An identity revocation violated its validity rules or reason-dependent effective height band.
    Revocation(RevocationError),
    /// A quorum certificate or a consensus message was rejected.
    Consensus(ConsensusError),
    /// A checked `u128` intermediate overflowed, or a total power was zero.
    Arithmetic(&'static str),
}

/// Reasons a quorum certificate or a consensus message is rejected.
///
/// These are the rejections of the two-phase protocol of [ADR-018] and of the
/// `QuorumCertificate` rules of `ledger.md#what-validators-sign`. They are a
/// family of their own and not variants of [`SetError`], because a certificate
/// naming the wrong set and a *set document* with a mismatched previous hash are
/// different findings that happen to compare two digests: the first is a
/// certificate a verifier must discard, the second is a transition a verifier
/// must reject, and a shared variant would make a log line ambiguous about which
/// happened.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ConsensusError {
    /// The certificate's `validator_set_hash` is not the hash of the set it was
    /// verified against.
    CertificateNamesAnotherSet {
        expected: crate::hash::Digest32,
        actual: crate::hash::Digest32,
    },
    /// The certificate finalizes a different block than the one it is attached
    /// to.
    CertificateForAnotherBlock {
        expected: crate::hash::Digest32,
        actual: crate::hash::Digest32,
    },
    /// The certificate's height is not the header's.
    CertificateHeightMismatch { header: u64, certificate: u64 },
    /// The certificate carries no signatures.
    ///
    /// "An empty or duplicate signature entry invalidates the certificate."
    CertificateEmpty,
    /// The certificate's signatures are not strictly sorted by `validator_id`,
    /// which covers both halves of "unique and sorted".
    CertificateNotSortedOrUnique,
    /// A `validator_id` in the certificate is not a member of the set.
    NotAMember { validator_id: String },
    /// A signature failed verification under the member's consensus key, in the
    /// certificate's own domain and chain.
    InvalidSignature { validator_id: String },
    /// The signed power does not satisfy the strict quorum predicate.
    BelowQuorum { signed_power: u64, total_power: u64 },
    /// A message arrived from a node that is not a member of the active set.
    SenderNotAMember { validator_id: String },
    /// A proposal arrived from a node that is not the proposer of its
    /// `(height, round)` under the round-robin rule.
    NotTheProposer {
        height: u64,
        round: u64,
        expected: String,
        actual: String,
    },
    /// A proposal's `block_id` is not the hash of the header it carries.
    ProposalBlockIdMismatch,
    /// A proposal's header does not carry the `(height, round)` the message
    /// claims.
    ProposalHeaderMismatch { field: &'static str },
    /// A proposal's `transactions` do not reproduce its header's
    /// `transactions_root`.
    ///
    /// The two digests are carried because the rejection is otherwise
    /// indistinguishable from a truncated payload, and the difference matters to
    /// whoever reads the log: a mismatch is a proposer that sent one header with
    /// two payloads, and that is attributable to the round's proposer by the
    /// proposer rule alone.
    ProposalTransactionsRootMismatch {
        /// The root the header declares.
        declared: crate::hash::Digest32,
        /// The root recomputed from the carried transactions.
        computed: crate::hash::Digest32,
    },
    /// A proposal's `valid_round` is not below its own round.
    ProposalValidRoundNotBelowRound { round: u64, valid_round: u64 },
    /// A proposer offered a value for a `(height, round)` it is not proposing.
    UnsolicitedValue { height: u64, round: u64 },
    /// A restored lock names a round without a block, or a block without a
    /// round.
    ///
    /// `lockedRound_p` and `lockedValue_p` are one fact in two fields, and a
    /// caller that restores half of it after a restart would start a node that
    /// believes it is unlocked at a round it is locked at. Failing at
    /// construction says so once; dropping the half-specified lock would not say
    /// it at all. See [REVIEW-049] RF-002.
    IncompleteRestoredLock {
        /// Whether `locked_round` was supplied.
        has_round: bool,
        /// Whether `locked_block_id` was supplied.
        has_block_id: bool,
    },
}

/// Reasons a single-key transaction authorization is rejected.
///
/// Every variant carries the height the qualification was evaluated at, because
/// the answer is only meaningful with it: the same key is authorized below
/// the block including the revocation and rejected at or above it, and a rejection message that
/// omitted the height would read as a statement about the key rather than about
/// the block. See
/// `ledger.md#what-enrolled-unrevoked-means-and-as-of-which-height` and [ADR-017].
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AuthorizationError {
    /// The authorizing public key does not derive the node ID the body names.
    KeyDoesNotDerive {
        /// The node ID the transaction body claims.
        node_id: String,
    },
    /// No finalized enrollment certificate names the node ID at this height.
    NotEnrolled {
        /// The node ID the transaction body claims.
        node_id: String,
        /// The height of the block including the transaction.
        height: u64,
    },
    /// A finalized `revoke_identity` names the node ID and was included at or below this height.
    Revoked {
        /// The node ID the transaction body claims.
        node_id: String,
        /// The height of the block including the transaction.
        height: u64,
        /// The height of the block that included the `revoke_identity`.
        included_height: u64,
    },
}

/// Reasons a cadence measurement or a reward-epoch index is rejected.
///
/// Every variant here is about a clock **outside** the chain, or about `height`,
/// which is the one chain quantity a validator cannot write freely. None of them
/// is about `timestamp_ms`, and that omission is the substance of [ADR-013]
/// part 3 rather than an accident of this enumeration.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CadenceError {
    /// Blocks arrived faster than `min_ms_per_block` permits.
    FasterThanBand {
        /// Blocks produced across the measured interval.
        blocks: u64,
        /// Real milliseconds the interval spans.
        elapsed_ms: u64,
        /// `elapsed_ms / blocks`, truncated; a diagnostic, not the comparison.
        observed_ms_per_block: u64,
    },
    /// Blocks arrived more slowly than `max_ms_per_block` permits.
    SlowerThanBand {
        /// Blocks produced across the measured interval.
        blocks: u64,
        /// Real milliseconds the interval spans.
        elapsed_ms: u64,
        /// `elapsed_ms / blocks`, truncated; a diagnostic, not the comparison.
        observed_ms_per_block: u64,
    },
    /// The interval carried fewer blocks than the genesis band requires before
    /// a measurement means anything.
    Inconclusive {
        /// Blocks produced across the measured interval.
        blocks: u64,
        /// The genesis minimum this measurement did not reach.
        min_measured_blocks: u64,
    },
    /// The later height was below the earlier one.
    HeightRegression,
    /// The later external timestamp was below the earlier one.
    ClockRegression,
    /// `block_interval_ms` was zero, which would make every quantity
    /// denominated in blocks meaningless rather than merely unenforced.
    DegenerateInterval,
    /// `reward_epoch_ms` was zero.
    DegenerateEpoch,
    /// A `mint` named a `reward_epoch` whose settlement floor the including
    /// height has not reached.
    RewardEpochAhead {
        /// The index the mint named.
        reward_epoch: u64,
        /// The height of the block the mint was to be included in.
        height: u64,
    },
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

/// Reasons a transport key attestation is rejected.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AttestationError {
    /// Schema version was not "0.1".
    UnsupportedVersion(String),
    /// Network ID did not match the expected network ID.
    NetworkIdMismatch { expected: String, actual: String },
    /// Attestation node ID does not match the derived node ID of the identity public key.
    NodeIdMismatch { expected: String, actual: String },
    /// The transport public key does not match the authenticated peer's transport key.
    TransportKeyMismatch,
    /// The attested transport key *is* the enrolled identity key.
    ///
    /// `identity.md#key-hierarchy` makes the two keys distinct as a validity
    /// rule and not as a description: a node that reuses one key for both roles
    /// makes the `node_id`-to-Peer-ID link recomputable by any offline reader of
    /// the ledger, which is TM-28 in its original form.
    TransportKeyEqualsIdentityKey,
    /// `created_at_ms` exceeds `expires_at_ms`.
    InvalidValidityWindow {
        created_at_ms: u64,
        expires_at_ms: u64,
    },
    /// `expires_at_ms - created_at_ms` exceeds
    /// `max_transport_attestation_validity_ms`.
    ValidityWindowTooLong { duration_ms: u64, maximum_ms: u64 },
    /// The attestation timestamp is expired or not yet active.
    Expired {
        now_ms: u64,
        created_at_ms: u64,
        expires_at_ms: u64,
    },
    /// Attestation signature failed verification under the enrolled identity public key.
    InvalidSignature,
}

/// Reasons a `revoke_identity` transaction body or its `effective_height` is rejected.
#[derive(Debug, Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum RevocationError {
    /// An unknown revocation reason was specified.
    UnknownReason(String),
    /// `effective_height` is below the floor `p + min_revocation_effective_delay_blocks`.
    EffectiveHeightBelowFloor {
        including_height: u64,
        effective_height: u64,
        floor: u64,
    },
    /// `effective_height` is above the ceiling for the given reason.
    EffectiveHeightAboveCeiling {
        including_height: u64,
        effective_height: u64,
        ceiling: u64,
    },
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
            Self::Attestation(e) => write!(f, "transport key attestation rejected: {e:?}"),
            Self::Json(e) => write!(f, "canonical JSON rejected: {e:?}"),
            Self::Merkle(e) => write!(f, "merkle input rejected: {e:?}"),
            Self::Parameter(e) => write!(f, "parameter validation failed: {e:?}"),
            Self::ValidatorSet(e) => write!(f, "validator set rejected: {e:?}"),
            Self::Election(e) => write!(f, "no valid election result: {e:?}"),
            Self::Cadence(e) => write!(f, "chain cadence rejected: {e:?}"),
            Self::Authorization(e) => write!(f, "transaction authorization rejected: {e:?}"),
            Self::Revocation(e) => write!(f, "identity revocation rejected: {e:?}"),
            Self::Consensus(e) => write!(f, "consensus message rejected: {e:?}"),
            Self::Arithmetic(ctx) => write!(f, "checked arithmetic failed in {ctx}"),
        }
    }
}

impl core::error::Error for Error {}

impl From<CadenceError> for Error {
    fn from(value: CadenceError) -> Self {
        Self::Cadence(value)
    }
}

impl From<AttestationError> for Error {
    fn from(value: AttestationError) -> Self {
        Self::Attestation(value)
    }
}

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

impl From<AuthorizationError> for Error {
    fn from(value: AuthorizationError) -> Self {
        Self::Authorization(value)
    }
}

impl From<RevocationError> for Error {
    fn from(value: RevocationError) -> Self {
        Self::Revocation(value)
    }
}

impl From<ConsensusError> for Error {
    fn from(value: ConsensusError) -> Self {
        Self::Consensus(value)
    }
}
