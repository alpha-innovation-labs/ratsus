use crate::app::nexus_demo_state::NexusDemo;
use crate::left_panel::sync_folder_order::sync_folder_order;
use crate::nexus_sessions::apply_session_refresh::apply_session_refresh;
use crate::nexus_sessions::session_info::NexusSession;
use crate::nexus_sessions::spawn_session_refresh_worker::SessionRefreshResult;

/// Drains completed background session refreshes and applies the newest successful result.
pub fn drain_session_refreshes(app: &mut NexusDemo) -> bool {
    let Some(refreshed_sessions) = latest_successful_refresh(app) else {
        return false;
    };
    let refreshed_sessions = refreshed_sessions
        .into_iter()
        .filter(|session| !app.closed_chat_session_ids.contains(&session.id))
        .collect();
    let changed = apply_session_refresh(&mut app.session_terminals, refreshed_sessions);
    if changed {
        app.folder_order = sync_folder_order(&app.folder_order, &app.session_terminals);
        app.keep_focused_session_visible();
    }
    changed
}

/// Returns the newest successful refresh currently waiting on the channel.
fn latest_successful_refresh(app: &mut NexusDemo) -> Option<Vec<NexusSession>> {
    let mut latest = None;
    while let Ok(result) = app.session_refresh_receiver.try_recv() {
        if let Some(refreshed_sessions) = successful_refresh(result) {
            latest = Some(refreshed_sessions);
        }
    }
    latest
}

/// Converts a refresh result into session data when the command succeeded.
fn successful_refresh(result: SessionRefreshResult) -> Option<Vec<NexusSession>> {
    result.ok()
}
