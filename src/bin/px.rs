//! px - Fast Project Switcher
//!
//! A CLI tool for quickly finding, opening, and managing git repositories
//! with fuzzy search and frecency-based ranking.

use anyhow::Result;
use clap::{Parser, Subcommand};
use rust_filesearch::config::PxConfig;
use rust_filesearch::px::{commands, index::ProjectIndex};

#[derive(Parser)]
#[command(name = "px")]
#[command(about = "Fast project switcher with fuzzy search and frecency tracking", version)]
#[command(author, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// List all projects
    List {
        /// Filter projects (has-changes, inactive-30d, inactive-90d)
        #[arg(long)]
        filter: Option<String>,

        /// Output format (json, path, pretty)
        #[arg(long, default_value = "pretty")]
        format: String,
    },

    /// Open project in editor
    Open {
        /// Project name/path query (fuzzy matched)
        query: String,

        /// Editor to use (code, cursor, vim, etc.)
        #[arg(long)]
        editor: Option<String>,
    },

    /// Show project information
    Info {
        /// Project name/path query (fuzzy matched)
        query: String,
    },

    /// Re-index projects by scanning configured directories
    Sync,

    /// Initialize px configuration
    Init,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let config = PxConfig::load()?;
    let mut index = ProjectIndex::load()?;

    match cli.command {
        Commands::List { filter, format: _ } => {
            commands::cmd_list(&index, filter)?;
        }
        Commands::Open { query, editor } => {
            let editor = editor.unwrap_or(config.default_editor);
            commands::cmd_open(&mut index, &query, &editor)?;
        }
        Commands::Info { query } => {
            commands::cmd_info(&index, &query)?;
        }
        Commands::Sync => {
            commands::cmd_sync(&mut index, &config.scan_dirs)?;
        }
        Commands::Init => {
            commands::cmd_init()?;
        }
    }

    Ok(())
}
