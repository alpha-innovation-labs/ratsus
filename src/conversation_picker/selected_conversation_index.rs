use crate::conversation_picker::conversation_picker_state::ConversationPickerState;

/// Returns the session index selected by the picker state and filtered result list.
pub fn selected_conversation_index(
    state: &ConversationPickerState,
    filtered_indices: &[usize],
) -> Option<usize> {
    filtered_indices.get(state.selected_position).copied()
}

#[cfg(test)]
mod tests {
    use super::selected_conversation_index;
    use crate::conversation_picker::conversation_picker_state::ConversationPickerState;

    /// Selection should map from visible position to source session index.
    #[test]
    fn maps_selected_position_to_session_index() {
        let state = ConversationPickerState {
            is_open: true,
            query: String::new(),
            is_filtering: false,
            selected_position: 1,
            pending_g: false,
            folder_filter: None,
            mode:
                crate::conversation_picker::conversation_picker_mode::ConversationPickerMode::Open,
            ..Default::default()
        };
        assert_eq!(selected_conversation_index(&state, &[4, 9]), Some(9));
    }

    /// Out-of-range selection should produce no selected session.
    #[test]
    fn out_of_range_selection_returns_none() {
        let state = ConversationPickerState {
            is_open: true,
            query: String::new(),
            is_filtering: false,
            selected_position: 2,
            pending_g: false,
            folder_filter: None,
            mode:
                crate::conversation_picker::conversation_picker_mode::ConversationPickerMode::Open,
            ..Default::default()
        };
        assert_eq!(selected_conversation_index(&state, &[4, 9]), None);
    }
}
