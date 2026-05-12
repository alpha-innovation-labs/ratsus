use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
};
use ratkit::primitives::termtui::{render_screen, Screen};

use crate::selection_contains_position::selection_contains_position;
use crate::selection_position::SelectionPosition;
use crate::terminal_copy_selection::TerminalCopySelection;

/// Renders a terminal screen and overlays active copy-selection highlighting.
pub fn render_screen_with_selection(
    screen: &Screen,
    selection: &TerminalCopySelection,
    area: Rect,
    buf: &mut Buffer,
) {
    render_screen(screen, area, buf);
    let Some(anchor) = selection.anchor else {
        return;
    };
    let Some(cursor) = selection.cursor else {
        return;
    };
    let highlight = Style::default().fg(Color::Black).bg(Color::Cyan);
    for row in 0..area.height {
        for col in 0..area.width {
            let position = SelectionPosition {
                row: i32::from(row) - screen.scrollback() as i32,
                col: i32::from(col),
            };
            if selection_contains_position(anchor, cursor, position) {
                if let Some(cell) = buf.cell_mut((area.x + col, area.y + row)) {
                    cell.set_style(highlight);
                }
            }
        }
    }
}
