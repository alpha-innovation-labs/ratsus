use crate::left_panel::session_list_row::SessionListRow;

/// Returns the session index that should receive focus after deletion.
pub fn session_index_after_delete(rows: &[SessionListRow], preferred_row: usize) -> Option<usize> {
    next_session_at_or_after(rows, preferred_row)
        .or_else(|| previous_session_before(rows, preferred_row))
}

/// Returns the first session row at or after the preferred row.
fn next_session_at_or_after(rows: &[SessionListRow], preferred_row: usize) -> Option<usize> {
    rows.iter().skip(preferred_row).find_map(session_index)
}

/// Returns the closest session row before the preferred row.
fn previous_session_before(rows: &[SessionListRow], preferred_row: usize) -> Option<usize> {
    rows.iter()
        .take(preferred_row)
        .rev()
        .find_map(session_index)
}

/// Extracts a session index from a visible row.
fn session_index(row: &SessionListRow) -> Option<usize> {
    match row {
        SessionListRow::Session { index } => Some(*index),
        SessionListRow::Folder { .. } | SessionListRow::FolderMore { .. } => None,
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::session_index_after_delete;
    use crate::left_panel::session_list_row::SessionListRow;

    /// Verifies focus moves to the next session row after a deletion gap.
    #[test]
    fn selects_next_session_after_deleted_row() {
        let rows = vec![
            SessionListRow::Folder {
                path: PathBuf::from("/tmp/a"),
                current_session_count: 1,
                total_session_count: 1,
            },
            SessionListRow::Folder {
                path: PathBuf::from("/tmp/b"),
                current_session_count: 1,
                total_session_count: 1,
            },
            SessionListRow::Session { index: 0 },
        ];

        assert_eq!(session_index_after_delete(&rows, 1), Some(0));
    }

    /// Verifies focus falls back to the previous session at the end of the list.
    #[test]
    fn falls_back_to_previous_session_at_end() {
        let rows = vec![SessionListRow::Session { index: 0 }];

        assert_eq!(session_index_after_delete(&rows, 3), Some(0));
    }
}
