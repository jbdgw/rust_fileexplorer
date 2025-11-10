#[cfg(feature = "tui")]
use crate::errors::Result;
#[cfg(feature = "tui")]
use crate::fs::traverse::{walk_no_filter, TraverseConfig};
#[cfg(feature = "tui")]
use crate::models::{Entry, EntryKind};
#[cfg(feature = "tui")]
use std::path::PathBuf;

#[cfg(feature = "tui")]
/// Application state for the TUI
pub struct App {
    pub path: PathBuf,
    pub entries: Vec<Entry>,
    pub filtered_entries: Vec<Entry>,
    pub selected_index: usize,
    pub filter: String,
    pub show_hidden: bool,
    pub dirs_first: bool,
    pub scroll_offset: usize,
    pub should_quit: bool,
}

#[cfg(feature = "tui")]
impl App {
    pub fn new(path: PathBuf) -> Result<Self> {
        let config = TraverseConfig {
            max_depth: None,
            follow_symlinks: false,
            include_hidden: false,
            respect_gitignore: true,
            threads: 4,
            quiet: true,
        };

        let entries = walk_no_filter(&path, &config)?;
        let filtered_entries = entries.clone();

        Ok(Self {
            path,
            entries,
            filtered_entries,
            selected_index: 0,
            filter: String::new(),
            show_hidden: false,
            dirs_first: true,
            scroll_offset: 0,
            should_quit: false,
        })
    }

    pub fn reload(&mut self) -> Result<()> {
        let config = TraverseConfig {
            max_depth: None,
            follow_symlinks: false,
            include_hidden: self.show_hidden,
            respect_gitignore: true,
            threads: 4,
            quiet: true,
        };

        self.entries = walk_no_filter(&self.path, &config)?;
        self.apply_filter();
        Ok(())
    }

    pub fn apply_filter(&mut self) {
        let filter_lower = self.filter.to_lowercase();

        self.filtered_entries = if filter_lower.is_empty() {
            self.entries.clone()
        } else {
            self.entries
                .iter()
                .filter(|e| e.name.to_lowercase().contains(&filter_lower))
                .cloned()
                .collect()
        };

        // Sort with dirs first if enabled
        if self.dirs_first {
            self.filtered_entries
                .sort_by(|a, b| match (a.kind, b.kind) {
                    (EntryKind::Dir, EntryKind::File) => std::cmp::Ordering::Less,
                    (EntryKind::File, EntryKind::Dir) => std::cmp::Ordering::Greater,
                    _ => a.name.cmp(&b.name),
                });
        } else {
            self.filtered_entries.sort_by(|a, b| a.name.cmp(&b.name));
        }

        // Reset selection if needed
        if self.selected_index >= self.filtered_entries.len() {
            self.selected_index = self.filtered_entries.len().saturating_sub(1);
        }
    }

    pub fn next(&mut self) {
        if self.filtered_entries.is_empty() {
            return;
        }
        self.selected_index = (self.selected_index + 1).min(self.filtered_entries.len() - 1);
    }

    pub fn previous(&mut self) {
        self.selected_index = self.selected_index.saturating_sub(1);
    }

    pub fn page_down(&mut self, page_size: usize) {
        if self.filtered_entries.is_empty() {
            return;
        }
        self.selected_index =
            (self.selected_index + page_size).min(self.filtered_entries.len() - 1);
    }

    pub fn page_up(&mut self, page_size: usize) {
        self.selected_index = self.selected_index.saturating_sub(page_size);
    }

    pub fn add_filter_char(&mut self, c: char) {
        self.filter.push(c);
        self.apply_filter();
    }

    pub fn remove_filter_char(&mut self) {
        self.filter.pop();
        self.apply_filter();
    }

    pub fn clear_filter(&mut self) {
        self.filter.clear();
        self.apply_filter();
    }

    pub fn toggle_hidden(&mut self) -> Result<()> {
        self.show_hidden = !self.show_hidden;
        self.reload()
    }

    pub fn toggle_dirs_first(&mut self) {
        self.dirs_first = !self.dirs_first;
        self.apply_filter();
    }

    pub fn selected_entry(&self) -> Option<&Entry> {
        self.filtered_entries.get(self.selected_index)
    }

    pub fn enter_selected(&mut self) -> Result<()> {
        if let Some(entry) = self.selected_entry() {
            if entry.kind == EntryKind::Dir {
                self.path = entry.path.clone();
                self.selected_index = 0;
                self.scroll_offset = 0;
                self.reload()?;
            }
        }
        Ok(())
    }

    pub fn go_up(&mut self) -> Result<()> {
        if let Some(parent) = self.path.parent() {
            self.path = parent.to_path_buf();
            self.selected_index = 0;
            self.scroll_offset = 0;
            self.reload()?;
        }
        Ok(())
    }

    pub fn quit(&mut self) {
        self.should_quit = true;
    }
}
