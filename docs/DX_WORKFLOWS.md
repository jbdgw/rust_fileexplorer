# Developer Experience (DX) Workflows for fexplorer

This guide shows real-world developer workflows and how to maximize productivity with fexplorer.

## Table of Contents
- [Daily Development Workflows](#daily-development-workflows)
- [Code Review & Quality](#code-review--quality)
- [Project Maintenance](#project-maintenance)
- [Debugging & Investigation](#debugging--investigation)
- [Team Collaboration](#team-collaboration)
- [Performance & Optimization](#performance--optimization)

---

## Daily Development Workflows

### 1. Morning Catchup - "What Changed While I Was Away?"

**Scenario:** You come back to work and want to see what files changed recently.

```bash
# See all files modified in the last 24 hours
fexplorer find . --after "1 day ago" --category source

# With git integration - see what changed since yesterday
fexplorer git . --since "HEAD@{yesterday}"

# Create a morning report
fexplorer find . --after "1 day ago" --template html > morning_changes.html
open morning_changes.html
```

**Pro Tip:** Create a profile for this:
```toml
# ~/.config/fexplorer/config.toml
[profiles.morning-catchup]
command = "find"
description = "Files changed since yesterday"
args = { after = "1 day ago", category = "source" }
```

Then just run:
```bash
fexplorer run morning-catchup
```

### 2. Feature Branch Workflow - "What Am I Working On?"

**Scenario:** You're on a feature branch and want to see all your changes.

```bash
# See files you've modified compared to main
fexplorer git . --since main

# Filter to just your code changes (no config/docs)
fexplorer git . --since main --status modified | grep -E "\.(rs|ts|js)$"

# See the size impact of your changes
fexplorer git . --since main | xargs du -sh

# Export for code review prep
fexplorer git . --since main --template markdown > CHANGES.md
```

**Power Move:** Create a pre-commit review script:
```bash
#!/bin/bash
# review_changes.sh
fexplorer git . --since main --template html > /tmp/my_changes.html
echo "Modified files:"
fexplorer git . --since main --status modified
echo "\nReview HTML: file:///tmp/my_changes.html"
```

### 3. Quick File Navigation - "Where Is That File?"

**Scenario:** You need to find a file quickly without remembering the exact path.

```bash
# Find by partial name
fexplorer find . --name "*user*service*"

# Find recent files you were working on
fexplorer find . --after "3 hours ago" --category source

# Interactive browse to find it visually
fexplorer tui .
# Then type to filter: "user" → see all files with "user" in name
```

**Shell Function:** Add to `.bashrc`/`.zshrc`:
```bash
# Fuzzy find and cd to directory
fcd() {
    local dir=$(fexplorer find . --kind dir --name "*$1*" --format json | \
                jq -r '.[0].path' | xargs dirname)
    [ -n "$dir" ] && cd "$dir"
}

# Usage: fcd user → cd to directory containing user files
```

### 4. Todo Management - "What Needs Attention?"

**Scenario:** Track TODOs, FIXMEs, and HACKs in your codebase.

```bash
# Find all TODOs
fexplorer grep . "TODO" --ext rs --ext js --ext ts --line-numbers

# Find FIXMEs and HACKs
fexplorer grep . "FIXME|HACK" --regex --line-numbers

# Count by priority
fexplorer grep . "TODO.*urgent" --regex --case-insensitive | wc -l
```

**Create a Todo Dashboard:**
```bash
#!/bin/bash
# todo_report.sh
echo "# Project TODOs - $(date)" > TODOS.md
echo "" >> TODOS.md

echo "## Urgent (FIXME)" >> TODOS.md
fexplorer grep . "FIXME" --line-numbers >> TODOS.md

echo "\n## Todo Items" >> TODOS.md
fexplorer grep . "TODO" --line-numbers >> TODOS.md

echo "\n## Technical Debt (HACK)" >> TODOS.md
fexplorer grep . "HACK" --line-numbers >> TODOS.md

echo "Report saved to TODOS.md"
```

---

## Code Review & Quality

### 5. Pre-Review Quality Check

**Scenario:** Before submitting a PR, you want to check for common issues.

```bash
# Check for large files being added
fexplorer git . --since main --status staged | \
    xargs fexplorer size . --top 10

# Find debug statements left in code
fexplorer grep . "console\.log|debugger|println!" --regex --ext js --ext rs

# Check for unfinished work
fexplorer grep . "WIP|XXX|TEMP" --regex

# Find files without tests (assuming *_test.* naming)
fexplorer find . --category source | \
    while read f; do
        test_file="${f%.*}_test${f##*.}"
        [ ! -f "$test_file" ] && echo "Missing test: $f"
    done
```

**Profile for PR Checklist:**
```toml
[profiles.pr-check]
command = "find"
description = "Modified source files for PR review"
args = { category = "source", after = "1 day ago" }
```

### 6. Review Someone Else's PR

**Scenario:** You're reviewing a teammate's PR and want context.

```bash
# Get the PR branch
git fetch origin pull/123/head:pr-123
git checkout pr-123

# See what files changed
fexplorer git . --since main

# Find the biggest changes
fexplorer git . --since main --format json | \
    jq -r '.[].path' | \
    xargs wc -l | sort -rn | head -20

# Look for new dependencies (package.json, Cargo.toml, etc.)
fexplorer git . --since main --category config

# Find potential security issues
fexplorer grep . "unsafe|eval|exec|password|secret|token" --regex --case-insensitive

# Export review notes
fexplorer git . --since main --template markdown > PR_REVIEW.md
```

### 7. Code Duplication Detection

**Scenario:** Check for copy-pasted code that should be refactored.

```bash
# Find duplicate files (exact copies)
fexplorer duplicates . --min-size 1KB --category source

# Find similar function names (potential duplicates)
fexplorer grep . "fn handle.*" --regex --ext rs

# Create refactoring candidates report
fexplorer duplicates . --min-size 5KB --template html > duplicates_report.html
```

---

## Project Maintenance

### 8. Disk Space Cleanup

**Scenario:** Your project is taking too much space.

```bash
# Find the largest files
fexplorer size . --top 50

# Find large build artifacts
fexplorer find . --min-size 10MB --name "*target*" --name "*node_modules*"

# Find old log files
fexplorer find . --ext log --before "30 days ago"

# Clean up analysis
fexplorer find . --category archive --min-size 1MB | \
    awk '{sum+=$2} END {print "Archive space:", sum/1024/1024, "MB"}'
```

**Cleanup Script:**
```bash
#!/bin/bash
# cleanup.sh - Safe cleanup workflow

# Show what we'd delete (dry run)
echo "=== Large Build Artifacts ==="
fexplorer find . --min-size 100MB --name "*target*" --name "*build*" --name "*dist*"

echo "\n=== Old Logs ==="
fexplorer find . --ext log --before "30 days ago"

echo "\n=== Duplicate Files ==="
fexplorer duplicates . --min-size 1MB --summary

read -p "Delete old logs? (y/N) " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    fexplorer find . --ext log --before "30 days ago" --format json | \
        jq -r '.[].path' | xargs rm -v
fi
```

### 9. Dependency Audit

**Scenario:** Understanding your project's dependencies.

```bash
# Find all package/dependency files
fexplorer find . --name "package.json" --name "Cargo.toml" --name "requirements.txt"

# See when dependencies were last updated
fexplorer find . --name "package-lock.json" --name "Cargo.lock"

# Find outdated lock files (older than 30 days)
fexplorer find . --name "*.lock" --before "30 days ago"

# Create dependency inventory
fexplorer find . --category config --template markdown > DEPENDENCIES.md
```

### 10. Migration Tracking

**Scenario:** You're migrating from one technology to another.

```bash
# Track old vs new files
OLD_COUNT=$(fexplorer find . --ext jsx | wc -l)
NEW_COUNT=$(fexplorer find . --ext tsx | wc -l)
echo "Migration: $NEW_COUNT/$((OLD_COUNT + NEW_COUNT)) files converted"

# Find files still using old patterns
fexplorer grep . "var " --ext js  # Finding old var usage

# List files needing migration
fexplorer find . --ext jsx --template html > migration_todo.html
```

**Migration Dashboard:**
```bash
#!/bin/bash
# migration_status.sh

echo "=== Migration Status ==="
echo "Old (.jsx): $(fexplorer find . --ext jsx | wc -l) files"
echo "New (.tsx): $(fexplorer find . --ext tsx | wc -l) files"
echo "\nOldest unmigrated files:"
fexplorer find . --ext jsx --sort mtime --order asc | head -10
echo "\nRecently migrated:"
fexplorer find . --ext tsx --after "7 days ago"
```

---

## Debugging & Investigation

### 11. Bug Investigation - "When Did This Break?"

**Scenario:** A bug appeared and you need to find when.

```bash
# Find files modified in suspicious timeframe
fexplorer find . --after "2025-11-01" --before "2025-11-05" --category source

# See what changed in specific module
fexplorer git src/auth --since "2025-11-01"

# Find recent error handling changes
fexplorer grep . "catch|error|exception" --regex --ext ts | \
    xargs fexplorer find . --after "2025-11-01"

# Create investigation timeline
fexplorer find . --after "2025-11-01" --sort mtime --template html > timeline.html
```

### 12. Performance Investigation

**Scenario:** App is slow, need to find large/complex files.

```bash
# Find largest source files (complexity indicator)
fexplorer find . --category source --min-size 20KB --sort size --order desc

# Find files with many imports (coupling indicator)
fexplorer grep . "^import|^use " --regex --ext rs | \
    cut -d: -f1 | sort | uniq -c | sort -rn | head -20

# Find potential performance issues
fexplorer grep . "\.map\(.*\.map\(|loop.*loop|recursive" --regex --ext rs
```

### 13. Security Audit

**Scenario:** Quick security scan of codebase.

```bash
# Find files with potential secrets
fexplorer grep . "api[_-]?key|secret|password|token|credential" --regex --case-insensitive

# Find unsafe Rust code
fexplorer grep . "unsafe " --ext rs --line-numbers

# Find SQL injection risks
fexplorer grep . "execute\(.*format!|query\(.*\+|SELECT.*\+" --regex --ext rs

# Find files with TODO security notes
fexplorer grep . "TODO.*security|SECURITY|XXX.*auth" --regex --case-insensitive
```

**Security Report:**
```bash
#!/bin/bash
# security_audit.sh

echo "# Security Audit - $(date)" > SECURITY_AUDIT.md

echo "\n## Potential Secrets" >> SECURITY_AUDIT.md
fexplorer grep . "api[_-]?key|secret|password" --regex -i --line-numbers >> SECURITY_AUDIT.md

echo "\n## Unsafe Code Blocks" >> SECURITY_AUDIT.md
fexplorer grep . "unsafe " --ext rs --line-numbers >> SECURITY_AUDIT.md

echo "\n## Security TODOs" >> SECURITY_AUDIT.md
fexplorer grep . "TODO.*security|FIXME.*auth" --regex -i >> SECURITY_AUDIT.md

echo "Audit saved to SECURITY_AUDIT.md"
```

---

## Team Collaboration

### 14. Onboarding New Developer

**Scenario:** Help a new team member understand the codebase.

```bash
# Create project overview
fexplorer tree . --max-depth 3 > PROJECT_STRUCTURE.md

# Show most important files (largest/most complex)
fexplorer size . --top 20 --template html > important_files.html

# Find entry points
fexplorer find . --name "main.*" --name "index.*" --name "app.*"

# List all configuration
fexplorer find . --category config --template markdown > CONFIG_FILES.md

# Create newcomer guide
cat > NEWCOMER_GUIDE.md << EOF
# Project Guide for New Developers

## Project Structure
$(fexplorer tree . --max-depth 2)

## Key Files
$(fexplorer size . --top 10)

## Recent Activity
$(fexplorer find . --after "7 days ago" --category source)

## Configuration Files
$(fexplorer find . --category config)
EOF
```

### 15. Knowledge Transfer

**Scenario:** Document which files do what for the team.

```bash
# Find files by feature area
fexplorer find . --name "*auth*" --category source > docs/AUTH_FILES.md
fexplorer find . --name "*api*" --category source > docs/API_FILES.md
fexplorer find . --name "*db*" --name "*database*" > docs/DB_FILES.md

# Create module inventory
for module in auth api db payment; do
    echo "## $module module" >> MODULE_INVENTORY.md
    fexplorer find . --name "*$module*" --category source >> MODULE_INVENTORY.md
done

# Document test coverage
fexplorer find . --name "*test*" --name "*spec*" --category source \
    --template markdown > TEST_INVENTORY.md
```

### 16. Release Preparation

**Scenario:** Preparing for a release.

```bash
# Files changed since last release
git tag  # Find last tag
fexplorer git . --since v1.2.0

# Generate changelog data
fexplorer git . --since v1.2.0 --template markdown > CHANGES_v1.3.0.md

# Check for uncommitted work
fexplorer git . --status modified
fexplorer git . --status untracked

# Verify build artifacts are gitignored
fexplorer find . --name "*target*" --name "*dist*" --name "*.log" | \
    while read f; do
        git check-ignore -q "$f" || echo "NOT IGNORED: $f"
    done

# Pre-release checklist
cat > RELEASE_CHECKLIST.md << EOF
# Release v1.3.0 Checklist

## Changed Files
$(fexplorer git . --since v1.2.0)

## Tests Status
$(fexplorer find . --name "*test*" --after "1 day ago")

## Documentation Updates Needed
$(fexplorer git . --since v1.2.0 | grep -E "\.md$")
EOF
```

---

## Performance & Optimization

### 17. Build Time Investigation

**Scenario:** Build is slow, find bottlenecks.

```bash
# Find largest source files (compile time)
fexplorer find . --category source --sort size --order desc | head -30

# Find files with many dependencies
fexplorer grep . "^use|^import|^#include" --regex --ext rs --ext ts | \
    cut -d: -f1 | sort | uniq -c | sort -rn | head -20

# Check for circular dependencies (manual review needed)
fexplorer find . --category source --name "mod.rs" --name "index.*"
```

### 18. Asset Optimization

**Scenario:** Optimize images and media files.

```bash
# Find large images
fexplorer find . --category image --min-size 500KB

# Find unoptimized videos
fexplorer find . --category video --sort size --order desc

# Generate optimization report
fexplorer find . --category media --template html > media_audit.html

# Find image duplicates (same file, different names)
fexplorer duplicates . --category image --min-size 10KB
```

### 19. Code Coverage Analysis

**Scenario:** Find untested code.

```bash
# Find source files without corresponding test files
fexplorer find . --category source --ext rs | while read src; do
    test_file="${src%.rs}_test.rs"
    [ ! -f "$test_file" ] && echo "No test for: $src"
done

# Find oldest untested files
fexplorer find . --category source --sort mtime --order asc | \
    head -20 | while read f; do
        grep -l "$f" tests/* 2>/dev/null || echo "Untested: $f"
    done
```

---

## Advanced DX Patterns

### 20. Multi-Language Project Management

**Scenario:** Polyglot project with Rust, TypeScript, Python.

```bash
# Language breakdown
echo "Rust files: $(fexplorer find . --ext rs | wc -l)"
echo "TypeScript files: $(fexplorer find . --ext ts | wc -l)"
echo "Python files: $(fexplorer find . --ext py | wc -l)"

# Find mixed-language modules
fexplorer find . --ext rs --ext ts --ext py | \
    xargs dirname | sort | uniq -c | grep -v "^ *1 "

# Profile per language
cat >> ~/.config/fexplorer/config.toml << EOF
[profiles.rust-code]
command = "find"
args = { ext = ["rs"], category = "source" }

[profiles.typescript-code]
command = "find"
args = { ext = ["ts", "tsx"], category = "source" }

[profiles.python-code]
command = "find"
args = { ext = ["py"], category = "source" }
EOF
```

### 21. Monorepo Management

**Scenario:** Large monorepo with multiple projects.

```bash
# Find all package roots
fexplorer find . --name "package.json" --name "Cargo.toml"

# Workspace analysis
fexplorer find . --name "Cargo.toml" | while read toml; do
    dir=$(dirname "$toml")
    size=$(fexplorer size "$dir" --du | head -1 | awk '{print $1}')
    echo "$size - $dir"
done | sort -rn

# Find cross-workspace dependencies
fexplorer grep . "path = \"../" --ext toml
```

### 22. Documentation Health

**Scenario:** Ensure documentation is up to date.

```bash
# Find outdated docs (older than code)
CODE_MTIME=$(fexplorer find . --category source --sort mtime --order desc | head -1 | awk '{print $3}')
fexplorer find . --ext md --before "$CODE_MTIME"

# Find undocumented modules
fexplorer find . --category source --kind dir | while read dir; do
    [ ! -f "$dir/README.md" ] && echo "Missing README: $dir"
done

# Documentation coverage
DOCS=$(fexplorer find . --ext md | wc -l)
CODE=$(fexplorer find . --category source | wc -l)
echo "Documentation ratio: $DOCS docs for $CODE source files"
```

### 23. CI/CD Integration

**Scenario:** Use in CI pipeline for quality gates.

```bash
#!/bin/bash
# ci_quality_gates.sh

EXIT_CODE=0

# Check: No large files committed
LARGE_FILES=$(fexplorer git . --since main --min-size 1MB | wc -l)
if [ $LARGE_FILES -gt 0 ]; then
    echo "❌ FAIL: Large files detected"
    fexplorer git . --since main --min-size 1MB
    EXIT_CODE=1
fi

# Check: No debug statements
DEBUG=$(fexplorer grep . "console\.log|debugger|println!" --regex | wc -l)
if [ $DEBUG -gt 0 ]; then
    echo "❌ FAIL: Debug statements found"
    fexplorer grep . "console\.log|debugger" --regex
    EXIT_CODE=1
fi

# Check: No secrets
SECRETS=$(fexplorer grep . "api[_-]?key|secret|password" --regex -i | wc -l)
if [ $SECRETS -gt 0 ]; then
    echo "⚠️  WARNING: Potential secrets detected"
    fexplorer grep . "api[_-]?key|secret|password" --regex -i
fi

# Generate metrics
fexplorer git . --since main --template html > ci_changes.html

exit $EXIT_CODE
```

---

## Power User Tips

### Create a .fexplorerrc Toolkit

```bash
# ~/.fexplorerrc
alias fx='fexplorer'
alias fxl='fexplorer list'
alias fxf='fexplorer find'
alias fxg='fexplorer grep'
alias fxs='fexplorer size'
alias fxt='fexplorer tui'

# Quick find and edit
fxedit() {
    file=$(fexplorer find . --name "*$1*" --format json | jq -r '.[0].path')
    [ -n "$file" ] && $EDITOR "$file"
}

# Recent work
fxrecent() {
    fexplorer find . --after "1 day ago" --category source
}

# Git changes
fxchanged() {
    fexplorer git . --since ${1:-main}
}

# Size report
fxbig() {
    fexplorer size . --top ${1:-20} --template html > /tmp/size_report.html
    open /tmp/size_report.html
}

# Source in your .bashrc/.zshrc:
# source ~/.fexplorerrc
```

### Combine with Other Tools

```bash
# With fzf for interactive selection
fexplorer find . --ext rs --format json | \
    jq -r '.[].path' | \
    fzf --preview 'bat --color=always {}'

# With ripgrep for faster text search
fexplorer find . --category source | xargs rg "TODO"

# With git for advanced workflows
fexplorer git . --since main --format json | \
    jq -r '.[].path' | \
    xargs git add

# With watch for continuous monitoring
watch -n 5 'fexplorer size . --top 10'
```

---

## Conclusion

The key to maximizing fexplorer's DX value:

1. **Create Profiles** for repetitive tasks
2. **Script Common Workflows** and share with team
3. **Integrate with Git** for change tracking
4. **Export Reports** for documentation
5. **Combine with Unix Tools** for powerful pipelines
6. **Use TUI** for exploration and discovery

Happy developing! 🚀
