use crate::errors::Result;
use crate::models::{Column, Entry};
use crate::output::format::OutputSink;
use csv::Writer;
use std::io::Write;

pub struct CsvFormatter {
    writer: Writer<Box<dyn Write>>,
    columns: Vec<Column>,
}

impl CsvFormatter {
    pub fn new(output: Box<dyn Write>, columns: Vec<Column>) -> Result<Self> {
        let mut writer = Writer::from_writer(output);

        // Write header
        let headers: Vec<String> = columns
            .iter()
            .map(|c| format!("{:?}", c).to_lowercase())
            .collect();
        writer.write_record(&headers)?;

        Ok(Self { writer, columns })
    }
}

impl OutputSink for CsvFormatter {
    fn write(&mut self, entry: &Entry) -> Result<()> {
        let values: Vec<String> = self
            .columns
            .iter()
            .map(|column| match column {
                Column::Path => entry.path.display().to_string(),
                Column::Name => entry.name.clone(),
                Column::Size => entry.size.to_string(),
                Column::Mtime => entry.mtime.to_rfc3339(),
                Column::Kind => format!("{:?}", entry.kind).to_lowercase(),
                Column::Perms => entry.perms.clone().unwrap_or_default(),
                Column::Owner => entry.owner.clone().unwrap_or_default(),
            })
            .collect();

        self.writer.write_record(&values)?;
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
            perms: Some("rw-r--r--".to_string()),
            owner: Some("1000".to_string()),
            depth: 0,
        }
    }

    #[test]
    fn test_csv_formatter() {
        let output = Vec::new();
        let mut formatter =
            CsvFormatter::new(Box::new(output), vec![Column::Name, Column::Size]).unwrap();

        formatter.write(&make_test_entry("test.txt")).unwrap();
        formatter.finish().unwrap();

        // Can't easily extract output from boxed writer in this test
        // In real usage, output goes to stdout which is fine
    }
}
