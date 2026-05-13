use ratkit::primitives::resizable_grid::PaneId;

use crate::app::state::app_state::AppState;

/// Resolves the current session index assigned to a terminal pane.
pub fn session_index_for_pane(app: &AppState, pane_id: PaneId) -> Option<usize> {
    let session_id = app.terminal_pane_sessions.get(&pane_id)?;
    app.session_terminals
        .iter()
        .position(|entry| entry.session.id == *session_id)
}
