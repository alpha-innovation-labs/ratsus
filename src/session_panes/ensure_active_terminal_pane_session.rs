use crate::app::app_state::AppState;
use crate::session_panes::session_index_for_pane::session_index_for_pane;

/// Ensures the active terminal pane has a valid session assignment.
pub fn ensure_active_terminal_pane_session(app: &mut AppState) {
    if session_index_for_pane(app, app.active_terminal_pane_id).is_some() {
        return;
    }
    let Some(session_id) = app
        .session_terminals
        .get(app.active_index)
        .map(|entry| entry.session.id.clone())
    else {
        return;
    };
    app.terminal_pane_sessions
        .insert(app.active_terminal_pane_id, session_id.clone());
    app.terminal_pane_session_bundles
        .entry(app.active_terminal_pane_id)
        .or_insert_with(|| vec![session_id]);
}
