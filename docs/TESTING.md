# Testing Guide for fexplorer v2.0

This guide provides comprehensive test scenarios for all implemented features.

## Setup

```bash
# Build the release binary
cargo build --release --all-features

# Create an alias for easier testing
alias fx='./target/release/fexplorer'
```

## Phase 1 - Core Features

### 1. List Command
```bash
# Basic listing
fx list .

# List with max depth
fx list . --max-depth 2

# List hidden files
fx list . --hidden

# Sort by size descending
fx list . --sort size --order desc

# Sort by modification time
fx list . --sort mtime --order asc --dirs-first

# Output formats
fx list . --format json
fx list . --format csv
fx list . --format ndjson
```

### 2. Tree Command
```bash
# Basic tree view
fx tree .

# Tree with max depth
fx tree . --max-depth 3

# Tree with directories first
fx tree . --dirs-first
```

### 3. Find Command
```bash
# Find by name pattern (glob)
fx find . --name "*.rs"
fx find . --name "*.md" --name "*.txt"

# Find by regex
fx find . --regex "test_.*\.rs$"

# Find by extension
fx find . --ext rs --ext toml

# Find by size
fx find . --min-size 10KB --max-size 1MB

# Find by date
fx find . --after "2025-11-01"
fx find . --after "7 days ago"
fx find . --before "2025-11-07"

# Find by type
fx find . --kind file
fx find . --kind dir

# Combined filters
fx find . --ext rs --min-size 1KB --after "7 days ago"
```

### 4. Size Command
```bash
# Show all files sorted by size
fx size .

# Show top 10 largest files
fx size . --top 10

# Aggregate directory sizes
fx size . --aggregate

# Show directory sizes (du-style)
fx size . --du
```

## Phase 2 - Enhanced Search

### 5. Grep Command (Content Search)
```bash
# Create test files
mkdir -p test_grep
echo "Hello World" > test_grep/file1.txt
echo "hello world" > test_grep/file2.txt
echo "HELLO WORLD" > test_grep/file3.txt
echo "Goodbye World" > test_grep/file4.txt

# Literal search
fx grep test_grep "Hello"

# Case insensitive search
fx grep test_grep "hello" --case-insensitive

# Regex search
fx grep test_grep "H[eE]llo" --regex

# Search with context
fx grep test_grep "Hello" --context 2

# Search with line numbers
fx grep test_grep "Hello" --line-numbers

# Search specific file types
fx grep . "fn main" --ext rs

# Combined options
fx grep . "error" --case-insensitive --line-numbers --context 1 --ext rs
```

### 6. Duplicates Command
```bash
# Create test duplicates
mkdir -p test_dupes
echo "content A" > test_dupes/file1.txt
echo "content A" > test_dupes/file2.txt
echo "content A" > test_dupes/file3.txt
echo "content B" > test_dupes/file4.txt
echo "content B" > test_dupes/file5.txt

# Find all duplicates
fx duplicates test_dupes --min-size 0B

# Find duplicates larger than 1KB
fx duplicates . --min-size 1KB

# Show summary only
fx duplicates test_dupes --min-size 0B --summary

# Find duplicates in large directory
fx duplicates ~/Documents --min-size 100KB
```

### 7. Category Filter
```bash
# Find source code files
fx find . --category source

# Find configuration files
fx find . --category config

# Find media files
fx find . --category media

# Find image files specifically
fx find . --category image

# Find documentation files
fx find . --category documentation

# Find archive files
fx find . --category archive

# Find data files
fx find . --category data
```

## Phase 3 - Workflows & Export

### 8. Export Templates
```bash
# Export to Markdown
fx list . --template markdown > output.md
fx find . --ext rs --template md > rust_files.md

# Export to HTML
fx list . --template html > output.html
fx size . --top 20 --template html > largest_files.html

# View the exports
cat output.md
open output.html  # macOS
xdg-open output.html  # Linux
```

### 9. Profiles System
```bash
# Initialize config with example profiles
fx profiles init

# List available profiles
fx profiles list

# Show specific profile
fx profiles show recent-code
fx profiles show large-files

# Run profiles
fx run recent-code
fx run recent-code ~/projects

# Run with overrides
fx run recent-code . --after "3 days ago"
fx run large-files . --top 5

# Create custom profile (edit ~/.config/fexplorer/config.toml)
# Then run it:
fx run my-custom-profile
```

### 10. Git Integration
```bash
# Navigate to a git repository first
cd /path/to/git/repo

# Show all files with git status
fx git .

# Filter by status
fx git . --status modified
fx git . --status untracked
fx git . --status staged
fx git . --status conflict

# Show changes since a ref
fx git . --since main
fx git . --since HEAD~5
fx git . --since v1.0.0

# Combined
fx git . --status modified --since main
```

## Phase 4 - Interactive

### 11. TUI Mode

**IMPORTANT:** The TUI requires a real terminal to run. It will NOT work:
- In automated scripts
- Through SSH without terminal allocation (`ssh host command`)
- In non-interactive environments

To test TUI mode, you MUST run it from a real terminal:

```bash
# Open a real terminal window (not through automation)
# Then run:
fx interactive .
fx tui .  # alias

# Once in TUI:
# - Use ↑↓ or j/k to navigate
# - Type to filter files (live search)
# - Press Enter to enter directories
# - Press - or ← to go up
# - Press Ctrl+. to toggle hidden files
# - Press Ctrl+d to toggle dirs-first sorting
# - Press Backspace to remove filter characters
# - Press Ctrl+u to clear filter
# - Press q or Esc to quit
```

**Test from a proper terminal:**
```bash
# macOS: Open Terminal.app or iTerm2
# Linux: Open gnome-terminal, konsole, or xterm
# Windows: Open Windows Terminal or PowerShell

cd /path/to/rust_filesearch
./target/release/fexplorer interactive .
```

## Advanced Testing Scenarios

### 12. Complex Find Queries
```bash
# Find large Rust files modified recently
fx find . --ext rs --min-size 10KB --after "7 days ago"

# Find all config files in subdirectories
fx find . --category config --max-depth 3

# Find files matching multiple patterns
fx find . --name "*.json" --name "*.yaml" --name "*.toml"

# Find files with regex pattern and size constraint
fx find . --regex "^test_.*" --min-size 1KB --max-size 100KB
```

### 13. Performance Testing
```bash
# Test on large directory
time fx list ~/Documents --max-depth 5

# Test parallel features
fx find ~/Documents --ext jpg --ext png  # Uses parallel traversal
fx grep ~/Documents "TODO" --ext rs  # Uses parallel search

# Test with progress indicator (if enabled)
fx list ~/Documents --progress
```

### 14. Output Format Testing
```bash
# JSON output for scripting
fx list . --format json | jq '.[] | select(.size > 10000)'

# CSV for spreadsheet import
fx size . --top 50 --format csv > sizes.csv

# NDJSON for streaming
fx find . --ext rs --format ndjson | while read line; do echo "$line" | jq '.path'; done
```

### 15. Integration with Other Tools
```bash
# Pipe to other commands
fx find . --ext rs | xargs wc -l

# Find and act on results
fx find . --ext tmp --kind file | xargs rm

# Export and open in browser
fx size . --template html > report.html && open report.html

# Git + Find combination
fx git . --status modified | xargs git add
```

## Test Checklists

### Basic Functionality ✓
- [ ] List command works with various options
- [ ] Tree command displays proper structure
- [ ] Find command with multiple filter types
- [ ] Size command shows accurate sizes

### Search Features ✓
- [ ] Grep finds literal matches
- [ ] Grep works with regex
- [ ] Duplicates finds duplicate files
- [ ] Category filter matches correct file types

### Workflows ✓
- [ ] Templates export to markdown
- [ ] Templates export to HTML
- [ ] Profiles can be created and run
- [ ] Git integration shows correct status

### Interactive ✓
- [ ] TUI launches successfully
- [ ] Navigation keys work
- [ ] Filtering works as you type
- [ ] Can enter/exit directories

### Edge Cases
- [ ] Large directories (1000+ files)
- [ ] Deep nesting (10+ levels)
- [ ] Unicode filenames
- [ ] Files with spaces
- [ ] Binary files (should skip in grep)
- [ ] Symlinks
- [ ] Permission denied scenarios

## Troubleshooting

### TUI Issues
```bash
# If TUI has display issues, check terminal support
echo $TERM

# Test with different terminal
TERM=xterm-256color fx tui .
```

### Performance Issues
```bash
# Disable parallel features for debugging
cargo build --release --no-default-features --features tui,grep,dedup,git,templates

# Enable quiet mode to reduce output overhead
fx find . --ext rs --quiet
```

### Git Integration Issues
```bash
# Ensure you're in a git repository
git status

# Check git is accessible
which git
```

## Automated Test Suite
```bash
# Run unit tests
cargo test --all-features

# Run specific test module
cargo test --test integration_tests

# Run with output
cargo test -- --nocapture
```

## Benchmarking
```bash
# Install hyperfine
cargo install hyperfine

# Benchmark list command
hyperfine 'fx list ~/Documents --max-depth 3'

# Compare with other tools
hyperfine 'fx find . --ext rs' 'find . -name "*.rs"'
hyperfine 'fx tree .' 'tree .'
```

## Creating Test Data

### Script to Generate Test Files
```bash
#!/bin/bash
# create_test_data.sh

mkdir -p test_data/{dir1,dir2,dir3}/{subdir1,subdir2}

# Create files of various sizes
echo "small" > test_data/small.txt
head -c 10KB /dev/urandom > test_data/medium.bin
head -c 1MB /dev/urandom > test_data/large.bin

# Create duplicates
cp test_data/medium.bin test_data/dir1/duplicate1.bin
cp test_data/medium.bin test_data/dir2/duplicate2.bin

# Create source files
cat > test_data/main.rs <<EOF
fn main() {
    println!("Hello, world!");
}
EOF

# Create config files
cat > test_data/config.toml <<EOF
[settings]
name = "test"
EOF

# Create hidden files
touch test_data/.hidden
touch test_data/dir1/.gitignore

echo "Test data created in ./test_data"
```

Run it:
```bash
chmod +x create_test_data.sh
./create_test_data.sh
```

## Expected Results Reference

### List Output Format
```
<path>  <size>  <mtime>  <type>
./file.txt  1.23 KiB  2025-11-07 12:34:56  file
```

### Tree Output Format
```
.
├── dir1
│   ├── file1.txt
│   └── file2.txt
└── dir2
    └── file3.txt
```

### Duplicates Output Format
```
Duplicate Group #1 (hash: 1234abcd...)
  File size: 1.00 MiB
  Count: 3 files
  Wasted space: 2.00 MiB
  Files:
    - ./file1.bin
    - ./file2.bin
    - ./file3.bin
```

### Git Status Output
```
./src/main.rs  12.3 KiB  2025-11-07 12:34:56  file
./src/lib.rs  8.5 KiB  2025-11-07 12:35:12  file

Git Status Summary:
  modified: 2
  untracked: 0
  staged: 0
```

Happy testing! 🚀
