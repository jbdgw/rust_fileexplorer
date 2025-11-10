use std::path::PathBuf;
use thiserror::Error;

/// Application-level errors with context
#[derive(Error, Debug)]
pub enum FsError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Failed to access path: {path}")]
    PathAccess {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("Invalid glob pattern: {pattern}")]
    InvalidGlob {
        pattern: String,
        #[source]
        source: globset::Error,
    },

    #[error("Invalid regex pattern: {pattern}")]
    InvalidRegex {
        pattern: String,
        #[source]
        source: regex::Error,
    },

    #[error("Failed to parse size: {input}")]
    InvalidSize { input: String },

    #[error("Failed to parse date: {input}")]
    InvalidDate { input: String },

    #[error("Invalid output format: {format}")]
    InvalidFormat { format: String },

    #[error("CSV error: {0}")]
    Csv(#[from] csv::Error),

    #[error("JSON serialization error: {0}")]
    Json(#[from] serde_json::Error),

    #[error("Watch error: {0}")]
    Watch(String),

    #[error("No entries found matching criteria")]
    NoEntriesFound,

    #[error("IO error: {context}")]
    IoError {
        context: String,
        #[source]
        source: std::io::Error,
    },
}

pub type Result<T> = std::result::Result<T, FsError>;
