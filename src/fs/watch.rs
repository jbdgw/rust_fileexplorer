#[cfg(feature = "watch")]
use crate::errors::{FsError, Result};
#[cfg(feature = "watch")]
use crate::models::WatchEvent;
#[cfg(feature = "watch")]
use notify::{Event, EventKind, RecursiveMode, Watcher};
#[cfg(feature = "watch")]
use std::path::Path;
#[cfg(feature = "watch")]
use std::sync::mpsc::channel;
#[cfg(feature = "watch")]
use std::time::Duration;

#[cfg(feature = "watch")]
pub struct FileWatcher {
    events: Vec<String>,
}

#[cfg(feature = "watch")]
impl FileWatcher {
    pub fn new(events: Vec<String>) -> Self {
        Self { events }
    }

    pub fn watch<F>(&self, path: &Path, mut callback: F) -> Result<()>
    where
        F: FnMut(WatchEvent),
    {
        let (tx, rx) = channel();

        let mut watcher = notify::recommended_watcher(move |res: notify::Result<Event>| {
            if let Ok(event) = res {
                let _ = tx.send(event);
            }
        })
        .map_err(|e| FsError::Watch(e.to_string()))?;

        watcher
            .watch(path, RecursiveMode::Recursive)
            .map_err(|e| FsError::Watch(e.to_string()))?;

        println!(
            "Watching {} for changes... (Ctrl+C to stop)",
            path.display()
        );

        loop {
            match rx.recv_timeout(Duration::from_millis(100)) {
                Ok(event) => {
                    if let Some(watch_event) = self.process_event(event) {
                        callback(watch_event);
                    }
                }
                Err(std::sync::mpsc::RecvTimeoutError::Timeout) => continue,
                Err(_) => break,
            }
        }

        Ok(())
    }

    fn process_event(&self, event: Event) -> Option<WatchEvent> {
        let event_type = match event.kind {
            EventKind::Create(_) => "create",
            EventKind::Modify(_) => "modify",
            EventKind::Remove(_) => "remove",
            _ => return None,
        };

        // Filter by requested event types
        if !self.events.is_empty() && !self.events.contains(&event_type.to_string()) {
            return None;
        }

        // Get the first path from the event
        let path = event.paths.first()?.clone();

        // Try to get metadata (may fail if file was removed)
        let (mtime, size) = if let Ok(metadata) = std::fs::metadata(&path) {
            let mtime = metadata.modified().ok().map(chrono::DateTime::from);
            let size = Some(metadata.len());
            (mtime, size)
        } else {
            (None, None)
        };

        Some(WatchEvent {
            event: event_type.to_string(),
            path,
            mtime,
            size,
        })
    }
}

#[cfg(not(feature = "watch"))]
pub struct FileWatcher;

#[cfg(not(feature = "watch"))]
impl FileWatcher {
    pub fn new(_events: Vec<String>) -> Self {
        Self
    }
}
