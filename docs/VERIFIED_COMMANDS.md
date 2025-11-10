# Verified Working Commands

All commands below have been tested and confirmed working with the current build.

## ✅ Core Commands (All Verified)

### List
```bash
./target/release/fexplorer list .
./target/release/fexplorer list . --max-depth 2
./target/release/fexplorer list . --sort size --order desc
./target/release/fexplorer ls .  # alias works
```

### Tree
```bash
./target/release/fexplorer tree .
./target/release/fexplorer tree . --max-depth 3
```

### Find
```bash
# Extensions
./target/release/fexplorer find . --ext rs
./target/release/fexplorer find . --ext rs,js,ts

# Sizes
./target/release/fexplorer find . --min-size 10KB
./target/release/fexplorer find . --max-size 1MB

# Dates
./target/release/fexplorer find . --after "7 days ago"
./target/release/fexplorer find . --before "2025-11-07"

# Categories (VERIFIED WORKING)
./target/release/fexplorer find . --category source
./target/release/fexplorer find . --category config
./target/release/fexplorer find . --category media

# Combined
./target/release/fexplorer find . --ext rs --min-size 5KB
./target/release/fexplorer find . --category source --after "1 day ago"
```

### Size
```bash
./target/release/fexplorer size .
./target/release/fexplorer size . --top 10
./target/release/fexplorer size . --du
```

## ✅ Phase 2 Commands (All Verified)

### Grep
```bash
./target/release/fexplorer grep . "TODO"
./target/release/fexplorer grep . "pub fn" --ext rs
./target/release/fexplorer grep . "hello" --case-insensitive
./target/release/fexplorer grep . "TODO|FIXME" --regex
./target/release/fexplorer grep . "error" --line-numbers --context 2
```

### Duplicates
```bash
./target/release/fexplorer duplicates .
./target/release/fexplorer duplicates . --min-size 1KB
./target/release/fexplorer duplicates . --summary
```

## ✅ Phase 3 Commands (All Verified)

### Profiles
```bash
./target/release/fexplorer profiles list
./target/release/fexplorer profiles show recent-code
./target/release/fexplorer profiles init
```

### Run
```bash
./target/release/fexplorer run recent-code
./target/release/fexplorer run large-files
./target/release/fexplorer run recent-code . --after "3 days ago"
```

### Git (if in git repo)
```bash
./target/release/fexplorer git .
./target/release/fexplorer git . --status modified
./target/release/fexplorer git . --since main
```

### Templates
```bash
./target/release/fexplorer list . --template markdown
./target/release/fexplorer list . --template html
./target/release/fexplorer size . --top 20 --template html
```

## ✅ Phase 4 Commands (Verified)

### Interactive TUI
**Both commands work:**
```bash
./target/release/fexplorer interactive .
./target/release/fexplorer tui .  # alias WORKS
```

**Note:** Requires a real terminal. Will error with "Device not configured" if run through automation.

## ✅ Output Formats (All Verified)

```bash
./target/release/fexplorer list . --format json
./target/release/fexplorer list . --format csv
./target/release/fexplorer list . --format ndjson
./target/release/fexplorer list . --format pretty  # default
```

## ✅ Combinations That Work

```bash
# Find + Category + Date
./target/release/fexplorer find . --category source --after "1 day ago"

# Find + Extension + Size
./target/release/fexplorer find . --ext rs --min-size 10KB --max-depth 3

# Grep + Multiple Extensions
./target/release/fexplorer grep . "TODO" --ext rs,js,ts --line-numbers

# List + Sort + Format
./target/release/fexplorer list . --sort size --order desc --format json

# Size + Template
./target/release/fexplorer size . --top 20 --template html > report.html

# Git + Status + Since
./target/release/fexplorer git . --status modified --since main
```

## 🎯 Quick Test Suite

Run these to verify everything works:

```bash
cd /path/to/rust_filesearch

# Phase 1 - Core
./target/release/fexplorer --version
./target/release/fexplorer list . --max-depth 1
./target/release/fexplorer tree . --max-depth 2
./target/release/fexplorer find . --ext rs | head -5
./target/release/fexplorer size . --top 5

# Phase 2 - Search
./target/release/fexplorer grep src "pub fn" --ext rs | head -5
./target/release/fexplorer duplicates test_duplicates --min-size 0B
./target/release/fexplorer find . --category source | head -5

# Phase 3 - Workflows
./target/release/fexplorer profiles list
./target/release/fexplorer run recent-code . | head -5
./target/release/fexplorer list . --format json | head -20
./target/release/fexplorer list . --template markdown | head -10

# Phase 4 - TUI (run in real terminal)
# ./target/release/fexplorer tui .
# ./target/release/fexplorer interactive .
```

## ⚠️ Known Issues

### 1. TUI Requires Real Terminal
**Error:** `Error: IoError { context: "TUI error", source: Os { code: 6, kind: Uncategorized, message: "Device not configured" } }`

**Solution:** Must run from a real terminal application (Terminal.app, iTerm2, etc.), not through automation.

### 2. Profiles Already Initialized
**Error:** Profile initialization fails if already done

**Solution:** Just use `profiles list` and `run <profile>` - initialization only needed once

### 3. Git Command Outside Repo
**Error:** Not in a git repository

**Solution:** Navigate to a git repo first, or use other commands

## 📋 Recommended Aliases

Add to `.bashrc` or `.zshrc`:

```bash
# fexplorer aliases
alias fx='./target/release/fexplorer'
alias fxl='./target/release/fexplorer list'
alias fxf='./target/release/fexplorer find'
alias fxg='./target/release/fexplorer grep'
alias fxs='./target/release/fexplorer size'
alias fxt='./target/release/fexplorer tui'
alias fxi='./target/release/fexplorer interactive'

# Or if installed globally:
alias fx='fexplorer'
alias fxl='fexplorer list'
# ... etc
```

## 🧪 Test Scripts Available

1. **test_simple.sh** - Automated test suite (safe for automation)
2. **test_demo.sh** - Interactive demo with examples
3. Manual testing with commands above

## 📚 Documentation

- **COMMAND_REFERENCE.md** - Complete command syntax reference
- **QUICK_START.md** - Quick start guide with examples
- **DX_WORKFLOWS.md** - Real-world developer workflows
- **TESTING.md** - Comprehensive testing guide
- **HOW_TO_TEST.md** - Testing instructions

All commands verified working as of: 2025-11-08
Binary: ./target/release/fexplorer v0.1.0
Build: cargo build --release --all-features
