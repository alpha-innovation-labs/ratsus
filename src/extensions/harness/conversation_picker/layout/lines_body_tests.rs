use crate::extensions::history_modal::layout::footer_text::conversation_picker_footer_text;
use crate::extensions::history_modal::layout::lines::{
    conversation_picker_lines, ConversationPickerLinesConfig,
};

/// Conversation picker body lines should not duplicate the dialog footer help.
#[test]
fn body_lines_exclude_dialog_footer_text() {
    let lines = conversation_picker_lines(ConversationPickerLinesConfig {
        query: "",
        items: &[],
        selected_position: 0,
        is_filtering: false,
        height: 5,
        width: 40,
        loader_tick: 0,
        dragging_session_index: None,
    });

    assert!(!lines
        .iter()
        .any(|line| line.to_string() == conversation_picker_footer_text()));
}
