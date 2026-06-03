use std::path::Path;

use ratkit::services::file_watcher::FileWatcher;

/// Starts a recursive Ratkit watcher for one workspace `plans` directory.
pub fn start_plan_directory_watcher(plans_dir: &Path) -> Option<FileWatcher> {
    std::fs::create_dir_all(plans_dir).ok()?;
    let mut watcher = FileWatcher::for_directory().ok()?;
    watcher.watch(plans_dir).ok()?;
    Some(watcher)
}
