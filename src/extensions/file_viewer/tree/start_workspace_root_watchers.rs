use std::path::PathBuf;

use ratkit::services::file_watcher::FileWatcher;

use crate::extensions::file_viewer::tree::start_root_watcher::start_root_watcher;

/// Starts recursive Ratkit directory watchers for all grouped file workspace roots.
pub fn start_workspace_root_watchers(roots: &[PathBuf]) -> Vec<FileWatcher> {
    roots
        .iter()
        .filter_map(|root| start_root_watcher(root))
        .collect()
}
