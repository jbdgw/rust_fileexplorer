# fexplorer User Guide
**Fast File System Explorer and Analysis Tool**

---

## 📚 Table of Contents

1. [Quick Start](#quick-start)
2. [Core Commands](#core-commands)
3. [Finding Files](#finding-files)
4. [Directory Trees](#directory-trees)
5. [Size Analysis](#size-analysis)
6. [Git Integration](#git-integration)
7. [Output Formats](#output-formats)
8. [Templates & Reports](#templates--reports)
9. [Configuration & Profiles](#configuration--profiles)
10. [Power User Workflows](#power-user-workflows)

---

## Quick Start

### Installation

```bash
cd rust_filesearch
cargo install --path . --bin fexplorer
```

Creates `~/.cargo/bin/fexplorer`

### Basic Usage

```bash
# Find files by pattern
fexplorer find . --name "*.rs"

# Show directory tree
fexplorer tree src/

# Analyze directory size
fexplorer size . --top 10

# Find git changes
fexplorer git . --status modified
```

### Recommended Aliases

```bash
# Add to ~/.zshrc
alias fx='fexplorer'
alias fxf='fexplorer find'
alias fxt='fexplorer tree'
alias fxs='fexplorer size'
alias fxg='fexplorer git'
```

---

## Core Commands

### `fexplorer find`
Search for files matching criteria.

**Basic syntax:**
```bash
fexplorer find <path> [filters] [output options]
```

**Examples:**
```bash
# Find by name pattern
fx find . --name "*.rs"

# Find by extension
fx find . --ext rs,toml

# Find by size
fx find . --min-size 1MB --max-size 100MB

# Find by date
fx find . --after "7 days ago"
fx find . --before "2024-01-01"

# Find by type
fx find . --kind file
fx find . --kind dir

# Combine filters
fx find . --name "test*" --ext rs --after "1 week ago"
```

### `fexplorer tree`
Display directory structure as ASCII tree.

**Examples:**
```bash
# Basic tree
fx tree src/

# Limit depth
fx tree . --max-depth 3

# Show directories first
fx tree . --dirs-first

# Show all files (including hidden)
fx tree . --all
```

### `fexplorer size`
Analyze directory sizes.

**Examples:**
```bash
# Top 10 largest entries
fx size . --top 10

# Aggregate by directory
fx size . --top 10 --aggregate

# Like du -sh
fx size . --du

# Human-readable output
fx size . --human
```

### `fexplorer git`
Git repository operations.

**Examples:**
```bash
# Show modified files
fx git . --status modified

# Show all uncommitted changes
fx git . --status uncommitted

# Changes since branch/tag
fx git . --since main
fx git . --since v1.0.0
```

---

## Finding Files

### By Name Pattern (Glob)

```bash
# Single pattern
fx find . --name "*.rs"

# Multiple patterns
fx find . --name "*.rs" --name "*.toml"

# Complex patterns
fx find . --name "test_*.rs"
fx find . --name "**/src/**/*.js"
```

**Glob syntax:**
- `*` - Match any characters except /
- `**` - Match any characters including /
- `?` - Match single character
- `[abc]` - Match a, b, or c
- `[!abc]` - Match anything except a, b, c

### By Extension

```bash
# Single extension
fx find . --ext rs

# Multiple extensions
fx find . --ext rs,toml,md

# Case insensitive
fx find . --ext RS,rs
```

### By Regex

```bash
# Find test files
fx find . --regex "test.*\.rs$"

# Find numbered files
fx find . --regex ".*_\d+\..*"

# Complex patterns
fx find . --regex "^(src|tests)/.*\.rs$"
```

### By Size

```bash
# Minimum size
fx find . --min-size 1MB

# Maximum size
fx find . --max-size 100MB

# Range
fx find . --min-size 10KB --max-size 1MB

# Exact size
fx find . --size 4096
```

**Size units:**
- `B` - Bytes
- `KB` - Kilobytes (1024 bytes)
- `MB` - Megabytes
- `GB` - Gigabytes

### By Date

```bash
# Modified after date
fx find . --after "2024-01-01"
fx find . --after "7 days ago"
fx find . --after "1 week ago"
fx find . --after "yesterday"

# Modified before date
fx find . --before "2023-12-31"
fx find . --before "30 days ago"

# Range
fx find . --after "1 month ago" --before "1 week ago"
```

**Date formats:**
- ISO: `2024-01-01`, `2024-01-01T10:30:00`
- Relative: `7 days ago`, `2 weeks ago`, `1 month ago`
- Keywords: `yesterday`, `today`

### By Type

```bash
# Only files
fx find . --kind file

# Only directories
fx find . --kind dir

# Only symlinks
fx find . --kind symlink

# Multiple types
fx find . --kind file,dir
```

### By Category

```bash
# Source code
fx find . --category source

# Documents
fx find . --category docs

# Media
fx find . --category media

# Archives
fx find . --category archive
```

**Categories:**
- `source` - .rs, .js, .py, .go, .ts, etc.
- `docs` - .md, .txt, .pdf, .doc, etc.
- `media` - .jpg, .png, .mp4, .mp3, etc.
- `archive` - .zip, .tar, .gz, etc.
- `config` - .toml, .yaml, .json, .env, etc.

---

## Directory Trees

### Basic Tree

```bash
fx tree src/
```

**Output:**
```
src/
├── main.rs
├── lib.rs
├── cli.rs
├── config.rs
├── models/
│   ├── entry.rs
│   ├── category.rs
│   └── git_status.rs
└── fs/
    ├── traverse.rs
    ├── git.rs
    └── filters.rs
```

### Tree Options

```bash
# Limit depth
fx tree . --max-depth 2

# Show directories only
fx tree . --dirs-only

# Sort directories first
fx tree . --dirs-first

# Show hidden files
fx tree . --all

# Show file sizes
fx tree . --size

# Follow symlinks
fx tree . --follow-links
```

### Advanced Tree

```bash
# Exclude patterns
fx tree . --exclude "target" --exclude "node_modules"

# Only show specific extensions
fx tree . --ext rs,toml

# Show gitignored files
fx tree . --no-gitignore
```

---

## Size Analysis

### Top N Largest

```bash
# Top 10 files/dirs by size
fx size . --top 10

# Top 20
fx size . --top 20
```

**Output:**
```
Size         Path
────────────────────────────────
45.2 MB      target/release/fexplorer
12.3 MB      target/debug/fexplorer
2.1 MB       Cargo.lock
892 KB       README.md
...
```

### Aggregated Size

```bash
# Show total size of each directory
fx size . --top 10 --aggregate
```

**Output:**
```
Size         Directory
────────────────────────────────
256.4 MB     target/
12.8 MB      src/
4.2 MB       docs/
1.1 MB       tests/
```

### Du-Style Output

```bash
# Like `du -sh *`
fx size . --du

# With depth limit
fx size . --du --max-depth 2
```

### Human-Readable

```bash
# Show sizes in human-readable format (default)
fx size . --human --top 10

# Show exact bytes
fx size . --top 10  # (--human is default)
```

---

## Git Integration

### Git Status

```bash
# Show all uncommitted changes
fx git . --status uncommitted

# Show only modified files
fx git . --status modified

# Show staged files
fx git . --status staged

# Show untracked files
fx git . --status untracked

# Show deleted files
fx git . --status deleted
```

**Output:**
```
Status       Path
────────────────────────────────
modified     src/main.rs
modified     Cargo.toml
untracked    new_file.rs
staged       README.md
```

### Changes Since Ref

```bash
# Changes since main branch
fx git . --since main

# Changes since tag
fx git . --since v1.0.0

# Changes since commit
fx git . --since abc123

# Changes since date
fx git . --since "2024-01-01"
```

**Output:**
```
Files changed since main:
  src/px/commands.rs
  src/px/index.rs
  Cargo.toml

Total: 3 files
```

### Combine with Find

```bash
# Find large uncommitted files
fx find . --min-size 1MB | \
  fx git . --status modified | \
  grep -f -

# Find modified Rust files
fx git . --status modified --ext rs
```

---

## Output Formats

### Pretty (Default)

Human-readable tables with colors:

```bash
fx find . --name "*.rs"
```

```
Name              Size      Modified
─────────────────────────────────────────
main.rs           12.3 KB   2024-01-10 10:30
lib.rs            8.9 KB    2024-01-10 09:15
config.rs         4.2 KB    2024-01-09 14:20
```

### JSON

Machine-readable structured data:

```bash
fx find . --name "*.rs" --format json
```

```json
[
  {
    "path": "/Users/you/project/src/main.rs",
    "name": "main.rs",
    "size": 12587,
    "kind": "file",
    "mtime": 1704881400,
    "depth": 2
  },
  ...
]
```

### NDJSON (Newline-Delimited JSON)

Streaming JSON (one object per line):

```bash
fx find . --name "*.rs" --format ndjson
```

```json
{"path":"/Users/you/project/src/main.rs","name":"main.rs","size":12587,"kind":"file","mtime":1704881400,"depth":2}
{"path":"/Users/you/project/src/lib.rs","name":"lib.rs","size":9103,"kind":"file","mtime":1704878100,"depth":2}
```

**Benefits:**
- Streamable (process line-by-line)
- Concatenable (cat file1.ndjson file2.ndjson)
- Efficient for large datasets

### CSV

Spreadsheet-compatible:

```bash
fx find . --name "*.rs" --format csv > files.csv
```

```csv
path,name,size,kind,mtime,depth
/Users/you/project/src/main.rs,main.rs,12587,file,1704881400,2
/Users/you/project/src/lib.rs,lib.rs,9103,file,1704878100,2
```

**Use cases:**
- Import to Excel/Numbers
- Data analysis in Python/R
- Database imports

---

## Templates & Reports

### HTML Template

Generate HTML report:

```bash
fx find . --after "7 days ago" --template html > weekly_report.html
```

**Output includes:**
- Styled table of files
- Sortable columns
- File counts and totals
- Timestamp of report generation

**Use cases:**
- Weekly activity reports
- Project documentation
- Share with non-technical stakeholders

### Markdown Template

Generate Markdown document:

```bash
fx find . --after "7 days ago" --template markdown > CHANGELOG.md
```

**Output:**
```markdown
# Files Report

Generated: 2024-01-10 14:30:00

## Files (23 total)

| Name | Size | Modified |
|------|------|----------|
| main.rs | 12.3 KB | 2024-01-10 10:30 |
| lib.rs | 8.9 KB | 2024-01-10 09:15 |
...

## Summary

- Total files: 23
- Total size: 245.8 KB
- Date range: Last 7 days
```

**Use cases:**
- Git commit messages
- Pull request descriptions
- Documentation updates
- Internal wikis

### Custom Templates

Create your own templates (future feature):

```bash
fx find . --template custom.tera > output.txt
```

---

## Configuration & Profiles

### Config File

Location: `~/.config/fexplorer/config.toml`

### Initialize Config

```bash
fx profiles init
```

Creates default config with example profiles.

### Profile Structure

```toml
[profiles.recent-code]
command = "find"
description = "Find recently modified source code"

[profiles.recent-code.args]
ext = ["rs", "go", "ts", "py"]
after = "7 days ago"

[profiles.cleanup]
command = "find"
description = "Find old log and temp files"

[profiles.cleanup.args]
ext = ["log", "tmp"]
before = "30 days ago"
min_size = "1MB"
```

### List Profiles

```bash
fx profiles list
```

**Output:**
```
Available profiles:
  - recent-code: Find recently modified source code
  - cleanup: Find old log and temp files
  - large-files: Find files larger than 100MB
```

### Run Profile

```bash
# Run saved profile
fx run recent-code ~/Developer

# Override profile args
fx run cleanup ~/Downloads --before "60 days ago"
```

### Create Profiles

Add to `~/.config/fexplorer/config.toml`:

```toml
[profiles.my-search]
command = "find"
description = "Custom search"

[profiles.my-search.args]
name = ["*.rs"]
min_size = "10KB"
after = "1 week ago"
```

---

## Power User Workflows

### Workflow 1: Daily Code Review

```bash
#!/bin/bash
# daily-review.sh

echo "=== Files Modified Today ==="
fx find ~/Developer --after "1 day ago" --ext rs,ts,py

echo ""
echo "=== Largest Changes ==="
fx find ~/Developer --after "1 day ago" --min-size 10KB --format pretty

echo ""
echo "=== Git Status Across Projects ==="
for proj in ~/Developer/*/; do
  if [ -d "$proj/.git" ]; then
    echo ">>> $proj"
    fx git "$proj" --status modified | head -5
  fi
done
```

### Workflow 2: Cleanup Old Files

```bash
#!/bin/bash
# cleanup-old-files.sh

# Find candidates
echo "Finding old, large files..."
fx find ~/Downloads \
  --before "90 days ago" \
  --min-size 10MB \
  --format csv > old_files.csv

# Review
echo "Review old_files.csv and delete as needed"
open old_files.csv

# Generate report
fx find ~/Downloads \
  --before "90 days ago" \
  --min-size 10MB \
  --template html > cleanup_report.html

open cleanup_report.html
```

### Workflow 3: Project Size Analysis

```bash
#!/bin/bash
# analyze-project-sizes.sh

echo "=== Largest Directories ==="
fx size ~/Developer --top 20 --aggregate

echo ""
echo "=== Largest Files ==="
fx size ~/Developer --top 20

echo ""
echo "=== target/ Directories (can be deleted) ==="
find ~/Developer -type d -name "target" -exec du -sh {} \;

echo ""
echo "=== node_modules/ Directories (can be deleted) ==="
find ~/Developer -type d -name "node_modules" -exec du -sh {} \;
```

### Workflow 4: Weekly Documentation

```bash
#!/bin/bash
# weekly-docs.sh

WEEK=$(date +%Y-W%V)

# Generate reports
fx find ~/Developer \
  --after "7 days ago" \
  --category source \
  --template markdown > "changes_$WEEK.md"

fx find ~/Developer \
  --after "7 days ago" \
  --template html > "activity_$WEEK.html"

# Add to git
git add "changes_$WEEK.md" "activity_$WEEK.html"
git commit -m "Weekly docs for $WEEK"
```

### Workflow 5: Find Duplicates

```bash
#!/bin/bash
# find-duplicates.sh

# Use feature "dedup" (if enabled)
cargo install --path . --features dedup

# Find duplicates
fx duplicates ~/Documents \
  --min-size 1MB \
  --summary \
  --format json > duplicates.json

# Process results
cat duplicates.json | jq -r '.[] | "\(.hash): \(.files[])"'
```

### Workflow 6: Git Statistics

```bash
#!/bin/bash
# git-stats.sh

echo "=== Uncommitted Changes Across All Projects ==="
find ~/Developer -type d -name ".git" | while read gitdir; do
  proj=$(dirname "$gitdir")
  changes=$(fx git "$proj" --status uncommitted 2>/dev/null | wc -l)
  if [ "$changes" -gt 0 ]; then
    echo "$changes changes in $proj"
  fi
done

echo ""
echo "=== Files Changed in Last Week ==="
find ~/Developer -type d -name ".git" | while read gitdir; do
  proj=$(dirname "$gitdir")
  echo ">>> $proj"
  fx git "$proj" --since "1 week ago" | head -5
done
```

---

## macOS-Specific Tips

### Permission Handling

macOS System Integrity Protection (SIP) protects certain directories:
- `~/Library/`
- `~/Photos Library.photoslibrary/`
- `/System/`
- `/private/`

**Solutions:**

1. Use `-q` flag to suppress warnings:
   ```bash
   fx -q find ~ --name "*.pdf"
   ```

2. Search specific directories instead:
   ```bash
   # Instead of:
   fx find ~

   # Use:
   fx find ~/Documents
   fx find ~/Developer
   fx find ~/Downloads
   ```

3. Grant Full Disk Access (if needed):
   - System Settings → Privacy & Security → Full Disk Access
   - Add Terminal or iTerm2

### Spotlight Integration

```bash
# Find files Spotlight can't (hidden, system)
fx find ~/Library --name "*.plist"

# Find by content (using grep feature)
fx find ~/Documents --content "TODO" --ext md
```

### Quick Look Integration

```bash
# Find and preview
fx find ~/Documents --name "*.pdf" --format json | \
  jq -r '.[0].path' | \
  xargs qlmanage -p
```

---

## Best Practices

### 1. Use Specific Paths

**Bad:**
```bash
fx find ~ --name "*.rs"  # Scans entire home directory
```

**Good:**
```bash
fx find ~/Developer --name "*.rs"  # Targeted search
```

### 2. Combine Filters

**Bad:**
```bash
fx find . --name "*.rs"
# Manually filter results...
```

**Good:**
```bash
fx find . --name "*.rs" --after "1 week ago" --min-size 1KB
```

### 3. Use Profiles for Repeated Searches

**Bad:**
```bash
# Run same command daily:
fx find ~/Developer --ext rs,ts --after "1 day ago"
```

**Good:**
```bash
# Create profile, run once:
fx run daily-changes ~/Developer
```

### 4. Export for Analysis

```bash
# Export to CSV for Excel/Python analysis
fx find ~/Developer --format csv > projects.csv

# Export to JSON for jq processing
fx find ~/Developer --format json | jq '.[] | select(.size > 1000000)'
```

### 5. Respect Gitignore

By default, fexplorer respects `.gitignore`:

```bash
# Good: Skips node_modules, target, etc.
fx find . --name "*.js"

# If you need gitignored files:
fx find . --name "*.js" --no-gitignore
```

---

## Troubleshooting

### Slow Performance

**Symptom:** Searches take too long

**Solutions:**

1. Limit depth:
   ```bash
   fx find . --max-depth 3 --name "*.rs"
   ```

2. Use specific paths:
   ```bash
   fx find ~/Developer/myproject --name "*.rs"
   ```

3. Enable parallel mode (default):
   ```bash
   fx find . --threads 8 --name "*.rs"
   ```

### Permission Errors

**Symptom:**
```
Warning: Failed to read /path/to/directory: Permission denied
```

**Solutions:**

1. Use `-q` to suppress warnings:
   ```bash
   fx -q find ~ --name "*.pdf"
   ```

2. Skip directories you don't need:
   ```bash
   fx find ~/Documents --name "*.pdf"
   ```

3. Grant permissions (macOS):
   - System Settings → Privacy & Security → Full Disk Access

### No Results

**Symptom:** Search returns empty

**Solutions:**

1. Check gitignore isn't filtering results:
   ```bash
   fx find . --name "node_modules" --no-gitignore
   ```

2. Check path is correct:
   ```bash
   ls /path/to/search  # Verify directory exists
   ```

3. Try broader search:
   ```bash
   fx find . --name "*keyword*"  # Add wildcards
   ```

---

## Comparison to Other Tools

| Feature | fexplorer | find | fd | ripgrep |
|---------|-----------|------|----|---------||
| Speed | Fast (parallel) | Slow | Very fast | Very fast |
| Gitignore | ✓ Yes | ✗ No | ✓ Yes | ✓ Yes |
| Templates | ✓ HTML/MD | ✗ No | ✗ No | ✗ No |
| Git status | ✓ Yes | ✗ No | ✗ No | ✗ No |
| Size analysis | ✓ Built-in | ✗ No | ✗ No | ✗ No |
| Tree view | ✓ Yes | ✗ No | ✗ No | ✗ No |
| JSON output | ✓ Yes | ✗ No | ✗ No | ✓ Yes |
| Profiles | ✓ Yes | ✗ No | ✗ No | ✗ No |

---

## FAQ

### Q: How is fexplorer different from find?

**A:** fexplorer is faster (parallel), respects gitignore, outputs templates, and has git integration. It's designed for modern development workflows.

### Q: Can I use fexplorer in CI/CD?

**A:** Yes! Use JSON output for machine parsing:
```bash
fx find . --name "*.test.js" --format json | jq length
```

### Q: Does fexplorer modify files?

**A:** No, fexplorer is read-only. It only searches and analyzes files.

### Q: Can I use regex with fexplorer?

**A:** Yes, with `--regex`:
```bash
fx find . --regex "test_.*\.rs$"
```

### Q: How do I exclude directories?

**A:** Use `--exclude`:
```bash
fx find . --name "*.js" --exclude "node_modules" --exclude "dist"
```

---

**Last Updated**: 2025-01-10
**Version**: 0.1.0
