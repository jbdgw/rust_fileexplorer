#!/bin/bash
# Simple test script for fexplorer v2.0 (no TUI)
# Safe to run in automated environments

set -e

FX="./target/release/fexplorer"
GREEN='\033[0;32m'
BLUE='\033[0;34m'
YELLOW='\033[1;33m'
RED='\033[0;31m'
NC='\033[0m'

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}fexplorer v2.0 - Simple Test Suite${NC}"
echo -e "${BLUE}========================================${NC}\n"

# Test counter
TESTS_PASSED=0
TESTS_FAILED=0

test_command() {
    local name="$1"
    local cmd="$2"

    echo -e "${YELLOW}Testing: ${NC}$name"
    echo -e "${BLUE}Command: ${NC}$cmd"

    if eval "$cmd" > /dev/null 2>&1; then
        echo -e "${GREEN}✓ PASSED${NC}\n"
        ((TESTS_PASSED++))
        return 0
    else
        echo -e "${RED}✗ FAILED${NC}\n"
        ((TESTS_FAILED++))
        return 1
    fi
}

test_with_output() {
    local name="$1"
    local cmd="$2"

    echo -e "${YELLOW}Testing: ${NC}$name"
    echo -e "${BLUE}Command: ${NC}$cmd"

    if output=$(eval "$cmd" 2>&1); then
        echo "$output" | head -5
        echo -e "${GREEN}✓ PASSED${NC}\n"
        ((TESTS_PASSED++))
        return 0
    else
        echo -e "${RED}✗ FAILED${NC}"
        echo "$output"
        echo ""
        ((TESTS_FAILED++))
        return 1
    fi
}

echo -e "${BLUE}=== Phase 1: Core Features ===${NC}\n"

test_command "Version check" "$FX --version"
test_command "Help output" "$FX --help"
test_with_output "List current directory" "$FX list . --max-depth 1"
test_with_output "Tree view" "$FX tree . --max-depth 2"
test_with_output "Find Rust files" "$FX find . --ext rs --max-depth 2"
test_with_output "Size analysis" "$FX size . --top 5"

echo -e "${BLUE}=== Phase 2: Enhanced Search ===${NC}\n"

# Create test files for grep
mkdir -p test_simple_grep
echo "Hello World" > test_simple_grep/file1.txt
echo "Goodbye World" > test_simple_grep/file2.txt

test_with_output "Content search (grep)" "$FX grep test_simple_grep 'Hello'"
test_with_output "Case insensitive grep" "$FX grep test_simple_grep 'hello' --case-insensitive"

# Create test files for duplicates
mkdir -p test_simple_dupes
echo "duplicate" > test_simple_dupes/dup1.txt
echo "duplicate" > test_simple_dupes/dup2.txt
echo "unique" > test_simple_dupes/unique.txt

test_with_output "Find duplicates" "$FX duplicates test_simple_dupes --min-size 0B"
test_with_output "Duplicate summary" "$FX duplicates test_simple_dupes --min-size 0B --summary"

test_with_output "Category filter - source" "$FX find . --category source --max-depth 2"
test_with_output "Category filter - config" "$FX find . --category config --max-depth 2"

# Cleanup
rm -rf test_simple_grep test_simple_dupes

echo -e "${BLUE}=== Phase 3: Workflows & Export ===${NC}\n"

test_command "Initialize profiles" "$FX profiles init"
test_with_output "List profiles" "$FX profiles list"
test_with_output "Show profile" "$FX profiles show recent-code"
test_with_output "Run profile" "$FX run recent-code . | head -5"

test_with_output "Export to JSON" "$FX list . --max-depth 1 --format json"
test_with_output "Export to CSV" "$FX list . --max-depth 1 --format csv"
test_with_output "Export to Markdown" "$FX list . --max-depth 1 --template markdown"

# Test HTML export (just check it succeeds)
test_command "Export to HTML" "$FX list . --max-depth 1 --template html > /tmp/fexplorer_test.html"

# Git integration (only if in a git repo)
if git rev-parse --git-dir > /dev/null 2>&1; then
    echo -e "${BLUE}=== Git Integration ===${NC}\n"
    test_with_output "Git status" "$FX git . | head -5"
else
    echo -e "${YELLOW}Skipping git tests (not in a git repository)${NC}\n"
fi

echo -e "${BLUE}=== Advanced Filters ===${NC}\n"

test_with_output "Find with size filter" "$FX find . --min-size 1KB --max-size 100KB --max-depth 2"
test_with_output "Find with date filter" "$FX find . --after '30 days ago' --max-depth 2"
test_with_output "Combined filters" "$FX find . --ext rs --min-size 5KB --max-depth 2"

echo -e "${BLUE}=== Sorting ===${NC}\n"

test_with_output "Sort by size" "$FX list . --sort size --order desc --max-depth 1"
test_with_output "Sort by mtime" "$FX list . --sort mtime --order desc --max-depth 1"
test_with_output "Sort by name" "$FX list . --sort name --dirs-first --max-depth 1"

echo -e "${BLUE}========================================${NC}"
echo -e "${BLUE}Test Results${NC}"
echo -e "${BLUE}========================================${NC}"
echo -e "${GREEN}Passed: $TESTS_PASSED${NC}"
echo -e "${RED}Failed: $TESTS_FAILED${NC}"
echo ""

if [ $TESTS_FAILED -eq 0 ]; then
    echo -e "${GREEN}All tests passed! ✓${NC}"
    exit 0
else
    echo -e "${RED}Some tests failed ✗${NC}"
    exit 1
fi
