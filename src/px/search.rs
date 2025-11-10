//! Fuzzy search for projects
//!
//! Provides fuzzy matching on project names and paths,
//! combined with frecency scoring for intelligent ranking.

use crate::px::project::Project;
use fuzzy_matcher::skim::SkimMatcherV2;
use fuzzy_matcher::FuzzyMatcher;

/// Project fuzzy searcher with integrated frecency ranking
pub struct ProjectSearcher {
    matcher: SkimMatcherV2,
}

impl ProjectSearcher {
    /// Create a new project searcher with default configuration
    pub fn new() -> Self {
        Self {
            matcher: SkimMatcherV2::default(),
        }
    }

    /// Search projects by fuzzy matching name/path
    ///
    /// Returns projects sorted by combined fuzzy match score + frecency.
    ///
    /// The ranking formula is:
    /// - Fuzzy match score (0-100): how well the query matches the project
    /// - Frecency score (0-∞): how frequently and recently the project was accessed
    /// - Combined: fuzzy_score * 0.7 + frecency_score * 0.3
    ///
    /// This prioritizes good matches while still surfacing frequently-used projects.
    pub fn search<'a>(&self, projects: &'a [Project], query: &str) -> Vec<&'a Project> {
        if query.trim().is_empty() {
            // No query - return all sorted by frecency
            return self.sort_by_frecency(projects);
        }

        let mut matches: Vec<(&Project, i64)> = projects
            .iter()
            .filter_map(|project| {
                // Try matching against both name and path
                let name_score = self.matcher.fuzzy_match(&project.name, query).unwrap_or(0);
                let path_score = self
                    .matcher
                    .fuzzy_match(&project.path.to_string_lossy(), query)
                    .unwrap_or(0);

                // Take the better of the two scores
                let fuzzy_score = name_score.max(path_score);

                if fuzzy_score > 0 {
                    // Combine fuzzy score with frecency
                    // Fuzzy scores are typically 0-100, frecency can be 0-150+
                    // Weight fuzzy matching more heavily (70%) but keep frecency influence (30%)
                    let combined_score =
                        (fuzzy_score as f64 * 0.7) + (project.frecency_score * 0.3);

                    Some((project, combined_score as i64))
                } else {
                    None
                }
            })
            .collect();

        // Sort by combined score (highest first)
        matches.sort_by(|a, b| b.1.cmp(&a.1));

        matches.into_iter().map(|(project, _)| project).collect()
    }

    /// Search for exact match (case-insensitive contains)
    ///
    /// Useful for when the user knows exactly what they want
    pub fn exact_search<'a>(&self, projects: &'a [Project], query: &str) -> Vec<&'a Project> {
        let query_lower = query.to_lowercase();
        let mut matches: Vec<&Project> = projects
            .iter()
            .filter(|p| {
                p.name.to_lowercase().contains(&query_lower)
                    || p.path
                        .to_string_lossy()
                        .to_lowercase()
                        .contains(&query_lower)
            })
            .collect();

        // Sort by frecency
        matches.sort_by(|a, b| {
            b.frecency_score
                .partial_cmp(&a.frecency_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });

        matches
    }

    /// Helper to sort projects by frecency only
    fn sort_by_frecency<'a>(&self, projects: &'a [Project]) -> Vec<&'a Project> {
        let mut sorted: Vec<&Project> = projects.iter().collect();
        sorted.sort_by(|a, b| {
            b.frecency_score
                .partial_cmp(&a.frecency_score)
                .unwrap_or(std::cmp::Ordering::Equal)
        });
        sorted
    }
}

impl Default for ProjectSearcher {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::px::project::{Project, ProjectGitStatus};
    use chrono::Utc;
    use std::path::PathBuf;

    fn create_test_project(name: &str, frecency: f64) -> Project {
        Project {
            path: PathBuf::from(format!("/test/{}", name)),
            name: name.to_string(),
            last_modified: Utc::now(),
            git_status: ProjectGitStatus {
                current_branch: "main".to_string(),
                has_uncommitted: false,
                ahead: 0,
                behind: 0,
                last_commit: None,
            },
            frecency_score: frecency,
            last_accessed: None,
            access_count: 0,
            readme_excerpt: None,
        }
    }

    #[test]
    fn test_empty_query() {
        let searcher = ProjectSearcher::new();
        let projects = vec![
            create_test_project("low-frecency", 10.0),
            create_test_project("high-frecency", 100.0),
        ];

        let results = searcher.search(&projects, "");
        assert_eq!(results.len(), 2);
        // Should be sorted by frecency
        assert_eq!(results[0].name, "high-frecency");
    }

    #[test]
    fn test_fuzzy_match() {
        let searcher = ProjectSearcher::new();
        let projects = vec![
            create_test_project("rust-filesearch", 50.0),
            create_test_project("python-script", 50.0),
            create_test_project("rust-analyzer", 50.0),
        ];

        let results = searcher.search(&projects, "rust");
        // Should match "rust-filesearch" and "rust-analyzer"
        assert!(results.len() >= 2);
        assert!(results.iter().any(|p| p.name.contains("rust")));
    }

    #[test]
    fn test_frecency_influences_ranking() {
        let searcher = ProjectSearcher::new();
        let projects = vec![
            create_test_project("rust-project", 10.0),   // Low frecency
            create_test_project("rust-awesome", 100.0),  // High frecency
        ];

        let results = searcher.search(&projects, "rust");
        assert_eq!(results.len(), 2);
        // Both match "rust", but high-frecency should rank higher
        assert_eq!(results[0].name, "rust-awesome");
    }

    #[test]
    fn test_exact_search() {
        let searcher = ProjectSearcher::new();
        let projects = vec![
            create_test_project("whatsgood-homepage", 50.0),
            create_test_project("rust-filesearch", 50.0),
        ];

        let results = searcher.exact_search(&projects, "whatsgood");
        assert_eq!(results.len(), 1);
        assert_eq!(results[0].name, "whatsgood-homepage");
    }
}

