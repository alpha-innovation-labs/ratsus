use crate::app::state::app_state::AppState;
use crate::extensions::history_modal::data::current_item_count::current_conversation_picker_item_count;

/// Moves conversation picker selection to the last visible row.
pub fn focus_conversation_picker_end(app: &mut AppState) {
    app.conversation_picker.selected_position =
        current_conversation_picker_item_count(app).saturating_sub(1);
}
