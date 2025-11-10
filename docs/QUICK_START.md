# fexplorer Quick Start Guide

## ✅ Fixed Issues

The tool now works! The problem was:
1. **Templates feature wasn't enabled by default** - Now it is
2. **Using `--names` instead of `--name`** - The CLI uses singular form

## 🚀 Quick Installation

```bash
# Build with all features
cargo build --release

# Or use the install script
./install.sh
```

## 📋 Common Commands

### Find Files Modified Recently
```bash
# Files from the last day
./target/release/fexplorer find . --after "1 day ago" --template html > changes.html

# Files from a specific date
./target/release/fexplorer find . --after "2025-11-07"
```

### Find by Name Pattern
```bash
# Find all Rust files (use --name, not --names!)
./target/release/fexplorer find . --name "*.rs"

# Find multiple patterns
./target/release/fexplorer find . --name "*.rs" --name "*.toml"
```

### Find by Size
```bash
# Files larger than 1MB
./target/release/fexplorer find . --min-size 1MB

# Files between 10KB and 1MB
./target/release/fexplorer find . --min-size 10KB --max-size 1MB
```

### Size Analysis
```bash
# Top 10 largest files/directories
./target/release/fexplorer size . --top 10 --aggregate

# Show all sizes sorted
./target/release/fexplorer size . --aggregate
```

### Directory Tree
```bash
# Show directory tree
./target/release/fexplorer tree src/

# Show tree with directories first
./target/release/fexplorer tree . --dirs-first
```

### Output Formats
```bash
# JSON output
./target/release/fexplorer find . --name "*.rs" --format json

# CSV output
./target/release/fexplorer find . --name "*.rs" --format csv

# HTML template
./target/release/fexplorer find . --name "*.rs" --template html

# Markdown template
./target/release/fexplorer find . --name "*.rs" --template markdown
```

## 🎯 The Command That Works

```bash
# Your original command (now working!)
./target/release/fexplorer find . --after "1 day ago" --template html > morning_changes.html
```

## ⚠️ Common Mistakes

| Wrong | Correct | Note |
|---------|-----------|------|
| --names "*.rs" | --name "*.rs" | Use singular form |
| --template without feature | Build with templates | Now enabled by default |

## 🔧 Default Features Enabled

- parallel - Fast parallel directory traversal
- templates - HTML/Markdown export
- grep - Content search in files

## 📚 More Help

```bash
./target/release/fexplorer --help
./target/release/fexplorer find --help
```
