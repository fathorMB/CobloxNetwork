//! Error types for `coblox-node`.

use thiserror::Error;

/// Everything that can fail in node execution.
#[derive(Debug, Error)]
pub enum NodeError {
    #[error("Core error: {0}")]
    Core(#[from] coblox_core::Error),

    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Hex decoding error: {0}")]
    Hex(#[from] hex::FromHexError),

    #[error("Protocol error: {0}")]
    Protocol(String),

    /// An envelope was refused at the wire boundary.
    ///
    /// It is a separate variant because it is the one error a running node must
    /// **not** die of: the boundary exists to be hit by whatever a peer sends,
    /// and a node that exits on the first bad envelope is a node any stranger
    /// can stop. Every other variant stays fatal — a WAL that will not `fsync`
    /// is not a message-level rejection. See [REVIEW-049] RF-001.
    #[error("envelope rejected at the wire boundary: {0}")]
    Rejected(String),
}

impl NodeError {
    /// Whether this error must stop the node.
    ///
    /// False only for [`NodeError::Rejected`].
    #[must_use]
    pub const fn is_fatal(&self) -> bool {
        !matches!(self, Self::Rejected(_))
    }
}

pub type Result<T> = std::result::Result<T, NodeError>;
