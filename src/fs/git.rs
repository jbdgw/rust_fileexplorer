#[cfg(feature = "git")]
use crate::errors::{FsError, Result};
#[cfg(feature = "git")]
use crate::models::Entry;
#[cfg(feature = "git")]
use std::collections::HashMap;
#[cfg(feature = "git")]
use std::path::{Path, PathBuf};
#[cfg(feature = "git")]
use std::process::Command;

#[cfg(feature = "git")]
/// Git file status
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum GitStatus {
    Untracked,
    Modified,
    Staged,
    Deleted,
    Renamed,
    Unmerged,
    Ignored,
    Clean,
}

#[cfg(feature = "git")]
impl GitStatus {
    pub fn from_porcelain_code(code: &str) -> Self {
        match code {
            "??" => GitStatus::Untracked,
            "M " | " M" | "MM" => GitStatus::Modified,
            "A " | " A" | "AM" => GitStatus::Staged,
            "D " | " D" => GitStatus::Deleted,
            "R " | " R" => GitStatus::Renamed,
            "U " | " U" | "UU" | "AA" | "DD" => GitStatus::Unmerged,
            "!!" => GitStatus::Ignored,
            _ => GitStatus::Clean,
        }
    }

    pub fn to_str(&self) -> &'static str {
        match self {
            GitStatus::Untracked => "untracked",
            GitStatus::Modified => "modified",
            GitStatus::Staged => "staged",
            GitStatus::Deleted => "deleted",
            GitStatus::Renamed => "renamed",
            GitStatus::Unmerged => "conflict",
            GitStatus::Ignored => "ignored",
            GitStatus::Clean => "clean",
        }
    }
}

#[cfg(feature = "git")]
/// Extended entry with git status
#[derive(Debug, Clone)]
pub struct GitEntry {
    pub entry: Entry,
    pub status: GitStatus,
}

#[cfg(feature = "git")]
/// Get git status for all files in a repository
pub fn get_git_status(repo_path: &Path) -> Result<HashMap<PathBuf, GitStatus>> {
    // Run git status --porcelain
    let output = Command::new("git")
        .args(["status", "--porcelain", "-uall"])
        .current_dir(repo_path)
        .output()
        .map_err(|e| FsError::IoError {
            context: "Failed to run git status command".to_string(),
            source: e,
        })?;

    if !output.status.success() {
        return Err(FsError::InvalidFormat {
            format: format!(
                "Git command failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ),
        });
    }

    let mut status_map = HashMap::new();
    let stdout = String::from_utf8_lossy(&output.stdout);

    for line in stdout.lines() {
        if line.len() < 4 {
            continue;
        }

        let status_code = &line[0..2];
        let file_path = line[3..].trim();

        // Handle renames (format: "R  old_name -> new_name")
        let file_path = if let Some(idx) = file_path.find(" -> ") {
            &file_path[idx + 4..]
        } else {
            file_path
        };

        let status = GitStatus::from_porcelain_code(status_code);
        let path = repo_path.join(file_path);

        status_map.insert(path, status);
    }

    Ok(status_map)
}

#[cfg(feature = "git")]
/// Check if a path is within a git repository
pub fn is_git_repo(path: &Path) -> bool {
    Command::new("git")
        .args(["rev-parse", "--git-dir"])
        .current_dir(path)
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

#[cfg(feature = "git")]
/// Get files changed since a specific ref (branch/commit/tag)
pub fn get_changed_since(repo_path: &Path, since_ref: &str) -> Result<Vec<PathBuf>> {
    let output = Command::new("git")
        .args(["diff", "--name-only", &format!("{}..HEAD", since_ref)])
        .current_dir(repo_path)
        .output()
        .map_err(|e| FsError::IoError {
            context: format!("Failed to get git diff since {}", since_ref),
            source: e,
        })?;

    if !output.status.success() {
        return Err(FsError::InvalidFormat {
            format: format!(
                "Git diff command failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ),
        });
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let paths = stdout
        .lines()
        .map(|line| repo_path.join(line.trim()))
        .collect();

    Ok(paths)
}

#[cfg(feature = "git")]
/// Enrich entries with git status information
pub fn enrich_with_git_status(entries: &[Entry], repo_path: &Path) -> Result<Vec<GitEntry>> {
    let status_map = get_git_status(repo_path)?;

    let git_entries = entries
        .iter()
        .map(|entry| {
            let status = status_map
                .get(&entry.path)
                .copied()
                .unwrap_or(GitStatus::Clean);

            GitEntry {
                entry: entry.clone(),
                status,
            }
        })
        .collect();

    Ok(git_entries)
}

#[cfg(test)]
#[cfg(feature = "git")]
mod tests {
    use super::*;

    #[test]
    fn test_git_status_from_code() {
        assert_eq!(GitStatus::from_porcelain_code("??"), GitStatus::Untracked);
        assert_eq!(GitStatus::from_porcelain_code("M "), GitStatus::Modified);
        assert_eq!(GitStatus::from_porcelain_code(" M"), GitStatus::Modified);
        assert_eq!(GitStatus::from_porcelain_code("A "), GitStatus::Staged);
        assert_eq!(GitStatus::from_porcelain_code("D "), GitStatus::Deleted);
        assert_eq!(GitStatus::from_porcelain_code("UU"), GitStatus::Unmerged);
    }

    #[test]
    fn test_git_status_to_str() {
        assert_eq!(GitStatus::Untracked.to_str(), "untracked");
        assert_eq!(GitStatus::Modified.to_str(), "modified");
        assert_eq!(GitStatus::Staged.to_str(), "staged");
        assert_eq!(GitStatus::Clean.to_str(), "clean");
    }
}
