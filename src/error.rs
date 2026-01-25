//! Custom error types for the nrd.sh compiler.

use std::path::PathBuf;

/// All errors that can occur during site compilation.
#[derive(Debug, thiserror::Error)]
pub enum Error {
    /// Failed to read a content file.
    #[error("failed to read {path}: {source}")]
    ReadFile {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    /// Failed to parse frontmatter.
    #[error("invalid frontmatter in {path}: {message}")]
    Frontmatter { path: PathBuf, message: String },

    /// Failed to write output file.
    #[error("failed to write {path}: {source}")]
    WriteFile {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    /// Failed to create output directory.
    #[error("failed to create directory {path}: {source}")]
    CreateDir {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    /// Content directory not found.
    #[error("content directory not found: {0}")]
    ContentDirNotFound(PathBuf),
}

/// Result type alias for compiler operations.
pub type Result<T> = std::result::Result<T, Error>;
