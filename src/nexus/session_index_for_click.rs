use ratatui::layout::Rect;

/// Converts a mouse row inside the list viewport into a session index.
pub fn session_index_for_click(
    row: u16,
    list_area: Rect,
    offset: usize,
    session_count: usize,
) -> Option<usize> {
    if row < list_area.y || row >= list_area.y.saturating_add(list_area.height) {
        return None;
    }

    let index = offset + usize::from(row - list_area.y);
    (index < session_count).then_some(index)
}

#[cfg(test)]
mod tests {
    use ratatui::layout::Rect;

    use super::session_index_for_click;

    /// Verifies that clicks inside the viewport map to session rows.
    #[test]
    fn maps_inside_click_to_session_index() {
        assert_eq!(
            session_index_for_click(4, Rect::new(0, 2, 20, 5), 10, 20),
            Some(12)
        );
    }

    /// Verifies that clicks outside the viewport do not select a session.
    #[test]
    fn ignores_outside_clicks() {
        assert_eq!(
            session_index_for_click(8, Rect::new(0, 2, 20, 5), 0, 20),
            None
        );
    }

    /// Verifies that clicks beyond the available sessions are ignored.
    #[test]
    fn ignores_clicks_beyond_session_count() {
        assert_eq!(
            session_index_for_click(6, Rect::new(0, 2, 20, 5), 10, 12),
            None
        );
    }
}
