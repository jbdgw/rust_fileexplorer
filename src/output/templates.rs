#[cfg(feature = "templates")]
use crate::errors::Result;
#[cfg(feature = "templates")]
use crate::models::Entry;
#[cfg(feature = "templates")]
use std::io::Write;

#[cfg(feature = "templates")]
/// Template format types
pub enum TemplateFormat {
    Markdown,
    Html,
}

#[cfg(feature = "templates")]
impl std::str::FromStr for TemplateFormat {
    type Err = String;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "markdown" | "md" => Ok(TemplateFormat::Markdown),
            "html" => Ok(TemplateFormat::Html),
            _ => Err(format!("Unknown template format: {}", s)),
        }
    }
}

#[cfg(feature = "templates")]
/// Export entries using a template format
pub fn export_with_template<W: Write>(
    writer: &mut W,
    entries: &[Entry],
    format: &TemplateFormat,
    title: Option<&str>,
) -> Result<()> {
    match format {
        TemplateFormat::Markdown => export_markdown(writer, entries, title),
        TemplateFormat::Html => export_html(writer, entries, title),
    }
}

#[cfg(feature = "templates")]
/// Export as Markdown table
fn export_markdown<W: Write>(writer: &mut W, entries: &[Entry], title: Option<&str>) -> Result<()> {
    // Write title if provided
    if let Some(title) = title {
        writeln!(writer, "# {}\n", title)?;
    }

    // Calculate totals
    let total_files = entries
        .iter()
        .filter(|e| e.kind == crate::models::EntryKind::File)
        .count();
    let total_size: u64 = entries
        .iter()
        .filter(|e| e.kind == crate::models::EntryKind::File)
        .map(|e| e.size)
        .sum();

    writeln!(writer, "**Total Files:** {}  ", total_files)?;
    writeln!(
        writer,
        "**Total Size:** {}  \n",
        humansize::format_size(total_size, humansize::BINARY)
    )?;

    // Write table header
    writeln!(writer, "| Path | Size | Modified | Type |")?;
    writeln!(writer, "|------|------|----------|------|")?;

    // Write entries
    for entry in entries {
        let size_str = if entry.kind == crate::models::EntryKind::File {
            humansize::format_size(entry.size, humansize::BINARY)
        } else {
            "-".to_string()
        };

        let kind_str = format!("{:?}", entry.kind);
        let mtime_str = entry.mtime.format("%Y-%m-%d %H:%M").to_string();

        writeln!(
            writer,
            "| {} | {} | {} | {} |",
            entry.path.display(),
            size_str,
            mtime_str,
            kind_str
        )?;
    }

    Ok(())
}

#[cfg(feature = "templates")]
/// Export as HTML table
fn export_html<W: Write>(writer: &mut W, entries: &[Entry], title: Option<&str>) -> Result<()> {
    // Calculate totals
    let total_files = entries
        .iter()
        .filter(|e| e.kind == crate::models::EntryKind::File)
        .count();
    let total_size: u64 = entries
        .iter()
        .filter(|e| e.kind == crate::models::EntryKind::File)
        .map(|e| e.size)
        .sum();

    let title_text = title.unwrap_or("File Explorer Results");

    // Write HTML header
    writeln!(writer, "<!DOCTYPE html>")?;
    writeln!(writer, "<html lang=\"en\">")?;
    writeln!(writer, "<head>")?;
    writeln!(writer, "    <meta charset=\"UTF-8\">")?;
    writeln!(
        writer,
        "    <meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">"
    )?;
    writeln!(writer, "    <title>{}</title>", title_text)?;
    writeln!(writer, "    <style>")?;
    writeln!(writer, "        body {{ font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, sans-serif; margin: 40px; background: #f5f5f5; }}")?;
    writeln!(writer, "        .container {{ max-width: 1200px; margin: 0 auto; background: white; padding: 30px; border-radius: 8px; box-shadow: 0 2px 8px rgba(0,0,0,0.1); }}")?;
    writeln!(writer, "        h1 {{ color: #333; margin-top: 0; }}")?;
    writeln!(writer, "        .summary {{ background: #e8f4f8; padding: 15px; border-radius: 4px; margin-bottom: 20px; }}")?;
    writeln!(writer, "        .summary strong {{ color: #0066cc; }}")?;
    writeln!(
        writer,
        "        table {{ width: 100%; border-collapse: collapse; margin-top: 20px; }}"
    )?;
    writeln!(writer, "        th {{ background: #0066cc; color: white; padding: 12px; text-align: left; font-weight: 600; }}")?;
    writeln!(
        writer,
        "        td {{ padding: 10px 12px; border-bottom: 1px solid #e0e0e0; }}"
    )?;
    writeln!(writer, "        tr:hover {{ background: #f8f8f8; }}")?;
    writeln!(
        writer,
        "        .file-path {{ font-family: 'Monaco', 'Menlo', monospace; color: #0066cc; }}"
    )?;
    writeln!(writer, "        .dir {{ color: #666; font-weight: 600; }}")?;
    writeln!(writer, "        .file {{ color: #333; }}")?;
    writeln!(
        writer,
        "        .symlink {{ color: #8b4513; font-style: italic; }}"
    )?;
    writeln!(writer, "    </style>")?;
    writeln!(writer, "</head>")?;
    writeln!(writer, "<body>")?;
    writeln!(writer, "    <div class=\"container\">")?;
    writeln!(writer, "        <h1>{}</h1>", title_text)?;

    // Write summary
    writeln!(writer, "        <div class=\"summary\">")?;
    writeln!(
        writer,
        "            <strong>Total Files:</strong> {} &nbsp;&nbsp;",
        total_files
    )?;
    writeln!(
        writer,
        "            <strong>Total Size:</strong> {}",
        humansize::format_size(total_size, humansize::BINARY)
    )?;
    writeln!(writer, "        </div>")?;

    // Write table
    writeln!(writer, "        <table>")?;
    writeln!(writer, "            <thead>")?;
    writeln!(writer, "                <tr>")?;
    writeln!(writer, "                    <th>Path</th>")?;
    writeln!(writer, "                    <th>Size</th>")?;
    writeln!(writer, "                    <th>Modified</th>")?;
    writeln!(writer, "                    <th>Type</th>")?;
    writeln!(writer, "                </tr>")?;
    writeln!(writer, "            </thead>")?;
    writeln!(writer, "            <tbody>")?;

    for entry in entries {
        let size_str = if entry.kind == crate::models::EntryKind::File {
            humansize::format_size(entry.size, humansize::BINARY)
        } else {
            "-".to_string()
        };

        let kind_class = match entry.kind {
            crate::models::EntryKind::Dir => "dir",
            crate::models::EntryKind::File => "file",
            crate::models::EntryKind::Symlink => "symlink",
        };

        let kind_str = format!("{:?}", entry.kind);
        let mtime_str = entry.mtime.format("%Y-%m-%d %H:%M").to_string();

        writeln!(writer, "                <tr>")?;
        writeln!(
            writer,
            "                    <td class=\"file-path {}\">{}</td>",
            kind_class,
            entry.path.display()
        )?;
        writeln!(writer, "                    <td>{}</td>", size_str)?;
        writeln!(writer, "                    <td>{}</td>", mtime_str)?;
        writeln!(writer, "                    <td>{}</td>", kind_str)?;
        writeln!(writer, "                </tr>")?;
    }

    writeln!(writer, "            </tbody>")?;
    writeln!(writer, "        </table>")?;
    writeln!(writer, "    </div>")?;
    writeln!(writer, "</body>")?;
    writeln!(writer, "</html>")?;

    Ok(())
}

#[cfg(test)]
#[cfg(feature = "templates")]
mod tests {
    use super::*;
    use crate::models::EntryKind;
    use chrono::Utc;
    use std::path::PathBuf;

    fn make_test_entry(name: &str, size: u64, kind: EntryKind) -> Entry {
        Entry {
            path: PathBuf::from(name),
            name: name.to_string(),
            size,
            kind,
            mtime: Utc::now(),
            perms: None,
            owner: None,
            depth: 0,
        }
    }

    #[test]
    fn test_markdown_export() {
        let entries = vec![
            make_test_entry("file1.txt", 100, EntryKind::File),
            make_test_entry("file2.txt", 200, EntryKind::File),
        ];

        let mut output = Vec::new();
        export_markdown(&mut output, &entries, Some("Test Report")).unwrap();
        let output_str = String::from_utf8(output).unwrap();

        assert!(output_str.contains("# Test Report"));
        assert!(output_str.contains("| Path | Size | Modified | Type |"));
        assert!(output_str.contains("file1.txt"));
        assert!(output_str.contains("file2.txt"));
    }

    #[test]
    fn test_html_export() {
        let entries = vec![make_test_entry("file1.txt", 100, EntryKind::File)];

        let mut output = Vec::new();
        export_html(&mut output, &entries, Some("Test Report")).unwrap();
        let output_str = String::from_utf8(output).unwrap();

        assert!(output_str.contains("<!DOCTYPE html>"));
        assert!(output_str.contains("<title>Test Report</title>"));
        assert!(output_str.contains("file1.txt"));
    }
}
