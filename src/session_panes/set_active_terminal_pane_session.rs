use crate::app::nexus_demo_state::NexusDemo;

/// Replaces the active terminal pane with a single-session bundle.
pub fn set_active_terminal_pane_session(app: &mut NexusDemo, session_id: String) {
    app.terminal_pane_sessions
        .insert(app.active_terminal_pane_id, session_id.clone());
    app.terminal_pane_session_bundles
        .insert(app.active_terminal_pane_id, vec![session_id]);
}
