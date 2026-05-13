use ratatui::layout::Rect;

/// Returns the right-edge scrollbar area when the left session list can scroll.
pub fn left_panel_scrollbar_area(session_list_area: Rect, total_rows: usize) -> Option<Rect> {
    if session_list_area.width == 0 || session_list_area.height == 0 {
        return None;
    }
    if total_rows <= usize::from(session_list_area.height) {
        return None;
    }
    Some(Rect::new(
        session_list_area.x + session_list_area.width.saturating_sub(1),
        session_list_area.y,
        1,
        session_list_area.height,
    ))
}

#[cfg(test)]
mod tests {
    use ratatui::layout::Rect;

    use super::left_panel_scrollbar_area;

    /// Verifies scrollable content receives a one-column right-edge scrollbar.
    #[test]
    fn returns_right_edge_area_for_scrollable_content() {
        assert_eq!(
            left_panel_scrollbar_area(Rect::new(2, 3, 20, 5), 10),
            Some(Rect::new(21, 3, 1, 5))
        );
    }

    /// Verifies content that fits in the viewport does not render a scrollbar.
    #[test]
    fn returns_none_when_content_fits() {
        assert_eq!(left_panel_scrollbar_area(Rect::new(2, 3, 20, 5), 5), None);
    }
}
