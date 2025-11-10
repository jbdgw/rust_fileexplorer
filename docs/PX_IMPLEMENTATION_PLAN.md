# px - Project Switcher Implementation Plan

## Overview

`px` is a fast project management CLI that will be built as a second binary in the fexplorer workspace, reusing ~80% of existing infrastructure.

## Architecture Decision: Second Binary in Workspace

```toml
# Cargo.toml
[[bin]]
name = "fexplorer"
path = "src/main.rs"

[[bin]]
name = "px"
path = "src/bin/px.rs"
```

**Benefits:**
- Shares all existing library code from `src/lib.rs`
- Reuses git operations, file traversal, config system
- Minimal duplication (~400 new lines vs 2000+ standalone)
- Single dependency tree, faster builds

## What We Already Have (from fexplorer)

### 1. Git Operations (src/fs/git.rs)
```rust
// Already implemented:
pub fn is_git_repo(path: &Path) -> bool                    // Line 325
pub fn enrich_with_git_status(...)                          // Line 342
pub fn get_changed_since(path: &Path, since: &str)          // Line 370
pub enum GitStatus { Clean, Modified, Staged, ... }         // Line 320
```

### 2. Fast Directory Scanning (src/fs/traverse.rs)
```rust
// Already implemented:
pub fn walk(path: &Path, config: &TraverseConfig, ...)      // Parallel traversal
pub struct TraverseConfig {
    max_depth, follow_symlinks, respect_gitignore, threads
}
```

### 3. Config Management (src/config.rs)
```rust
// Already implemented:
pub struct Config { profiles: HashMap<String, Profile> }
impl Config {
    pub fn load() -> Result<Self>                            // Line 60
    pub fn config_file_path() -> Result<PathBuf>             // Line 80
}
// Loads from ~/.config/fexplorer/config.toml
```

### 4. Metadata & Serialization (src/models/)
```rust
// Already implemented:
pub struct Entry {
    path: PathBuf,
    name: String,
    size: u64,
    mtime: DateTime<Local>,
    kind: EntryKind,
}
// Has serde serialization for JSON
```

### 5. Date Filtering (src/fs/filters.rs)
```rust
// Already implemented for "inactive-30d" filter:
pub struct DateFilter {
    after: Option<DateTime<Local>>,
    before: Option<DateTime<Local>>,
}
```

## New Code Required (~400-500 lines)

### 1. New Dependencies (Cargo.toml)

```toml
[dependencies]
# Add these to existing dependencies:
nucleo = "0.5"              # Fast fuzzy matching
# OR
fuzzy-matcher = "0.3.7"     # Alternative fuzzy matcher

# Optional for frecency:
bincode = "1.3"             # Faster serialization than JSON

# Already have everything else:
# - git2 ✓
# - walkdir ✓
# - serde ✓
# - toml ✓
# - chrono ✓
```

### 2. New Modules Structure

```
src/
  bin/
    px.rs                   # NEW: Main entry point (~150 LOC)
  px/                       # NEW: px-specific modules
    mod.rs                  # NEW: Module declarations
    project.rs              # NEW: Project model (~100 LOC)
    index.rs                # NEW: Index management (~150 LOC)
    search.rs               # NEW: Fuzzy search (~80 LOC)
    frecency.rs             # NEW: Frecency tracking (~80 LOC)
    commands.rs             # NEW: CLI commands (~200 LOC)
  # Reuse existing:
  fs/                       # ✓ Reuse git.rs, traverse.rs
  config.rs                 # ✓ Extend with px config
  models/                   # ✓ Reuse Entry, etc.
  errors.rs                 # ✓ Reuse error types
```

## Implementation Details

### Phase 1: Core Project Model (src/px/project.rs)

```rust
use crate::models::Entry;
use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    /// Absolute path to project root
    pub path: PathBuf,
    
    /// Project name (directory name)
    pub name: String,
    
    /// Last modified time
    pub last_modified: DateTime<Local>,
    
    /// Git status
    pub git_status: ProjectGitStatus,
    
    /// Frecency score (frequency + recency)
    pub frecency_score: f64,
    
    /// Last accessed timestamp
    pub last_accessed: Option<DateTime<Local>>,
    
    /// Access count
    pub access_count: u32,
    
    /// First line of README (if exists)
    pub readme_excerpt: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectGitStatus {
    pub current_branch: String,
    pub has_uncommitted: bool,
    pub ahead: usize,
    pub behind: usize,
    pub last_commit: Option<CommitInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommitInfo {
    pub hash: String,
    pub message: String,
    pub author: String,
    pub timestamp: DateTime<Local>,
}

impl Project {
    /// Create from git repository path
    /// Reuses fexplorer::fs::git functions
    pub fn from_git_repo(path: PathBuf) -> Result<Self> {
        // Use existing is_git_repo, get git status, etc.
    }
    
    /// Extract README excerpt
    pub fn extract_readme_excerpt(path: &Path) -> Option<String> {
        // Find README.md and read first line
    }
}
```

### Phase 2: Index Management (src/px/index.rs)

```rust
use crate::px::project::Project;
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectIndex {
    /// All discovered projects
    pub projects: HashMap<String, Project>,
    
    /// Last sync timestamp
    pub last_sync: DateTime<Local>,
    
    /// Version for schema migrations
    pub version: u32,
}

impl ProjectIndex {
    /// Load from ~/.cache/px/projects.json
    pub fn load() -> Result<Self> {
        let cache_path = Self::cache_path()?;
        if cache_path.exists() {
            let data = std::fs::read_to_string(&cache_path)?;
            Ok(serde_json::from_str(&data)?)
        } else {
            Ok(Self::default())
        }
    }
    
    /// Save to cache
    pub fn save(&self) -> Result<()> {
        let cache_path = Self::cache_path()?;
        if let Some(parent) = cache_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        let json = serde_json::to_string_pretty(self)?;
        std::fs::write(&cache_path, json)?;
        Ok(())
    }
    
    /// Scan directories and rebuild index
    /// REUSES fexplorer's traverse.rs + git.rs
    pub fn sync(&mut self, scan_dirs: &[PathBuf]) -> Result<usize> {
        use crate::fs::{traverse::{walk_no_filter, TraverseConfig}, git::is_git_repo};
        
        let mut new_projects = HashMap::new();
        
        for scan_dir in scan_dirs {
            let config = TraverseConfig {
                max_depth: Some(3),  // Don't go too deep
                respect_gitignore: true,
                threads: 4,
                ..Default::default()
            };
            
            // Use existing walk function
            let entries = walk_no_filter(scan_dir, &config)?;
            
            for entry in entries {
                if entry.kind == EntryKind::Dir && is_git_repo(&entry.path) {
                    if let Ok(project) = Project::from_git_repo(entry.path.clone()) {
                        new_projects.insert(project.path.to_string_lossy().to_string(), project);
                    }
                }
            }
        }
        
        let count = new_projects.len();
        self.projects = new_projects;
        self.last_sync = Local::now();
        self.save()?;
        Ok(count)
    }
    
    /// Update frecency score after access
    pub fn record_access(&mut self, project_path: &str) -> Result<()> {
        if let Some(project) = self.projects.get_mut(project_path) {
            project.access_count += 1;
            project.last_accessed = Some(Local::now());
            project.update_frecency_score();
            self.save()?;
        }
        Ok(())
    }
    
    fn cache_path() -> Result<PathBuf> {
        let cache_dir = dirs::cache_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find cache directory"))?;
        Ok(cache_dir.join("px").join("projects.json"))
    }
}
```

### Phase 3: Fuzzy Search (src/px/search.rs)

```rust
use crate::px::project::Project;
use nucleo::{Config, Nucleo};

pub struct ProjectSearcher {
    matcher: Nucleo<Project>,
}

impl ProjectSearcher {
    pub fn new() -> Self {
        Self {
            matcher: Nucleo::new(Config::DEFAULT, Arc::new(|| {}), None, 1),
        }
    }
    
    /// Search projects by fuzzy matching name/path
    pub fn search<'a>(&self, projects: &'a [Project], query: &str) -> Vec<&'a Project> {
        // Fuzzy match against project.name and project.path
        // Return sorted by match score
        
        // If using fuzzy-matcher instead:
        use fuzzy_matcher::FuzzyMatcher;
        use fuzzy_matcher::skim::SkimMatcherV2;
        
        let matcher = SkimMatcherV2::default();
        let mut matches: Vec<_> = projects
            .iter()
            .filter_map(|p| {
                let name_score = matcher.fuzzy_match(&p.name, query).unwrap_or(0);
                let path_score = matcher.fuzzy_match(&p.path.to_string_lossy(), query).unwrap_or(0);
                let score = name_score.max(path_score);
                
                if score > 0 {
                    Some((p, score))
                } else {
                    None
                }
            })
            .collect();
        
        matches.sort_by(|a, b| b.1.cmp(&a.1));
        matches.into_iter().map(|(p, _)| p).collect()
    }
}
```

### Phase 4: Frecency Tracking (src/px/frecency.rs)

```rust
use chrono::{DateTime, Duration, Local};

/// Calculate frecency score (Firefox-style)
/// Combines frequency (access count) and recency (last access time)
pub fn calculate_frecency(
    access_count: u32,
    last_accessed: Option<DateTime<Local>>,
) -> f64 {
    let frequency_score = (access_count as f64).ln_1p() * 10.0;
    
    let recency_score = if let Some(last_access) = last_accessed {
        let age = Local::now().signed_duration_since(last_access);
        recency_weight(age)
    } else {
        0.0
    };
    
    frequency_score + recency_score
}

fn recency_weight(age: Duration) -> f64 {
    let days = age.num_days();
    
    match days {
        0..=4 => 100.0,      // Within 4 days
        5..=14 => 70.0,      // Within 2 weeks
        15..=31 => 50.0,     // Within month
        32..=90 => 30.0,     // Within 3 months
        _ => 10.0,           // Older
    }
}

impl Project {
    pub fn update_frecency_score(&mut self) {
        self.frecency_score = calculate_frecency(
            self.access_count,
            self.last_accessed,
        );
    }
}
```

### Phase 5: CLI Commands (src/px/commands.rs)

```rust
use crate::px::{index::ProjectIndex, search::ProjectSearcher};
use anyhow::Result;
use std::process::Command;

pub fn cmd_list(
    index: &ProjectIndex,
    filter: Option<String>,
) -> Result<()> {
    let mut projects: Vec<_> = index.projects.values().collect();
    
    // Apply filters
    if let Some(f) = filter {
        match f.as_str() {
            "has-changes" => {
                projects.retain(|p| p.git_status.has_uncommitted);
            }
            "inactive-30d" => {
                let cutoff = Local::now() - chrono::Duration::days(30);
                projects.retain(|p| {
                    p.last_accessed.map_or(true, |t| t < cutoff)
                });
            }
            _ => {}
        }
    }
    
    // Sort by frecency
    projects.sort_by(|a, b| {
        b.frecency_score.partial_cmp(&a.frecency_score).unwrap()
    });
    
    // Print
    for project in projects {
        println!("{} - {}", project.name, project.path.display());
        if project.git_status.has_uncommitted {
            println!("  ⚠️  Uncommitted changes");
        }
    }
    
    Ok(())
}

pub fn cmd_open(
    index: &mut ProjectIndex,
    query: &str,
    editor: &str,
) -> Result<()> {
    let searcher = ProjectSearcher::new();
    let projects: Vec<_> = index.projects.values().cloned().collect();
    let results = searcher.search(&projects, query);
    
    if let Some(project) = results.first() {
        println!("Opening {} in {}...", project.name, editor);
        
        Command::new(editor)
            .arg(&project.path)
            .spawn()?;
        
        // Record access for frecency
        index.record_access(&project.path.to_string_lossy())?;
        
        Ok(())
    } else {
        Err(anyhow::anyhow!("No project found matching '{}'", query))
    }
}

pub fn cmd_info(
    index: &ProjectIndex,
    query: &str,
) -> Result<()> {
    let searcher = ProjectSearcher::new();
    let projects: Vec<_> = index.projects.values().cloned().collect();
    let results = searcher.search(&projects, query);
    
    if let Some(project) = results.first() {
        println!("Project: {}", project.name);
        println!("Path: {}", project.path.display());
        println!("Branch: {}", project.git_status.current_branch);
        
        if project.git_status.has_uncommitted {
            println!("Status: ⚠️  Uncommitted changes");
        } else {
            println!("Status: ✓ Clean");
        }
        
        if let Some(commit) = &project.git_status.last_commit {
            println!("\nLast commit:");
            println!("  {}", commit.message);
            println!("  by {} at {}", commit.author, commit.timestamp.format("%Y-%m-%d %H:%M"));
        }
        
        if let Some(readme) = &project.readme_excerpt {
            println!("\nREADME: {}", readme);
        }
        
        Ok(())
    } else {
        Err(anyhow::anyhow!("No project found matching '{}'", query))
    }
}

pub fn cmd_sync(
    index: &mut ProjectIndex,
    config: &PxConfig,
) -> Result<()> {
    println!("Scanning directories...");
    let count = index.sync(&config.scan_dirs)?;
    println!("✓ Indexed {} projects", count);
    Ok(())
}
```

### Phase 6: Main CLI Entry Point (src/bin/px.rs)

```rust
use clap::{Parser, Subcommand};
use fexplorer::px::{commands, index::ProjectIndex};
use anyhow::Result;

#[derive(Parser)]
#[command(name = "px")]
#[command(about = "Fast project switcher", version)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all projects
    List {
        /// Filter: has-changes, inactive-30d
        #[arg(long)]
        filter: Option<String>,
    },
    
    /// Open project in editor
    Open {
        /// Project name/path query
        query: String,
        
        /// Editor to use (code, cursor, etc.)
        #[arg(long, default_value = "code")]
        editor: String,
    },
    
    /// Show project info
    Info {
        /// Project name/path query
        query: String,
    },
    
    /// Re-index projects
    Sync,
    
    /// Initialize config
    Init,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let mut index = ProjectIndex::load()?;
    let config = PxConfig::load()?;
    
    match cli.command {
        Commands::List { filter } => {
            commands::cmd_list(&index, filter)?;
        }
        Commands::Open { query, editor } => {
            commands::cmd_open(&mut index, &query, &editor)?;
        }
        Commands::Info { query } => {
            commands::cmd_info(&index, &query)?;
        }
        Commands::Sync => {
            commands::cmd_sync(&mut index, &config)?;
        }
        Commands::Init => {
            PxConfig::init()?;
        }
    }
    
    Ok(())
}
```

### Phase 7: Config Extension (add to src/config.rs)

```rust
// Add to existing Config struct:

#[derive(Debug, Serialize, Deserialize)]
pub struct PxConfig {
    /// Directories to scan for projects
    pub scan_dirs: Vec<PathBuf>,
    
    /// Default editor
    pub default_editor: String,
    
    /// Obsidian vault path (optional)
    pub obsidian_vault: Option<PathBuf>,
}

impl PxConfig {
    pub fn load() -> Result<Self> {
        let config_path = Self::config_file_path()?;
        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            Ok(toml::from_str(&content)?)
        } else {
            Ok(Self::default())
        }
    }
    
    pub fn config_file_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or_else(|| anyhow::anyhow!("Could not find config directory"))?;
        Ok(config_dir.join("px").join("config.toml"))
    }
    
    pub fn init() -> Result<()> {
        let config_path = Self::config_file_path()?;
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        let default_config = Self::default();
        let toml = toml::to_string_pretty(&default_config)?;
        std::fs::write(&config_path, toml)?;
        
        println!("Created config at: {}", config_path.display());
        Ok(())
    }
}

impl Default for PxConfig {
    fn default() -> Self {
        let home = dirs::home_dir().unwrap_or_default();
        Self {
            scan_dirs: vec![
                home.join("projects"),
                home.join("code"),
                home.join("Developer"),
            ],
            default_editor: "code".to_string(),
            obsidian_vault: None,
        }
    }
}
```

Example `~/.config/px/config.toml`:
```toml
scan_dirs = [
    "~/projects",
    "~/code",
    "~/Developer",
]

default_editor = "cursor"

# Optional: link to Obsidian vault
obsidian_vault = "~/Documents/Obsidian/MyVault"
```

## Implementation Phases

### Phase 1: Foundation (1-2 hours)
- [ ] Add new dependencies to Cargo.toml
- [ ] Create src/px/ module structure
- [ ] Create src/bin/px.rs
- [ ] Implement Project model
- [ ] Basic config loading

### Phase 2: Indexing (2-3 hours)
- [ ] Implement ProjectIndex
- [ ] Wire up fexplorer's traverse.rs for scanning
- [ ] Wire up fexplorer's git.rs for status
- [ ] Test sync command

### Phase 3: Search (1-2 hours)
- [ ] Add fuzzy-matcher dependency
- [ ] Implement fuzzy search
- [ ] Test list + filtering

### Phase 4: Commands (2-3 hours)
- [ ] Implement list command
- [ ] Implement open command with editor spawn
- [ ] Implement info command with git details
- [ ] Implement sync command

### Phase 5: Frecency (1-2 hours)
- [ ] Implement frecency calculation
- [ ] Track access on open
- [ ] Sort by frecency in list

### Phase 6: Nice to Have (2-4 hours)
- [ ] Shell integration helpers
- [ ] Obsidian note linking
- [ ] Performance optimization

**Total Estimated Time: 10-16 hours**

## Performance Targets

- **Search latency:** < 50ms for 1000+ projects
- **Index sync:** < 5s for 1000+ projects (parallel scanning)
- **Startup time:** < 100ms for cached operations

**How we achieve this:**
- Reuse fexplorer's parallel traversal (rayon + jwalk)
- JSON cache in ~/.cache/px/
- Fuzzy matching is already fast (nucleo/fuzzy-matcher)
- Git operations only on index sync, not search

## Testing Strategy

```bash
# Unit tests
cargo test --bin px

# Integration test
cd /tmp
px init
px sync
px list
px open rust  # Should open a rust project
px info rust
px list --filter has-changes
```

## Shell Integration

Create helper function users can add to their shell:

**~/.zshrc or ~/.bashrc:**
```bash
# px - quick cd
pcd() {
    local dir=$(px list --format path | fzf)
    if [ -n "$dir" ]; then
        cd "$dir"
    fi
}

# px - quick open
po() {
    px open "$1"
}
```

## File Structure Summary

```
rust_filesearch/
├── Cargo.toml                    # Modified: add deps + px binary
├── src/
│   ├── main.rs                   # Existing: fexplorer
│   ├── lib.rs                    # Existing: shared library
│   ├── bin/
│   │   └── px.rs                 # NEW: px entry point (150 LOC)
│   ├── px/                       # NEW: px modules
│   │   ├── mod.rs                # NEW
│   │   ├── project.rs            # NEW (100 LOC)
│   │   ├── index.rs              # NEW (150 LOC)
│   │   ├── search.rs             # NEW (80 LOC)
│   │   ├── frecency.rs           # NEW (80 LOC)
│   │   └── commands.rs           # NEW (200 LOC)
│   ├── config.rs                 # Modified: add PxConfig
│   ├── fs/                       # Existing: REUSE
│   │   ├── git.rs                # Existing: REUSE
│   │   └── traverse.rs           # Existing: REUSE
│   └── models/                   # Existing: REUSE
└── README.md                     # Update with px docs
```

**Total New Code:** ~760 lines
**Total Reused Code:** ~5000+ lines from fexplorer

## Benefits of This Approach

1. ✅ **Code Reuse:** 80%+ of functionality already exists
2. ✅ **Maintenance:** Single dependency tree
3. ✅ **Performance:** Built on proven fast traversal
4. ✅ **Consistency:** Same error handling, config patterns
5. ✅ **Build Time:** Shared compilation, faster CI
6. ✅ **Distribution:** Can ship both tools or separately

## Distribution Options

```bash
# Build both tools
cargo build --release

# Results in:
# - target/release/fexplorer
# - target/release/px

# Or build just one:
cargo build --release --bin px
cargo build --release --bin fexplorer

# Install both:
cargo install --path .

# Install just px:
cargo install --path . --bin px
```

## Next Steps

When ready to implement:

1. **Review this plan** - confirm approach
2. **Create branch** - `git checkout -b feature/px-tool`
3. **Phase 1** - scaffold structure
4. **Iterate** - implement phases 1-6
5. **Test** - verify performance targets
6. **Document** - update README with px usage
7. **Release** - v0.2.0 with dual binaries

---

**Estimated Total Implementation Time:** 10-16 hours

**Key Advantage:** By building on fexplorer, we get a production-ready tool in ~20% of the time it would take to build standalone.
