use crate::app::app_state::AppState;
use crate::expo::start_observation_cache_load::start_observation_cache_load;

/// Polls the Ratkit observation watcher and refreshes the cache after state-file changes.
pub fn poll_observation_watcher(app: &mut AppState) {
    let Some(watcher) = app.observation_watcher.as_mut() else {
        return;
    };
    if !watcher.check_for_changes() {
        return;
    }
    let changed_paths = watcher.get_changed_paths();
    if changed_paths
        .iter()
        .any(|path| app.chat_harness.is_observation_state_path(path))
    {
        start_observation_cache_load(app);
    }
}
