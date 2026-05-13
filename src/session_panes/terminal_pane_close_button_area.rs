use ratatui::layout::Rect;

/// Returns the clickable close-button area for a split terminal pane.
pub fn terminal_pane_close_button_area(area: Rect) -> Option<Rect> {
    if area.width < 4 || area.height == 0 {
        return None;
    }
    Some(Rect::new(
        area.x.saturating_add(area.width.saturating_sub(3)),
        area.y,
        1,
        1,
    ))
}
