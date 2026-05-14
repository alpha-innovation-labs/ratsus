use std::path::Path;

use ratkit::services::file_watcher::FileWatcher;

/// Starts a Ratkit file watcher for a selected preview file.
pub fn start_selected_file_watcher(path: &Path, is_dir: bool) -> Option<FileWatcher> {
    if is_dir {
        return None;
    }
    let mut watcher = FileWatcher::for_file().ok()?;
    watcher.watch(path).ok()?;
    Some(watcher)
}
