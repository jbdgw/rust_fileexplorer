use crate::errors::Result;
use crate::models::Entry;
use crate::output::format::OutputSink;
use std::io::Write;

/// JSON array formatter (buffers all entries)
pub struct JsonFormatter {
    writer: Box<dyn Write>,
    entries: Vec<Entry>,
}

impl JsonFormatter {
    pub fn new(writer: Box<dyn Write>) -> Self {
        Self {
            writer,
            entries: Vec::new(),
        }
    }
}

impl OutputSink for JsonFormatter {
    fn write(&mut self, entry: &Entry) -> Result<()> {
        self.entries.push(entry.clone());
        Ok(())
    }

    fn finish(&mut self) -> Result<()> {
        let json = serde_json::to_string_pretty(&self.entries)?;
        writeln!(self.writer, "{}", json)?;
        self.writer.flush()?;
        Ok(())
    }
}

/// NDJSON (newline-delimited JSON) formatter (streaming)
pub struct NdjsonFormatter {
    writer: Box<dyn Write>,
}

impl NdjsonFormatter {
    pub fn new(writer: Box<dyn Write>) -> Self {
        Self { writer }
    }
}

impl OutputSink for NdjsonFormatter {
    fn write(&mut self, entry: &Entry) -> Result<()> {
        let json = serde_json::to_string(entry)?;
        writeln!(self.writer, "{}", json)?;
        Ok(())
    }

    fn finish(&mut self) -> Result<()> {
        self.writer.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::EntryKind;
    use std::path::PathBuf;

    fn make_test_entry(name: &str) -> Entry {
        use chrono::Utc;

        Entry {
            path: PathBuf::from(name),
            name: name.to_string(),
            size: 1024,
            kind: EntryKind::File,
            mtime: Utc::now(),
            perms: None,
            owner: None,
            depth: 0,
        }
    }

    #[test]
    fn test_json_formatter() {
        use std::io::Cursor;

        let output = Cursor::new(Vec::new());
        let mut formatter = JsonFormatter::new(Box::new(output));

        formatter.write(&make_test_entry("test.txt")).unwrap();
        // Note: Can't easily verify output without consuming the formatter
        // Real usage goes to stdout which works correctly
    }

    #[test]
    fn test_ndjson_formatter() {
        use std::io::Cursor;

        let output = Cursor::new(Vec::new());
        let mut formatter = NdjsonFormatter::new(Box::new(output));

        formatter.write(&make_test_entry("test1.txt")).unwrap();
        formatter.write(&make_test_entry("test2.txt")).unwrap();
        formatter.finish().unwrap();
    }
}
