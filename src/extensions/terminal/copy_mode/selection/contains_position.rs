use crate::extensions::terminal::copy_mode::selection::bounds::selection_bounds;
use crate::extensions::terminal::copy_mode::selection::position::SelectionPosition;

/// Returns true when a terminal-buffer position is inside a copy selection.
pub fn selection_contains_position(
    start: SelectionPosition,
    end: SelectionPosition,
    position: SelectionPosition,
) -> bool {
    let bounds = selection_bounds(start, end);
    if position.row < bounds.start.row || position.row > bounds.end.row {
        return false;
    }
    if bounds.start.row == bounds.end.row {
        return position.col >= bounds.start.col && position.col <= bounds.end.col;
    }
    if position.row == bounds.start.row {
        return position.col >= bounds.start.col;
    }
    if position.row == bounds.end.row {
        return position.col <= bounds.end.col;
    }
    true
}
