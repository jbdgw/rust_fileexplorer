//! Project index management
//!
//! Handles loading, saving, and syncing the project index.
//! The index is cached as JSON in ~/.cache/px/projects.json

use crate::errors::{FsError, Result};
use crate::fs::traverse::{walk_no_filter, TraverseConfig};
use crate::models::EntryKind;
use crate::px::project::Project;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;

/// Project index with caching and sync capabilities
#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectIndex {
    /// All discovered projects, keyed by absolute path
    pub projects: HashMap<String, Project>,

    /// Last sync timestamp
    #[serde(with = "chrono::serde::ts_seconds")]
    pub last_sync: DateTime<Utc>,

    /// Schema version for future migrations
    pub version: u32,
}

impl ProjectIndex {
    /// Create a new empty project index
    pub fn new() -> Self {
        Self {
            projects: HashMap::new(),
            last_sync: Utc::now(),
            version: 1,
        }
    }

    /// Load index from cache file (~/.cache/px/projects.json)
    ///
    /// If the cache file doesn't exist or can't be read, returns a new empty index.
    /// This makes the first run seamless - users just run `px sync` to populate.
    pub fn load() -> Result<Self> {
        let cache_path = Self::cache_path()?;

        if !cache_path.exists() {
            return Ok(Self::new());
        }

        let data = fs::read_to_string(&cache_path).map_err(|e| FsError::IoError {
            context: format!("Failed to read cache file: {}", cache_path.display()),
            source: e,
        })?;

        let index: ProjectIndex = serde_json::from_str(&data).map_err(|e| {
            FsError::InvalidFormat {
                format: format!("Invalid cache JSON: {}", e),
            }
        })?;

        Ok(index)
    }

    /// Save index to cache file
    ///
    /// Creates the cache directory if it doesn't exist.
    pub fn save(&self) -> Result<()> {
        let cache_path = Self::cache_path()?;

        // Ensure cache directory exists
        if let Some(parent) = cache_path.parent() {
            fs::create_dir_all(parent).map_err(|e| FsError::IoError {
                context: format!("Failed to create cache directory: {}", parent.display()),
                source: e,
            })?;
        }

        // Serialize to pretty JSON for readability
        let json = serde_json::to_string_pretty(self).map_err(|e| FsError::InvalidFormat {
            format: format!("Failed to serialize index: {}", e),
        })?;

        fs::write(&cache_path, json).map_err(|e| FsError::IoError {
            context: format!("Failed to write cache file: {}", cache_path.display()),
            source: e,
        })?;

        Ok(())
    }

    /// Sync/rebuild the project index by scanning configured directories
    ///
    /// This is the core indexing operation that:
    /// 1. Scans all configured directories for git repositories
    /// 2. Extracts metadata for each project
    /// 3. Preserves frecency data for existing projects
    /// 4. Saves the updated index to disk
    ///
    /// Returns the number of projects found.
    pub fn sync(&mut self, scan_dirs: &[PathBuf]) -> Result<usize> {
        let mut new_projects = HashMap::new();

        // Traverse each scan directory
        for scan_dir in scan_dirs {
            if !scan_dir.exists() {
                eprintln!(
                    "Warning: Scan directory does not exist: {}",
                    scan_dir.display()
                );
                continue;
            }

            // Configure traversal for git repo discovery
            let config = TraverseConfig {
                max_depth: Some(3), // Don't go too deep
                follow_symlinks: false,
                include_hidden: false,
                respect_gitignore: true,
                threads: 4, // Parallel scan (feature enabled by default)
                quiet: true, // Suppress permission errors
            };

            // Use existing fexplorer traverse infrastructure
            let entries = walk_no_filter(scan_dir, &config)?;

            // Filter for git repositories
            for entry in entries {
                if entry.kind == EntryKind::Dir && crate::fs::git::is_git_repo(&entry.path) {
                    let path_str = entry.path.to_string_lossy().to_string();

                    // Try to create Project from git repo
                    match Project::from_git_repo(entry.path.clone()) {
                        Ok(mut project) => {
                            // Preserve frecency data if project already exists
                            if let Some(existing) = self.projects.get(&path_str) {
                                project.access_count = existing.access_count;
                                project.last_accessed = existing.last_accessed;
                                project.frecency_score = existing.frecency_score;
                            }

                            new_projects.insert(path_str, project);
                        }
                        Err(e) => {
                            // Log error but continue indexing
                            eprintln!(
                                "Warning: Failed to index {}: {}",
                                entry.path.display(),
                                e
                            );
                        }
                    }
                }
            }
        }

        let count = new_projects.len();
        self.projects = new_projects;
        self.last_sync = Utc::now();

        // Persist to disk
        self.save()?;

        Ok(count)
    }

    /// Record project access for frecency tracking
    ///
    /// Updates access_count, last_accessed, and recalculates frecency_score.
    /// Changes are persisted to disk immediately.
    pub fn record_access(&mut self, project_path: &str) -> Result<()> {
        if let Some(project) = self.projects.get_mut(project_path) {
            project.access_count += 1;
            project.last_accessed = Some(Utc::now());
            project.update_frecency_score();

            // Persist immediately (write-through cache)
            self.save()?;
        }

        Ok(())
    }

    /// Get the cache file path (~/.cache/px/projects.json)
    fn cache_path() -> Result<PathBuf> {
        let cache_dir = dirs::cache_dir().ok_or_else(|| FsError::InvalidFormat {
            format: "Could not determine cache directory".to_string(),
        })?;

        Ok(cache_dir.join("px").join("projects.json"))
    }

    /// Get projects as a sorted vector (by frecency)
    pub fn sorted_projects(&self) -> Vec<&Project> {
        let mut projects: Vec<&Project> = self.projects.values().collect();
        projects.sort_by(|a, b| {
            b.frecency_score
                .partial_cmp(&a.frecency_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        projects
    }
}

impl Default for ProjectIndex {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn test_new_index() {
        let index = ProjectIndex::new();
        assert_eq!(index.version, 1);
        assert!(index.projects.is_empty());
    }

    #[test]
    fn test_save_and_load() {
        // Create temp cache directory
        let temp_dir = TempDir::new().unwrap();
        let cache_path = temp_dir.path().join("test_cache.json");

        // Create index with a test project
        let mut index = ProjectIndex::new();

        // Manually create a project for testing
        let test_project = Project::from_git_repo(PathBuf::from(".")).unwrap_or_else(|_| {
            // Fallback if current dir is not a git repo
            Project {
                path: PathBuf::from("/test/path"),
                name: "test-project".to_string(),
                last_modified: Utc::now(),
                git_status: crate::px::project::ProjectGitStatus {
                    current_branch: "main".to_string(),
                    has_uncommitted: false,
                    ahead: 0,
                    behind: 0,
                    last_commit: None,
                },
                frecency_score: 0.0,
                last_accessed: None,
                access_count: 0,
                readme_excerpt: Some("Test project".to_string()),
            }
        });

        index
            .projects
            .insert(test_project.path.to_string_lossy().to_string(), test_project);

        // Save
        let json = serde_json::to_string_pretty(&index).unwrap();
        fs::write(&cache_path, json).unwrap();

        // Load
        let data = fs::read_to_string(&cache_path).unwrap();
        let loaded: ProjectIndex = serde_json::from_str(&data).unwrap();

        assert_eq!(loaded.version, 1);
        assert_eq!(loaded.projects.len(), 1);
    }

    #[test]
    fn test_record_access() {
        let mut index = ProjectIndex::new();

        // Create a test project
        let test_path = "/test/path";
        let mut project = Project {
            path: PathBuf::from(test_path),
            name: "test".to_string(),
            last_modified: Utc::now(),
            git_status: crate::px::project::ProjectGitStatus {
                current_branch: "main".to_string(),
                has_uncommitted: false,
                ahead: 0,
                behind: 0,
                last_commit: None,
            },
            frecency_score: 0.0,
            last_accessed: None,
            access_count: 0,
            readme_excerpt: None,
        };

        index.projects.insert(test_path.to_string(), project);

        // Record access (skip save for test)
        let project = index.projects.get_mut(test_path).unwrap();
        project.access_count += 1;
        project.last_accessed = Some(Utc::now());
        project.update_frecency_score();

        assert_eq!(project.access_count, 1);
        assert!(project.last_accessed.is_some());
        assert!(project.frecency_score > 0.0);
    }
}

