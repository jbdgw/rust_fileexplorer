# fexplorer Installation & Setup

## ✅ Installation Complete!

fexplorer is now installed globally at: `~/.cargo/bin/fexplorer`

## Verify Installation

```bash
# Check it's in your PATH
which fexplorer
# Should output: /Users/yourusername/.cargo/bin/fexplorer

# Check version
fexplorer --version
# Should output: fexplorer 0.1.0

# Test it works from any directory
cd ~
fexplorer find . --name "*.rs"
```

## Quick Usage

```bash
# Find files
fexplorer find . --name "*.rs"

# Find recent files and generate HTML report
fexplorer find . --after "1 day ago" --template html > report.html

# Show directory tree
fexplorer tree src/

# Find large files
fexplorer size . --top 10 --aggregate

# Get help
fexplorer --help
fexplorer find --help
```

## Optional: Shell Aliases

Add convenient shortcuts to your iTerm2/Zsh:

```bash
# Add this to your ~/.zshrc
source ~/.fexplorer_aliases.sh
```

Then reload your shell:
```bash
source ~/.zshrc
```

### Available Aliases

After sourcing the aliases file:

```bash
# Short commands
fx --help              # fexplorer
fxf . --name "*.rs"    # fexplorer find
fxt src/               # fexplorer tree
fxs . --top 10         # fexplorer size

# Quick tasks
fxtoday                # Files modified today
fxlarge                # Files larger than 10MB
fxtop                  # Top 10 largest files

# Functions
fxhtml .               # Generate HTML report and open in browser
fxopen rust            # Find and open file matching "rust"
```

## Troubleshooting

### "command not found: fexplorer"

Check if `~/.cargo/bin` is in your PATH:

```bash
echo $PATH | grep -q ".cargo/bin" && echo "✓ cargo/bin is in PATH" || echo "✗ Need to add cargo/bin to PATH"
```

If not in PATH, add to your `~/.zshrc`:

```bash
# Add this line to ~/.zshrc
export PATH="$HOME/.cargo/bin:$PATH"

# Then reload
source ~/.zshrc
```

### Reinstalling

If you make changes to the code:

```bash
cd /path/to/rust_filesearch
cargo install --path . --force
```

### Uninstalling

```bash
cargo uninstall fexplorer
```

## What's Installed

- **Binary:** `~/.cargo/bin/fexplorer`
- **Config location:** `~/.config/fexplorer/config.toml` (created on first use)
- **Cache location:** `~/.cache/px/` (for future px tool)

## Using in iTerm2

fexplorer works in:
- ✅ iTerm2
- ✅ Terminal.app
- ✅ Any shell (Zsh, Bash, Fish)
- ✅ From any directory

## Next Steps

1. Try it out: `fexplorer find ~ --name "*.pdf" --min-size 1MB`
2. Add aliases: `source ~/.fexplorer_aliases.sh`
3. Check out `QUICK_START.md` for more examples
4. Read `PX_IMPLEMENTATION_PLAN.md` for the upcoming project switcher

## Performance

fexplorer is fast:
- Parallel directory traversal (4 threads by default)
- Respects .gitignore by default
- Efficient caching for repeated operations

Customize thread count:
```bash
fexplorer find . --threads 8 --name "*.rs"
```
