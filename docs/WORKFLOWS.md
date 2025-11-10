# Combined Workflows: fexplorer + px

**Leveraging Both Tools for Maximum Development Efficiency**

This guide shows powerful workflows that combine `fexplorer` (file exploration and analysis) with `px` (project management and switching) to supercharge your development environment.

---

## 🧠 Philosophy: Two Tools, One Workflow

**fexplorer** = Deep filesystem intelligence (find, analyze, report)
**px** = Fast project switching and workspace management (open, track, switch)

Together they form a complete development environment navigation system.

---

## 📚 Table of Contents

1. [Quick Reference](#quick-reference)
2. [Morning Startup Workflows](#morning-startup-workflows)
3. [Active Development Workflows](#active-development-workflows)
4. [Project Discovery & Analysis](#project-discovery--analysis)
5. [Maintenance & Cleanup Workflows](#maintenance--cleanup-workflows)
6. [Reporting & Documentation](#reporting--documentation)
7. [Advanced Integration Patterns](#advanced-integration-patterns)
8. [Shell Integration](#shell-integration)

---

## Quick Reference

### Command Comparison

| Task | fexplorer | px |
|------|-----------|-----|
| Find files in current project | `fx find . --name "*.rs"` | N/A |
| Switch to a project | N/A | `px open myproject` |
| List all projects | N/A | `px list` |
| Show project info | N/A | `px info myproject` |
| Find files across all projects | `fx find ~/Developer --name "*.rs"` | N/A |
| Show directory size | `fx size . --top 10` | N/A |
| List projects with changes | N/A | `px list --filter has-changes` |
| Git status in project | `fx git . --status modified` | (Shown in `px info`) |

### When to Use Each Tool

**Use fexplorer when:**
- You need to find specific files
- You want to analyze disk usage
- You need to generate reports
- You're exploring unfamiliar codebases
- You need structured output (JSON/CSV)

**Use px when:**
- You need to switch between projects
- You want to open a project quickly
- You need project-level statistics
- You want frecency-based project ranking
- You need to track project access patterns

---

## Morning Startup Workflows

### Workflow 1: Resume Yesterday's Work

**Goal:** Quickly find and open the project you were working on yesterday.

```bash
# Find your most recently accessed project
px list | head -5

# Open it directly
px open <project-name>

# See what changed overnight (if team project)
fx git . --since "yesterday"

# See what you were working on
fx git . --status modified
```

**Why it works:** `px` uses frecency to surface your most-used projects at the top. Combined with `fx git`, you instantly see your current state.

---

### Workflow 2: Check Status Across All Projects

**Goal:** See which projects have uncommitted changes before starting work.

```bash
# Quick overview
px list --filter has-changes

# Detailed view of each project with changes
px list --filter has-changes | tail -n +3 | head -n -1 | awk '{print $1}' | while read proj; do
  echo "=== $proj ==="
  px info $proj
  echo ""
done

# Find large uncommitted files
for proj in ~/Developer/*/; do
  if [ -d "$proj/.git" ]; then
    echo ">>> $(basename $proj)"
    fx git "$proj" --status modified | head -5
  fi
done
```

**Why it works:** `px` identifies projects with changes. `fx git` shows you exactly what's changed in each one.

---

### Workflow 3: Daily Standup Report

**Goal:** Generate a summary of what you worked on yesterday.

```bash
#!/bin/bash
# daily-standup.sh

echo "📊 Daily Standup Report for $(date +%Y-%m-%d)"
echo "================================================"
echo ""

echo "🏗️  Projects Accessed Yesterday:"
px list | head -10
echo ""

echo "📝 Files Modified Yesterday:"
fx find ~/Developer \
  --after "1 day ago" \
  --category source \
  --format pretty
echo ""

echo "⚠️  Projects with Uncommitted Changes:"
px list --filter has-changes
echo ""

echo "💾 Disk Usage (Top 5 Projects):"
for proj in ~/Developer/*/; do
  size=$(fx size "$proj" --du | head -1 | awk '{print $1}')
  name=$(basename "$proj")
  echo "  $size  $name"
done | sort -rh | head -5
```

**Output:**
```
📊 Daily Standup Report for 2025-01-10
================================================

🏗️  Projects Accessed Yesterday:
Project                        Branch          Status
────────────────────────────────────────────────────
whatsgood-homepage            main            ✓ clean
rust_filesearch               main            ⚠ changes
foster-greatness-web          feature/auth    ↑ ahead

📝 Files Modified Yesterday:
Name              Size      Modified
─────────────────────────────────────────
commands.rs       12.3 KB   2025-01-10 10:30
index.rs          8.9 KB    2025-01-10 09:15
...
```

---

## Active Development Workflows

### Workflow 4: Quick Project Switch with Context

**Goal:** Switch to a project and immediately see its current state.

```bash
# Create function in ~/.zshrc
pxo() {
  local query="$1"

  # Open project
  px open "$query"

  # Show project info
  px info "$query"

  # Show recent file activity
  echo ""
  echo "📄 Recently Modified Files:"
  fx find . --after "7 days ago" --ext rs,ts,py,go --format pretty | head -10

  # Show current branch and status
  echo ""
  echo "🌿 Git Status:"
  fx git . --status uncommitted
}

# Usage
pxo whatsgood
```

**Why it works:** You get immediate context: project info, recent changes, and git status all in one command.

---

### Workflow 5: Find Files Across All Projects

**Goal:** Search for a file or pattern across your entire development workspace.

```bash
# Find all Rust test files
fx find ~/Developer --name "*test*.rs" --format pretty

# Find large config files
fx find ~/Developer --category config --min-size 10KB

# Find recently modified TypeScript files
fx find ~/Developer --ext ts,tsx --after "3 days ago"

# Once you find the file, open its project
fx find ~/Developer --name "specific-file.ts" --format json | \
  jq -r '.[0].path' | \
  xargs dirname | \
  xargs -I {} px open "$(basename {})"
```

**Why it works:** `fexplorer` searches fast across all projects. Once you find what you need, `px` opens the parent project instantly.

---

### Workflow 6: Fuzzy Project Navigation with File Preview

**Goal:** Use fuzzy finding to navigate projects while seeing file contents.

```bash
# Install fzf if you haven't
brew install fzf

# Create function in ~/.zshrc
pxf() {
  local selected=$(px list --format plain | fzf --preview 'px info {}')

  if [ -n "$selected" ]; then
    px open "$selected"

    # Show recent activity
    echo ""
    fx find . --after "7 days ago" --format pretty | head -10
  fi
}

# Usage
pxf
```

**Why it works:** Interactive fuzzy search with live preview combines `px`'s project management with `fzf`'s powerful UI.

---

## Project Discovery & Analysis

### Workflow 7: Discover Unfamiliar Codebase

**Goal:** Quickly understand a new project's structure and recent activity.

```bash
#!/bin/bash
# discover-project.sh <project-name>

PROJECT="$1"

echo "🔍 Analyzing $PROJECT..."
echo "=========================="
echo ""

# Open project
px open "$PROJECT"

# Project metadata
echo "📊 Project Info:"
px info "$PROJECT"
echo ""

# Directory structure
echo "📁 Directory Structure:"
fx tree . --max-depth 2 --dirs-only
echo ""

# File breakdown by type
echo "📝 File Types:"
fx find . --format json | jq -r '.[].name | split(".") | .[-1]' | sort | uniq -c | sort -rn | head -10
echo ""

# Recent activity
echo "🕐 Recent Activity (Last 30 Days):"
fx find . --after "30 days ago" --category source --format pretty | head -15
echo ""

# Code size
echo "💾 Code Size:"
fx size . --top 10 --aggregate
echo ""

# Git history
echo "🌿 Git Activity:"
fx git . --since "30 days ago" | head -20
```

**Output gives you:**
- Project metadata (branch, commits, README)
- Directory structure overview
- File type distribution
- Recent development activity
- Size analysis
- Git change history

---

### Workflow 8: Find Similar Projects

**Goal:** Discover projects using similar technologies or patterns.

```bash
# Find all projects using a specific tech stack
find-stack() {
  local tech="$1"  # e.g., "react", "rust", "python"

  echo "Projects using $tech:"
  echo "======================"

  for proj in ~/Developer/*/; do
    proj_name=$(basename "$proj")

    # Check for indicator files
    case "$tech" in
      react)
        if fx find "$proj" --name "package.json" --format json | \
           jq -e '.[0] | select(.path)' > /dev/null 2>&1; then
          if grep -q "react" "$proj/package.json" 2>/dev/null; then
            echo "  ✓ $proj_name"
            px info "$proj_name" | grep "Branch:"
          fi
        fi
        ;;
      rust)
        if [ -f "$proj/Cargo.toml" ]; then
          echo "  ✓ $proj_name"
          px info "$proj_name" | grep "Branch:"
        fi
        ;;
      python)
        if fx find "$proj" --name "*.py" --format json | \
           jq -e '.[0] | select(.path)' > /dev/null 2>&1; then
          echo "  ✓ $proj_name"
          px info "$proj_name" | grep "Branch:"
        fi
        ;;
    esac
  done
}

# Usage
find-stack rust
find-stack react
find-stack python
```

---

## Maintenance & Cleanup Workflows

### Workflow 9: Identify Inactive Projects

**Goal:** Find projects you haven't touched in months for potential archival.

```bash
#!/bin/bash
# find-inactive-projects.sh

echo "🗄️  Inactive Projects Report"
echo "=============================="
echo ""

echo "Not accessed in 90+ days:"
px list --filter inactive-90d
echo ""

echo "Not accessed in 30+ days:"
px list --filter inactive-30d
echo ""

echo "Projects with old uncommitted changes:"
px list --filter has-changes | while read line; do
  proj=$(echo "$line" | awk '{print $1}')

  # Find oldest modified file
  oldest=$(fx find ~/Developer/"$proj" \
    --format json 2>/dev/null | \
    jq -r '.[] | "\(.mtime) \(.path)"' | \
    sort -n | head -1)

  if [ -n "$oldest" ]; then
    age_days=$(( ($(date +%s) - $(echo "$oldest" | awk '{print $1}')) / 86400 ))
    if [ "$age_days" -gt 30 ]; then
      echo "  $proj: uncommitted changes $age_days days old"
    fi
  fi
done
```

---

### Workflow 10: Cleanup Build Artifacts

**Goal:** Find and remove build artifacts (target/, node_modules/, etc.) to free disk space.

```bash
#!/bin/bash
# cleanup-build-artifacts.sh

echo "🧹 Finding Build Artifacts..."
echo "==============================="
echo ""

# Find target/ directories (Rust)
echo "📦 Rust target/ directories:"
for proj in ~/Developer/*/; do
  if [ -d "$proj/target" ]; then
    size=$(fx size "$proj/target" --du | head -1 | awk '{print $1}')
    echo "  $size  $(basename $proj)/target/"
  fi
done | sort -rh
echo ""

# Find node_modules/ directories
echo "📦 Node node_modules/ directories:"
for proj in ~/Developer/*/; do
  if [ -d "$proj/node_modules" ]; then
    size=$(fx size "$proj/node_modules" --du | head -1 | awk '{print $1}')
    echo "  $size  $(basename $proj)/node_modules/"
  fi
done | sort -rh
echo ""

# Calculate potential savings
echo "💾 Potential Disk Space Savings:"
total_target=$(find ~/Developer -type d -name "target" -exec du -sk {} \; | awk '{s+=$1} END {print s/1024/1024}')
total_node=$(find ~/Developer -type d -name "node_modules" -exec du -sk {} \; | awk '{s+=$1} END {print s/1024/1024}')
echo "  Rust target/: ${total_target} GB"
echo "  Node node_modules/: ${total_node} GB"
echo "  Total: $(echo "$total_target + $total_node" | bc) GB"
echo ""

read -p "Remove all target/ and node_modules/? (y/N) " confirm
if [[ "$confirm" == "y" ]]; then
  find ~/Developer -type d -name "target" -prune -exec rm -rf {} \;
  find ~/Developer -type d -name "node_modules" -prune -exec rm -rf {} \;
  echo "✓ Cleanup complete! Re-run 'px sync' to update index."
fi
```

---

### Workflow 11: Find Large Uncommitted Files

**Goal:** Identify large files that shouldn't be committed.

```bash
#!/bin/bash
# find-large-uncommitted.sh

echo "🚨 Large Uncommitted Files"
echo "==========================="
echo ""

px list --filter has-changes | tail -n +3 | head -n -1 | awk '{print $1}' | while read proj; do
  proj_path=$(px info "$proj" 2>/dev/null | grep "Path:" | awk '{print $2}')

  if [ -n "$proj_path" ]; then
    large_files=$(fx find "$proj_path" --min-size 1MB --format json 2>/dev/null)

    if [ -n "$large_files" ]; then
      uncommitted=$(fx git "$proj_path" --status modified 2>/dev/null)

      # Check for large files in uncommitted list
      echo "$large_files" | jq -r '.[] | "\(.size) \(.path)"' | while read size path; do
        filename=$(basename "$path")
        if echo "$uncommitted" | grep -q "$filename"; then
          size_mb=$(echo "scale=2; $size / 1024 / 1024" | bc)
          echo "  ⚠️  $proj: $filename (${size_mb} MB)"
        fi
      done
    fi
  fi
done
```

---

## Reporting & Documentation

### Workflow 12: Weekly Activity Report

**Goal:** Generate comprehensive weekly report combining project activity and file changes.

```bash
#!/bin/bash
# weekly-report.sh

WEEK=$(date +%Y-W%V)
REPORT_FILE="weekly_report_$WEEK.md"

cat > "$REPORT_FILE" <<EOF
# Weekly Development Report
**Week:** $WEEK
**Generated:** $(date +"%Y-%m-%d %H:%M")

---

## 📊 Project Activity

### Most Active Projects
$(px list | head -10)

### Projects with Uncommitted Changes
$(px list --filter has-changes)

---

## 📝 File Changes (Last 7 Days)

### Source Code Changes
EOF

# Add source code changes
fx find ~/Developer \
  --after "7 days ago" \
  --category source \
  --template markdown >> "$REPORT_FILE"

cat >> "$REPORT_FILE" <<EOF

---

## 🌿 Git Activity

### Modified Files by Project
EOF

# Add git status per project
px list --filter has-changes | tail -n +3 | head -n -1 | awk '{print $1}' | while read proj; do
  echo "#### $proj" >> "$REPORT_FILE"
  proj_path=$(px info "$proj" 2>/dev/null | grep "Path:" | awk '{print $2}')
  if [ -n "$proj_path" ]; then
    fx git "$proj_path" --status modified 2>/dev/null | head -10 >> "$REPORT_FILE"
  fi
  echo "" >> "$REPORT_FILE"
done

cat >> "$REPORT_FILE" <<EOF

---

## 💾 Disk Usage

### Largest Projects
$(for proj in ~/Developer/*/; do
  size=$(fx size "$proj" --du 2>/dev/null | head -1 | awk '{print $1}')
  name=$(basename "$proj")
  echo "  $size  $name"
done | sort -rh | head -10)

---

## 📈 Statistics

- Total projects tracked: $(px list 2>/dev/null | tail -1 | awk '{print $2}')
- Projects modified this week: $(px list --filter has-changes 2>/dev/null | wc -l)
- Files changed: $(fx find ~/Developer --after "7 days ago" --category source --format json 2>/dev/null | jq '. | length')

EOF

echo "✓ Report generated: $REPORT_FILE"
open "$REPORT_FILE"
```

---

### Workflow 13: Project Portfolio Website

**Goal:** Generate HTML dashboard of all your projects.

```bash
#!/bin/bash
# generate-portfolio.sh

OUTPUT="portfolio.html"

cat > "$OUTPUT" <<'EOF'
<!DOCTYPE html>
<html>
<head>
  <title>My Projects</title>
  <style>
    body { font-family: system-ui; max-width: 1200px; margin: 0 auto; padding: 20px; }
    h1 { color: #333; }
    .project { border: 1px solid #ddd; padding: 15px; margin: 10px 0; border-radius: 8px; }
    .project h3 { margin-top: 0; }
    .stats { color: #666; font-size: 0.9em; }
    .changes { color: #d73a49; }
    .clean { color: #28a745; }
  </style>
</head>
<body>
  <h1>📁 My Development Projects</h1>
  <p>Generated: $(date +"%Y-%m-%d %H:%M")</p>
EOF

# Add project cards
px list --format plain | while read proj; do
  info=$(px info "$proj" 2>/dev/null)

  path=$(echo "$info" | grep "Path:" | sed 's/Path:     //')
  branch=$(echo "$info" | grep "Branch:" | sed 's/Branch:   //')
  status=$(echo "$info" | grep "Status:" | sed 's/Status:   //')

  # Get file counts
  file_count=$(fx find "$path" --kind file --format json 2>/dev/null | jq '. | length')

  # Get size
  size=$(fx size "$path" --du 2>/dev/null | head -1 | awk '{print $1}')

  # Determine status class
  if echo "$status" | grep -q "Clean"; then
    status_class="clean"
  else
    status_class="changes"
  fi

  cat >> "$OUTPUT" <<CARD
  <div class="project">
    <h3>$proj</h3>
    <div class="stats">
      <strong>Branch:</strong> $branch<br>
      <strong>Status:</strong> <span class="$status_class">$status</span><br>
      <strong>Files:</strong> $file_count<br>
      <strong>Size:</strong> $size<br>
      <strong>Path:</strong> <code>$path</code>
    </div>
  </div>
CARD
done

cat >> "$OUTPUT" <<'EOF'
</body>
</html>
EOF

echo "✓ Portfolio generated: $OUTPUT"
open "$OUTPUT"
```

---

## Advanced Integration Patterns

### Workflow 14: Project Health Check

**Goal:** Automated health check for code quality issues.

```bash
#!/bin/bash
# project-health-check.sh <project-name>

PROJECT="$1"
PROJECT_PATH=$(px info "$PROJECT" 2>/dev/null | grep "Path:" | awk '{print $2}')

if [ -z "$PROJECT_PATH" ]; then
  echo "Project not found"
  exit 1
fi

echo "🏥 Project Health Check: $PROJECT"
echo "===================================="
echo ""

# Check for uncommitted changes
echo "1️⃣  Git Status:"
if fx git "$PROJECT_PATH" --status uncommitted 2>/dev/null | grep -q "^"; then
  echo "   ⚠️  Uncommitted changes detected"
else
  echo "   ✓ Clean working tree"
fi
echo ""

# Check for large files
echo "2️⃣  Large Files (>1MB):"
large_count=$(fx find "$PROJECT_PATH" --min-size 1MB --format json 2>/dev/null | jq '. | length')
if [ "$large_count" -gt 0 ]; then
  echo "   ⚠️  Found $large_count large files"
  fx find "$PROJECT_PATH" --min-size 1MB --format pretty | head -5
else
  echo "   ✓ No large files"
fi
echo ""

# Check for old files
echo "3️⃣  Stale Files (>6 months):"
old_count=$(fx find "$PROJECT_PATH" --before "180 days ago" --format json 2>/dev/null | jq '. | length')
if [ "$old_count" -gt 10 ]; then
  echo "   ⚠️  Found $old_count files not modified in 6+ months"
else
  echo "   ✓ Most files recently modified"
fi
echo ""

# Check for TODO/FIXME
echo "4️⃣  TODOs/FIXMEs:"
if command -v rg > /dev/null; then
  todo_count=$(rg -c "TODO|FIXME" "$PROJECT_PATH" 2>/dev/null | wc -l)
  echo "   ℹ️  Found $todo_count files with TODOs/FIXMEs"
else
  echo "   ⊘  ripgrep not installed (skip)"
fi
echo ""

# Check disk usage
echo "5️⃣  Disk Usage:"
size=$(fx size "$PROJECT_PATH" --du 2>/dev/null | head -1 | awk '{print $1}')
echo "   📊 Total size: $size"
echo ""

# Overall score
echo "======================================"
echo "✓ Health check complete"
```

---

### Workflow 15: Automated Project Sync

**Goal:** Keep px index and file system in sync automatically.

```bash
#!/bin/bash
# auto-sync.sh

# Add to crontab: */30 * * * * /path/to/auto-sync.sh

LOG_FILE="$HOME/.local/share/px/sync.log"

{
  echo "=== Sync started at $(date) ==="

  # Sync px index
  px sync

  # Check for new projects
  NEW_COUNT=$(px list 2>/dev/null | tail -1 | awk '{print $2}')
  echo "Total projects: $NEW_COUNT"

  # Generate summary
  echo "Projects with changes: $(px list --filter has-changes 2>/dev/null | wc -l)"
  echo "Inactive 30d: $(px list --filter inactive-30d 2>/dev/null | wc -l)"

  echo "=== Sync complete at $(date) ==="
  echo ""
} >> "$LOG_FILE" 2>&1
```

**Setup:**
```bash
# Make executable
chmod +x auto-sync.sh

# Add to crontab (every 30 minutes)
crontab -e
# Add line:
# */30 * * * * /path/to/auto-sync.sh
```

---

## Shell Integration

### Complete .zshrc Integration

Add this to your `~/.zshrc` for seamless integration:

```bash
# ============================================
# fexplorer + px Integration
# ============================================

# Aliases
alias fx='fexplorer'
alias fxf='fexplorer find'
alias fxt='fexplorer tree'
alias fxs='fexplorer size'
alias fxg='fexplorer git'

# Quick open with context
pxo() {
  local query="$1"
  px open "$query"
  echo ""
  echo "📄 Recent Changes:"
  fx find . --after "7 days ago" --format pretty | head -10
  echo ""
  fx git . --status uncommitted
}

# Fuzzy project finder
pxf() {
  if ! command -v fzf &> /dev/null; then
    echo "fzf not installed. Run: brew install fzf"
    return 1
  fi

  local selected=$(px list --format plain 2>/dev/null | \
    fzf --preview 'px info {} 2>/dev/null' \
        --preview-window=right:60%:wrap)

  if [ -n "$selected" ]; then
    pxo "$selected"
  fi
}

# Find and open
fxo() {
  local pattern="$1"
  local file=$(fx find ~/Developer --name "*$pattern*" --format json | \
    jq -r '.[0].path')

  if [ -n "$file" ] && [ "$file" != "null" ]; then
    local proj_dir=$(dirname "$file")
    while [ "$proj_dir" != "$HOME/Developer" ] && [ ! -d "$proj_dir/.git" ]; do
      proj_dir=$(dirname "$proj_dir")
    done

    if [ -d "$proj_dir/.git" ]; then
      local proj_name=$(basename "$proj_dir")
      echo "Found in project: $proj_name"
      echo "File: $file"
      echo ""
      read -p "Open project? (Y/n) " confirm
      if [[ "$confirm" != "n" ]]; then
        pxo "$proj_name"
      fi
    else
      echo "File found but not in a git project: $file"
    fi
  else
    echo "No files matching '$pattern' found"
  fi
}

# Daily standup
standup() {
  echo "📊 Daily Standup - $(date +%Y-%m-%d)"
  echo "====================================="
  echo ""
  echo "🏗️  Most Active Projects:"
  px list | head -5
  echo ""
  echo "📝 Files Modified (Last 24h):"
  fx find ~/Developer --after "1 day ago" --category source | head -10
  echo ""
  echo "⚠️  Uncommitted Changes:"
  px list --filter has-changes
}

# Project health
health() {
  local project="${1:-.}"

  if [ "$project" = "." ]; then
    # Current directory
    echo "🏥 Health Check: $(basename $(pwd))"
  else
    echo "🏥 Health Check: $project"
  fi

  echo "===================================="
  echo ""

  echo "Git Status:"
  fx git "$project" --status uncommitted | head -10
  echo ""

  echo "Large Files:"
  fx find "$project" --min-size 1MB --top 5
  echo ""

  echo "Disk Usage:"
  fx size "$project" --du
}

# Quick stats
pxstats() {
  echo "📊 Project Statistics"
  echo "======================"
  echo ""
  echo "Total projects: $(px list 2>/dev/null | tail -1 | awk '{print $2}')"
  echo "With changes: $(px list --filter has-changes 2>/dev/null | wc -l | xargs)"
  echo "Inactive 30d: $(px list --filter inactive-30d 2>/dev/null | wc -l | xargs)"
  echo "Inactive 90d: $(px list --filter inactive-90d 2>/dev/null | wc -l | xargs)"
  echo ""
  echo "Disk Usage (Top 5):"
  for proj in ~/Developer/*/; do
    size=$(fx size "$proj" --du 2>/dev/null | head -1 | awk '{print $1}')
    name=$(basename "$proj")
    echo "  $size  $name"
  done | sort -rh | head -5
}

echo "✓ fexplorer + px integration loaded"
echo "  Commands: pxo, pxf, fxo, standup, health, pxstats"
```

**Reload shell:**
```bash
source ~/.zshrc
```

---

## Tips for Maximum Efficiency

### 1. Use Aliases Everywhere

```bash
# Short aliases save time
alias p='px open'
alias pi='px info'
alias pl='px list'

# Open most recent project
alias pr='px open $(px list | head -4 | tail -1 | awk "{print \$1}")'
```

### 2. Combine with Other Tools

```bash
# fzf integration
fx find . --format json | jq -r '.[].path' | fzf

# ripgrep integration
fx find . --name "*.rs" --format json | jq -r '.[].path' | xargs rg "TODO"

# Open in editor
fx find . --name "config.rs" --format json | jq -r '.[0].path' | xargs code
```

### 3. Create Project Templates

```bash
# Save common searches as functions
find-tests() {
  fx find "${1:-.}" --name "*test*" --ext rs,ts,py
}

find-configs() {
  fx find "${1:-.}" --category config
}

find-recent() {
  fx find "${1:-.}" --after "${2:-7 days ago}" --category source
}
```

### 4. Automate Repetitive Tasks

```bash
# Daily morning routine
morning() {
  echo "☀️  Good morning! Here's your dev environment status:"
  echo ""
  standup
  echo ""
  pxstats
  echo ""
  echo "Running sync..."
  px sync -q
  echo "✓ Ready to code!"
}

# End of day wrap-up
eod() {
  echo "🌙 End of day wrap-up:"
  echo ""
  px list --filter has-changes
  echo ""
  read -p "Generate daily report? (y/N) " confirm
  if [[ "$confirm" == "y" ]]; then
    standup > "daily_$(date +%Y%m%d).md"
    echo "✓ Report saved"
  fi
}
```

---

## Performance Tips

### Optimize px Sync

```bash
# Sync only specific directories
px sync ~/Developer/active-projects

# Run sync in background
(px sync > /dev/null 2>&1 &)
```

### Optimize fexplorer Searches

```bash
# Use --max-depth for shallow searches
fx find . --max-depth 2 --name "*.rs"

# Use --threads for parallel processing
fx find ~/Developer --threads 8 --name "*.rs"

# Use specific extensions instead of broad searches
fx find . --ext rs,toml  # Fast
fx find . --name "*"      # Slow
```

### Cache Results

```bash
# Cache large searches
fx find ~/Developer --format json > /tmp/all-files.json

# Use cached results
cat /tmp/all-files.json | jq '.[] | select(.size > 1000000)'
```

---

## Troubleshooting

### "px: command not found"

```bash
# Ensure installed
cargo install --path . --bin px

# Check PATH
echo $PATH | grep -q ".cargo/bin" || echo 'export PATH="$HOME/.cargo/bin:$PATH"' >> ~/.zshrc
```

### "No projects found"

```bash
# Run sync
px sync

# Check config
cat ~/.config/px/config.toml

# Verify scan directories exist
ls ~/Developer/JB
```

### Slow Performance

```bash
# Reduce scan depth in config
# Edit ~/.config/px/config.toml and adjust max_depth

# Exclude large directories
# Add to .gitignore in project roots: target/, node_modules/
```

---

## Next Steps

1. **Install both tools:**
   ```bash
   cargo install --path . --bin fexplorer
   cargo install --path . --bin px
   ```

2. **Set up shell integration:**
   ```bash
   # Add functions from "Shell Integration" section to ~/.zshrc
   source ~/.zshrc
   ```

3. **Configure px:**
   ```bash
   px init
   # Edit ~/.config/px/config.toml with your directories
   px sync
   ```

4. **Try the workflows:**
   ```bash
   # Start with morning routine
   morning

   # Explore your projects
   pxf

   # Try fuzzy file finding
   fxo component
   ```

5. **Customize to your needs:**
   - Add your own aliases
   - Create project-specific profiles
   - Build automation scripts

---

**Happy coding! 🚀**

*Last updated: 2025-01-10*
