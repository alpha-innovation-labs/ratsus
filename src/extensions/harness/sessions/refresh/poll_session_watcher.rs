use crate::app::state::app_state::AppState;
use crate::extensions::harness::sessions::refresh::apply_session_refreshes::apply_session_refreshes;

/// Polls the session watcher and refreshes session metadata after registry changes.
pub fn poll_session_watcher(app: &mut AppState) -> bool {
    if !session_watcher_changed(app) {
        return false;
    }
    match app.chat_harness.refresh_sessions() {
        Ok(refreshed_sessions) => apply_session_refreshes(app, refreshed_sessions),
        Err(_) => false,
    }
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
