use ratkit::primitives::termtui::Screen;

use crate::copy_mode::selection_position::SelectionPosition;

/// State for mprocs-style copy selection over a frozen terminal screen.
#[derive(Debug, Clone, Default)]
pub struct TerminalCopySelection {
    pub snapshot: Option<Screen>,
    pub anchor: Option<SelectionPosition>,
    pub cursor: Option<SelectionPosition>,
}
