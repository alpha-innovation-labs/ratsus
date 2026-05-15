use crate::app::state::app_state::AppState;
use crate::ui::grid_layout::persistence::load_persisted_multiplexer_state::load_persisted_multiplexer_state;
use crate::ui::grid_layout::persistence::restore_multiplexer_state_into_app::restore_persisted_multiplexer_state_into_app;

/// Restores the suspended in-memory multiplexer state, falling back to disk state.
pub fn restore_available_multiplexer_state_into_app(app: &mut AppState) {
    if let Some(persisted) = app.suspended_multiplexer_state.clone() {
        restore_persisted_multiplexer_state_into_app(app, persisted);
        return;
    }
    if let Some(persisted) = load_persisted_multiplexer_state() {
        restore_persisted_multiplexer_state_into_app(app, persisted);
    }
}
