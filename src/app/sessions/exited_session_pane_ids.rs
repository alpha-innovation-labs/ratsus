use ratkit::primitives::resizable_grid::PaneId;

use crate::app::state::app_state::AppState;

/// Returns terminal pane ids currently displaying sessions that are about to be removed.
pub fn exited_session_pane_ids(app: &AppState, exited_indices: &[usize]) -> Vec<PaneId> {
    let exited_ids = exited_session_ids(app, exited_indices);
    app.terminal_pane_sessions
        .iter()
        .filter_map(|(pane_id, session_id)| exited_ids.contains(session_id).then_some(*pane_id))
        .collect()
}

/// Returns stable session ids for exited session indexes.
fn exited_session_ids(app: &AppState, exited_indices: &[usize]) -> Vec<String> {
    exited_indices
        .iter()
        .filter_map(|index| app.session_terminals.get(*index))
        .map(|entry| entry.session.id.clone())
        .collect()
}
