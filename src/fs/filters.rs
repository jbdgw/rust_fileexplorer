use crate::errors::{FsError, Result};
use crate::models::{Entry, EntryKind, FileCategory};
use crate::util::{parse_date, parse_size};
use chrono::{DateTime, Utc};
use globset::{Glob, GlobSet, GlobSetBuilder};
use regex::Regex;

/// A predicate that can be applied to entries
pub trait Predicate: Send + Sync {
    fn test(&self, entry: &Entry) -> bool;
}

/// Combines multiple predicates with AND logic
pub struct AndPredicate {
    predicates: Vec<Box<dyn Predicate>>,
}

impl AndPredicate {
    pub fn new(predicates: Vec<Box<dyn Predicate>>) -> Self {
        Self { predicates }
    }
}

impl Predicate for AndPredicate {
    fn test(&self, entry: &Entry) -> bool {
        self.predicates.iter().all(|p| p.test(entry))
    }
}

/// Glob pattern filter
pub struct GlobFilter {
    globset: GlobSet,
}

impl GlobFilter {
    pub fn new(patterns: &[String]) -> Result<Self> {
        let mut builder = GlobSetBuilder::new();
        for pattern in patterns {
            let glob = Glob::new(pattern).map_err(|e| FsError::InvalidGlob {
                pattern: pattern.clone(),
                source: e,
            })?;
            builder.add(glob);
        }
        let globset = builder.build().map_err(|e| FsError::InvalidGlob {
            pattern: "combined".to_string(),
            source: e,
        })?;
        Ok(Self { globset })
    }
}

impl Predicate for GlobFilter {
    fn test(&self, entry: &Entry) -> bool {
        self.globset.is_match(&entry.name)
    }
}

/// Regex pattern filter
pub struct RegexFilter {
    regex: Regex,
}

impl RegexFilter {
    pub fn new(pattern: &str) -> Result<Self> {
        let regex = Regex::new(pattern).map_err(|e| FsError::InvalidRegex {
            pattern: pattern.to_string(),
            source: e,
        })?;
        Ok(Self { regex })
    }
}

impl Predicate for RegexFilter {
    fn test(&self, entry: &Entry) -> bool {
        self.regex.is_match(&entry.name)
    }
}

/// Extension filter
pub struct ExtensionFilter {
    extensions: Vec<String>,
}

impl ExtensionFilter {
    pub fn new(extensions: &[String]) -> Self {
        Self {
            extensions: extensions.iter().map(|e| e.to_lowercase()).collect(),
        }
    }
}

impl Predicate for ExtensionFilter {
    fn test(&self, entry: &Entry) -> bool {
        if let Some(ext) = entry.path.extension().and_then(|e| e.to_str()) {
            self.extensions.contains(&ext.to_lowercase())
        } else {
            false
        }
    }
}

/// Size range filter
pub struct SizeFilter {
    min: Option<u64>,
    max: Option<u64>,
}

impl SizeFilter {
    pub fn new(min: Option<&str>, max: Option<&str>) -> Result<Self> {
        let min = min.map(parse_size).transpose()?;
        let max = max.map(parse_size).transpose()?;
        Ok(Self { min, max })
    }
}

impl Predicate for SizeFilter {
    fn test(&self, entry: &Entry) -> bool {
        if entry.kind == EntryKind::Dir {
            return true; // Skip dirs for size filtering
        }

        if let Some(min) = self.min {
            if entry.size < min {
                return false;
            }
        }

        if let Some(max) = self.max {
            if entry.size > max {
                return false;
            }
        }

        true
    }
}

/// Date range filter
pub struct DateFilter {
    after: Option<DateTime<Utc>>,
    before: Option<DateTime<Utc>>,
}

impl DateFilter {
    pub fn new(after: Option<&str>, before: Option<&str>) -> Result<Self> {
        let after = after.map(parse_date).transpose()?;
        let before = before.map(parse_date).transpose()?;
        Ok(Self { after, before })
    }
}

impl Predicate for DateFilter {
    fn test(&self, entry: &Entry) -> bool {
        if let Some(after) = &self.after {
            if entry.mtime < *after {
                return false;
            }
        }

        if let Some(before) = &self.before {
            if entry.mtime > *before {
                return false;
            }
        }

        true
    }
}

/// Kind filter
pub struct KindFilter {
    kinds: Vec<EntryKind>,
}

impl KindFilter {
    pub fn new(kinds: &[EntryKind]) -> Self {
        Self {
            kinds: kinds.to_vec(),
        }
    }
}

impl Predicate for KindFilter {
    fn test(&self, entry: &Entry) -> bool {
        self.kinds.contains(&entry.kind)
    }
}

/// Category filter - matches files by smart categorization
pub struct CategoryFilter {
    category: String,
}

impl CategoryFilter {
    pub fn new(category: &str) -> Self {
        Self {
            category: category.to_lowercase(),
        }
    }

    /// Check if a FileCategory matches the filter
    fn matches_category(&self, file_category: &FileCategory) -> bool {
        match self.category.as_str() {
            "source" => matches!(file_category, FileCategory::Source { .. }),
            "build" => matches!(file_category, FileCategory::Build),
            "config" => matches!(file_category, FileCategory::Config { .. }),
            "docs" | "documentation" => matches!(file_category, FileCategory::Documentation),
            "media" => matches!(file_category, FileCategory::Media { .. }),
            "image" => matches!(
                file_category,
                FileCategory::Media {
                    media_type: crate::models::MediaType::Image
                }
            ),
            "video" => matches!(
                file_category,
                FileCategory::Media {
                    media_type: crate::models::MediaType::Video
                }
            ),
            "audio" => matches!(
                file_category,
                FileCategory::Media {
                    media_type: crate::models::MediaType::Audio
                }
            ),
            "data" => matches!(file_category, FileCategory::Data { .. }),
            "archive" => matches!(file_category, FileCategory::Archive),
            "executable" | "exec" => matches!(file_category, FileCategory::Executable),
            _ => false,
        }
    }
}

impl Predicate for CategoryFilter {
    fn test(&self, entry: &Entry) -> bool {
        // Only categorize files, not directories
        if entry.kind != EntryKind::File {
            return false;
        }

        // Get file extension
        if let Some(ext) = entry.path.extension().and_then(|e| e.to_str()) {
            let category = FileCategory::from_extension(ext);
            self.matches_category(&category)
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn make_test_entry(name: &str, size: u64, kind: EntryKind) -> Entry {
        use chrono::Utc;

        Entry {
            path: PathBuf::from(name),
            name: name.to_string(),
            size,
            kind,
            mtime: Utc::now(),
            perms: None,
            owner: None,
            depth: 0,
        }
    }

    #[test]
    fn test_glob_filter() {
        let filter = GlobFilter::new(&["*.rs".to_string()]).unwrap();
        assert!(filter.test(&make_test_entry("main.rs", 100, EntryKind::File)));
        assert!(!filter.test(&make_test_entry("main.txt", 100, EntryKind::File)));
    }

    #[test]
    fn test_regex_filter() {
        let filter = RegexFilter::new(r"^test_.*\.rs$").unwrap();
        assert!(filter.test(&make_test_entry("test_foo.rs", 100, EntryKind::File)));
        assert!(!filter.test(&make_test_entry("main.rs", 100, EntryKind::File)));
    }

    #[test]
    fn test_extension_filter() {
        let filter = ExtensionFilter::new(&["rs".to_string(), "toml".to_string()]);
        assert!(filter.test(&make_test_entry("main.rs", 100, EntryKind::File)));
        assert!(filter.test(&make_test_entry("Cargo.toml", 100, EntryKind::File)));
        assert!(!filter.test(&make_test_entry("readme.md", 100, EntryKind::File)));
    }

    #[test]
    fn test_size_filter() {
        let filter = SizeFilter::new(Some("1KB"), Some("10KB")).unwrap();
        assert!(!filter.test(&make_test_entry("small.txt", 500, EntryKind::File)));
        assert!(filter.test(&make_test_entry("medium.txt", 5000, EntryKind::File)));
        assert!(!filter.test(&make_test_entry("large.txt", 20000, EntryKind::File)));
    }

    #[test]
    fn test_kind_filter() {
        let filter = KindFilter::new(&[EntryKind::File]);
        assert!(filter.test(&make_test_entry("file.txt", 100, EntryKind::File)));
        assert!(!filter.test(&make_test_entry("dir", 0, EntryKind::Dir)));
    }

    #[test]
    fn test_category_filter_source() {
        let filter = CategoryFilter::new("source");
        assert!(filter.test(&make_test_entry("main.rs", 100, EntryKind::File)));
        assert!(filter.test(&make_test_entry("app.py", 100, EntryKind::File)));
        assert!(!filter.test(&make_test_entry("image.png", 100, EntryKind::File)));
    }

    #[test]
    fn test_category_filter_media() {
        let filter = CategoryFilter::new("image");
        assert!(filter.test(&make_test_entry("photo.jpg", 100, EntryKind::File)));
        assert!(filter.test(&make_test_entry("icon.png", 100, EntryKind::File)));
        assert!(!filter.test(&make_test_entry("video.mp4", 100, EntryKind::File)));
    }

    #[test]
    fn test_category_filter_config() {
        let filter = CategoryFilter::new("config");
        assert!(filter.test(&make_test_entry("Cargo.toml", 100, EntryKind::File)));
        assert!(filter.test(&make_test_entry("config.yaml", 100, EntryKind::File)));
        assert!(!filter.test(&make_test_entry("main.rs", 100, EntryKind::File)));
    }
}
