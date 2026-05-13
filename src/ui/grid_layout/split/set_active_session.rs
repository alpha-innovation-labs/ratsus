use crate::app::state::app_state::AppState;

/// Replaces the active terminal pane with a single-session bundle.
pub fn set_active_terminal_pane_session(app: &mut AppState, session_id: String) {
    app.terminal_pane_sessions
        .insert(app.active_terminal_pane_id, session_id.clone());
    app.terminal_pane_session_bundles
        .insert(app.active_terminal_pane_id, vec![session_id]);
}
