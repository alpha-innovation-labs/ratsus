use ratkit::services::file_watcher::FileWatcher;

use crate::extensions::harness::nexus::config::chat_status_file_path::nexus_chat_status_file_path;

/// Starts a Ratkit file watcher for the Nexus chat status file.
pub fn start_chat_status_file_watcher() -> Option<FileWatcher> {
    let mut watcher = FileWatcher::for_file().ok()?;
    watcher.watch(&nexus_chat_status_file_path()).ok()?;
    Some(watcher)
}
