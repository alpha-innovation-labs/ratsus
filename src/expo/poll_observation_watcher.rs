use crate::app::nexus_demo_state::NexusDemo;
use crate::expo::path_is_observation_state::path_is_observation_state;
use crate::expo::start_observation_cache_load::start_observation_cache_load;

/// Polls the Ratkit observation watcher and refreshes the cache after state-file changes.
pub fn poll_observation_watcher(app: &mut NexusDemo) {
    let Some(watcher) = app.observation_watcher.as_mut() else {
        return;
    };
    if !watcher.check_for_changes() {
        return;
    }
    let changed_paths = watcher.get_changed_paths();
    if changed_paths
        .iter()
        .any(|path| path_is_observation_state(path))
    {
        start_observation_cache_load(app);
    }
}
