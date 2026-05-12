use crate::apply_session_refresh::apply_session_refresh;
use crate::nexus_demo_state::NexusDemo;
use crate::session_info::NexusSession;
use crate::spawn_session_refresh_worker::SessionRefreshResult;

/// Drains completed background session refreshes and applies the newest successful result.
pub fn drain_session_refreshes(app: &mut NexusDemo) -> bool {
    let Some(refreshed_sessions) = latest_successful_refresh(app) else {
        return false;
    };
    let changed = apply_session_refresh(&mut app.session_terminals, refreshed_sessions);
    if changed {
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
    match result {
        Ok(refreshed_sessions) => Some(refreshed_sessions),
        Err(_) => None,
    }
}
