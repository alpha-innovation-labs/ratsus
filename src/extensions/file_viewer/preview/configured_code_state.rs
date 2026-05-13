use ratkit::widgets::code_widget::CodeState;

/// Creates default code-widget state for the file preview pane.
pub(super) fn configured_code_state() -> CodeState {
    let mut state = CodeState::default();
    state.display.show_line_numbers = true;
    state.display.show_outline = true;
    state.display.highlight_current_line = true;
    state
}
