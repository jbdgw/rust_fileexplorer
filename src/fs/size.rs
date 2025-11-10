use crate::models::{Entry, EntryKind};
use std::collections::HashMap;
use std::path::PathBuf;

/// Compute directory sizes by aggregating file sizes
pub fn compute_dir_sizes(entries: &[Entry]) -> HashMap<PathBuf, u64> {
    let mut sizes: HashMap<PathBuf, u64> = HashMap::new();

    // First, collect all file sizes
    for entry in entries {
        if entry.kind == EntryKind::File {
            // Add size to file's own path
            sizes.insert(entry.path.clone(), entry.size);

            // Add size to all parent directories
            let mut current = entry.path.parent();
            while let Some(parent) = current {
                *sizes.entry(parent.to_path_buf()).or_insert(0) += entry.size;
                current = parent.parent();
            }
        }
    }

    sizes
}

/// Update entries with computed directory sizes
pub fn update_entries_with_dir_sizes(entries: &mut [Entry], dir_sizes: &HashMap<PathBuf, u64>) {
    for entry in entries.iter_mut() {
        if entry.kind == EntryKind::Dir {
            if let Some(&size) = dir_sizes.get(&entry.path) {
                entry.size = size;
            }
        }
    }
}

/// Get top N entries by size
pub fn get_top_by_size(entries: &[Entry], n: usize) -> Vec<Entry> {
    let mut sorted = entries.to_vec();
    sorted.sort_by(|a, b| b.size.cmp(&a.size));
    sorted.into_iter().take(n).collect()
}

/// Compute total size of all entries
pub fn compute_total_size(entries: &[Entry]) -> u64 {
    entries.iter().map(|e| e.size).sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::path::PathBuf;

    fn make_entry(path: &str, size: u64, kind: EntryKind) -> Entry {
        use chrono::Utc;

        let path_buf = PathBuf::from(path);
        let name = path_buf
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

        Entry {
            path: path_buf,
            name,
            size,
            kind,
            mtime: Utc::now(),
            perms: None,
            owner: None,
            depth: 0,
        }
    }

    #[test]
    fn test_compute_dir_sizes() {
        use std::path::Path;

        let entries = vec![
            make_entry("/root", 0, EntryKind::Dir),
            make_entry("/root/file1.txt", 100, EntryKind::File),
            make_entry("/root/file2.txt", 200, EntryKind::File),
            make_entry("/root/subdir", 0, EntryKind::Dir),
            make_entry("/root/subdir/file3.txt", 50, EntryKind::File),
        ];

        let sizes = compute_dir_sizes(&entries);

        assert_eq!(sizes.get(Path::new("/root")), Some(&350));
        assert_eq!(sizes.get(Path::new("/root/subdir")), Some(&50));
    }

    #[test]
    fn test_update_entries_with_dir_sizes() {
        let mut entries = vec![
            make_entry("/root", 0, EntryKind::Dir),
            make_entry("/root/file.txt", 100, EntryKind::File),
        ];

        let sizes = compute_dir_sizes(&entries);
        update_entries_with_dir_sizes(&mut entries, &sizes);

        assert_eq!(entries[0].size, 100); // Directory size updated
        assert_eq!(entries[1].size, 100); // File size unchanged
    }

    #[test]
    fn test_get_top_by_size() {
        let entries = vec![
            make_entry("small.txt", 10, EntryKind::File),
            make_entry("large.txt", 1000, EntryKind::File),
            make_entry("medium.txt", 100, EntryKind::File),
        ];

        let top = get_top_by_size(&entries, 2);
        assert_eq!(top.len(), 2);
        assert_eq!(top[0].size, 1000);
        assert_eq!(top[1].size, 100);
    }

    #[test]
    fn test_compute_total_size() {
        let entries = vec![
            make_entry("file1.txt", 100, EntryKind::File),
            make_entry("file2.txt", 200, EntryKind::File),
            make_entry("file3.txt", 50, EntryKind::File),
        ];

        assert_eq!(compute_total_size(&entries), 350);
    }
}
