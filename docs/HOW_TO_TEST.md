# How to Test fexplorer v2.0

## Quick Start

### Option 1: Run Automated Test Suite (Recommended)
```bash
./test_simple.sh
```

This runs a comprehensive test of all features except TUI (which requires a real terminal).

### Option 2: Manual Quick Tests
```bash
# Set up alias for convenience
alias fx='./target/release/fexplorer'

# Test basic commands
fx --version
fx list . --max-depth 2
fx find . --ext rs
fx size . --top 10
```

### Option 3: Test TUI (Must use real terminal)
**IMPORTANT:** Open a real terminal application first (Terminal.app, iTerm2, etc.)

```bash
cd /path/to/rust_filesearch
./target/release/fexplorer interactive .
```

The TUI will NOT work through automation or in non-interactive environments.

## What Each Test Script Does

### test_simple.sh
- ✅ Safe to run in automated environments
- ✅ Tests all features except TUI
- ✅ Creates temporary test files
- ✅ Cleans up after itself
- ✅ Shows pass/fail for each test
- ❌ Cannot test interactive TUI mode

**Run it:**
```bash
./test_simple.sh
```

### test_demo.sh
- ✅ Demonstrates all features with examples
- ✅ Shows actual command output
- ✅ More verbose than test_simple.sh
- ✅ Good for learning the commands
- ❌ Cannot test interactive TUI mode

**Run it:**
```bash
./test_demo.sh
```

## Test by Feature

### Core Features (Phase 1)
```bash
fx='./target/release/fexplorer'

# List
$fx list . --max-depth 2
$fx list . --sort size --order desc

# Tree
$fx tree . --max-depth 3

# Find
$fx find . --ext rs
$fx find . --min-size 10KB
$fx find . --after "7 days ago"

# Size
$fx size . --top 10
$fx size . --du
```

### Enhanced Search (Phase 2)
```bash
# Grep (content search)
$fx grep src "pub fn" --ext rs --line-numbers

# Duplicates
mkdir test_dup
echo "same" > test_dup/a.txt
echo "same" > test_dup/b.txt
$fx duplicates test_dup --min-size 0B
rm -rf test_dup

# Categories
$fx find . --category source
$fx find . --category config
```

### Workflows & Export (Phase 3)
```bash
# Profiles (already initialized)
$fx profiles list
$fx run recent-code .
$fx run large-files .

# Export templates
$fx list . --template markdown
$fx size . --top 20 --template html > report.html

# Git integration (in a git repo)
$fx git .
$fx git . --status modified
```

### Interactive (Phase 4)
**Open a real terminal first!**
```bash
$fx interactive .
# or
$fx tui .
```

## Testing Checklist

Use this checklist to verify all features:

### ✓ Phase 1 - Core Features
- [ ] `--version` shows version
- [ ] `--help` shows help
- [ ] `list` command works
- [ ] `list --sort size` works
- [ ] `tree` command displays tree
- [ ] `find --ext rs` finds Rust files
- [ ] `find --min-size 10KB` finds large files
- [ ] `find --after "7 days ago"` finds recent files
- [ ] `size --top 10` shows largest files

### ✓ Phase 2 - Enhanced Search
- [ ] `grep` finds text in files
- [ ] `grep --case-insensitive` works
- [ ] `grep --regex` works with patterns
- [ ] `duplicates` finds duplicate files
- [ ] `duplicates --summary` shows summary
- [ ] `find --category source` finds source files
- [ ] `find --category config` finds config files

### ✓ Phase 3 - Workflows
- [ ] `profiles list` shows profiles
- [ ] `profiles show <name>` shows profile details
- [ ] `run <profile>` executes profile
- [ ] `--template markdown` exports markdown
- [ ] `--template html` exports HTML
- [ ] `--format json` outputs JSON
- [ ] `--format csv` outputs CSV
- [ ] `git` command shows git status (in repo)

### ✓ Phase 4 - Interactive
- [ ] `interactive` launches TUI (in real terminal)
- [ ] ↑↓ navigation works
- [ ] Typing filters files
- [ ] Enter opens directories
- [ ] - goes up to parent
- [ ] Ctrl+. toggles hidden files
- [ ] q quits

## Expected Behavior

### List Command
```
<path>  <size>  <mtime>  <type>
./file.txt  1.23 KiB  2025-11-07 12:34:56  file
```

### Find Command
```
./src/main.rs  26.28 KiB  2025-11-07 18:32:55  file
./src/cli.rs  12.92 KiB  2025-11-07 18:00:17  file
```

### Grep Command
```
src/main.rs:26:5: pub fn main() -> Result<()> {

Found 1 matches in 1 files
```

### Duplicates Command
```
Duplicate Group #1 (hash: 1234abcd...)
  File size: 1.00 KiB
  Count: 3 files
  Wasted space: 2.00 KiB
  Files:
    - ./file1.txt
    - ./file2.txt
    - ./file3.txt
```

### Size Command
```
./src/main.rs  26.28 KiB  2025-11-07 18:32:55  file
./src/cli.rs  12.92 KiB  2025-11-07 18:00:17  file
./TESTING.md  10.28 KiB  2025-11-08 00:27:43  file
```

### Profiles Command
```
Available profiles:
  recent-code - Find recently modified source code files
  large-files - Find files larger than 100MB
  cleanup - Find old log and temp files for cleanup
```

## Common Issues

### "unrecognized subcommand"
**Solution:** Make sure you built with all features:
```bash
cargo build --release --all-features
```

### TUI Error: "Device not configured"
**Solution:** You're trying to run TUI in a non-interactive environment. Open a real terminal:
```bash
# macOS: Open Terminal.app or iTerm2
# Linux: Open gnome-terminal, konsole, xterm
# Then run the command
```

### "Profile already exists"
**Solution:** Profiles were already initialized. Just use them:
```bash
./target/release/fexplorer profiles list
./target/release/fexplorer run recent-code .
```

### Permission denied
**Solution:** Make sure the binary is executable:
```bash
chmod +x ./target/release/fexplorer
```

## Integration Tests

Run the Rust test suite:
```bash
# Run all tests
cargo test --all-features

# Run specific test module
cargo test --test integration_tests

# Run with output
cargo test -- --nocapture
```

## Performance Testing

```bash
# Install hyperfine for benchmarking
cargo install hyperfine

# Benchmark commands
hyperfine './target/release/fexplorer list . --max-depth 5'
hyperfine './target/release/fexplorer find . --ext rs'

# Compare with other tools
hyperfine './target/release/fexplorer find . --ext rs' 'find . -name "*.rs"'
```

## What to Test

### Basic Functionality
1. All commands run without errors
2. Output is properly formatted
3. Filters work as expected
4. Sorting works correctly

### Edge Cases
1. Large directories (1000+ files)
2. Deep nesting (10+ levels)
3. Unicode filenames
4. Filenames with spaces
5. Empty directories
6. Symlinks

### Performance
1. Large directory traversal
2. Parallel features (grep, find)
3. Duplicate detection on many files

### Integration
1. Piping to other commands
2. JSON output parsing
3. HTML/Markdown generation
4. Git integration in repos

## Quick Smoke Test

Run these commands to verify basic functionality:

```bash
fx='./target/release/fexplorer'

# 1. Version and help
$fx --version && echo "✓ Version" || echo "✗ Version"
$fx --help > /dev/null && echo "✓ Help" || echo "✗ Help"

# 2. Core commands
$fx list . --max-depth 1 > /dev/null && echo "✓ List" || echo "✗ List"
$fx tree . --max-depth 2 > /dev/null && echo "✓ Tree" || echo "✗ Tree"
$fx find . --ext rs > /dev/null && echo "✓ Find" || echo "✗ Find"
$fx size . --top 5 > /dev/null && echo "✓ Size" || echo "✗ Size"

# 3. Search features
$fx grep src "fn" --ext rs > /dev/null && echo "✓ Grep" || echo "✗ Grep"

# 4. Profiles
$fx profiles list > /dev/null && echo "✓ Profiles" || echo "✗ Profiles"

# 5. Export
$fx list . --format json > /dev/null && echo "✓ JSON" || echo "✗ JSON"
$fx list . --template markdown > /dev/null && echo "✓ Markdown" || echo "✗ Markdown"
```

## Full Test Run

To run a comprehensive test of all features:

```bash
# 1. Run automated test suite
./test_simple.sh

# 2. Run demo (see features in action)
./test_demo.sh

# 3. Run cargo tests
cargo test --all-features

# 4. Manually test TUI (in real terminal)
./target/release/fexplorer interactive .
```

## Documentation

- **TESTING.md** - Comprehensive testing guide with all scenarios
- **QUICK_START.md** - Quick reference for common use cases
- **test_simple.sh** - Automated test suite
- **test_demo.sh** - Feature demonstration script
- **HOW_TO_TEST.md** - This file

Happy testing! 🚀
