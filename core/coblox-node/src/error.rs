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
}

pub type Result<T> = std::result::Result<T, NodeError>;
