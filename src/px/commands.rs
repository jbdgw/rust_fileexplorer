//! CLI command implementations
//!
//! Implements the core px commands: list, open, info, sync, init

use crate::config::PxConfig;
use crate::errors::{FsError, Result};
use crate::px::index::ProjectIndex;
use crate::px::search::ProjectSearcher;
use chrono::Duration;
use std::path::PathBuf;
use std::process::Command;

/// Initialize px configuration
pub fn cmd_init() -> Result<()> {
    PxConfig::init()
}

/// Rebuild the project index by scanning configured directories
pub fn cmd_sync(index: &mut ProjectIndex, scan_dirs: &[PathBuf]) -> Result<()> {
    if scan_dirs.is_empty() {
        println!("⚠️  No scan directories configured!");
        println!("Run `px init` to create a config file, then edit:");
        println!("  {}", PxConfig::config_file_path()?.display());
        return Ok(());
    }

    println!("Scanning {} directories...", scan_dirs.len());
    for dir in scan_dirs {
        println!("  • {}", dir.display());
    }
    println!();

    let start = std::time::Instant::now();
    let count = index.sync(scan_dirs)?;
    let elapsed = start.elapsed();

    println!("✓ Indexed {} projects in {:.2}s", count, elapsed.as_secs_f64());

    Ok(())
}

/// List all projects with optional filtering
pub fn cmd_list(index: &ProjectIndex, filter: Option<String>) -> Result<()> {
    let mut projects: Vec<_> = index.sorted_projects();

    // Apply filters
    if let Some(ref filter_name) = filter {
        projects.retain(|p| match filter_name.as_str() {
            "has-changes" => p.git_status.has_uncommitted,
            "inactive-30d" => {
                let cutoff = chrono::Utc::now() - Duration::days(30);
                p.last_accessed.map_or(true, |t| t < cutoff)
            }
            "inactive-90d" => {
                let cutoff = chrono::Utc::now() - Duration::days(90);
                p.last_accessed.map_or(true, |t| t < cutoff)
            }
            _ => true,
        });
    }

    if projects.is_empty() {
        if filter.is_some() {
            println!("No projects found matching filter");
        } else {
            println!("No projects indexed yet. Run `px sync` to scan for projects.");
        }
        return Ok(());
    }

    // Print header
    println!("{:<30} {:<15} {:<8}", "Project", "Branch", "Status");
    println!("{}", "─".repeat(60));

    // Print projects
    for project in &projects {
        let status = if project.git_status.has_uncommitted {
            "⚠ changes"
        } else if project.git_status.ahead > 0 {
            "↑ ahead"
        } else if project.git_status.behind > 0 {
            "↓ behind"
        } else {
            "✓ clean"
        };

        println!(
            "{:<30} {:<15} {:<8}",
            truncate(&project.name, 28),
            truncate(&project.git_status.current_branch, 13),
            status
        );
    }

    println!();
    println!("Total: {} projects", projects.len());

    Ok(())
}

/// Open a project in an editor and iTerm2
pub fn cmd_open(index: &mut ProjectIndex, query: &str, editor: &str) -> Result<()> {
    let searcher = ProjectSearcher::new();
    let projects: Vec<_> = index.projects.values().cloned().collect();
    let results = searcher.search(&projects, query);

    if results.is_empty() {
        println!("No projects found matching '{}'", query);
        return Ok(());
    }

    let project = results[0];
    let project_path = project.path.clone();
    let project_name = project.name.clone();

    println!("Opening {} in {} + iTerm2...", project_name, editor);
    println!("  Path: {}", project_path.display());

    // Spawn editor
    let editor_status = Command::new(editor)
        .arg(&project_path)
        .status()
        .map_err(|e| FsError::IoError {
            context: format!("Failed to spawn editor '{}'", editor),
            source: e,
        })?;

    if !editor_status.success() {
        eprintln!("⚠️  Editor '{}' exited with error", editor);
    }

    // Open iTerm2 window at project directory
    let applescript = format!(
        r#"
        tell application "iTerm"
            create window with default profile
            tell current session of current window
                write text "cd '{}'"
                write text "clear"
            end tell
        end tell
        "#,
        project_path.display()
    );

    let iterm_result = Command::new("osascript")
        .arg("-e")
        .arg(&applescript)
        .status();

    match iterm_result {
        Ok(status) if status.success() => {
            println!("✓ Opened iTerm2 window at project directory");
        }
        Ok(_) => {
            eprintln!("⚠️  Failed to open iTerm2 window (check if iTerm2 is installed)");
        }
        Err(e) => {
            eprintln!("⚠️  Could not execute osascript: {}", e);
        }
    }

    // Record access for frecency tracking
    index.record_access(&project_path.to_string_lossy())?;

    Ok(())
}

/// Show detailed project information
pub fn cmd_info(index: &ProjectIndex, query: &str) -> Result<()> {
    let searcher = ProjectSearcher::new();
    let projects: Vec<_> = index.projects.values().cloned().collect();
    let results = searcher.search(&projects, query);

    if results.is_empty() {
        println!("No projects found matching '{}'", query);
        return Ok(());
    }

    let project = results[0];

    // Project header
    println!();
    println!("📁 {}", project.name);
    println!("{}", "=".repeat(60));

    // Basic info
    println!("Path:     {}", project.path.display());
    println!("Branch:   {}", project.git_status.current_branch);

    // Git status
    let status = if project.git_status.has_uncommitted {
        "⚠️  Uncommitted changes"
    } else {
        "✓ Clean"
    };
    println!("Status:   {}", status);

    // Ahead/behind
    if project.git_status.ahead > 0 || project.git_status.behind > 0 {
        println!(
            "Sync:     ↑ {} ahead, ↓ {} behind",
            project.git_status.ahead, project.git_status.behind
        );
    }

    // Last commit
    if let Some(ref commit) = project.git_status.last_commit {
        println!();
        println!("Last commit:");
        println!("  {} - {}", commit.hash, commit.message);
        println!(
            "  by {} at {}",
            commit.author,
            commit.timestamp.format("%Y-%m-%d %H:%M")
        );
    }

    // README excerpt
    if let Some(ref readme) = project.readme_excerpt {
        println!();
        println!("README:");
        println!("  {}", readme);
    }

    // Frecency stats
    if project.access_count > 0 {
        println!();
        println!("Access stats:");
        println!("  Count:   {}", project.access_count);
        if let Some(last_access) = project.last_accessed {
            println!(
                "  Last:    {}",
                last_access.format("%Y-%m-%d %H:%M")
            );
        }
        println!("  Score:   {:.1}", project.frecency_score);
    }

    println!();

    Ok(())
}

/// Helper to truncate strings with ellipsis
fn truncate(s: &str, max_len: usize) -> String {
    if s.len() <= max_len {
        s.to_string()
    } else {
        format!("{}...", &s[..max_len - 3])
    }
}

