use ratkit::primitives::resizable_grid::PaneId;

use crate::app::state::app_state::AppState;

/// Finds the terminal pane bundle that contains a session id.
pub fn pane_id_for_session(app: &AppState, session_id: &str) -> Option<PaneId> {
    app.terminal_pane_session_bundles
        .iter()
        .find_map(|(pane_id, session_ids)| {
            session_ids
                .iter()
                .any(|candidate| candidate == session_id)
                .then_some(*pane_id)
        })
}
