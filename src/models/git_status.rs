use super::Entry;
use serde::{Deserialize, Serialize};

/// Represents a file with git status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GitEntry {
    /// The filesystem entry
    #[serde(flatten)]
    pub entry: Entry,
    /// Git status of the file
    pub status: GitStatus,
    /// Current branch name (if in a repo)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub branch: Option<String>,
}

/// Git file status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum GitStatus {
    /// File is not tracked by git
    Untracked,
    /// File has modifications in working directory
    Modified,
    /// File is staged for commit
    Staged,
    /// File has merge conflicts
    Conflict,
    /// File is tracked and unchanged
    Clean,
    /// File is ignored by .gitignore
    Ignored,
}
