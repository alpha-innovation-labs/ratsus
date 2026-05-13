use ratatui::layout::Rect;

use crate::ui::left_panel::session::list_row::SessionListRow;

/// Converts a mouse row inside the list viewport into a visible session-list row.
pub fn session_row_for_click(
    row: u16,
    list_area: Rect,
    offset: usize,
    visible_rows: &[SessionListRow],
) -> Option<SessionListRow> {
    if row < list_area.y || row >= list_area.y.saturating_add(list_area.height) {
        return None;
    }

    let index = offset + usize::from(row - list_area.y);
    visible_rows.get(index).cloned()
}

#[cfg(test)]
mod tests {
    use ratatui::layout::Rect;

    use super::session_row_for_click;
    use crate::ui::left_panel::session::list_row::SessionListRow;

    /// Verifies that clicks inside the viewport map to visible rows.
    #[test]
    fn maps_inside_click_to_visible_row() {
        let rows = vec![
            SessionListRow::Session { index: 10 },
            SessionListRow::Session { index: 11 },
        ];

        assert_eq!(
            session_row_for_click(3, Rect::new(0, 2, 20, 5), 0, &rows),
            Some(SessionListRow::Session { index: 11 })
        );
    }
}
