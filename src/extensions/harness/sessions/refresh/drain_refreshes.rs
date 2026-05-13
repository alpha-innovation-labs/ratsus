use crate::app::state::app_state::AppState;
use crate::extensions::harness::core::chat_session::ChatSession;
use crate::extensions::harness::sessions::refresh::spawn_refresh_worker::SessionRefreshResult;
use crate::ui::left_panel::order::sync_folder_order::sync_folder_order;

/// Drains completed background session refreshes and applies the newest successful result.
pub fn drain_session_refreshes(app: &mut AppState) -> bool {
    let Some(refreshed_sessions) = latest_successful_refresh(app) else {
        return false;
    };
    let refreshed_sessions = refreshed_sessions
        .into_iter()
        .filter(|session| !app.closed_chat_session_ids.contains(&session.id))
        .collect();
    let changed = app
        .chat_harness
        .merge_session_refresh(&mut app.session_terminals, refreshed_sessions);
    if changed {
        app.folder_order = sync_folder_order(&app.folder_order, &app.session_terminals);
        app.keep_focused_session_visible();
    }
    changed
}

/// Returns the newest successful refresh currently waiting on the channel.
fn latest_successful_refresh(app: &mut AppState) -> Option<Vec<ChatSession>> {
    let mut latest = None;
    while let Ok(result) = app.session_refresh_receiver.try_recv() {
        if let Some(refreshed_sessions) = successful_refresh(result) {
            latest = Some(refreshed_sessions);
        }
    }
    latest
}

/// Converts a refresh result into session data when the command succeeded.
fn successful_refresh(result: SessionRefreshResult) -> Option<Vec<ChatSession>> {
    result.ok()
}
