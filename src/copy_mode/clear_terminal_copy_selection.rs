use crate::copy_mode::terminal_copy_selection::TerminalCopySelection;

/// Clears any pending or active terminal copy selection.
pub fn clear_terminal_copy_selection(selection: &mut TerminalCopySelection) {
    selection.snapshot = None;
    selection.anchor = None;
    selection.cursor = None;
}
