use ratatui::layout::Rect;

use crate::ui::grid_layout::pane::close_button_area::terminal_pane_close_button_area;

/// Close button should sit on the pane title row near the right border.
#[test]
fn places_button_near_right_border() {
    assert_eq!(
        terminal_pane_close_button_area(Rect::new(10, 4, 20, 8)),
        Some(Rect::new(27, 4, 1, 1))
    );
}

/// Tiny areas should not expose a close button target.
#[test]
fn skips_tiny_areas() {
    assert_eq!(terminal_pane_close_button_area(Rect::new(0, 0, 3, 1)), None);
}
