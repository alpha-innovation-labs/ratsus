use crate::extensions::history_modal::data::state::HistoryModalState;

/// Clamps picker selection to the available filtered result count.
pub fn clamp_history_modal_selection(
    state: &mut HistoryModalState,
    result_count: usize,
) {
    if result_count == 0 {
        state.selected_position = 0;
        return;
    }
    state.selected_position = state.selected_position.min(result_count - 1);
}
