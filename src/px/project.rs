//! Project data structures and operations
//!
//! Represents a git repository project with metadata, git status,
//! and frecency tracking for intelligent ranking.

use crate::errors::{FsError, Result};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

/// Represents a project (git repository) with metadata and access tracking
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    /// Absolute path to project root
    pub path: PathBuf,

    /// Project name (directory name)
    pub name: String,

    /// Last modified time (from git or filesystem)
    #[serde(with = "chrono::serde::ts_seconds")]
    pub last_modified: DateTime<Utc>,

    /// Git status information
    pub git_status: ProjectGitStatus,

    /// Frecency score (frequency + recency)
    pub frecency_score: f64,

    /// Last accessed timestamp
    #[serde(
        with = "chrono::serde::ts_seconds_option",
        skip_serializing_if = "Option::is_none",
        default
    )]
    pub last_accessed: Option<DateTime<Utc>>,

    /// Access count for frecency calculation
    pub access_count: u32,

    /// First line of README (if exists)
    #[serde(skip_serializing_if = "Option::is_none")]
    pub readme_excerpt: Option<String>,
}

/// Git repository status information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectGitStatus {
    /// Current branch name
    pub current_branch: String,

    /// Has uncommitted changes (modified, staged, or untracked files)
    pub has_uncommitted: bool,

    /// Commits ahead of remote
    pub ahead: usize,

    /// Commits behind remote
    pub behind: usize,

    /// Most recent commit information
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_commit: Option<CommitInfo>,
}

/// Information about a git commit
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitInfo {
    /// Short commit hash
    pub hash: String,

    /// Commit message (first line)
    pub message: String,

    /// Commit author name
    pub author: String,

    /// Commit timestamp
    #[serde(with = "chrono::serde::ts_seconds")]
    pub timestamp: DateTime<Utc>,
}

impl Project {
    /// Create a Project from a git repository path
    ///
    /// Extracts git status, last commit, README excerpt, and initializes
    /// frecency tracking fields.
    pub fn from_git_repo(path: PathBuf) -> Result<Self> {
        // Validate that path is a directory
        if !path.is_dir() {
            return Err(FsError::InvalidFormat {
                format: format!("{} is not a directory", path.display()),
            });
        }

        // Check if it's a git repository
        if !crate::fs::git::is_git_repo(&path) {
            return Err(FsError::InvalidFormat {
                format: format!("{} is not a git repository", path.display()),
            });
        }

        // Extract project name from directory name
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("unknown")
            .to_string();

        // Get git status information
        let git_status = Self::get_git_status(&path)?;

        // Get last modified time (from git or filesystem)
        let last_modified = Self::get_last_modified_time(&path, &git_status)?;

        // Extract README excerpt
        let readme_excerpt = Self::extract_readme_excerpt(&path);

        Ok(Project {
            path,
            name,
            last_modified,
            git_status,
            frecency_score: 0.0,
            last_accessed: None,
            access_count: 0,
            readme_excerpt,
        })
    }

    /// Get comprehensive git status for a repository
    fn get_git_status(repo_path: &Path) -> Result<ProjectGitStatus> {
        // Get current branch
        let current_branch = Self::get_current_branch(repo_path)?;

        // Check for uncommitted changes
        let has_uncommitted = Self::has_uncommitted_changes(repo_path)?;

        // Get ahead/behind counts
        let (ahead, behind) = Self::get_ahead_behind(repo_path)?;

        // Get last commit info
        let last_commit = Self::get_last_commit(repo_path).ok();

        Ok(ProjectGitStatus {
            current_branch,
            has_uncommitted,
            ahead,
            behind,
            last_commit,
        })
    }

    /// Get the current branch name
    fn get_current_branch(repo_path: &Path) -> Result<String> {
        let output = Command::new("git")
            .args(["branch", "--show-current"])
            .current_dir(repo_path)
            .output()
            .map_err(|e| FsError::IoError {
                context: "Failed to get git branch".to_string(),
                source: e,
            })?;

        if !output.status.success() {
            return Ok("(detached)".to_string());
        }

        let branch = String::from_utf8_lossy(&output.stdout).trim().to_string();
        Ok(if branch.is_empty() {
            "(detached)".to_string()
        } else {
            branch
        })
    }

    /// Check if repository has uncommitted changes
    fn has_uncommitted_changes(repo_path: &Path) -> Result<bool> {
        let output = Command::new("git")
            .args(["status", "--porcelain"])
            .current_dir(repo_path)
            .output()
            .map_err(|e| FsError::IoError {
                context: "Failed to check git status".to_string(),
                source: e,
            })?;

        Ok(!output.stdout.is_empty())
    }

    /// Get commits ahead/behind of remote
    fn get_ahead_behind(repo_path: &Path) -> Result<(usize, usize)> {
        // Try to get upstream branch
        let output = Command::new("git")
            .args(["rev-list", "--left-right", "--count", "HEAD...@{u}"])
            .current_dir(repo_path)
            .output();

        match output {
            Ok(output) if output.status.success() => {
                let counts = String::from_utf8_lossy(&output.stdout);
                let parts: Vec<&str> = counts.trim().split_whitespace().collect();

                if parts.len() == 2 {
                    let ahead = parts[0].parse().unwrap_or(0);
                    let behind = parts[1].parse().unwrap_or(0);
                    return Ok((ahead, behind));
                }

                Ok((0, 0))
            }
            _ => Ok((0, 0)), // No upstream or error - return (0, 0)
        }
    }

    /// Get information about the last commit
    fn get_last_commit(repo_path: &Path) -> Result<CommitInfo> {
        let output = Command::new("git")
            .args([
                "log",
                "-1",
                "--format=%h|%s|%an|%at",
                "--date=unix",
            ])
            .current_dir(repo_path)
            .output()
            .map_err(|e| FsError::IoError {
                context: "Failed to get last commit".to_string(),
                source: e,
            })?;

        if !output.status.success() {
            return Err(FsError::InvalidFormat {
                format: "Failed to get last commit".to_string(),
            });
        }

        let output_str = String::from_utf8_lossy(&output.stdout);
        let parts: Vec<&str> = output_str.trim().split('|').collect();

        if parts.len() < 4 {
            return Err(FsError::InvalidFormat {
                format: "Invalid git log format".to_string(),
            });
        }

        let timestamp_secs: i64 = parts[3].parse().map_err(|_| FsError::InvalidFormat {
            format: "Invalid timestamp".to_string(),
        })?;

        Ok(CommitInfo {
            hash: parts[0].to_string(),
            message: parts[1].to_string(),
            author: parts[2].to_string(),
            timestamp: DateTime::from_timestamp(timestamp_secs, 0).unwrap_or_else(Utc::now),
        })
    }

    /// Get last modified time from git or filesystem
    fn get_last_modified_time(
        repo_path: &Path,
        git_status: &ProjectGitStatus,
    ) -> Result<DateTime<Utc>> {
        // Use last commit timestamp if available
        if let Some(commit) = &git_status.last_commit {
            return Ok(commit.timestamp);
        }

        // Fallback to filesystem modified time
        let metadata = fs::metadata(repo_path).map_err(|e| FsError::IoError {
            context: format!("Failed to read directory metadata: {}", repo_path.display()),
            source: e,
        })?;

        let modified = metadata
            .modified()
            .map_err(|e| FsError::IoError {
                context: "Failed to get modified time".to_string(),
                source: e,
            })?;

        Ok(DateTime::from(modified))
    }

    /// Extract first line of README file
    pub fn extract_readme_excerpt(repo_path: &Path) -> Option<String> {
        // Common README file names
        let readme_names = ["README.md", "README.MD", "readme.md", "README", "Readme.md"];

        for name in &readme_names {
            let readme_path = repo_path.join(name);
            if let Ok(content) = fs::read_to_string(&readme_path) {
                // Get first non-empty line, skip markdown heading markers
                for line in content.lines() {
                    let trimmed = line.trim().trim_start_matches('#').trim();
                    if !trimmed.is_empty() {
                        return Some(trimmed.to_string());
                    }
                }
            }
        }

        None
    }

    /// Update frecency score based on current access_count and last_accessed
    ///
    /// This should be called after updating access tracking fields.
    pub fn update_frecency_score(&mut self) {
        self.frecency_score = crate::px::frecency::calculate_frecency(
            self.access_count,
            self.last_accessed,
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_readme_excerpt() {
        use std::fs;
        use tempfile::TempDir;

        let temp_dir = TempDir::new().unwrap();
        let readme_path = temp_dir.path().join("README.md");

        // Test with markdown heading
        fs::write(&readme_path, "# My Project\nDescription here").unwrap();
        let excerpt = Project::extract_readme_excerpt(temp_dir.path());
        assert_eq!(excerpt, Some("My Project".to_string()));

        // Test with plain text
        fs::write(&readme_path, "Simple description").unwrap();
        let excerpt = Project::extract_readme_excerpt(temp_dir.path());
        assert_eq!(excerpt, Some("Simple description".to_string()));

        // Test with empty lines
        fs::write(&readme_path, "\n\n# Title\n").unwrap();
        let excerpt = Project::extract_readme_excerpt(temp_dir.path());
        assert_eq!(excerpt, Some("Title".to_string()));
    }
}
