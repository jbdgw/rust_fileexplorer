#[cfg(feature = "grep")]
use crate::errors::{FsError, Result};
#[cfg(feature = "grep")]
use crate::models::{ContentMatch, Entry};
#[cfg(feature = "grep")]
use grep_matcher::Matcher;
#[cfg(feature = "grep")]
use grep_regex::RegexMatcherBuilder;
#[cfg(feature = "grep")]
use grep_searcher::{sinks, BinaryDetection, SearcherBuilder};
#[cfg(feature = "grep")]
use std::fs::File;
#[cfg(feature = "grep")]
use std::io::{BufRead, BufReader};
#[cfg(feature = "grep")]
use std::path::Path;

#[cfg(feature = "grep")]
pub struct ContentSearcher {
    matcher: grep_regex::RegexMatcher,
    context_lines: usize,
    #[allow(dead_code)]
    line_numbers: bool,
}

#[cfg(feature = "grep")]
impl ContentSearcher {
    /// Create a new content searcher
    pub fn new(
        pattern: &str,
        is_regex: bool,
        case_insensitive: bool,
        context_lines: usize,
        line_numbers: bool,
    ) -> Result<Self> {
        let pattern_to_use = if is_regex {
            pattern.to_string()
        } else {
            regex::escape(pattern)
        };

        let matcher = RegexMatcherBuilder::new()
            .case_insensitive(case_insensitive)
            .build(&pattern_to_use)
            .map_err(|e| FsError::InvalidFormat {
                format: format!("Invalid regex pattern '{}': {}", pattern, e),
            })?;

        Ok(Self {
            matcher,
            context_lines,
            line_numbers,
        })
    }

    /// Search a single file for matches
    pub fn search_file(&self, entry: &Entry) -> Result<Vec<ContentMatch>> {
        let path = &entry.path;

        // Skip directories and symlinks
        if !path.is_file() {
            return Ok(Vec::new());
        }

        let mut matches = Vec::new();
        let mut searcher = SearcherBuilder::new()
            .binary_detection(BinaryDetection::quit(b'\x00'))
            .line_number(true)
            .build();

        let result = searcher.search_path(
            &self.matcher,
            path,
            sinks::UTF8(|lnum, line| {
                // Extract context if needed
                let (context_before, context_after) = if self.context_lines > 0 {
                    self.extract_context(path, lnum as usize, self.context_lines)
                        .unwrap_or_else(|_| (Vec::new(), Vec::new()))
                } else {
                    (Vec::new(), Vec::new())
                };

                // Find the match column
                let column = self
                    .matcher
                    .find(line.as_bytes())
                    .ok()
                    .and_then(|m| m.map(|m| m.start() + 1))
                    .unwrap_or(1);

                matches.push(ContentMatch {
                    entry: entry.clone(),
                    line_number: lnum as usize,
                    column,
                    matched_text: line.trim_end().to_string(),
                    context_before,
                    context_after,
                });

                Ok(true)
            }),
        );

        // Ignore binary file errors and permission denied
        match result {
            Ok(_) => Ok(matches),
            Err(e) => {
                if e.to_string().contains("binary") || e.to_string().contains("Permission denied") {
                    Ok(Vec::new())
                } else {
                    Err(FsError::Io(std::io::Error::other(e)))
                }
            }
        }
    }

    /// Extract context lines around a match
    fn extract_context(
        &self,
        path: &Path,
        match_line: usize,
        context: usize,
    ) -> Result<(Vec<String>, Vec<String>)> {
        let file = File::open(path)?;
        let reader = BufReader::new(file);

        let mut before = Vec::new();
        let mut after = Vec::new();
        let mut current_line = 1;

        for line in reader.lines() {
            let line = line?;

            if current_line < match_line && current_line >= match_line.saturating_sub(context) {
                before.push(line);
            } else if current_line > match_line && current_line <= match_line + context {
                after.push(line);
            } else if current_line > match_line + context {
                break;
            }

            current_line += 1;
        }

        Ok((before, after))
    }
}

#[cfg(feature = "grep")]
/// Search multiple files in parallel
pub fn search_files(entries: &[Entry], searcher: &ContentSearcher) -> Result<Vec<ContentMatch>> {
    #[cfg(feature = "parallel")]
    {
        use rayon::prelude::*;
        let matches: Vec<ContentMatch> = entries
            .par_iter()
            .filter_map(|entry| searcher.search_file(entry).ok())
            .flatten()
            .collect();
        Ok(matches)
    }

    #[cfg(not(feature = "parallel"))]
    {
        let mut matches = Vec::new();
        for entry in entries {
            if let Ok(mut entry_matches) = searcher.search_file(entry) {
                matches.append(&mut entry_matches);
            }
        }
        Ok(matches)
    }
}

#[cfg(test)]
#[cfg(feature = "grep")]
mod tests {
    use super::*;
    use crate::models::{Entry, EntryKind};
    use chrono::Utc;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::tempdir;

    fn make_test_entry(path: PathBuf) -> Entry {
        Entry {
            path: path.clone(),
            name: path.file_name().unwrap().to_string_lossy().to_string(),
            size: 0,
            kind: EntryKind::File,
            mtime: Utc::now(),
            perms: None,
            owner: None,
            depth: 0,
        }
    }

    #[test]
    fn test_literal_search() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        fs::write(&file_path, "Hello World\nThis is a test\nHello again").unwrap();

        let entry = make_test_entry(file_path);
        let searcher = ContentSearcher::new("Hello", false, false, 0, false).unwrap();
        let matches = searcher.search_file(&entry).unwrap();

        assert_eq!(matches.len(), 2);
        assert_eq!(matches[0].line_number, 1);
        assert_eq!(matches[1].line_number, 3);
    }

    #[test]
    fn test_regex_search() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        fs::write(&file_path, "test123\ntest456\nabc789").unwrap();

        let entry = make_test_entry(file_path);
        let searcher = ContentSearcher::new(r"test\d+", true, false, 0, false).unwrap();
        let matches = searcher.search_file(&entry).unwrap();

        assert_eq!(matches.len(), 2);
    }

    #[test]
    fn test_case_insensitive() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        fs::write(&file_path, "Hello\nhello\nHELLO").unwrap();

        let entry = make_test_entry(file_path);
        let searcher = ContentSearcher::new("hello", false, true, 0, false).unwrap();
        let matches = searcher.search_file(&entry).unwrap();

        assert_eq!(matches.len(), 3);
    }

    #[test]
    fn test_context_lines() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        fs::write(&file_path, "line1\nline2\nmatch\nline4\nline5").unwrap();

        let entry = make_test_entry(file_path);
        let searcher = ContentSearcher::new("match", false, false, 1, false).unwrap();
        let matches = searcher.search_file(&entry).unwrap();

        assert_eq!(matches.len(), 1);
        assert_eq!(matches[0].context_before.len(), 1);
        assert_eq!(matches[0].context_after.len(), 1);
        assert_eq!(matches[0].context_before[0], "line2");
        assert_eq!(matches[0].context_after[0], "line4");
    }
}
