#!/bin/bash
# Quick demonstration script for fexplorer v2.0 features

set -e  # Exit on error

FX="./target/release/fexplorer"

# Colors for output
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

header() {
    echo -e "\n${BLUE}========================================${NC}"
    echo -e "${BLUE}$1${NC}"
    echo -e "${BLUE}========================================${NC}\n"
}

info() {
    echo -e "${GREEN}▶${NC} $1"
}

command_demo() {
    echo -e "${YELLOW}$ $1${NC}"
    eval "$1"
    echo ""
}

# Check if binary exists
if [ ! -f "$FX" ]; then
    echo "Error: Binary not found at $FX"
    echo "Run: cargo build --release --all-features"
    exit 1
fi

header "1. Basic List Commands"
info "List current directory with max depth 2"
command_demo "$FX list . --max-depth 2 | head -10"

info "List sorted by size (largest first)"
command_demo "$FX list . --sort size --order desc --max-depth 1 | head -10"

header "2. Find Command with Filters"
info "Find all Rust source files"
command_demo "$FX find . --ext rs --max-depth 3 | head -10"

info "Find large files (>10KB)"
command_demo "$FX find . --min-size 10KB --max-depth 2 | head -10"

info "Find files modified in last 7 days"
command_demo "$FX find . --after '7 days ago' --max-depth 2 | head -10"

header "3. Size Analysis"
info "Top 10 largest files"
command_demo "$FX size . --top 10"

header "4. Content Search (Grep)"
info "Search for 'pub fn' in Rust files"
command_demo "$FX grep src 'pub fn' --ext rs --line-numbers | head -15"

header "5. Duplicate File Detection"
info "Creating test duplicate files..."
mkdir -p test_demo_dupes
echo "duplicate content" > test_demo_dupes/file1.txt
echo "duplicate content" > test_demo_dupes/file2.txt
echo "duplicate content" > test_demo_dupes/file3.txt
echo "unique content" > test_demo_dupes/file4.txt

info "Finding duplicates"
command_demo "$FX duplicates test_demo_dupes --min-size 0B"

info "Duplicate summary"
command_demo "$FX duplicates test_demo_dupes --min-size 0B --summary"

info "Cleaning up test files..."
rm -rf test_demo_dupes

header "6. Category Filters"
info "Find all source code files"
command_demo "$FX find . --category source --max-depth 2 | head -10"

info "Find all configuration files"
command_demo "$FX find . --category config --max-depth 2 | head -10"

header "7. Export to HTML"
info "Generating HTML report of largest files..."
command_demo "$FX size . --top 20 --template html > /tmp/fexplorer_report.html"
echo "Report saved to: /tmp/fexplorer_report.html"
echo "Open with: open /tmp/fexplorer_report.html (macOS) or xdg-open /tmp/fexplorer_report.html (Linux)"

header "8. Export to Markdown"
info "Generating Markdown list..."
command_demo "$FX list . --max-depth 2 --template markdown | head -30"

header "9. Profile Management"
info "Checking if profiles exist..."
if $FX profiles list 2>/dev/null | grep -q "No profiles"; then
    info "Initializing example profiles..."
    command_demo "$FX profiles init"
fi

info "Listing available profiles"
command_demo "$FX profiles list"

info "Running recent-code profile"
command_demo "$FX run recent-code . | head -10"

header "10. Output Formats"
info "JSON output (first 5 entries)"
command_demo "$FX list . --max-depth 1 --format json | jq '.[0:5]'"

info "CSV output"
command_demo "$FX list . --max-depth 1 --format csv | head -5"

header "11. Tree View"
info "Directory tree (depth 2)"
command_demo "$FX tree . --max-depth 2"

if git rev-parse --git-dir > /dev/null 2>&1; then
    header "12. Git Integration (if in git repo)"
    info "Files with git changes"
    command_demo "$FX git . | head -10"
else
    echo -e "\n${YELLOW}Skipping git integration demo (not in a git repository)${NC}\n"
fi

header "13. Interactive TUI Mode"
echo "Launch interactive mode with:"
echo -e "${YELLOW}$ $FX interactive .${NC}"
echo ""
echo "Controls:"
echo "  ↑↓ or j/k    - Navigate"
echo "  Type         - Filter files"
echo "  Enter        - Open directory"
echo "  - or ←       - Go up"
echo "  Ctrl+.       - Toggle hidden files"
echo "  Ctrl+d       - Toggle dirs-first"
echo "  Backspace    - Remove filter char"
echo "  Ctrl+u       - Clear filter"
echo "  q or Esc     - Quit"

header "Demo Complete!"
echo "See TESTING.md for comprehensive testing guide"
echo ""
echo "Quick commands to try:"
echo "  $FX --help                      # Show all commands"
echo "  $FX find . --ext rs             # Find Rust files"
echo "  $FX grep . 'TODO' --ext rs      # Search for TODOs"
echo "  $FX duplicates . --min-size 1KB # Find duplicates"
echo "  $FX interactive .               # Launch TUI"
echo ""
