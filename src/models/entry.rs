use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Represents a filesystem entry with metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Entry {
    pub path: PathBuf,
    pub name: String,
    pub size: u64,
    pub kind: EntryKind,
    #[serde(with = "chrono::serde::ts_seconds")]
    pub mtime: DateTime<Utc>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub perms: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub owner: Option<String>,
    pub depth: usize,
}

/// File system entry types
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum EntryKind {
    File,
    Dir,
    Symlink,
}

impl EntryKind {
    pub fn from_metadata(metadata: &std::fs::Metadata) -> Self {
        let file_type = metadata.file_type();
        if file_type.is_symlink() {
            EntryKind::Symlink
        } else if file_type.is_dir() {
            EntryKind::Dir
        } else {
            EntryKind::File
        }
    }
}

/// Watch events
#[cfg(feature = "watch")]
#[derive(Debug, Clone, Serialize)]
pub struct WatchEvent {
    pub event: String,
    pub path: PathBuf,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub mtime: Option<DateTime<Utc>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub size: Option<u64>,
}
