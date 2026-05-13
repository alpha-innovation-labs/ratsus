use crate::app::state::app_state::AppState;
use crate::ui::left_panel::focus::session_visible_row_index::session_visible_row_index;
use crate::ui::left_panel::session::list_row::SessionListRow;

/// Returns active folder/session rows that should be pinned above the scroll viewport.
pub fn pinned_left_panel_rows(
    app: &AppState,
    rows: &[SessionListRow],
    visible_end: usize,
) -> Vec<(usize, SessionListRow)> {
    let mut pinned = Vec::new();
    if let Some(folder) = active_folder_row(app, rows) {
        push_if_hidden(&mut pinned, folder, app.session_scroll, visible_end);
    }
    if let Some(row_index) = session_visible_row_index(rows, app.active_index) {
        push_if_hidden(
            &mut pinned,
            (row_index, rows[row_index].clone()),
            app.session_scroll,
            visible_end,
        );
    }
    pinned
}

/// Adds a row to the pinned list when its real row is outside the viewport.
fn push_if_hidden(
    pinned: &mut Vec<(usize, SessionListRow)>,
    row: (usize, SessionListRow),
    visible_start: usize,
    visible_end: usize,
) {
    if row.0 < visible_start || row.0 >= visible_end {
        pinned.push(row);
    }
}

/// Finds the folder row for the active session.
fn active_folder_row(app: &AppState, rows: &[SessionListRow]) -> Option<(usize, SessionListRow)> {
    let active_folder = &app
        .session_terminals
        .get(app.active_index)?
        .session
        .working_dir;
    rows.iter().enumerate().find_map(|(index, row)| match row {
        SessionListRow::Folder { path, .. } if path == active_folder => Some((index, row.clone())),
        _ => None,
    })
}

#[cfg(test)]
mod tests {
    use super::push_if_hidden;
    use crate::ui::left_panel::session::list_row::SessionListRow;

    /// Verifies hidden rows are pinned.
    #[test]
    fn pins_rows_outside_viewport() {
        let mut pinned = Vec::new();
        push_if_hidden(&mut pinned, (1, SessionListRow::Session { index: 0 }), 3, 6);
        assert_eq!(pinned.len(), 1);
    }

    /// Verifies visible rows are not pinned.
    #[test]
    fn skips_rows_inside_viewport() {
        let mut pinned = Vec::new();
        push_if_hidden(&mut pinned, (4, SessionListRow::Session { index: 0 }), 3, 6);
        assert!(pinned.is_empty());
    }
}
