use crate::conversation_picker::conversation_picker_state::ConversationPickerState;

/// Moves the highlighted picker row within the current filtered result count.
pub fn move_conversation_picker_selection(
    state: &mut ConversationPickerState,
    direction: isize,
    result_count: usize,
) {
    if result_count == 0 {
        state.selected_position = 0;
        return;
    }

    state.selected_position = state
        .selected_position
        .saturating_add_signed(direction)
        .min(result_count - 1);
}

#[cfg(test)]
mod tests {
    use super::move_conversation_picker_selection;
    use crate::conversation_picker::conversation_picker_state::ConversationPickerState;

    /// Moving down should advance until the last result.
    #[test]
    fn move_down_clamps_to_last_result() {
        let mut state = ConversationPickerState {
            is_open: true,
            query: String::new(),
            is_filtering: false,
            selected_position: 1,
            pending_g: false,
            folder_filter: None,
            mode:
                crate::conversation_picker::conversation_picker_mode::ConversationPickerMode::Open,
            mouse_down_position: None,
            mouse_drag_moved: false,
        };
        move_conversation_picker_selection(&mut state, 10, 3);
        assert_eq!(state.selected_position, 2);
    }

    /// Moving up should stop at the first result.
    #[test]
    fn move_up_clamps_to_first_result() {
        let mut state = ConversationPickerState {
            is_open: true,
            query: String::new(),
            is_filtering: false,
            selected_position: 1,
            pending_g: false,
            folder_filter: None,
            mode:
                crate::conversation_picker::conversation_picker_mode::ConversationPickerMode::Open,
            mouse_down_position: None,
            mouse_drag_moved: false,
        };
        move_conversation_picker_selection(&mut state, -10, 3);
        assert_eq!(state.selected_position, 0);
    }

    /// Empty results should reset the selection to zero.
    #[test]
    fn empty_results_reset_selection() {
        let mut state = ConversationPickerState {
            is_open: true,
            query: String::new(),
            is_filtering: false,
            selected_position: 4,
            pending_g: false,
            folder_filter: None,
            mode:
                crate::conversation_picker::conversation_picker_mode::ConversationPickerMode::Open,
            mouse_down_position: None,
            mouse_drag_moved: false,
        };
        move_conversation_picker_selection(&mut state, 1, 0);
        assert_eq!(state.selected_position, 0);
    }
}
