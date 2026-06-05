use crate::app::state::app_state::AppState;

/// Moves conversation picker selection to the first visible row.
pub fn focus_history_modal_start(app: &mut AppState) {
    app.history_modal.selected_position = 0;
}
