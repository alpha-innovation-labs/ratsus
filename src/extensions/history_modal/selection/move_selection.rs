use crate::extensions::history_modal::data::state::HistoryModalState;
use crate::ui::keyboard::list::wrapped_position::wrapped_list_position;

/// Moves the highlighted picker row within the current filtered result count.
pub fn move_history_modal_selection(
    state: &mut HistoryModalState,
    direction: isize,
    result_count: usize,
) {
    state.selected_position =
        wrapped_list_position(state.selected_position, direction, result_count);
}

#[cfg(test)]
mod tests {
    use super::move_history_modal_selection;
    use crate::extensions::history_modal::data::mode::HistoryModalMode;
    use crate::extensions::history_modal::data::state::HistoryModalState;

    /// Moving down from the last result should wrap to the first result.
    #[test]
    fn move_down_wraps_to_first_result() {
        let mut state = state_with_selection(2);

        move_history_modal_selection(&mut state, 1, 3);

        assert_eq!(state.selected_position, 0);
    }

    /// Moving up from the first result should wrap to the last result.
    #[test]
    fn move_up_wraps_to_last_result() {
        let mut state = state_with_selection(0);

        move_history_modal_selection(&mut state, -1, 3);

        assert_eq!(state.selected_position, 2);
    }

    /// Large movements should wrap within the result count.
    #[test]
    fn large_movements_wrap_inside_results() {
        let mut state = state_with_selection(1);

        move_history_modal_selection(&mut state, -10, 3);

        assert_eq!(state.selected_position, 0);
    }

    /// Empty results should reset the selection to zero.
    #[test]
    fn empty_results_reset_selection() {
        let mut state = state_with_selection(4);

        move_history_modal_selection(&mut state, 1, 0);

        assert_eq!(state.selected_position, 0);
    }

    /// Builds conversation picker state for movement tests.
    fn state_with_selection(selected_position: usize) -> HistoryModalState {
        HistoryModalState {
            is_open: true,
            query: String::new(),
            is_filtering: false,
            selected_position,
            pending_g: false,
            folder_filter: None,
            mode: HistoryModalMode::Open,
            mouse_down_position: None,
            mouse_drag_moved: false,
        }
    }
}
