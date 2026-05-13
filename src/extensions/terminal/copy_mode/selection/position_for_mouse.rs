use ratatui::layout::Rect;
use ratkit::MouseEvent;

use crate::extensions::terminal::copy_mode::selection::position::SelectionPosition;

/// Converts a mouse event in a terminal area into a scrollback-aware selection position.
pub fn selection_position_for_mouse(
    mouse: MouseEvent,
    area: Rect,
    scrollback: usize,
) -> Option<SelectionPosition> {
    if !mouse.is_inside(area) {
        return None;
    }
    Some(SelectionPosition {
        row: i32::from(mouse.row.saturating_sub(area.y)) - scrollback as i32,
        col: i32::from(mouse.column.saturating_sub(area.x)),
    })
}
