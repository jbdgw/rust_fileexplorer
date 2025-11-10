#[cfg(feature = "tui")]
pub mod app;
#[cfg(feature = "tui")]
pub mod ui;

#[cfg(feature = "tui")]
pub use app::App;
