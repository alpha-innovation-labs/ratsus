use std::path::Path;

use ratkit::services::file_watcher::FileWatcher;

/// Starts a Ratkit directory watcher for the file-tree root.
pub fn start_root_watcher(root: &Path) -> Option<FileWatcher> {
    let mut watcher = FileWatcher::for_directory().ok()?;
    watcher.watch(root).ok()?;
    Some(watcher)
}
