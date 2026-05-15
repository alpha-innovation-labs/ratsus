use crate::app::state::app_state::AppState;
use crate::extensions::harness::sessions::refresh::drain_session_refresh_receiver::drain_session_refresh_receiver;
use crate::extensions::harness::sessions::refresh::start_session_refresh_load::start_session_refresh_load;

/// Polls the session watcher and schedules metadata refreshes off the UI thread.
pub fn poll_session_watcher(app: &mut AppState) -> bool {
    let refreshed = drain_session_refresh_receiver(app);
    if session_watcher_changed(app) {
        start_session_refresh_load(app);
    }
    refreshed
}

/// Returns whether the backend session watcher reported a relevant changed path.
fn session_watcher_changed(app: &mut AppState) -> bool {
    let Some(watcher) = app.session_watcher.as_mut() else {
        return false;
    };
    if !watcher.check_for_changes() {
        return false;
    }
    watcher
        .get_changed_paths()
        .iter()
        .any(|path| app.chat_harness.is_session_update_path(path))
}
