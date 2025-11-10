// Library interface for fexplorer
// Allows using fexplorer functionality as a library

pub mod cli;
pub mod config;
pub mod errors;
pub mod fs;
pub mod models;
pub mod output;
pub mod util;

#[cfg(feature = "tui")]
pub mod tui;

// px project switcher module
pub mod px;

pub use errors::{FsError, Result};
pub use models::{Column, Entry, EntryKind, OutputFormat, SortKey, SortOrder};
