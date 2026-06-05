use crate::extensions::history_modal::data::state::ConversationPickerState;

/// Clamps picker selection to the available filtered result count.
pub fn clamp_conversation_picker_selection(
    state: &mut ConversationPickerState,
    result_count: usize,
) {
    if result_count == 0 {
        state.selected_position = 0;
        return;
    }
    state.selected_position = state.selected_position.min(result_count - 1);
}
