use crate::app::nexus_demo_state::NexusDemo;
use crate::layout::focused_pane::FocusedPane;
use crate::session_panes::set_active_terminal_pane_bundle_session::set_active_terminal_pane_bundle_session;

/// Adds an existing session to the active terminal pane bundle and focuses it there.
pub fn place_existing_session_in_active_terminal_pane(app: &mut NexusDemo, index: usize) {
    let Some(session_id) = app
        .session_terminals
        .get(index)
        .map(|entry| entry.session.id.clone())
    else {
        return;
    };
    let previous_session_id = app
        .session_terminals
        .get(app.active_index)
        .map(|entry| entry.session.id.clone());
    if let Some(previous_session_id) = previous_session_id {
        app.terminal_pane_session_bundles
            .entry(app.active_terminal_pane_id)
            .or_default()
            .push(previous_session_id);
    }
    set_active_terminal_pane_bundle_session(app, app.active_terminal_pane_id, session_id);
    app.active_index = index;
    app.focused_index = index;
    app.focused_pane = FocusedPane::Terminal;
    app.keep_focused_session_visible();
}
