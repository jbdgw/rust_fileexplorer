use crate::errors::Result;
use crate::fs::filters::Predicate;
use crate::fs::metadata::extract_entry;
use crate::models::Entry;
use ignore::WalkBuilder;
use std::path::Path;

/// Configuration for filesystem traversal
#[derive(Debug, Clone)]
pub struct TraverseConfig {
    pub max_depth: Option<usize>,
    pub follow_symlinks: bool,
    pub include_hidden: bool,
    pub respect_gitignore: bool,
    pub threads: usize,
    pub quiet: bool,
}

impl Default for TraverseConfig {
    fn default() -> Self {
        Self {
            max_depth: None,
            follow_symlinks: false,
            include_hidden: false,
            respect_gitignore: true,
            threads: 1,
            quiet: false,
        }
    }
}

/// Walk a directory tree and yield entries matching the predicate
pub fn walk<P>(root: &Path, config: &TraverseConfig, predicate: Option<&P>) -> Result<Vec<Entry>>
where
    P: Predicate + ?Sized,
{
    let mut builder = WalkBuilder::new(root);

    builder
        .follow_links(config.follow_symlinks)
        .hidden(!config.include_hidden)
        .git_ignore(config.respect_gitignore)
        .git_exclude(config.respect_gitignore);

    if let Some(depth) = config.max_depth {
        builder.max_depth(Some(depth));
    }

    let mut entries = Vec::new();

    for result in builder.build() {
        match result {
            Ok(dir_entry) => {
                let path = dir_entry.path();
                let depth = dir_entry.depth();

                match extract_entry(path, depth) {
                    Ok(entry) => {
                        // Apply predicate filter if provided
                        if let Some(pred) = predicate {
                            if pred.test(&entry) {
                                entries.push(entry);
                            }
                        } else {
                            entries.push(entry);
                        }
                    }
                    Err(e) => {
                        // Log error but continue traversal
                        if !config.quiet {
                            eprintln!("Warning: Failed to extract entry for {:?}: {}", path, e);
                        }
                    }
                }
            }
            Err(e) => {
                if !config.quiet {
                    eprintln!("Warning: Error during traversal: {}", e);
                }
            }
        }
    }

    Ok(entries)
}

/// Walk a directory tree without filtering (convenience function)
pub fn walk_no_filter(root: &Path, config: &TraverseConfig) -> Result<Vec<Entry>> {
    let mut builder = WalkBuilder::new(root);

    builder
        .follow_links(config.follow_symlinks)
        .hidden(!config.include_hidden)
        .git_ignore(config.respect_gitignore)
        .git_exclude(config.respect_gitignore);

    if let Some(depth) = config.max_depth {
        builder.max_depth(Some(depth));
    }

    let mut entries = Vec::new();

    for result in builder.build() {
        match result {
            Ok(dir_entry) => {
                let path = dir_entry.path();
                let depth = dir_entry.depth();

                match extract_entry(path, depth) {
                    Ok(entry) => {
                        entries.push(entry);
                    }
                    Err(e) => {
                        // Log error but continue traversal
                        if !config.quiet {
                            eprintln!("Warning: Failed to extract entry for {:?}: {}", path, e);
                        }
                    }
                }
            }
            Err(e) => {
                if !config.quiet {
                    eprintln!("Warning: Error during traversal: {}", e);
                }
            }
        }
    }

    Ok(entries)
}

/// Parallel walk implementation (requires "parallel" feature)
#[cfg(feature = "parallel")]
pub fn walk_parallel<P>(
    root: &Path,
    config: &TraverseConfig,
    predicate: Option<&P>,
) -> Result<Vec<Entry>>
where
    P: Predicate + Sync,
{
    use jwalk::WalkDir;
    use rayon::prelude::*;

    let mut builder = WalkDir::new(root);

    builder = builder
        .follow_links(config.follow_symlinks)
        .skip_hidden(!config.include_hidden);

    if let Some(depth) = config.max_depth {
        builder = builder.max_depth(depth);
    }

    let entries: Vec<Entry> = builder
        .into_iter()
        .par_bridge()
        .filter_map(|result| result.ok())
        .filter_map(|dir_entry| {
            let path = dir_entry.path();
            let depth = dir_entry.depth;

            match extract_entry(&path, depth) {
                Ok(entry) => {
                    if let Some(pred) = predicate {
                        if pred.test(&entry) {
                            Some(entry)
                        } else {
                            None
                        }
                    } else {
                        Some(entry)
                    }
                }
                Err(_) => None,
            }
        })
        .collect();

    Ok(entries)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::tempdir;

    #[test]
    fn test_walk_basic() {
        let dir = tempdir().unwrap();
        let file1 = dir.path().join("file1.txt");
        let file2 = dir.path().join("file2.txt");
        fs::write(&file1, "test").unwrap();
        fs::write(&file2, "test").unwrap();

        let config = TraverseConfig::default();
        let entries = walk_no_filter(dir.path(), &config).unwrap();

        // Should have at least the directory itself and two files
        assert!(entries.len() >= 3);
    }

    #[test]
    fn test_walk_max_depth() {
        let dir = tempdir().unwrap();
        let subdir = dir.path().join("subdir");
        fs::create_dir(&subdir).unwrap();
        fs::write(subdir.join("file.txt"), "test").unwrap();

        let config = TraverseConfig {
            max_depth: Some(1),
            ..Default::default()
        };

        let entries = walk_no_filter(dir.path(), &config).unwrap();

        // Should not include files in subdir
        assert!(entries.iter().all(|e| e.depth <= 1));
    }

    #[test]
    fn test_walk_hidden() {
        let dir = tempdir().unwrap();
        let hidden = dir.path().join(".hidden");
        fs::write(&hidden, "test").unwrap();

        // Without include_hidden
        let config = TraverseConfig::default();
        let entries = walk_no_filter(dir.path(), &config).unwrap();
        assert!(!entries.iter().any(|e| e.name == ".hidden"));

        // With include_hidden
        let config = TraverseConfig {
            include_hidden: true,
            ..Default::default()
        };
        let entries = walk_no_filter(dir.path(), &config).unwrap();
        assert!(entries.iter().any(|e| e.name == ".hidden"));
    }
}
