use crate::errors::Result;
use crate::models::{Entry, EntryKind};
use chrono::{DateTime, Utc};
use std::fs;
use std::path::Path;

#[cfg(unix)]
use std::os::unix::fs::PermissionsExt;

/// Extract entry metadata from a path
pub fn extract_entry(path: &Path, depth: usize) -> Result<Entry> {
    let metadata = fs::symlink_metadata(path)?;
    let mtime = extract_mtime(&metadata)?;
    let kind = EntryKind::from_metadata(&metadata);

    let size = if kind == EntryKind::Dir {
        0 // Directory size computed separately if needed
    } else {
        metadata.len()
    };

    let name = path
        .file_name()
        .and_then(|n| n.to_str())
        .unwrap_or("")
        .to_string();

    let perms = extract_permissions(&metadata);
    let owner = extract_owner(path);

    Ok(Entry {
        path: path.to_path_buf(),
        name,
        size,
        kind,
        mtime,
        perms,
        owner,
        depth,
    })
}

/// Extract modification time from metadata
fn extract_mtime(metadata: &fs::Metadata) -> Result<DateTime<Utc>> {
    let mtime = metadata.modified()?;
    Ok(DateTime::from(mtime))
}

/// Extract permission string (Unix-style)
#[cfg(unix)]
fn extract_permissions(metadata: &fs::Metadata) -> Option<String> {
    let mode = metadata.permissions().mode();
    Some(format_permissions(mode))
}

#[cfg(not(unix))]
fn extract_permissions(_metadata: &fs::Metadata) -> Option<String> {
    None
}

#[cfg(unix)]
fn format_permissions(mode: u32) -> String {
    let user = triplet(mode, 6);
    let group = triplet(mode, 3);
    let other = triplet(mode, 0);
    format!("{}{}{}", user, group, other)
}

#[cfg(unix)]
fn triplet(mode: u32, shift: u32) -> String {
    let r = if mode & (0o4 << shift) != 0 { 'r' } else { '-' };
    let w = if mode & (0o2 << shift) != 0 { 'w' } else { '-' };
    let x = if mode & (0o1 << shift) != 0 { 'x' } else { '-' };
    format!("{}{}{}", r, w, x)
}

/// Extract owner information (best effort)
#[cfg(unix)]
fn extract_owner(path: &Path) -> Option<String> {
    use std::os::unix::fs::MetadataExt;

    if let Ok(metadata) = fs::metadata(path) {
        let uid = metadata.uid();
        // For simplicity, just return UID; could use libc to get username
        Some(format!("{}", uid))
    } else {
        None
    }
}

#[cfg(not(unix))]
fn extract_owner(_path: &Path) -> Option<String> {
    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::File;
    use tempfile::tempdir;

    #[test]
    fn test_extract_entry_file() {
        let dir = tempdir().unwrap();
        let file_path = dir.path().join("test.txt");
        File::create(&file_path).unwrap();

        let entry = extract_entry(&file_path, 0).unwrap();
        assert_eq!(entry.name, "test.txt");
        assert_eq!(entry.kind, EntryKind::File);
        assert_eq!(entry.depth, 0);
    }

    #[test]
    fn test_extract_entry_dir() {
        let dir = tempdir().unwrap();
        let entry = extract_entry(dir.path(), 0).unwrap();
        assert_eq!(entry.kind, EntryKind::Dir);
    }

    #[cfg(unix)]
    #[test]
    fn test_format_permissions() {
        assert_eq!(format_permissions(0o755), "rwxr-xr-x");
        assert_eq!(format_permissions(0o644), "rw-r--r--");
        assert_eq!(format_permissions(0o777), "rwxrwxrwx");
    }
}
