use crate::extensions::terminal::copy_mode::selection::copy_selection::TerminalCopySelection;
use crate::extensions::terminal::copy_mode::selection::position::SelectionPosition;

/// Extends copy selection to the latest drag position.
pub fn update_terminal_copy_selection(
    selection: &mut TerminalCopySelection,
    cursor: SelectionPosition,
) {
    if selection.anchor.is_some() {
        selection.cursor = Some(cursor);
    }
}
