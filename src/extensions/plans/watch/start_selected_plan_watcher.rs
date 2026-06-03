use std::path::Path;

use ratkit::services::file_watcher::FileWatcher;

/// Starts a Ratkit file watcher for the active Markdown plan preview.
pub fn start_selected_plan_watcher(path: &Path) -> Option<FileWatcher> {
    let mut watcher = FileWatcher::for_file().ok()?;
    watcher.watch(path).ok()?;
    Some(watcher)
}
