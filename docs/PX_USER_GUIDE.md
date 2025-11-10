# px User Guide
**Fast Project Switcher with Fuzzy Search and Frecency Tracking**

---

## 📚 Table of Contents

1. [Quick Start](#quick-start)
2. [Core Concepts](#core-concepts)
3. [Commands Reference](#commands-reference)
4. [Configuration](#configuration)
5. [Fuzzy Search & Frecency](#fuzzy-search--frecency)
6. [Workflows & Use Cases](#workflows--use-cases)
7. [Shell Integration](#shell-integration)
8. [iTerm2 Integration](#iterm2-integration)
9. [Troubleshooting](#troubleshooting)
10. [Power User Tips](#power-user-tips)

---

## Quick Start

### Installation

```bash
cd rust_filesearch
cargo install --path . --bin px
```

Installs `px` to `~/.cargo/bin/px`

### First Run

```bash
# 1. Initialize configuration
px init

# 2. Edit config to add your project directories
open ~/.config/px/config.toml

# 3. Scan for projects
px sync

# 4. List all projects
px list

# 5. Open a project
px open myproject
```

---

## Core Concepts

### What is px?

`px` is an intelligent project switcher that helps you navigate between git repositories quickly. It combines:

1. **Fast Discovery**: Scans configured directories for git repos
2. **Fuzzy Search**: Find projects with partial, out-of-order matches
3. **Frecency Ranking**: Projects you use often rank higher
4. **Smart Opening**: Opens both editor AND terminal at project

### How It Works

```
px sync
   ↓
Scans directories → Finds git repos → Extracts metadata → Caches to ~/.cache/px/
   ↓
px open myproject
   ↓
Fuzzy matches → Ranks by frecency → Opens Cursor + iTerm2 → Updates access stats
```

### Key Features

- ⚡ **Fast**: 243 projects indexed in ~12 seconds
- 🔍 **Fuzzy Search**: Type "wsg" to find "whatsgood-homepage"
- 🧠 **Smart Ranking**: Frequently-used projects surface first
- 🎯 **Dual Opening**: Editor + terminal in one command
- 📊 **Git Integration**: See branch, status, commits, ahead/behind
- 🎨 **Rich Display**: Colors, emojis, formatted tables

---

## Commands Reference

### `px init`
Initialize px configuration with sensible defaults.

```bash
px init
```

**Output:**
```
✓ Created px config at: ~/.config/px/config.toml

Edit this file to customize:
  - scan_dirs: directories to search for projects
  - default_editor: editor command (code, cursor, vim, etc.)
  - obsidian_vault: optional Obsidian vault path
```

**What it does:**
- Creates `~/.config/px/config.toml`
- Sets default scan directories (~/Developer, ~/projects, ~/code)
- Sets default editor (code)

---

### `px sync`
Scan configured directories and rebuild the project index.

```bash
px sync
```

**Output:**
```
Scanning 4 directories...
  • /Users/you/Developer/JB
  • /Users/you/Developer/DGW
  • /Users/you/Developer/peterbrooks
  • /Users/you/Developer/claude_code

✓ Indexed 243 projects in 12.02s
```

**What it does:**
1. Recursively scans scan_dirs (up to 3 levels deep)
2. Finds git repositories
3. Extracts metadata:
   - Current branch
   - Git status (uncommitted, ahead/behind)
   - Last commit info
   - README excerpt
4. Preserves frecency data for existing projects
5. Saves to `~/.cache/px/projects.json`

**When to run:**
- After adding new projects to your system
- After modifying scan_dirs in config
- Periodically (weekly) to refresh git status

---

### `px list`
List all indexed projects sorted by frecency.

```bash
# List all projects
px list

# Filter by uncommitted changes
px list --filter has-changes

# Filter by inactivity
px list --filter inactive-30d
px list --filter inactive-90d
```

**Output:**
```
Project                        Branch          Status
────────────────────────────────────────────────────────────
whatsgood-content              main            ⚠ changes
rust_filesearch                main            ↑ ahead
mentor4good                    main            ✓ clean
skill_seeker_mcp               development     ⚠ changes
...

Total: 243 projects
```

**Status indicators:**
- `✓ clean` - No uncommitted changes, synced with remote
- `⚠ changes` - Uncommitted changes (modified, staged, or untracked)
- `↑ ahead` - Commits ahead of remote
- `↓ behind` - Commits behind remote

**Filters:**
- `has-changes` - Projects with uncommitted changes
- `inactive-30d` - Not accessed in last 30 days
- `inactive-90d` - Not accessed in last 90 days

---

### `px open <query>`
Fuzzy search for a project and open it in editor + terminal.

```bash
# Open with default editor (from config)
px open whatsgood

# Override editor
px open rust --editor vim
px open mentor --editor cursor
```

**What it does:**
1. Fuzzy matches `<query>` against project names and paths
2. Ranks results by combined fuzzy score + frecency
3. Opens best match in configured editor (e.g., Cursor)
4. Creates new iTerm2 window, cd'd into project
5. Updates access stats (increments access_count, updates last_accessed)
6. Recalculates frecency score

**Output:**
```
Opening whatsgood-content in cursor + iTerm2...
  Path: /Users/you/Developer/claude_code/whatsgood-content
✓ Opened iTerm2 window at project directory
```

**Fuzzy matching examples:**
- `px open wsg` → matches "whatsgood-homepage"
- `px open rust` → matches "rust_filesearch"
- `px open mentor` → matches "mentor4good"
- `px open erpnext` → matches "erpnext_agent"

---

### `px info <query>`
Show detailed information about a project.

```bash
px info rust_filesearch
```

**Output:**
```
📁 rust_filesearch
============================================================
Path:     /Users/you/Developer/claude_code/rust_filesearch
Branch:   main
Status:   ⚠️  Uncommitted changes

Last commit:
  27d8d42 - fix: use US Central timezone for session timestamps
  by Jordan Bartlett at 2025-10-29 19:46

README:
  🦀 fexplorer - Fast File System Explorer

Access stats:
  Count:   5
  Last:    2025-01-10 14:30
  Score:   142.3
```

**Information displayed:**
- Full path
- Current branch
- Git status (clean, uncommitted, ahead/behind)
- Last commit (hash, message, author, timestamp)
- README excerpt (first non-empty line)
- Access statistics (if accessed before)

---

## Configuration

### Config File Location

```
~/.config/px/config.toml
```

### Default Configuration

```toml
scan_dirs = [
    "/Users/you/Developer",
    "/Users/you/projects",
    "/Users/you/code",
]
default_editor = "code"
```

### Recommended Configuration

```toml
scan_dirs = [
    "/Users/you/Developer/JB",
    "/Users/you/Developer/DGW",
    "/Users/you/Developer/peterbrooks",
    "/Users/you/Developer/claude_code",
]
default_editor = "cursor"

# Optional: Link to Obsidian vault
# obsidian_vault = "/Users/you/Documents/Obsidian/MyVault"
```

### Configuration Options

#### `scan_dirs`
Array of directories to scan for git repositories.

**Best practices:**
- Be specific (scan ~/Developer/client, not ~/Developer)
- Avoid home directory (too slow, too many results)
- Organize projects into logical folders

**Example:**
```toml
scan_dirs = [
    "/Users/you/work/acme-corp",
    "/Users/you/personal/side-projects",
    "/Users/you/learning/tutorials",
]
```

#### `default_editor`
Command to run when opening projects.

**Common values:**
- `"cursor"` - Cursor AI Editor
- `"code"` - VS Code
- `"vim"` - Vim
- `"nvim"` - Neovim
- `"subl"` - Sublime Text

**Requirements:**
- Command must be in PATH
- Command must accept directory path as argument

#### `obsidian_vault` (optional)
Path to Obsidian vault for note integration (future feature).

```toml
obsidian_vault = "/Users/you/Documents/Obsidian/DevNotes"
```

---

## Fuzzy Search & Frecency

### Fuzzy Search

Fuzzy search allows partial, out-of-order matches.

**Examples:**
| Query | Matches |
|-------|---------|
| `wsg` | "**w**hat**sg**ood-homepage" |
| `rfs` | "**r**ust-**f**ile**s**earch" |
| `m4g` | "**m**entor**4g**ood" |
| `skill` | "**skill**_seeker_mcp" |

**Algorithm:**
Uses SkimMatcherV2 (fuzzy-matcher crate) which:
- Scores matches by character proximity
- Favors consecutive matches
- Considers case (prefer exact case matches)
- Works on both project name and full path

### Frecency Ranking

Frecency = **Fre**quency + Re**cency**

Projects are ranked by a combined score:
```
final_score = fuzzy_score * 0.7 + frecency_score * 0.3
```

**Frequency component:**
```
frequency_score = ln(access_count + 1) * 10.0
```
- Logarithmic scaling prevents very high counts from dominating
- Projects accessed 10 times: ~23 points
- Projects accessed 100 times: ~46 points

**Recency component:**
```
recency_score = time_decay(last_accessed)
```
Time decay buckets:
- 0-4 days ago: **+100 points** (very recent)
- 5-14 days ago: **+70 points** (recent)
- 15-31 days ago: **+50 points** (this month)
- 32-90 days ago: **+30 points** (this quarter)
- 90+ days ago: **+10 points** (old)

**Example:**
```
Project A: accessed 5 times, 2 days ago
  frequency: ln(6) * 10 = 17.9
  recency: 100 (within 4 days)
  total: 117.9

Project B: accessed 20 times, 60 days ago
  frequency: ln(21) * 10 = 30.4
  recency: 30 (within 90 days)
  total: 60.4

Result: Project A ranks higher (more recent)
```

### How Frecency Evolves

```
Initial state (never accessed):
  access_count = 0
  last_accessed = None
  frecency_score = 0

After first access:
  access_count = 1
  last_accessed = now
  frecency_score = ln(2)*10 + 100 = 106.9

After 10 accesses over 2 weeks:
  access_count = 10
  last_accessed = 2 days ago
  frecency_score = ln(11)*10 + 100 = 124.0

After 30 accesses over 3 months (last access 45 days ago):
  access_count = 30
  last_accessed = 45 days ago
  frecency_score = ln(31)*10 + 30 = 64.3
```

**Key insight:** Recent activity matters more than historical frequency.

---

## Workflows & Use Cases

### Daily Workflow

#### Morning Startup
```bash
# See what you were working on yesterday
px list | head -10

# Jump to main project
px open client
```

#### Context Switching
```bash
# Working on feature
px open client

# Quick bug fix in different project
px open api

# Back to feature
px open client
```

#### End of Day Review
```bash
# See what you worked on today
px list | head -10

# Check for uncommitted work
px list --filter has-changes

# Commit changes before leaving
px info client  # See what changed
```

### Project Discovery

#### Find Projects by Name
```bash
# Vague memory of project name
px info skill    # Finds "skill_seeker_mcp"
px info mentor   # Finds "mentor4good"
```

#### Find Inactive Projects
```bash
# Find projects you haven't touched in 90+ days
px list --filter inactive-90d

# Review for archival
px info old-project
```

### Maintenance Workflows

#### Weekly Cleanup
```bash
# Find all uncommitted work
px list --filter has-changes

# For each project, review and commit
px open project1
# (in terminal: git status, git commit)
```

#### Monthly Sync
```bash
# Refresh project index
px sync

# Review recent activity
px list | head -20

# Archive inactive projects
px list --filter inactive-90d > inactive.txt
# (review list, archive projects)
```

### Team Workflows

#### Onboarding New Developer
```bash
# Developer clones all team repos to ~/work/
# Add scan directory
echo 'scan_dirs = ["/Users/dev/work"]' >> ~/.config/px/config.toml

# Index projects
px sync

# Now they can quickly navigate
px open api
px open frontend
px open docs
```

#### Multi-Project Features
```bash
# Feature spans multiple repos
px open api       # Backend changes
px open frontend  # UI changes
px open shared    # Common library changes

# All projects now rank high in frecency
px list | grep -E "api|frontend|shared"
```

---

## Shell Integration

### Recommended Aliases

Add to `~/.zshrc` or `~/.bashrc`:

```bash
# Short aliases
alias po='px open'
alias pi='px info'
alias pl='px list'
alias ps='px sync'

# Filtered lists
alias pc='px list --filter has-changes'
alias pin='px list --filter inactive-30d'
alias pr='px list | head -20'  # Recent projects
```

**Usage:**
```bash
po rust           # Quick open
pi mentor         # Quick info
pc                # Check uncommitted changes
pr                # See recent projects
```

### Advanced Functions

#### Quick cd with fzf
```bash
pcd() {
  local project=$(px list | awk '{print $1}' | fzf)
  if [ -n "$project" ]; then
    local path=$(px info "$project" | grep "Path:" | awk '{print $2}')
    cd "$path"
  fi
}
```

**Usage:**
```bash
pcd   # Interactive project picker → cd to project
```

#### Open Multiple Projects
```bash
pom() {
  for proj in "$@"; do
    px open "$proj"
  done
}
```

**Usage:**
```bash
pom client api frontend  # Open 3 projects at once
```

#### Project Status Dashboard
```bash
pstatus() {
  echo "📊 Project Status Dashboard"
  echo ""
  echo "Uncommitted changes:"
  px list --filter has-changes | tail -n +3 | wc -l
  echo ""
  echo "Inactive (30d):"
  px list --filter inactive-30d | tail -n +3 | wc -l
  echo ""
  echo "Inactive (90d):"
  px list --filter inactive-90d | tail -n +3 | wc -l
  echo ""
  echo "Most active projects:"
  px list | head -8
}
```

**Usage:**
```bash
pstatus  # Daily health check
```

### Prompt Integration

Show current project in shell prompt:

```bash
# Add to ~/.zshrc
px_current_project() {
  local current_dir="$PWD"
  px list --format json 2>/dev/null | \
    jq -r ".[] | select(.path == \"$current_dir\") | .name" 2>/dev/null
}

# Add to PROMPT (oh-my-zsh theme)
PROMPT='%{$fg[cyan]%}$(px_current_project)%{$reset_color%} %{$fg[yellow]%}%~%{$reset_color%} $ '
```

---

## iTerm2 Integration

### How It Works

When you run `px open`, it:

1. Opens project in Cursor (or configured editor)
2. Uses AppleScript to create new iTerm2 window
3. Automatically runs `cd /path/to/project`
4. Runs `clear` for clean slate

### AppleScript Behind the Scenes

```applescript
tell application "iTerm"
    create window with default profile
    tell current session of current window
        write text "cd '/path/to/project'"
        write text "clear"
    end tell
end tell
```

### Requirements

- iTerm2 must be installed
- Terminal/iTerm2 must have Automation permission
  - System Settings → Privacy & Security → Automation
  - Enable "Terminal" or "iTerm2" to control "iTerm2"

### Customization

#### Use Different Terminal Profile

Modify `src/px/commands.rs`:

```rust
let applescript = format!(
    r#"
    tell application "iTerm"
        create window with profile "MyCustomProfile"
        tell current session of current window
            write text "cd '{}'"
            write text "clear"
        end tell
    end tell
    "#,
    project_path.display()
);
```

#### Run Commands on Open

```rust
let applescript = format!(
    r#"
    tell application "iTerm"
        create window with default profile
        tell current session of current window
            write text "cd '{}'"
            write text "git status"
            write text "ls -la"
        end tell
    end tell
    "#,
    project_path.display()
);
```

#### Open in Existing Window (Tab)

```rust
let applescript = format!(
    r#"
    tell application "iTerm"
        tell current window
            create tab with default profile
            tell current session of current tab
                write text "cd '{}'"
                write text "clear"
            end tell
        end tell
    end tell
    "#,
    project_path.display()
);
```

### Troubleshooting

#### iTerm2 Window Doesn't Open

1. Check iTerm2 is installed: `ls /Applications/iTerm.app`
2. Check Automation permissions:
   - System Settings → Privacy & Security → Automation
   - Ensure iTerm2 or Terminal can control iTerm2
3. Test AppleScript manually:
   ```bash
   osascript -e 'tell application "iTerm" to create window with default profile'
   ```

#### Wrong Directory in Terminal

- Ensure project path doesn't contain special characters
- Paths are automatically escaped with single quotes

#### Terminal Opens But Immediately Closes

- Check default profile in iTerm2 preferences
- Ensure "Close session when it ends" is not set to "Always"

---

## Troubleshooting

### No Projects Indexed

**Symptom:**
```bash
px list
No projects indexed yet. Run `px sync` to scan for projects.
```

**Solution:**
```bash
px sync
```

**Cause:** Index is empty (first run, or cache deleted).

---

### Can't Find Project

**Symptom:**
```bash
px open myproject
No projects found matching 'myproject'
```

**Solutions:**

1. Check project is indexed:
   ```bash
   px list | grep myproject
   ```

2. Resync if project is new:
   ```bash
   px sync
   px open myproject
   ```

3. Check project is in scan_dirs:
   ```bash
   cat ~/.config/px/config.toml
   # Ensure project path is under one of scan_dirs
   ```

4. Verify project is a git repo:
   ```bash
   cd /path/to/myproject
   git status  # Should work
   ```

---

### Editor Fails to Open

**Symptom:**
```bash
px open rust
Opening rust_filesearch in code...
Error: Failed to spawn editor 'code'
Caused by: No such file or directory
```

**Solution:**

1. Check editor is in PATH:
   ```bash
   which cursor  # or: which code, which vim
   ```

2. Update config with correct editor:
   ```bash
   # Edit ~/.config/px/config.toml
   default_editor = "cursor"  # Change to correct command
   ```

3. Test editor manually:
   ```bash
   cursor /path/to/project  # Should work
   ```

---

### Slow Sync Performance

**Symptom:**
```bash
px sync
Scanning 4 directories...
✓ Indexed 243 projects in 45.23s  # Too slow!
```

**Solutions:**

1. Reduce scan depth (modify code):
   ```rust
   // In src/px/index.rs, line 115
   max_depth: Some(2),  // Change from 3 to 2
   ```

2. Narrow scan_dirs:
   ```toml
   # Instead of:
   scan_dirs = ["/Users/you/Developer"]

   # Use:
   scan_dirs = [
     "/Users/you/Developer/client",
     "/Users/you/Developer/api",
   ]
   ```

3. Check for network drives (slow):
   ```bash
   # Avoid scanning network-mounted directories
   ```

---

### Frecency Not Updating

**Symptom:**
Projects you use often don't rank higher.

**Solution:**

1. Check index is being saved:
   ```bash
   ls -lh ~/.cache/px/projects.json
   ```

2. Manually inspect index:
   ```bash
   cat ~/.cache/px/projects.json | jq '.projects[] | select(.name == "myproject")'
   ```

3. Clear cache and resync:
   ```bash
   rm ~/.cache/px/projects.json
   px sync
   ```

---

## Power User Tips

### Tip 1: Project Aliases

Create aliases for your most-used projects:

```bash
# Add to ~/.zshrc
alias poc='px open client'
alias poa='px open api'
alias pof='px open frontend'
alias pow='px open whatsgood'
```

### Tip 2: Daily Standup Report

```bash
#!/bin/bash
# ~/bin/px-standup

echo "🌅 Daily Standup Report"
echo "======================="
echo ""
echo "📊 Projects worked on yesterday:"
px list | head -10 | tail -n +3

echo ""
echo "⚠️  Projects with uncommitted changes:"
px list --filter has-changes | tail -n +3

echo ""
echo "🎯 Top 5 most active projects (by frecency):"
px list | head -8 | tail -n +3
```

Run every morning:
```bash
px-standup
```

### Tip 3: Auto-Sync on Shell Startup

Add to `~/.zshrc`:

```bash
# Auto-sync once per day
px_auto_sync() {
  local last_sync_file="$HOME/.cache/px/.last_sync_date"
  local today=$(date +%Y-%m-%d)

  if [ ! -f "$last_sync_file" ] || [ "$(cat $last_sync_file)" != "$today" ]; then
    echo "🔄 Syncing px projects..."
    px sync > /dev/null 2>&1
    echo "$today" > "$last_sync_file"
  fi
}

# Run on shell startup (background)
px_auto_sync &
```

### Tip 4: Project Jump History

Track your project navigation:

```bash
# Add to ~/.zshrc
px_open_with_history() {
  local project="$1"
  px open "$project"
  echo "$(date '+%Y-%m-%d %H:%M:%S') - $project" >> ~/.px_history
}

alias po='px_open_with_history'

# View history
px_history() {
  tail -20 ~/.px_history
}
```

### Tip 5: Project Groups

Open related projects together:

```bash
# Add to ~/.zshrc
px_group_open() {
  case "$1" in
    fullstack)
      px open frontend
      px open api
      px open shared
      ;;
    docs)
      px open documentation
      px open examples
      px open tutorials
      ;;
    *)
      echo "Unknown group: $1"
      echo "Available: fullstack, docs"
      ;;
  esac
}

alias pog='px_group_open'
```

**Usage:**
```bash
pog fullstack  # Opens frontend, api, shared
pog docs       # Opens documentation, examples, tutorials
```

---

## Best Practices

### 1. Organize Projects by Category

```toml
scan_dirs = [
    "/Users/you/work/client-projects",
    "/Users/you/work/internal-tools",
    "/Users/you/personal/side-projects",
    "/Users/you/learning/courses",
]
```

Benefits:
- Faster sync (smaller directories)
- Easier to find projects by category
- Better mental model of your projects

### 2. Use Descriptive Project Names

Good:
- `acme-client-portal`
- `ml-sentiment-analyzer`
- `react-component-library`

Bad:
- `project1`
- `app`
- `test`

Benefits:
- Better fuzzy search results
- Easier to identify in `px list`
- Self-documenting

### 3. Run Sync Weekly

```bash
# Add to crontab or launchd
# Every Sunday at 9am
0 9 * * 0 /Users/you/.cargo/bin/px sync
```

Benefits:
- Fresh git status
- Discover new projects
- Remove deleted projects

### 4. Review Inactive Projects Monthly

```bash
# First day of month
px list --filter inactive-90d > ~/inactive-projects-$(date +%Y-%m).txt
```

Benefits:
- Archive old projects
- Free up disk space
- Keep index lean

### 5. Commit Before Switching

```bash
# Wrapper that checks for uncommitted changes
px_open_safe() {
  local current_project=$(px list --format json | jq -r ".[] | select(.path == \"$PWD\") | .name")

  if [ -n "$current_project" ]; then
    if git status --short | grep -q .; then
      echo "⚠️  Uncommitted changes in $current_project"
      read -p "Commit first? (y/n) " -n 1 -r
      echo
      if [[ $REPLY =~ ^[Yy]$ ]]; then
        git add -A
        git commit
      fi
    fi
  fi

  px open "$1"
}

alias po='px_open_safe'
```

---

## Keyboard Shortcuts

### Recommended Setup (iTerm2)

1. Open iTerm2 → Preferences → Keys
2. Add global shortcut:
   - Hotkey: `⌥Space` (Option+Space)
   - Action: "Run Coprocess"
   - Command: `px open`

Now you can:
1. Press `⌥Space`
2. Type project name
3. Hit Enter
4. Project opens instantly

### Alternative: Alfred Workflow

Create Alfred workflow:

1. Keyword trigger: `px {query}`
2. Run Script:
   ```bash
   /Users/you/.cargo/bin/px open "$1"
   ```

Usage:
- `⌘Space` (Alfred hotkey)
- Type: `px whatsgood`
- Enter

---

## Future Features

Planned for upcoming versions:

### v0.3
- [ ] Interactive TUI mode (fzf-style picker)
- [ ] Project preview pane
- [ ] Configurable color themes

### v0.4
- [ ] Project tagging (`px tag add project1 work`)
- [ ] Filter by tags (`px list --tag work`)
- [ ] Project groups (open multiple projects)

### v0.5
- [ ] Obsidian integration
- [ ] Auto-create project notes
- [ ] Link projects to Obsidian notes

### v1.0
- [ ] AI-powered project summaries
- [ ] "What did I work on this week?"
- [ ] Smart project suggestions
- [ ] Git activity analysis

---

## Contributing

Want to contribute? Here's how:

### Report Bugs

```bash
# Create detailed bug report with:
px --version
px list | wc -l  # Number of indexed projects
cat ~/.config/px/config.toml
cat ~/.cache/px/projects.json | jq '.version'
```

### Request Features

Open issue with:
- Use case description
- Example commands
- Expected behavior

### Submit Pull Requests

```bash
git clone https://github.com/yourusername/rust_filesearch
cd rust_filesearch
cargo test  # Ensure tests pass
cargo fmt   # Format code
cargo clippy -- -D warnings  # Lint
```

---

## FAQ

### Q: Why did you build px instead of using cd?

**A:** `cd` requires remembering full paths. `px` lets you:
- Fuzzy search by partial names
- Jump to frequently-used projects faster (frecency)
- See project status at a glance
- Open editor + terminal in one command

### Q: How is this different from tmux or screen?

**A:** `px` is complementary:
- `tmux/screen`: Session management within terminal
- `px`: Project discovery and navigation across projects

Use together: `px open` to find project, `tmux` for session management.

### Q: Does px work with non-git projects?

**A:** Currently, no. `px` only indexes git repositories. Non-git projects are ignored during sync.

**Workaround:** Initialize git in your project:
```bash
cd myproject
git init
git add .
git commit -m "Initial commit"
px sync  # Now indexed
```

### Q: Can I use px with GitHub Codespaces or remote dev environments?

**A:** Yes! Install px on the remote machine:
```bash
cargo install --path . --bin px
px init
px sync
```

Configure scan_dirs to point to your codespace projects.

### Q: How much disk space does px use?

**A:** Minimal:
- Binary: ~8MB
- Cache: ~100KB per 100 projects
- Config: <1KB

Example: 243 projects = ~250KB cache file

### Q: Does px sync automatically?

**A:** No, you must run `px sync` manually. This is intentional:
- Sync takes time (seconds)
- User controls when to refresh
- Preserves frecency data

**Tip:** Set up weekly cron job for auto-sync.

### Q: Can I export my project list?

**A:** Yes! The cache is JSON:
```bash
cat ~/.cache/px/projects.json | jq .
```

Convert to other formats:
```bash
# CSV
cat ~/.cache/px/projects.json | jq -r '.projects[] | [.name, .path, .git_status.current_branch] | @csv'

# Markdown
cat ~/.cache/px/projects.json | jq -r '.projects[] | "- **\(.name)** - \(.path)"'
```

---

**Last Updated**: 2025-01-10
**Version**: 0.1.0
