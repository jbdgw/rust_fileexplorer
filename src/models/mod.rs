// Core entry types
mod entry;
pub use entry::{Entry, EntryKind};

#[cfg(feature = "watch")]
pub use entry::WatchEvent;

// Content search (grep feature)
mod match_result;
pub use match_result::ContentMatch;

// Duplicate detection
mod duplicate;
pub use duplicate::DuplicateGroup;

// Git integration
#[cfg(feature = "git")]
mod git_status;
#[cfg(feature = "git")]
pub use git_status::{GitEntry, GitStatus};

// Smart categorization
mod category;
pub use category::{FileCategory, MediaType};

// Sorting and display
use serde::{Deserialize, Serialize};

/// Sorting keys for entries
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SortKey {
    Name,
    Size,
    Mtime,
    Kind,
}

/// Sorting order
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum SortOrder {
    Asc,
    Desc,
}

/// Output columns to display
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Column {
    Path,
    Name,
    Size,
    Mtime,
    Kind,
    Perms,
    Owner,
}

impl Column {
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "path" => Some(Column::Path),
            "name" => Some(Column::Name),
            "size" => Some(Column::Size),
            "mtime" => Some(Column::Mtime),
            "kind" => Some(Column::Kind),
            "perms" => Some(Column::Perms),
            "owner" => Some(Column::Owner),
            _ => None,
        }
    }
}

/// Output format
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OutputFormat {
    Pretty,
    Json,
    Ndjson,
    Csv,
}

impl OutputFormat {
    #[allow(clippy::should_implement_trait)]
    pub fn from_str(s: &str) -> Option<Self> {
        match s.to_lowercase().as_str() {
            "pretty" => Some(OutputFormat::Pretty),
            "json" => Some(OutputFormat::Json),
            "ndjson" => Some(OutputFormat::Ndjson),
            "csv" => Some(OutputFormat::Csv),
            _ => None,
        }
    }
}
