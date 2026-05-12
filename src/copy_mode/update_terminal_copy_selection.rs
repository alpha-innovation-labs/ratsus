use crate::copy_mode::selection_position::SelectionPosition;
use crate::copy_mode::terminal_copy_selection::TerminalCopySelection;

/// Extends copy selection to the latest drag position.
pub fn update_terminal_copy_selection(
    selection: &mut TerminalCopySelection,
    cursor: SelectionPosition,
) {
    if selection.anchor.is_some() {
        selection.cursor = Some(cursor);
    }
}
