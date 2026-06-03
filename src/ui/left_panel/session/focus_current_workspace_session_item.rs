use std::path::PathBuf;

use crate::app::state::app_state::AppState;
use crate::ui::left_panel::focus::focus_row::focus_left_panel_row;
use crate::ui::left_panel::session::list_row::SessionListRow;

/// Focuses the nth visible session item in the workspace represented by the selected session.
pub fn focus_current_workspace_session_item(app: &mut AppState, item_index: usize) {
    let Some(workspace_path) = current_session_workspace_path(app) else {
        return;
    };
    let Some(row_index) = current_workspace_session_item_row(app, &workspace_path, item_index)
    else {
        return;
    };
    focus_left_panel_row(app, row_index);
}

/// Returns the workspace path represented by the focused or active session.
fn current_session_workspace_path(app: &AppState) -> Option<PathBuf> {
    app.session_terminals
        .get(app.focused_index)
        .or_else(|| app.session_terminals.get(app.active_index))
        .map(|entry| entry.session.working_dir.clone())
        .or_else(|| app.selected_workspace_path.clone())
}

/// Returns the visible row for the nth session item in one workspace.
fn current_workspace_session_item_row(
    app: &AppState,
    workspace_path: &PathBuf,
    item_index: usize,
) -> Option<usize> {
    app.visible_rows()
        .iter()
        .enumerate()
        .filter_map(|(row_index, row)| session_item_index(app, row).map(|index| (row_index, index)))
        .filter(|(_, session_index)| session_is_in_workspace(app, *session_index, workspace_path))
        .nth(item_index)
        .map(|(row_index, _)| row_index)
}

/// Returns the session index represented by selectable session item rows.
fn session_item_index(app: &AppState, row: &SessionListRow) -> Option<usize> {
    let index = row.session_index()?;
    app.session_terminals.get(index)?;
    Some(index)
}

/// Returns whether a session belongs to the requested workspace path.
fn session_is_in_workspace(app: &AppState, session_index: usize, workspace_path: &PathBuf) -> bool {
    app.session_terminals
        .get(session_index)
        .is_some_and(|entry| entry.session.working_dir == *workspace_path)
}
