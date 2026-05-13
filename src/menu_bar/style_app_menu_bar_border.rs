use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::style::Style;

use crate::rendering::default_border_color::default_border_color;

/// Applies the shared pane border color to the rendered menu bar border cells.
pub fn style_app_menu_bar_border(buffer: &mut Buffer, area: Rect) {
    if area.width == 0 || area.height == 0 {
        return;
    }

    let style = Style::default().fg(default_border_color());
    let right = area.x.saturating_add(area.width.saturating_sub(1));
    let bottom = area.y.saturating_add(area.height.saturating_sub(1));

    for column in area.x..=right {
        if let Some(cell) = buffer.cell_mut((column, area.y)) {
            cell.set_style(style);
        }
        if let Some(cell) = buffer.cell_mut((column, bottom)) {
            cell.set_style(style);
        }
    }

    for row in area.y..=bottom {
        if let Some(cell) = buffer.cell_mut((area.x, row)) {
            cell.set_style(style);
        }
        if let Some(cell) = buffer.cell_mut((right, row)) {
            cell.set_style(style);
        }
    }
}

#[cfg(test)]
mod tests {
    use ratatui::buffer::Buffer;
    use ratatui::layout::Rect;
    use ratatui::style::Color;

    use super::style_app_menu_bar_border;
    use crate::rendering::default_border_color::default_border_color;

    /// Verifies that all menu bar border cells receive the shared pane border color.
    #[test]
    fn styles_menu_bar_border_with_default_pane_color() {
        let area = Rect::new(1, 1, 4, 3);
        let mut buffer = Buffer::empty(Rect::new(0, 0, 8, 6));

        style_app_menu_bar_border(&mut buffer, area);

        let expected = default_border_color();
        assert_eq!(buffer.cell((1, 1)).unwrap().fg, expected);
        assert_eq!(buffer.cell((4, 1)).unwrap().fg, expected);
        assert_eq!(buffer.cell((1, 3)).unwrap().fg, expected);
        assert_eq!(buffer.cell((4, 3)).unwrap().fg, expected);
    }

    /// Verifies that non-border cells are not styled as menu bar border cells.
    #[test]
    fn leaves_menu_bar_inner_cells_unchanged() {
        let area = Rect::new(1, 1, 4, 3);
        let mut buffer = Buffer::empty(Rect::new(0, 0, 8, 6));

        style_app_menu_bar_border(&mut buffer, area);

        assert_eq!(buffer.cell((2, 2)).unwrap().fg, Color::Reset);
    }
}
