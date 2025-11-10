# fexplorer Command Reference

Quick reference for all commands with verified syntax.

## Common Patterns

### Basic Usage
```bash
# Path is always last (or defaults to .)
fexplorer <command> [options] [path]

# Examples
fexplorer list .
fexplorer find . --ext rs
fexplorer grep src "pattern"
```

### Multiple Values
```bash
# Some options are repeatable
fexplorer find . --name "*.rs" --name "*.toml"
fexplorer find . --ext rs --ext js --ext ts

# Extensions can also be comma-separated
fexplorer find . --ext rs,js,ts
```

---

## Commands Quick Reference

### list - List entries with metadata
```bash
fexplorer list [PATH] [OPTIONS]

# Examples
fexplorer list .
fexplorer list . --max-depth 2
fexplorer list . --sort size --order desc
fexplorer list . --dirs-first
fexplorer list . --hidden
fexplorer list . --format json
fexplorer list . --template markdown
```

**Options:**
- `--sort <KEY>`: name, size, mtime, kind
- `--order <ORDER>`: asc, desc
- `--dirs-first`: Show directories before files
- `--max-depth <N>`: Limit traversal depth
- `--hidden`: Include hidden files
- `--format <FORMAT>`: pretty, json, ndjson, csv
- `--template <TEMPLATE>`: markdown, html

---

### tree - Display directory tree
```bash
fexplorer tree [PATH] [OPTIONS]

# Examples
fexplorer tree .
fexplorer tree . --max-depth 3
fexplorer tree . --dirs-first
fexplorer tree . --hidden
```

**Options:**
- `--max-depth <N>`: Limit depth
- `--dirs-first`: Directories before files
- `--hidden`: Show hidden entries

---

### find - Find files matching criteria
```bash
fexplorer find [PATH] [OPTIONS]

# Examples - NAME PATTERNS
fexplorer find . --name "*.rs"
fexplorer find . --name "*.rs" --name "*.toml"
fexplorer find . --regex "test_.*\.rs$"

# Examples - EXTENSIONS
fexplorer find . --ext rs
fexplorer find . --ext rs,js,ts
fexplorer find . --ext rs --ext js

# Examples - SIZE
fexplorer find . --min-size 10KB
fexplorer find . --max-size 1MB
fexplorer find . --min-size 10KB --max-size 1MB

# Examples - DATE
fexplorer find . --after "2025-11-01"
fexplorer find . --after "7 days ago"
fexplorer find . --before "2025-11-07"
fexplorer find . --after "7 days ago" --before "1 day ago"

# Examples - KIND
fexplorer find . --kind file
fexplorer find . --kind dir

# Examples - CATEGORY
fexplorer find . --category source
fexplorer find . --category config
fexplorer find . --category media

# Examples - COMBINED
fexplorer find . --ext rs --min-size 10KB
fexplorer find . --category source --after "7 days ago"
fexplorer find . --ext rs --min-size 5KB --max-depth 3
```

**Options:**
- `--name <PATTERN>`: Glob pattern (repeatable)
- `--regex <PATTERN>`: Regex for names
- `--ext <EXT>`: Extensions (comma-separated or repeatable)
- `--min-size <SIZE>`: Min size (e.g., 10KB, 2MiB)
- `--max-size <SIZE>`: Max size (e.g., 10MB, 2GiB)
- `--after <DATE>`: Modified after (ISO8601, YYYY-MM-DD, or "7 days ago")
- `--before <DATE>`: Modified before
- `--kind <KIND>`: file, dir, symlink
- `--category <CAT>`: source, build, config, docs, media, data, archive, executable

**Categories:**
- `source`: .rs, .js, .ts, .py, .java, .go, .c, .cpp, etc.
- `config`: .toml, .yaml, .json, .ini, .env, etc.
- `docs`: .md, .txt, .pdf, .doc, etc.
- `media`: images, audio, video
- `data`: .csv, .xml, .sqlite, etc.
- `archive`: .zip, .tar, .gz, etc.
- `executable`: .exe, .app, .sh, etc.

---

### size - Calculate and display sizes
```bash
fexplorer size [PATH] [OPTIONS]

# Examples
fexplorer size .
fexplorer size . --top 10
fexplorer size . --top 20 --template html
fexplorer size . --aggregate
fexplorer size . --du
```

**Options:**
- `--top <N>`: Show top N largest files
- `--aggregate`: Compute directory sizes
- `--du`: Show in du-style format
- `--template <T>`: markdown, html

---

### grep - Search file contents
```bash
fexplorer grep [PATH] <PATTERN> [OPTIONS]

# Examples - BASIC
fexplorer grep . "TODO"
fexplorer grep src "pub fn"
fexplorer grep . "hello" --case-insensitive

# Examples - REGEX
fexplorer grep . "fn \w+\(" --regex
fexplorer grep . "TODO|FIXME" --regex

# Examples - FILE FILTERING
fexplorer grep . "error" --ext rs
fexplorer grep . "import" --ext js,ts,tsx

# Examples - CONTEXT
fexplorer grep . "TODO" --context 2
fexplorer grep . "error" --line-numbers --context 1
```

**Options:**
- `<PATTERN>`: Text pattern to search (required)
- `--regex`: Treat pattern as regex
- `--case-insensitive`: Case-insensitive search
- `--ext <EXT>`: Filter by extension
- `--context <N>`: Lines of context
- `--line-numbers`: Show line numbers

---

### duplicates - Find duplicate files
```bash
fexplorer duplicates [PATH] [OPTIONS]

# Examples
fexplorer duplicates .
fexplorer duplicates . --min-size 1KB
fexplorer duplicates . --min-size 100KB --summary
fexplorer duplicates ~/Pictures --category image
```

**Options:**
- `--min-size <SIZE>`: Minimum file size to check
- `--summary`: Show summary only

---

### git - Git integration
```bash
fexplorer git [PATH] [OPTIONS]

# Examples - STATUS
fexplorer git .
fexplorer git . --status modified
fexplorer git . --status untracked
fexplorer git . --status staged

# Examples - SINCE
fexplorer git . --since main
fexplorer git . --since HEAD~5
fexplorer git . --since v1.0.0

# Examples - COMBINED
fexplorer git . --status modified --since main
fexplorer git src/auth --since "7 days ago"
```

**Options:**
- `--status <STATUS>`: modified, untracked, staged, conflict, ignored, clean
- `--since <REF>`: Git ref, branch, tag, or commit

**Note:** Must be run inside a git repository

---

### interactive / tui - Interactive file browser
```bash
fexplorer interactive [PATH]
fexplorer tui [PATH]  # alias

# Examples
fexplorer interactive .
fexplorer tui ~/projects
```

**Keyboard Controls:**
- `↑↓` or `j/k`: Navigate
- `Enter`: Open directory
- `-` or `←`: Go to parent
- `Type`: Filter files (live search)
- `Backspace`: Remove filter character
- `Ctrl+u`: Clear filter
- `Ctrl+.`: Toggle hidden files
- `Ctrl+d`: Toggle dirs-first
- `q` or `Esc`: Quit

**Note:** Requires a real terminal (won't work in automation)

---

### profiles - Manage saved query profiles
```bash
# List all profiles
fexplorer profiles list

# Show specific profile
fexplorer profiles show <NAME>

# Initialize example profiles
fexplorer profiles init

# Examples
fexplorer profiles list
fexplorer profiles show recent-code
fexplorer profiles show large-files
```

---

### run - Execute a saved profile
```bash
fexplorer run <PROFILE> [PATH] [OVERRIDES...]

# Examples
fexplorer run recent-code
fexplorer run recent-code ~/projects
fexplorer run recent-code . --after "3 days ago"
fexplorer run large-files . --top 50
```

**Override Args:**
Pass `--key value` to override profile settings

---

### completions - Generate shell completions
```bash
fexplorer completions <SHELL>

# Examples
fexplorer completions bash > ~/.bash_completion.d/fexplorer
fexplorer completions zsh > ~/.zsh/completions/_fexplorer
fexplorer completions fish > ~/.config/fish/completions/fexplorer.fish
```

**Shells:** bash, zsh, fish, powershell, elvish

---

## Common Options (Global)

These work with most commands:

```bash
--max-depth <N>        # Limit directory depth
--hidden               # Include hidden files
--no-gitignore         # Don't respect .gitignore
--follow-symlinks      # Follow symlinks
--format <FORMAT>      # Output format: pretty, json, ndjson, csv
--template <TEMPLATE>  # Export template: markdown, html
--columns <COLUMNS>    # Columns to show (comma-separated)
--no-color             # Disable colors
--threads <N>          # Parallel threads (default: 4)
--progress             # Show progress bar
-q, --quiet            # Suppress warnings
-v, --verbose          # Verbose output
```

---

## Output Formats

### Pretty (Default)
Human-readable table:
```
./file.txt  1.23 KiB  2025-11-07 12:34:56  file
```

### JSON
Machine-readable, for scripting:
```bash
fexplorer list . --format json | jq '.[0]'
```

### NDJSON
Newline-delimited JSON (streaming):
```bash
fexplorer find . --ext rs --format ndjson | while read line; do
  echo "$line" | jq '.path'
done
```

### CSV
For spreadsheets:
```bash
fexplorer list . --format csv > files.csv
```

### Markdown
For documentation:
```bash
fexplorer list . --template markdown > FILES.md
```

### HTML
Rich reports:
```bash
fexplorer size . --top 20 --template html > report.html
```

---

## Size Format

Sizes accept human-readable formats:
- Bytes: `1024` or `1024B`
- Kilobytes: `10KB` or `10KiB`
- Megabytes: `5MB` or `5MiB`
- Gigabytes: `1GB` or `1GiB`

Binary (base-2) vs Decimal (base-10):
- `KB/MB/GB` = 1000-based
- `KiB/MiB/GiB` = 1024-based

---

## Date Format

Dates accept multiple formats:

**Absolute:**
- ISO 8601: `2025-11-07T12:34:56Z`
- Simple: `2025-11-07`

**Relative:**
- `7 days ago`
- `2 weeks ago`
- `1 month ago`
- `1 year ago`

---

## Quick Examples by Use Case

### Find Recent Work
```bash
fexplorer find . --after "1 day ago" --category source
```

### Find Large Files
```bash
fexplorer size . --top 20
fexplorer find . --min-size 10MB
```

### Search Code
```bash
fexplorer grep . "TODO" --ext rs --line-numbers
fexplorer grep . "fn \w+" --regex --ext rs
```

### Git Workflow
```bash
fexplorer git . --since main
fexplorer git . --status modified --template html > changes.html
```

### Clean Up
```bash
fexplorer duplicates . --min-size 1MB --summary
fexplorer find . --ext log --before "30 days ago"
```

### Documentation
```bash
fexplorer tree . --max-depth 3 > STRUCTURE.md
fexplorer size . --top 20 --template html > SIZE_REPORT.html
```

---

## Error Messages

### "unrecognized subcommand"
Make sure you built with all features:
```bash
cargo build --release --all-features
```

### "Device not configured" (TUI)
TUI requires a real terminal. Open Terminal.app or similar.

### "not in a git repository"
The `git` command requires running inside a git repo.

### "Profile not found"
Initialize profiles first:
```bash
fexplorer profiles init
```

---

## Tips

1. **Use aliases:**
   ```bash
   alias fx='fexplorer'
   alias fxf='fexplorer find'
   alias fxg='fexplorer grep'
   ```

2. **Combine with pipes:**
   ```bash
   fexplorer find . --ext rs --format json | jq '.[].path' | fzf
   ```

3. **Create profiles for common tasks:**
   Edit `~/.config/fexplorer/config.toml`

4. **Use JSON for scripting:**
   ```bash
   fexplorer list . --format json | jq 'map(select(.size > 10000))'
   ```

5. **Export for documentation:**
   ```bash
   fexplorer tree . --max-depth 2 > STRUCTURE.md
   ```
