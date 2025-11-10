use crate::errors::Result;
use crate::models::Entry;

/// Trait for output formatters
pub trait OutputSink {
    /// Write a single entry
    fn write(&mut self, entry: &Entry) -> Result<()>;

    /// Finish writing and flush any buffered data
    fn finish(&mut self) -> Result<()>;
}
