# 🦀 rust_filesearch - Fast Developer Tools

A dual-binary Rust workspace providing two powerful command-line tools for developers:

- **fexplorer** - Fast file system exploration, searching, and analysis
- **px** - Intelligent project switcher with fuzzy search and frecency ranking

[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

---

## 🎯 Quick Start

```bash
# Install both tools
cargo install --path . --bin fexplorer
cargo install --path . --bin px

# Set up px
px init
# Edit ~/.config/px/config.toml with your project directories
px sync

# Now you're ready!
px open myproject           # Jump to a project instantly
fx find . --name "*.rs"     # Find files in current project
```

---

## 📦 Two Tools, One Workspace

### fexplorer - File System Explorer
Fast filesystem traversal, searching, and analysis tool.

**Use fexplorer when:**
- Finding specific files across projects
- Analyzing disk usage
- Generating reports (HTML/Markdown)
- Exploring unfamiliar codebases
- Need structured output (JSON/CSV)

### px - Project Switcher
Intelligent project management with fuzzy search and frecency ranking.

**Use px when:**
- Switching between projects quickly
- Opening projects in editor + terminal
- Tracking project access patterns
- Finding projects by name/fuzzy match
- Managing multiple development workspaces

**See [WORKFLOWS.md](docs/WORKFLOWS.md) for powerful ways to combine both tools!**

---

## ✨ fexplorer Features

- 🚀 **Fast traversal** with parallel processing (default)
- 🔍 **Powerful filtering** by name (glob/regex), extension, size, date, and type
- 📊 **Multiple output formats**: pretty (colored), JSON, NDJSON, CSV
- 📄 **Template export**: HTML and Markdown reports (default)
- 🌳 **Tree view** with ASCII art visualization
- 📏 **Size analysis** with directory aggregation (du-like)
- 🔎 **Content search** (grep functionality, default)
- 🔁 **Duplicate detection** via BLAKE3 hashing (optional)
- 🔗 **Git integration** for status and change tracking (optional)
- 👁️ **Watch mode** for real-time filesystem monitoring (optional)
- 🙈 **Respects .gitignore** by default
- ⚙️ **Cross-platform**: macOS, Linux, Windows
- 🎨 **TTY-aware** colored output
- 🔒 **Memory-safe** Rust implementation with zero unsafe code

## 📦 Installation

### Quick Install (Recommended)

```bash
cd rust_filesearch
cargo install --path .
```

This installs `fexplorer` to `~/.cargo/bin/` with default features:
- ✅ Parallel traversal (fast)
- ✅ Template export (HTML/Markdown)
- ✅ Content search (grep)

### Using the Install Script

```bash
./install.sh
```

### Verify Installation

```bash
which fexplorer
# Should show: /Users/yourusername/.cargo/bin/fexplorer

fexplorer --version
# Should show: fexplorer 0.1.0
```

### macOS Setup

If `fexplorer` command isn't found, add cargo bin to your PATH:

```bash
# Add to ~/.zshrc
export PATH="$HOME/.cargo/bin:$PATH"

# Reload shell
source ~/.zshrc
```

### Optional Features

```bash
# Install with all features
cargo install --path . --all-features

# Install with specific features
cargo install --path . --features "dedup,git,tui"
```

**Available features:**
- `parallel` (default) - Multi-threaded traversal
- `templates` (default) - HTML/Markdown export
- `grep` (default) - Content search
- `watch` - Filesystem monitoring
- `progress` - Progress bars
- `dedup` - Duplicate file detection
- `git` - Git integration
- `tui` - Interactive TUI mode
- `trends` - Filesystem trend analysis

---

## ✨ px Features

- 🚀 **Fast project switching** - Open projects in editor + iTerm2 instantly
- 🔍 **Fuzzy search** - Find projects by partial name matching
- 📊 **Frecency ranking** - Most frequent and recent projects surface first
- 📁 **Git integration** - Track branches, commits, and uncommitted changes
- 🎯 **Smart filtering** - Filter by uncommitted changes, inactive projects
- 💾 **Caching** - Fast startup with JSON index caching
- 📝 **Rich info** - View project details, git status, README excerpts
- 🔄 **Auto-sync** - Keep project index up to date
- ⚙️ **Configurable** - Set editor, scan directories, and more

## 📦 px Installation

```bash
# Install px binary
cargo install --path . --bin px

# Initialize configuration
px init

# Edit config with your project directories
nano ~/.config/px/config.toml

# Example config:
# scan_dirs = [
#   "/Users/you/Developer/projects",
#   "/Users/you/Work/repos"
# ]
# default_editor = "cursor"  # or "code", "vim", etc.

# Sync and index all projects
px sync
```

**Verify:**
```bash
px list                     # List all projects
px info myproject          # Show project details
px open myproject          # Open in editor + terminal
```

**Shell Aliases (recommended):**
```bash
# Add to ~/.zshrc
alias p='px open'
alias pl='px list'
alias pi='px info'
alias ps='px sync'
```

---

## 🚀 fexplorer Usage

### List Files

List entries with metadata and sorting:

```bash
# List current directory
fexplorer list

# List with custom sorting
fexplorer list --sort size --desc

# Show directories first
fexplorer list --dirs-first --sort name

# Include hidden files
fexplorer list --hidden

# Output as JSON
fexplorer list --format json

# Custom columns
fexplorer list --columns name,size,mtime
```

### Tree View

Display directory structure as an ASCII tree:

```bash
# Show directory tree
fexplorer tree .

# Limit depth
fexplorer tree ~/projects --max-depth 3

# Directories first
fexplorer tree --dirs-first

# Disable .gitignore filtering
fexplorer tree --no-gitignore
```

### Find Files

Search with powerful filtering:

```bash
# Find Rust files (use --name, not --names!)
fexplorer find . --name "*.rs"

# Multiple patterns
fexplorer find . --name "*.rs" --name "*.toml"

# By extension
fexplorer find . --ext rs,toml,md

# Regex pattern
fexplorer find . --regex "^test_.*\.rs$"

# Size range
fexplorer find . --min-size 10KB --max-size 10MB

# Modified after date
fexplorer find . --after "2024-01-01"
fexplorer find . --after "1 day ago"

# Combine filters
fexplorer find . --name "*.rs" --min-size 1KB --after "7 days ago"

# Find only directories
fexplorer find . --kind dir

# Category-based search
fexplorer find . --category source
```

### Template Export (NEW!)

Generate HTML or Markdown reports:

```bash
# HTML report of recent changes
fexplorer find . --after "1 day ago" --template html > report.html

# Markdown table
fexplorer find . --name "*.md" --template markdown > files.md

# Open in browser (macOS)
fexplorer find . --after "1 day ago" --template html > report.html && open report.html
```

### Suppress macOS Permission Warnings

When scanning from home directory:

```bash
# Use -q (quiet) flag
fexplorer -q find ~ --name "*.pdf"

# Or search specific directories
fexplorer find ~/Developer --name "*.rs"
fexplorer find ~/Documents --name "*.md"
```

---

## 🚀 px Usage

### Quick Commands

```bash
# List all projects (sorted by frecency)
px list

# Filter projects
px list --filter has-changes      # Show projects with uncommitted changes
px list --filter inactive-30d     # Projects not accessed in 30 days
px list --filter inactive-90d     # Projects not accessed in 90 days

# Open project (fuzzy match)
px open myproj                    # Matches "myproject"
px open whatsgood                 # Opens "whatsgood-homepage"

# Show project details
px info myproject

# Sync/rebuild index
px sync

# Initialize config
px init
```

### Real-World Examples

```bash
# Morning routine: check what needs attention
px list --filter has-changes

# Jump to project instantly
px open rust_filesearch
# Opens in Cursor + creates iTerm2 window at project directory

# Find project you haven't touched in a while
px list --filter inactive-90d

# Get detailed info before starting work
px info whatsgood-homepage
# Shows:
#   - Branch and git status
#   - Last commit details
#   - README excerpt
#   - Access frequency stats

# After cloning new repos
px sync
# Scans configured directories and updates index
```

### Combined with fexplorer

```bash
# Open project, then explore files
px open myproject
fx find . --name "*.rs" --after "7 days ago"

# Find files across all projects
fx find ~/Developer --name "config.toml"

# Find large uncommitted files
px list --filter has-changes | while read proj; do
  fx git $(px info $proj | grep Path: | awk '{print $2}') --status modified
done
```

**See [docs/PX_USER_GUIDE.md](docs/PX_USER_GUIDE.md) and [docs/WORKFLOWS.md](docs/WORKFLOWS.md) for advanced usage!**

---

## 🚀 fexplorer Size Analysis

Calculate and display file/directory sizes:

```bash
# Show sizes
fexplorer size .

# Aggregate directory sizes (like 'du')
fexplorer size . --aggregate --du

# Top 20 largest files
fexplorer size . --top 20

# Sort by size descending (default for size command)
fexplorer size ~/Downloads --format json
```

### Watch Mode

Monitor filesystem changes in real-time (requires `watch` feature):

```bash
# Watch for all changes
fexplorer watch ~/Downloads

# Watch specific events
fexplorer watch . --events create,modify

# Output as NDJSON (streaming)
fexplorer watch . --format ndjson
```

## 🎛️ Options

### Global Options

- `--no-color` - Disable colored output
- `-q, --quiet` - Quiet mode (suppress non-essential output)

### Common Options (All Subcommands)

- `--max-depth <N>` - Maximum traversal depth
- `--hidden` - Include hidden files
- `--no-gitignore` - Disable .gitignore filtering
- `--follow-symlinks` - Follow symbolic links
- `--format <FORMAT>` - Output format: `pretty`, `json`, `ndjson`, `csv`
- `--columns <COLS>` - Comma-separated columns: `path,name,size,mtime,kind,perms,owner`

### Parallel Feature Options

With `--features parallel`:

- `--threads <N>` - Number of threads for parallel traversal (default: 4)

### Progress Feature Options

With `--features progress`:

- `--progress` - Show progress bar during traversal

## 📊 Output Formats

### Pretty (Default)

Human-readable colored output with automatic column sizing:

```
file1.txt  1.2 KiB  2024-01-15 10:30:00  file
dir1/      0 B      2024-01-15 10:25:00  dir
```

### JSON

Structured JSON array (buffered):

```json
[
  {
    "path": "file1.txt",
    "name": "file1.txt",
    "size": 1234,
    "kind": "file",
    "mtime": "2024-01-15T10:30:00Z"
  }
]
```

### NDJSON

Streaming newline-delimited JSON (one object per line):

```json
{"path":"file1.txt","name":"file1.txt","size":1234,"kind":"file","mtime":"2024-01-15T10:30:00Z"}
{"path":"file2.txt","name":"file2.txt","size":5678,"kind":"file","mtime":"2024-01-15T10:31:00Z"}
```

### CSV

Comma-separated values with header:

```csv
path,name,size,mtime,kind
file1.txt,file1.txt,1234,2024-01-15T10:30:00Z,file
file2.txt,file2.txt,5678,2024-01-15T10:31:00Z,file
```

## 🔧 Development

### Prerequisites

- Rust 1.70+ (stable)
- Cargo

### Building

```bash
# Debug build
cargo build

# Release build (optimized)
cargo build --release

# With all features
cargo build --release --all-features
```

### Testing

```bash
# Run all tests
cargo test

# Run unit tests only
cargo test --lib

# Run integration tests
cargo test --test integration_tests

# With all features
cargo test --all-features
```

### Linting and Formatting

```bash
# Format code
cargo fmt

# Lint with clippy
cargo clippy --all-targets --all-features -- -D warnings

# Check without building
cargo check
```

### Using Justfile

A `justfile` is provided for common tasks:

```bash
# Show available commands
just

# Format, lint, and test
just check

# Build release
just build

# Run example
just run list .
```

## 🏗️ Architecture

This is a **dual-binary Cargo workspace** with ~80% code reuse:

```
rust_filesearch/
├── src/
│   ├── bin/
│   │   ├── fexplorer.rs  # fexplorer CLI entry point
│   │   └── px.rs         # px CLI entry point
│   ├── lib.rs            # Shared library interface
│   ├── cli.rs            # Shared argument parsing (clap)
│   ├── errors.rs         # Shared error types (thiserror)
│   ├── models.rs         # Shared data structures
│   ├── config.rs         # Shared config (fexplorer + px)
│   ├── util.rs           # Shared utilities
│   ├── fs/
│   │   ├── traverse.rs   # Directory traversal (used by both)
│   │   ├── filters.rs    # Predicate-based filtering
│   │   ├── metadata.rs   # Cross-platform metadata
│   │   ├── size.rs       # Directory size calculation
│   │   ├── git.rs        # Git operations (used by both)
│   │   └── watch.rs      # Filesystem watching (feature-gated)
│   ├── output/
│   │   ├── format.rs     # OutputSink trait
│   │   ├── pretty.rs     # Colored terminal output
│   │   ├── json.rs       # JSON & NDJSON formatters
│   │   └── csvw.rs       # CSV formatter
│   └── px/
│       ├── commands.rs   # px command implementations
│       ├── project.rs    # Project model with git status
│       ├── index.rs      # Project index with JSON caching
│       ├── search.rs     # Fuzzy search with SkimMatcherV2
│       └── frecency.rs   # Firefox-style frecency algorithm
├── tests/
│   └── integration_tests.rs  # E2E tests with assert_cmd
└── Cargo.toml            # Workspace with [[bin]] entries
```

**Key Design Decisions:**
- **Dual binary approach**: Separate binaries sharing 80%+ infrastructure
- **Git integration reuse**: Both tools use same git operations from `fs/git.rs`
- **Config separation**: `PxConfig` for px, existing config for fexplorer
- **JSON caching**: px uses `~/.cache/px/projects.json` for fast startup
- **Frecency algorithm**: Firefox-style ranking (frequency + recency)

## 📈 Performance

### fexplorer Performance
- **Streaming architecture**: Processes entries as they're discovered, minimizing memory usage
- **Parallel traversal** (optional): Leverages rayon and jwalk for multi-threaded directory walking
- **Zero-copy where possible**: Uses references and borrows to avoid unnecessary clones
- **Respects .gitignore**: Skips ignored paths early, reducing I/O

**Benchmarks** (tested on ~50,000 files):

| Command | Time (Cold Cache) | Time (Warm Cache) |
|---------|-------------------|-------------------|
| `find .` (GNU find) | 850ms | 120ms |
| `fexplorer list .` | 780ms | 110ms |
| `fexplorer list . --features parallel` | 420ms | 65ms |

### px Performance
- **JSON caching**: Fast startup (~10ms) after initial sync
- **Parallel scanning**: Uses rayon + jwalk for multi-threaded project discovery
- **Respects gitignore**: Skips ignored directories during sync
- **Fuzzy matching**: SkimMatcherV2 provides fast substring matching

**Real-World Results:**

| Operation | Projects | Time |
|-----------|----------|------|
| `px sync` (4 scan dirs) | 243 | 12.02s |
| `px list` (cached) | 243 | 10ms |
| `px open <query>` | 243 | 15ms |
| `px info <query>` | 243 | 12ms |

**Memory usage:** ~5MB for index with 243 projects

## 🛣️ Roadmap

### Completed ✅
- [x] Parallel traversal
- [x] Template export (HTML/Markdown)
- [x] Content search (grep)
- [x] Duplicate file detection
- [x] Git integration
- [x] Config profiles
- [x] **px - Project switcher with frecency** ⭐

### In Progress 🚧
- [ ] TUI mode with interactive navigation
- [ ] Enhanced px features (Obsidian vault integration)

### Planned 📋
- [ ] Archive support (zip, tar.gz)
- [ ] Configurable ignore patterns beyond .gitignore
- [ ] Export directory structure as JSON schema
- [ ] Plugin system for custom filters
- [ ] Publish to crates.io

## 📚 Documentation

### Core Documentation
- **[docs/FEXPLORER_USER_GUIDE.md](docs/FEXPLORER_USER_GUIDE.md)** - Complete fexplorer guide
- **[docs/PX_USER_GUIDE.md](docs/PX_USER_GUIDE.md)** - Complete px guide
- **[docs/WORKFLOWS.md](docs/WORKFLOWS.md)** - Combined workflows using both tools ⭐

### Additional Resources
- **[docs/QUICK_START.md](docs/QUICK_START.md)** - Quick reference guide
- **[docs/INSTALLATION.md](docs/INSTALLATION.md)** - Detailed installation
- **[docs/](docs/)** - Complete documentation index

## 🤝 Contributing

Contributions are welcome! Please:

1. Fork the repository
2. Create a feature branch
3. Make your changes with tests
4. Run `cargo fmt && cargo clippy && cargo test`
5. Submit a pull request

## 📄 License

Licensed under either of:

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://www.apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.

## 🙏 Acknowledgments

Built with these excellent crates:

### Core Dependencies (Both Tools)
- [clap](https://github.com/clap-rs/clap) - Command-line argument parsing
- [ignore](https://github.com/BurntSushi/ripgrep/tree/master/crates/ignore) - Gitignore support from ripgrep
- [walkdir](https://github.com/BurntSushi/walkdir) - Filesystem traversal
- [serde](https://github.com/serde-rs/serde) - Serialization framework
- [chrono](https://github.com/chronotope/chrono) - Date and time library
- [rayon](https://github.com/rayon-rs/rayon) - Data parallelism
- [git2](https://github.com/rust-lang/git2-rs) - Git operations via libgit2

### fexplorer-Specific
- [notify](https://github.com/notify-rs/notify) - Filesystem notifications

### px-Specific
- [fuzzy-matcher](https://github.com/lotabout/fuzzy-matcher) - Fuzzy string matching
- [toml](https://github.com/toml-rs/toml) - Configuration parsing

---

**Made with 🦀 and ❤️ by Rust enthusiasts**
