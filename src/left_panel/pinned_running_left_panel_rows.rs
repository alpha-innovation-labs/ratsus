use std::collections::BTreeSet;

use crate::app::app_state::AppState;
use crate::left_panel::session_list_row::SessionListRow;
use crate::left_panel::session_visible_row_index::session_visible_row_index;

/// Returns running session context rows that should be pinned above active context.
pub fn pinned_running_left_panel_rows(
    app: &AppState,
    rows: &[SessionListRow],
    visible_end: usize,
) -> Vec<(usize, SessionListRow)> {
    let mut pinned = Vec::new();
    let mut seen = BTreeSet::new();
    for session_index in hidden_running_session_indexes(app, rows, visible_end) {
        push_hidden_folder(
            app,
            rows,
            session_index,
            visible_end,
            &mut pinned,
            &mut seen,
        );
        push_hidden_session(rows, session_index, &mut pinned, &mut seen);
    }
    pinned
}

/// Returns running session indexes whose rows are outside the viewport.
fn hidden_running_session_indexes(
    app: &AppState,
    rows: &[SessionListRow],
    visible_end: usize,
) -> Vec<usize> {
    app.session_terminals
        .iter()
        .enumerate()
        .filter_map(|(index, entry)| {
            (index != app.active_index
                && entry.session.is_running
                && session_row_is_hidden(app, rows, index, visible_end))
            .then_some(index)
        })
        .collect()
}

/// Returns true when a session row is outside the viewport.
fn session_row_is_hidden(
    app: &AppState,
    rows: &[SessionListRow],
    session_index: usize,
    visible_end: usize,
) -> bool {
    session_visible_row_index(rows, session_index)
        .is_some_and(|row_index| row_index < app.session_scroll || row_index >= visible_end)
}

/// Adds the hidden folder row for a running session when needed.
fn push_hidden_folder(
    app: &AppState,
    rows: &[SessionListRow],
    session_index: usize,
    visible_end: usize,
    pinned: &mut Vec<(usize, SessionListRow)>,
    seen: &mut BTreeSet<usize>,
) {
    let Some(folder_row) = folder_row_for_session(app, rows, session_index) else {
        return;
    };
    if folder_row.0 < app.session_scroll || folder_row.0 >= visible_end {
        push_unique(pinned, seen, folder_row);
    }
}

/// Adds the hidden session row for a running session.
fn push_hidden_session(
    rows: &[SessionListRow],
    session_index: usize,
    pinned: &mut Vec<(usize, SessionListRow)>,
    seen: &mut BTreeSet<usize>,
) {
    if let Some(row_index) = session_visible_row_index(rows, session_index) {
        push_unique(pinned, seen, (row_index, rows[row_index].clone()));
    }
}

/// Adds a row if it has not already been pinned.
fn push_unique(
    pinned: &mut Vec<(usize, SessionListRow)>,
    seen: &mut BTreeSet<usize>,
    row: (usize, SessionListRow),
) {
    if seen.insert(row.0) {
        pinned.push(row);
    }
}

/// Finds the folder row that contains a session.
fn folder_row_for_session(
    app: &AppState,
    rows: &[SessionListRow],
    session_index: usize,
) -> Option<(usize, SessionListRow)> {
    let folder = &app
        .session_terminals
        .get(session_index)?
        .session
        .working_dir;
    rows.iter().enumerate().find_map(|(index, row)| match row {
        SessionListRow::Folder { path, .. } if path == folder => Some((index, row.clone())),
        _ => None,
    })
}

#[cfg(test)]
mod tests {
    use super::push_unique;
    use crate::left_panel::session_list_row::SessionListRow;
    use std::collections::BTreeSet;

    /// Verifies duplicate pinned rows are skipped.
    #[test]
    fn skips_duplicate_rows() {
        let mut pinned = Vec::new();
        let mut seen = BTreeSet::new();
        push_unique(
            &mut pinned,
            &mut seen,
            (1, SessionListRow::Session { index: 0 }),
        );
        push_unique(
            &mut pinned,
            &mut seen,
            (1, SessionListRow::Session { index: 0 }),
        );
        assert_eq!(pinned.len(), 1);
    }
}
