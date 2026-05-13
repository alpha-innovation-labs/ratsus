use ratatui::layout::Rect;

/// Splits the left pane into session-list and hotkey-footer areas.
pub fn split_left_pane_content(area: Rect) -> (Rect, Rect) {
    if area.height <= 1 {
        return (area, Rect::default());
    }
    let list_height = area.height.saturating_sub(1);
    let list_area = Rect::new(area.x, area.y, area.width, list_height);
    let footer_area = Rect::new(area.x, area.y + list_height, area.width, 1);
    (list_area, footer_area)
}

#[cfg(test)]
mod tests {
    use ratatui::layout::Rect;

    use super::split_left_pane_content;

    /// Verifies that a normal left pane reserves the last row for the footer.
    #[test]
    fn reserves_footer_row() {
        assert_eq!(
            split_left_pane_content(Rect::new(2, 3, 20, 10)),
            (Rect::new(2, 3, 20, 9), Rect::new(2, 12, 20, 1))
        );
    }

    /// Verifies that tiny panes do not underflow while splitting content.
    #[test]
    fn keeps_tiny_area_as_list() {
        assert_eq!(
            split_left_pane_content(Rect::new(2, 3, 20, 1)),
            (Rect::new(2, 3, 20, 1), Rect::default())
        );
    }
}
