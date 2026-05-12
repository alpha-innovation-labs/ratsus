use crate::copy_mode::terminal_copy_selection::TerminalCopySelection;

/// Returns true when terminal copy selection has a visible selected range.
pub fn is_terminal_copy_selection_active(selection: &TerminalCopySelection) -> bool {
    selection.snapshot.is_some() && selection.anchor.is_some() && selection.cursor.is_some()
}
