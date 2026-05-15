use crate::app::state::app_state::AppState;
use crate::ui::grid_layout::persistence::capture_multiplexer_state::capture_multiplexer_state;
use crate::ui::grid_layout::persistence::save_persisted_multiplexer_state::save_persisted_multiplexer_state;

/// Persists split-pane multiplexer state and intentionally ignores storage failures.
pub fn persist_multiplexer_state(app: &AppState) {
    let state = capture_multiplexer_state(app);
    let _ = save_persisted_multiplexer_state(&state);
}
