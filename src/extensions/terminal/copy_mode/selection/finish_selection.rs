use crate::extensions::terminal::copy_mode::selection::clear_selection::clear_terminal_copy_selection;
use crate::extensions::terminal::copy_mode::selection::copy_selection::TerminalCopySelection;
use crate::extensions::terminal::copy_mode::selection::is_selection_active::is_terminal_copy_selection_active;
use crate::extensions::terminal::copy_mode::selection::selected_text::selected_text_from_terminal_copy_selection;

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
