use crate::app::app_state::AppState;
use crate::left_panel::focused_left_row::focused_left_row;
use crate::left_panel::persist_session_order_preferences::persist_session_order_preferences;
use crate::left_panel::session_list_row::SessionListRow;

/// Collapses the project folder associated with the focused left-panel row.
pub fn collapse_focused_project(app: &mut AppState) {
    let Some(path) = focused_project_path(app) else {
        return;
    };
    app.collapsed_folders.insert(path.clone());
    if let Some(row_index) = app.visible_rows().iter().position(
        |row| matches!(row, SessionListRow::Folder { path: row_path, .. } if row_path == &path),
    ) {
        app.focused_row = row_index;
    }
    app.keep_focused_row_visible();
    persist_session_order_preferences(app);
}

/// Returns the project path associated with the focused left-panel row.
fn focused_project_path(app: &AppState) -> Option<std::path::PathBuf> {
    match focused_left_row(app)? {
        SessionListRow::Folder { path, .. } | SessionListRow::FolderMore { path } => Some(path),
        SessionListRow::Session { index } => app
            .session_terminals
            .get(index)
            .map(|entry| entry.session.working_dir.clone()),
    }
}
