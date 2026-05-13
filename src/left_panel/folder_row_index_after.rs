use crate::left_panel::session_list_row::SessionListRow;

/// Returns the next or previous folder row index relative to the focused row.
pub fn folder_row_index_after(
    rows: &[SessionListRow],
    focused_row: usize,
    direction: isize,
) -> Option<usize> {
    if direction > 0 {
        return rows
            .iter()
            .enumerate()
            .skip(focused_row.saturating_add(1))
            .find_map(folder_index);
    }
    if direction < 0 {
        return rows
            .iter()
            .enumerate()
            .take(focused_row.min(rows.len()))
            .rev()
            .find_map(folder_index);
    }
    None
}

/// Returns the row index when the row is a folder row.
fn folder_index((index, row): (usize, &SessionListRow)) -> Option<usize> {
    matches!(row, SessionListRow::Folder { .. }).then_some(index)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::folder_row_index_after;
    use crate::left_panel::session_list_row::SessionListRow;

    /// Finds the next folder strictly after the focused row.
    #[test]
    fn finds_next_folder_after_current_row() {
        let rows = rows_fixture();

        assert_eq!(folder_row_index_after(&rows, 1, 1), Some(3));
    }

    /// Finds the previous folder strictly before the focused row.
    #[test]
    fn finds_previous_folder_before_current_row() {
        let rows = rows_fixture();

        assert_eq!(folder_row_index_after(&rows, 4, -1), Some(3));
    }

    /// Returns none when no folder exists in the requested direction.
    #[test]
    fn returns_none_without_folder_in_direction() {
        let rows = rows_fixture();

        assert_eq!(folder_row_index_after(&rows, 0, -1), None);
        assert_eq!(folder_row_index_after(&rows, 4, 1), None);
    }

    /// Builds visible rows with two folders and child sessions.
    fn rows_fixture() -> Vec<SessionListRow> {
        vec![
            SessionListRow::Folder {
                path: PathBuf::from("/tmp/a"),
                current_session_count: 1,
                total_session_count: 1,
            },
            SessionListRow::Session { index: 0 },
            SessionListRow::Session { index: 1 },
            SessionListRow::Folder {
                path: PathBuf::from("/tmp/b"),
                current_session_count: 1,
                total_session_count: 1,
            },
            SessionListRow::Session { index: 2 },
        ]
    }
}
