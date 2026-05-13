use crate::extensions::terminal::copy_mode::selection::copy_selection::TerminalCopySelection;

/// Clears any pending or active terminal copy selection.
pub fn clear_terminal_copy_selection(selection: &mut TerminalCopySelection) {
    selection.snapshot = None;
    selection.anchor = None;
    selection.cursor = None;
}
