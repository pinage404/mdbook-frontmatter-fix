//! Crate-specific error types.

use std::path::PathBuf;
use thiserror::Error;

/// Errors that can occur while injecting frontmatter.
#[derive(Debug, Error)]
pub enum InjectError {
    /// Git subprocess failed or returned no usable output.
    #[error("git query failed for '{path}': {reason}")]
    GitFailure { path: PathBuf, reason: String },

    /// The file has no commit history and `fallback_on_no_history` is false.
    #[error("'{path}' has no git history and fallback is disabled")]
    NoHistory { path: PathBuf },

    /// I/O error running git.
    #[error("I/O error running git for '{path}': {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },
}
