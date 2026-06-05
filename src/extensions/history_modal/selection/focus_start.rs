use crate::app::state::app_state::AppState;

/// Moves conversation picker selection to the first visible row.
pub fn focus_conversation_picker_start(app: &mut AppState) {
    app.conversation_picker.selected_position = 0;
}
