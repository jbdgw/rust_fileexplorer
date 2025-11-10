//! px - Project Switcher Module
//!
//! This module provides fast project management and switching capabilities
//! with fuzzy search, frecency tracking, and editor integration.

pub mod commands;
pub mod frecency;
pub mod index;
pub mod project;
pub mod search;

// Re-export main types for convenience
pub use index::ProjectIndex;
pub use project::{CommitInfo, Project, ProjectGitStatus};
