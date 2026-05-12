use ratkit::primitives::termtui::Screen;

use crate::selection_position::SelectionPosition;
use crate::terminal_copy_selection::TerminalCopySelection;

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
