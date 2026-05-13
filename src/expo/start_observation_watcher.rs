use ratkit::services::file_watcher::FileWatcher;

use crate::expo::nexus_observations_dir::nexus_observations_dir;

/// Starts a Ratkit directory watcher for Nexus observation files.
pub fn start_observation_watcher() -> Option<FileWatcher> {
    let mut watcher = FileWatcher::for_directory().ok()?;
    watcher.watch(&nexus_observations_dir()).ok()?;
    Some(watcher)
}
