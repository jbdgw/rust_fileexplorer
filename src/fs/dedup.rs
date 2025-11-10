#[cfg(feature = "dedup")]
use crate::errors::Result;
#[cfg(feature = "dedup")]
use crate::models::{DuplicateGroup, Entry, EntryKind};
#[cfg(feature = "dedup")]
use blake3::Hasher;
#[cfg(feature = "dedup")]
use std::fs::File;
#[cfg(feature = "dedup")]
use std::io::{BufReader, Read};
#[cfg(feature = "dedup")]
use std::sync::Arc;

#[cfg(feature = "dedup")]
/// Find duplicate files by content hash
pub fn find_duplicates(entries: &[Entry], min_size: u64) -> Result<Vec<DuplicateGroup>> {
    // Step 1: Group by size (fast pre-filter)
    let mut size_groups: std::collections::HashMap<u64, Vec<Entry>> =
        std::collections::HashMap::new();

    for entry in entries {
        // Skip directories and files smaller than min_size
        if entry.kind != EntryKind::File || entry.size < min_size {
            continue;
        }

        size_groups
            .entry(entry.size)
            .or_default()
            .push(entry.clone());
    }

    // Step 2: For size groups with multiple files, compute hashes
    // Collect candidates (files with same size)
    let candidates: Vec<_> = size_groups
        .into_iter()
        .filter(|(_, entries)| entries.len() > 1)
        .flat_map(|(_, entries)| entries)
        .collect();

    if candidates.is_empty() {
        return Ok(Vec::new());
    }

    // Hash files in parallel
    #[cfg(feature = "parallel")]
    {
        use rayon::prelude::*;
        use std::sync::Mutex;
        let hash_map: Arc<Mutex<std::collections::HashMap<String, Vec<Entry>>>> =
            Arc::new(Mutex::new(std::collections::HashMap::new()));

        candidates.par_iter().for_each(|entry| {
            if let Ok(hash) = hash_file(&entry.path) {
                if let Ok(mut map) = hash_map.lock() {
                    map.entry(hash).or_default().push(entry.clone());
                }
            }
        });

        // Extract results
        let hash_results = Arc::try_unwrap(hash_map).unwrap().into_inner().unwrap();

        // Build duplicate groups
        let mut groups: Vec<DuplicateGroup> = hash_results
            .into_iter()
            .filter(|(_, entries)| entries.len() > 1)
            .map(|(hash, entries)| {
                let size = entries[0].size;
                DuplicateGroup::new(hash, size, entries)
            })
            .collect();

        groups.sort_by(|a, b| b.wasted_space.cmp(&a.wasted_space));
        Ok(groups)
    }

    #[cfg(not(feature = "parallel"))]
    {
        let mut hash_results = std::collections::HashMap::new();
        for entry in &candidates {
            if let Ok(hash) = hash_file(&entry.path) {
                hash_results
                    .entry(hash)
                    .or_default()
                    .push(entry.clone());
            }
        }

        // Build duplicate groups
        let mut groups: Vec<DuplicateGroup> = hash_results
            .into_iter()
            .filter(|(_, entries)| entries.len() > 1)
            .map(|(hash, entries)| {
                let size = entries[0].size;
                DuplicateGroup::new(hash, size, entries)
            })
            .collect();

        groups.sort_by(|a, b| b.wasted_space.cmp(&a.wasted_space));
        Ok(groups)
    }
}

#[cfg(feature = "dedup")]
/// Compute BLAKE3 hash of a file
fn hash_file(path: &std::path::Path) -> Result<String> {
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut hasher = Hasher::new();

    let mut buffer = [0u8; 8192];
    loop {
        let n = reader.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }

    Ok(hasher.finalize().to_hex().to_string())
}

#[cfg(feature = "dedup")]
/// Calculate total wasted space from duplicate groups
pub fn calculate_wasted_space(groups: &[DuplicateGroup]) -> u64 {
    groups.iter().map(|g| g.wasted_space).sum()
}

#[cfg(feature = "dedup")]
/// Get summary statistics for duplicates
pub struct DuplicateStats {
    pub total_groups: usize,
    pub total_files: usize,
    pub total_wasted_space: u64,
    pub largest_group_size: u64,
    pub largest_group_count: usize,
}

#[cfg(feature = "dedup")]
impl DuplicateStats {
    pub fn from_groups(groups: &[DuplicateGroup]) -> Self {
        let total_groups = groups.len();
        let total_files: usize = groups.iter().map(|g| g.count).sum();
        let total_wasted_space = calculate_wasted_space(groups);

        let largest_by_size = groups.iter().max_by_key(|g| g.wasted_space);
        let largest_group_size = largest_by_size.map(|g| g.wasted_space).unwrap_or(0);

        let largest_by_count = groups.iter().max_by_key(|g| g.count);
        let largest_group_count = largest_by_count.map(|g| g.count).unwrap_or(0);

        Self {
            total_groups,
            total_files,
            total_wasted_space,
            largest_group_size,
            largest_group_count,
        }
    }
}

#[cfg(test)]
#[cfg(feature = "dedup")]
mod tests {
    use super::*;
    use crate::models::Entry;
    use chrono::Utc;
    use std::fs;
    use std::path::PathBuf;
    use tempfile::tempdir;

    fn make_test_entry(path: PathBuf, size: u64) -> Entry {
        Entry {
            path: path.clone(),
            name: path.file_name().unwrap().to_string_lossy().to_string(),
            size,
            kind: EntryKind::File,
            mtime: Utc::now(),
            perms: None,
            owner: None,
            depth: 0,
        }
    }

    #[test]
    fn test_find_duplicates() {
        let dir = tempdir().unwrap();

        // Create identical files
        let file1 = dir.path().join("file1.txt");
        let file2 = dir.path().join("file2.txt");
        let file3 = dir.path().join("file3.txt");

        let content = "Hello World";
        fs::write(&file1, content).unwrap();
        fs::write(&file2, content).unwrap();
        fs::write(&file3, "Different content").unwrap();

        let entries = vec![
            make_test_entry(file1, content.len() as u64),
            make_test_entry(file2, content.len() as u64),
            make_test_entry(file3, 17),
        ];

        let groups = find_duplicates(&entries, 0).unwrap();

        assert_eq!(groups.len(), 1);
        assert_eq!(groups[0].count, 2);
        assert_eq!(groups[0].size, content.len() as u64);
    }

    #[test]
    fn test_min_size_filter() {
        let dir = tempdir().unwrap();

        let small1 = dir.path().join("small1.txt");
        let small2 = dir.path().join("small2.txt");

        fs::write(&small1, "hi").unwrap();
        fs::write(&small2, "hi").unwrap();

        let entries = vec![make_test_entry(small1, 2), make_test_entry(small2, 2)];

        // Should find duplicates with min_size=0
        let groups = find_duplicates(&entries, 0).unwrap();
        assert_eq!(groups.len(), 1);

        // Should not find duplicates with min_size=10
        let groups = find_duplicates(&entries, 10).unwrap();
        assert_eq!(groups.len(), 0);
    }

    #[test]
    fn test_wasted_space_calculation() {
        let dir = tempdir().unwrap();

        // Create 3 identical 1KB files
        let file1 = dir.path().join("file1.txt");
        let file2 = dir.path().join("file2.txt");
        let file3 = dir.path().join("file3.txt");

        let content = vec![0u8; 1024];
        fs::write(&file1, &content).unwrap();
        fs::write(&file2, &content).unwrap();
        fs::write(&file3, &content).unwrap();

        let entries = vec![
            make_test_entry(file1, 1024),
            make_test_entry(file2, 1024),
            make_test_entry(file3, 1024),
        ];

        let groups = find_duplicates(&entries, 0).unwrap();

        assert_eq!(groups.len(), 1);
        // 3 copies of 1KB file = 2KB wasted (original + 2 duplicates)
        assert_eq!(groups[0].wasted_space, 2048);
    }

    #[test]
    fn test_duplicate_stats() {
        let dir = tempdir().unwrap();

        // Create duplicates
        let file1 = dir.path().join("file1.txt");
        let file2 = dir.path().join("file2.txt");
        let file3 = dir.path().join("file3.txt");

        fs::write(&file1, "content").unwrap();
        fs::write(&file2, "content").unwrap();
        fs::write(&file3, "different").unwrap();

        let entries = vec![
            make_test_entry(file1, 7),
            make_test_entry(file2, 7),
            make_test_entry(file3, 9),
        ];

        let groups = find_duplicates(&entries, 0).unwrap();
        let stats = DuplicateStats::from_groups(&groups);

        assert_eq!(stats.total_groups, 1);
        assert_eq!(stats.total_files, 2);
        assert_eq!(stats.total_wasted_space, 7);
    }
}
