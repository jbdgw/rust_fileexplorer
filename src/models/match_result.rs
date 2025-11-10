use super::Entry;
use serde::{Deserialize, Serialize};

/// Represents a content match from grep/search operations
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ContentMatch {
    /// The entry containing the match
    pub entry: Entry,
    /// Line number where match was found (1-indexed)
    pub line_number: usize,
    /// Column where match starts (1-indexed)
    pub column: usize,
    /// The matched text/line
    pub matched_text: String,
    /// Lines before the match for context
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub context_before: Vec<String>,
    /// Lines after the match for context
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub context_after: Vec<String>,
}
