pub mod filters;
pub mod metadata;
pub mod size;
pub mod traverse;
pub mod watch;

#[cfg(feature = "grep")]
pub mod content;

#[cfg(feature = "dedup")]
pub mod dedup;

#[cfg(feature = "git")]
pub mod git;
