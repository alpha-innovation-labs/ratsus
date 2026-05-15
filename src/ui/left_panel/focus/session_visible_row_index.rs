use crate::ui::left_panel::session::list_row::SessionListRow;

/// Finds the visible row index for a flat session index.
pub fn session_visible_row_index(rows: &[SessionListRow], session_index: usize) -> Option<usize> {
    rows.iter()
        .position(|row| row.session_index() == Some(session_index))
}

#[cfg(test)]
mod tests {
    use super::session_visible_row_index;
    use crate::ui::left_panel::session::list_row::SessionListRow;

    /// Verifies a flat session index can be located in a visible tree row list.
    #[test]
    fn finds_session_row() {
        let rows = vec![
            SessionListRow::Session { index: 2 },
            SessionListRow::Session { index: 4 },
        ];

        assert_eq!(session_visible_row_index(&rows, 4), Some(1));
    }
}
