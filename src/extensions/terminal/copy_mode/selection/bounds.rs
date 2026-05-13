use crate::extensions::terminal::copy_mode::selection::position::SelectionPosition;

/// Ordered start and end positions for a terminal copy selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct SelectionBounds {
    pub start: SelectionPosition,
    pub end: SelectionPosition,
}

/// Returns selection bounds ordered from top-left to bottom-right.
pub fn selection_bounds(start: SelectionPosition, end: SelectionPosition) -> SelectionBounds {
    if (start.row, start.col) <= (end.row, end.col) {
        SelectionBounds { start, end }
    } else {
        SelectionBounds {
            start: end,
            end: start,
        }
    }
}
