use ratkit::primitives::termtui::Screen;

use crate::extensions::terminal::copy_mode::selection::copy_selection::TerminalCopySelection;
use crate::extensions::terminal::copy_mode::selection::position::SelectionPosition;

/// Stores a pending copy-selection anchor from a mouse down event.
pub fn start_terminal_copy_selection(
    selection: &mut TerminalCopySelection,
    screen: Screen,
    anchor: SelectionPosition,
) {
    selection.snapshot = Some(screen);
    selection.anchor = Some(anchor);
    selection.cursor = None;
}
