use crate::extensions::history_modal::layout::footer_text::history_modal_footer_text;
use crate::extensions::history_modal::layout::lines::{
    history_modal_lines, HistoryModalLinesConfig,
};

/// Conversation picker body lines should not duplicate the dialog footer help.
#[test]
fn body_lines_exclude_dialog_footer_text() {
    let lines = history_modal_lines(HistoryModalLinesConfig {
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
        .any(|line| line.to_string() == history_modal_footer_text()));
}
