use crate::clear_terminal_copy_selection::clear_terminal_copy_selection;
use crate::is_terminal_copy_selection_active::is_terminal_copy_selection_active;
use crate::selected_text_from_terminal_copy_selection::selected_text_from_terminal_copy_selection;
use crate::terminal_copy_selection::TerminalCopySelection;

/// Returns selected text and clears copy selection when a drag selection is complete.
pub fn finish_terminal_copy_selection(selection: &mut TerminalCopySelection) -> Option<String> {
    let selected_text = if is_terminal_copy_selection_active(selection) {
        selected_text_from_terminal_copy_selection(selection)
    } else {
        None
    };
    clear_terminal_copy_selection(selection);
    selected_text
}
