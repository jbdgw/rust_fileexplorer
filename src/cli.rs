use crate::errors::{FsError, Result};
use crate::models::{Column, EntryKind, OutputFormat, SortKey, SortOrder};
use clap::{Parser, Subcommand, ValueEnum};
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(name = "fexplorer")]
#[command(author, version, about = "A fast, modern file system explorer and toolkit", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,

    /// Disable colored output
    #[arg(long, global = true)]
    pub no_color: bool,

    /// Quiet mode (suppress warnings)
    #[arg(long, short = 'q', global = true)]
    pub quiet: bool,

    /// Verbose mode (show detailed output)
    #[arg(long, short = 'v', global = true)]
    pub verbose: bool,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    /// List entries with metadata and sorting
    #[command(visible_alias = "ls")]
    List {
        /// Root path to list
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Sort by key
        #[arg(long, value_name = "KEY")]
        sort: Option<String>,

        /// Sort order (asc or desc)
        #[arg(long, default_value = "asc")]
        order: String,

        /// Show directories first
        #[arg(long)]
        dirs_first: bool,

        #[command(flatten)]
        common: CommonArgs,
    },

    /// Display directory tree with ASCII art
    Tree {
        /// Root path to display
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Show directories first
        #[arg(long)]
        dirs_first: bool,

        #[command(flatten)]
        common: CommonArgs,
    },

    /// Find files matching criteria
    Find {
        /// Root path to search
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Name glob patterns (repeatable)
        #[arg(long = "name")]
        names: Vec<String>,

        /// Regex pattern for names
        #[arg(long)]
        regex: Option<String>,

        /// File extensions (comma-separated)
        #[arg(long, value_delimiter = ',')]
        ext: Vec<String>,

        /// Minimum size (e.g., 10KB, 2MiB)
        #[arg(long)]
        min_size: Option<String>,

        /// Maximum size (e.g., 10MB, 2GiB)
        #[arg(long)]
        max_size: Option<String>,

        /// Modified after date (ISO8601 or YYYY-MM-DD)
        #[arg(long)]
        after: Option<String>,

        /// Modified before date (ISO8601 or YYYY-MM-DD)
        #[arg(long)]
        before: Option<String>,

        /// Filter by kind (file, dir, symlink)
        #[arg(long, value_delimiter = ',')]
        kind: Vec<String>,

        /// Filter by category (source, build, config, docs, media, data, archive, executable)
        #[arg(long)]
        category: Option<String>,

        #[command(flatten)]
        common: CommonArgs,
    },

    /// Calculate and display sizes
    Size {
        /// Root path to analyze
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Show top N entries by size
        #[arg(long)]
        top: Option<usize>,

        /// Aggregate directory sizes
        #[arg(long)]
        aggregate: bool,

        /// Display like 'du' command
        #[arg(long)]
        du: bool,

        #[command(flatten)]
        common: CommonArgs,
    },

    /// Search file contents (grep functionality)
    #[cfg(feature = "grep")]
    Grep {
        /// Root path to search
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Pattern to search for
        #[arg(value_name = "PATTERN")]
        pattern: String,

        /// Use regex matching (default is literal)
        #[arg(long, short = 'e')]
        regex: bool,

        /// Case insensitive search
        #[arg(long, short = 'i')]
        case_insensitive: bool,

        /// File extensions to search (comma-separated)
        #[arg(long, value_delimiter = ',')]
        ext: Vec<String>,

        /// Number of context lines to show
        #[arg(long, short = 'C', default_value = "0")]
        context: usize,

        /// Show line numbers
        #[arg(long, short = 'n')]
        line_numbers: bool,

        #[command(flatten)]
        common: CommonArgs,
    },

    /// Find duplicate files by content hash
    #[cfg(feature = "dedup")]
    Duplicates {
        /// Root path to analyze
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Minimum file size to check (e.g., 1MB)
        #[arg(long, default_value = "0")]
        min_size: String,

        /// Show wasted space summary
        #[arg(long)]
        summary: bool,

        #[command(flatten)]
        common: CommonArgs,
    },

    /// Git integration - show files with git status
    #[cfg(feature = "git")]
    Git {
        /// Root path (must be in a git repository)
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Filter by git status
        #[arg(long, value_enum)]
        status: Option<GitStatusFilter>,

        /// Show files changed since ref (branch/commit/tag)
        #[arg(long)]
        since: Option<String>,

        #[command(flatten)]
        common: CommonArgs,
    },

    /// Interactive file explorer (TUI mode)
    #[cfg(feature = "tui")]
    #[command(visible_alias = "tui")]
    Interactive {
        /// Root path to explore
        #[arg(default_value = ".")]
        path: PathBuf,
    },

    /// Save a filesystem snapshot for trend analysis
    #[cfg(feature = "trends")]
    Snapshot {
        /// Root path to snapshot
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Description for this snapshot
        #[arg(long)]
        description: Option<String>,
    },

    /// Analyze filesystem trends over time
    #[cfg(feature = "trends")]
    Trends {
        /// Root path to analyze
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Show trends since date
        #[arg(long)]
        since: Option<String>,

        /// Display as ASCII chart
        #[arg(long)]
        chart: bool,
    },

    /// Generate shell completions
    Completions {
        /// Shell to generate completions for
        #[arg(value_enum)]
        shell: Shell,
    },

    /// Manage saved query profiles
    Profiles {
        #[command(subcommand)]
        command: ProfileCommand,
    },

    /// Run a saved query profile
    Run {
        /// Profile name to execute
        profile: String,

        /// Override the path argument
        #[arg(long)]
        path: Option<PathBuf>,

        /// Additional arguments to override profile settings
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        args: Vec<String>,
    },

    /// Watch for filesystem changes (requires watch feature)
    #[cfg(feature = "watch")]
    Watch {
        /// Root path to watch
        #[arg(default_value = ".")]
        path: PathBuf,

        /// Events to monitor (comma-separated: create,modify,remove)
        #[arg(long, value_delimiter = ',')]
        events: Vec<String>,

        /// Output format (ndjson recommended for watch)
        #[arg(long, default_value = "ndjson")]
        format: String,
    },

    /// Manage plugins (requires plugins feature)
    #[cfg(feature = "plugins")]
    Plugins {
        #[command(subcommand)]
        command: PluginCommand,
    },
}

/// Profile subcommands
#[derive(Subcommand, Debug)]
pub enum ProfileCommand {
    /// List all saved profiles
    List,

    /// Show details of a specific profile
    Show {
        /// Profile name
        name: String,
    },

    /// Initialize config with example profiles
    Init,
}

/// Plugin subcommands
#[derive(Subcommand, Debug)]
#[cfg(feature = "plugins")]
pub enum PluginCommand {
    /// List installed plugins
    List,

    /// Enable a plugin
    Enable {
        /// Plugin name
        name: String,
    },

    /// Disable a plugin
    Disable {
        /// Plugin name
        name: String,
    },
}

/// Git status filters
#[derive(Debug, Clone, Copy, ValueEnum)]
#[cfg(feature = "git")]
pub enum GitStatusFilter {
    Untracked,
    Modified,
    Staged,
    Conflict,
    Clean,
    Ignored,
}

/// Shell types for completion generation
#[derive(Debug, Clone, Copy, ValueEnum)]
pub enum Shell {
    Bash,
    Zsh,
    Fish,
    Powershell,
    Elvish,
}

/// Common arguments shared across commands
#[derive(Parser, Debug, Clone)]
pub struct CommonArgs {
    /// Maximum depth to traverse
    #[arg(long)]
    pub max_depth: Option<usize>,

    /// Include hidden files
    #[arg(long)]
    pub hidden: bool,

    /// Disable gitignore filtering
    #[arg(long)]
    pub no_gitignore: bool,

    /// Follow symbolic links
    #[arg(long)]
    pub follow_symlinks: bool,

    /// Output format (pretty, json, ndjson, csv)
    #[arg(long, default_value = "pretty")]
    pub format: String,

    /// Columns to display (comma-separated)
    #[arg(long, value_delimiter = ',')]
    pub columns: Vec<String>,

    /// Number of threads for parallel traversal
    #[cfg(feature = "parallel")]
    #[arg(long, default_value = "4")]
    pub threads: usize,

    /// Show progress bar
    #[cfg(feature = "progress")]
    #[arg(long)]
    pub progress: bool,

    /// Export using template (markdown, html)
    #[cfg(feature = "templates")]
    #[arg(long)]
    pub template: Option<String>,
}

impl Default for CommonArgs {
    fn default() -> Self {
        Self {
            max_depth: None,
            hidden: false,
            no_gitignore: false,
            follow_symlinks: false,
            format: "pretty".to_string(),
            columns: Vec::new(),
            #[cfg(feature = "parallel")]
            threads: 4,
            #[cfg(feature = "progress")]
            progress: false,
            #[cfg(feature = "templates")]
            template: None,
        }
    }
}

impl CommonArgs {
    pub fn output_format(&self) -> Result<OutputFormat> {
        OutputFormat::from_str(&self.format).ok_or_else(|| FsError::InvalidFormat {
            format: self.format.clone(),
        })
    }

    pub fn columns(&self) -> Result<Vec<Column>> {
        if self.columns.is_empty() {
            // Default columns
            return Ok(vec![
                Column::Path,
                Column::Size,
                Column::Mtime,
                Column::Kind,
            ]);
        }

        self.columns
            .iter()
            .map(|s| {
                Column::from_str(s).ok_or_else(|| FsError::InvalidFormat {
                    format: format!("Invalid column: {}", s),
                })
            })
            .collect()
    }
}

// Helper functions (kept for backwards compatibility)

pub fn parse_sort_key(s: &str) -> Result<SortKey> {
    match s.to_lowercase().as_str() {
        "name" => Ok(SortKey::Name),
        "size" => Ok(SortKey::Size),
        "mtime" => Ok(SortKey::Mtime),
        "kind" => Ok(SortKey::Kind),
        _ => Err(FsError::InvalidFormat {
            format: format!("Invalid sort key: {}", s),
        }),
    }
}

pub fn parse_sort_order(s: &str) -> Result<SortOrder> {
    match s.to_lowercase().as_str() {
        "asc" | "ascending" => Ok(SortOrder::Asc),
        "desc" | "descending" => Ok(SortOrder::Desc),
        _ => Err(FsError::InvalidFormat {
            format: format!("Invalid sort order: {}", s),
        }),
    }
}

pub fn parse_entry_kinds(kinds: &[String]) -> Result<Vec<EntryKind>> {
    kinds
        .iter()
        .map(|s| match s.to_lowercase().as_str() {
            "file" => Ok(EntryKind::File),
            "dir" | "directory" => Ok(EntryKind::Dir),
            "symlink" | "link" => Ok(EntryKind::Symlink),
            _ => Err(FsError::InvalidFormat {
                format: format!("Invalid kind: {}", s),
            }),
        })
        .collect()
}

pub fn determine_sort_order(_asc: bool, desc: bool) -> SortOrder {
    if desc {
        SortOrder::Desc
    } else {
        SortOrder::Asc
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sort_key() {
        assert!(matches!(parse_sort_key("name").unwrap(), SortKey::Name));
        assert!(matches!(parse_sort_key("size").unwrap(), SortKey::Size));
        assert!(matches!(parse_sort_key("mtime").unwrap(), SortKey::Mtime));
        assert!(parse_sort_key("invalid").is_err());
    }

    #[test]
    fn test_parse_sort_order() {
        assert!(matches!(parse_sort_order("asc").unwrap(), SortOrder::Asc));
        assert!(matches!(parse_sort_order("desc").unwrap(), SortOrder::Desc));
        assert!(matches!(
            parse_sort_order("ascending").unwrap(),
            SortOrder::Asc
        ));
        assert!(parse_sort_order("invalid").is_err());
    }

    #[test]
    fn test_parse_entry_kinds() {
        let kinds = parse_entry_kinds(&["file".to_string(), "dir".to_string()]).unwrap();
        assert_eq!(kinds.len(), 2);
        assert!(kinds.contains(&EntryKind::File));
        assert!(kinds.contains(&EntryKind::Dir));
    }

    #[test]
    fn test_determine_sort_order() {
        assert!(matches!(determine_sort_order(false, false), SortOrder::Asc));
        assert!(matches!(determine_sort_order(false, true), SortOrder::Desc));
        assert!(matches!(determine_sort_order(true, false), SortOrder::Asc));
    }
}
