use crate::errors::Result;
use crate::models::{Column, Entry, EntryKind};
use crate::output::format::OutputSink;
use crate::util::{format_size_human, is_tty};
use nu_ansi_term::Color;
use std::io::Write;

pub struct PrettyFormatter {
    writer: Box<dyn Write>,
    columns: Vec<Column>,
    use_color: bool,
}

impl PrettyFormatter {
    pub fn new(writer: Box<dyn Write>, columns: Vec<Column>, no_color: bool) -> Self {
        let use_color = is_tty() && !no_color;
        Self {
            writer,
            columns,
            use_color,
        }
    }

    fn format_entry(&self, entry: &Entry) -> String {
        let mut parts = Vec::new();

        for column in &self.columns {
            let value = match column {
                Column::Path => self.colorize_path(&entry.path.display().to_string(), entry.kind),
                Column::Name => self.colorize_path(&entry.name, entry.kind),
                Column::Size => format_size_human(entry.size),
                Column::Mtime => entry.mtime.format("%Y-%m-%d %H:%M:%S").to_string(),
                Column::Kind => format!("{:?}", entry.kind).to_lowercase(),
                Column::Perms => entry.perms.clone().unwrap_or_default(),
                Column::Owner => entry.owner.clone().unwrap_or_default(),
            };
            parts.push(value);
        }

        parts.join("  ")
    }

    fn colorize_path(&self, path: &str, kind: EntryKind) -> String {
        if !self.use_color {
            return path.to_string();
        }

        match kind {
            EntryKind::Dir => Color::Blue.bold().paint(path).to_string(),
            EntryKind::Symlink => Color::Cyan.paint(path).to_string(),
            EntryKind::File => {
                // Color executables differently if possible
                if path.ends_with(".exe") || path.ends_with(".sh") {
                    Color::Green.bold().paint(path).to_string()
                } else {
                    path.to_string()
                }
            }
        }
    }
}

impl OutputSink for PrettyFormatter {
    fn write(&mut self, entry: &Entry) -> Result<()> {
        writeln!(self.writer, "{}", self.format_entry(entry))?;
        Ok(())
    }

    fn finish(&mut self) -> Result<()> {
        self.writer.flush()?;
        Ok(())
    }
}

/// Tree view formatter for hierarchical display
pub struct TreeFormatter {
    writer: Box<dyn Write>,
    use_color: bool,
    dirs_first: bool,
}

impl TreeFormatter {
    pub fn new(writer: Box<dyn Write>, no_color: bool, dirs_first: bool) -> Self {
        let use_color = is_tty() && !no_color;
        Self {
            writer,
            use_color,
            dirs_first,
        }
    }

    pub fn write_tree(&mut self, entries: &[Entry]) -> Result<()> {
        // Sort entries if dirs_first is enabled
        let mut sorted_entries = entries.to_vec();
        if self.dirs_first {
            sorted_entries.sort_by(|a, b| {
                // First by kind (dirs before files), then by name
                match (a.kind, b.kind) {
                    (EntryKind::Dir, EntryKind::File) => std::cmp::Ordering::Less,
                    (EntryKind::File, EntryKind::Dir) => std::cmp::Ordering::Greater,
                    _ => a.name.cmp(&b.name),
                }
            });
        }

        for entry in &sorted_entries {
            self.write_tree_entry(entry)?;
        }

        self.writer.flush()?;
        Ok(())
    }

    fn write_tree_entry(&mut self, entry: &Entry) -> Result<()> {
        let indent = "  ".repeat(entry.depth);
        let prefix = if entry.depth > 0 { "├── " } else { "" };

        let name = self.colorize_name(&entry.name, entry.kind);
        writeln!(self.writer, "{}{}{}", indent, prefix, name)?;
        Ok(())
    }

    fn colorize_name(&self, name: &str, kind: EntryKind) -> String {
        if !self.use_color {
            return name.to_string();
        }

        match kind {
            EntryKind::Dir => Color::Blue.bold().paint(format!("{}/", name)).to_string(),
            EntryKind::Symlink => Color::Cyan.paint(format!("{} @", name)).to_string(),
            EntryKind::File => name.to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn make_test_entry(name: &str, kind: EntryKind) -> Entry {
        use chrono::Utc;

        Entry {
            path: PathBuf::from(name),
            name: name.to_string(),
            size: 1024,
            kind,
            mtime: Utc::now(),
            perms: Some("rw-r--r--".to_string()),
            owner: Some("1000".to_string()),
            depth: 0,
        }
    }

    #[test]
    fn test_pretty_formatter() {
        use std::io::Cursor;

        let output = Cursor::new(Vec::new());
        let mut formatter =
            PrettyFormatter::new(Box::new(output), vec![Column::Name, Column::Size], true);

        let entry = make_test_entry("test.txt", EntryKind::File);
        formatter.write(&entry).unwrap();
        formatter.finish().unwrap();
    }

    #[test]
    fn test_tree_formatter() {
        use std::io::Cursor;

        let output = Cursor::new(Vec::new());
        let mut formatter = TreeFormatter::new(Box::new(output), true, false);

        let entries = vec![
            make_test_entry("root", EntryKind::Dir),
            Entry {
                depth: 1,
                ..make_test_entry("file.txt", EntryKind::File)
            },
        ];

        formatter.write_tree(&entries).unwrap();
    }
}
