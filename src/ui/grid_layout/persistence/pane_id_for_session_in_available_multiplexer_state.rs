use ratkit::primitives::resizable_grid::PaneId;

use crate::app::state::app_state::AppState;
use crate::ui::grid_layout::persistence::load_persisted_multiplexer_state::load_persisted_multiplexer_state;
use crate::ui::grid_layout::persistence::persisted_multiplexer_state::PersistedMultiplexerState;

/// Finds a session's pane in suspended in-memory state, then persisted disk state.
pub fn pane_id_for_session_in_available_multiplexer_state(
    app: &AppState,
    session_id: &str,
) -> Option<PaneId> {
    app.suspended_multiplexer_state
        .as_ref()
        .and_then(|state| pane_id_for_session_in_state(state, session_id))
        .or_else(|| {
            load_persisted_multiplexer_state()
                .as_ref()
                .and_then(|state| pane_id_for_session_in_state(state, session_id))
        })
}

/// Finds a session's pane inside one persisted multiplexer state.
fn pane_id_for_session_in_state(
    state: &PersistedMultiplexerState,
    session_id: &str,
) -> Option<PaneId> {
    state
        .terminal_pane_session_bundles
        .iter()
        .find_map(|(pane_id, session_ids)| {
            session_ids
                .iter()
                .any(|candidate| candidate == session_id)
                .then_some(*pane_id)
        })
}
