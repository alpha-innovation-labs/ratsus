use crate::left_panel::session_list_row::SessionListRow;

/// Returns the first visible row occupied by any session scheduled for deletion.
pub fn delete_focus_row_index(rows: &[SessionListRow], deleted_indices: &[usize]) -> usize {
    rows.iter()
        .position(|row| matches!(row, SessionListRow::Session { index } if deleted_indices.contains(index)))
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::delete_focus_row_index;
    use crate::left_panel::session_list_row::SessionListRow;

    /// Verifies deletion focus starts at the removed session's visible row.
    #[test]
    fn returns_deleted_session_row() {
        let rows = vec![
            SessionListRow::Session { index: 0 },
            SessionListRow::Session { index: 1 },
        ];

        assert_eq!(delete_focus_row_index(&rows, &[1]), 1);
    }
}
