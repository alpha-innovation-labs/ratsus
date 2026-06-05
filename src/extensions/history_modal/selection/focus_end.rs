use crate::app::state::app_state::AppState;
use crate::extensions::history_modal::data::current_item_count::current_history_modal_item_count;

/// Moves conversation picker selection to the last visible row.
pub fn focus_history_modal_end(app: &mut AppState) {
    app.history_modal.selected_position =
        current_history_modal_item_count(app).saturating_sub(1);
}
