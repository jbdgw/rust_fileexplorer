use super::Entry;
use serde::{Deserialize, Serialize};

/// Represents a group of duplicate files
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DuplicateGroup {
    /// Hash of the file contents (BLAKE3)
    pub hash: String,
    /// Size of each file in bytes
    pub size: u64,
    /// Number of duplicates in this group
    pub count: usize,
    /// List of duplicate entries
    pub entries: Vec<Entry>,
    /// Total wasted space (size * (count - 1))
    pub wasted_space: u64,
}

impl DuplicateGroup {
    pub fn new(hash: String, size: u64, entries: Vec<Entry>) -> Self {
        let count = entries.len();
        let wasted_space = if count > 1 {
            size * (count as u64 - 1)
        } else {
            0
        };

        Self {
            hash,
            size,
            count,
            entries,
            wasted_space,
        }
    }
}
