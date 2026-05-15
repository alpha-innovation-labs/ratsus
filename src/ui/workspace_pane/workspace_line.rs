use std::path::Path;

use ratatui::text::Line;

use crate::app::state::app_state::AppState;
use crate::ui::left_panel::folder::has_running_session::folder_has_running_session;
use crate::ui::left_panel::render::session_row_line::{folder_row_line, FolderRowLineConfig};
use crate::ui::workspace_pane::session_count_for_workspace::session_count_for_workspace;

/// Builds one workspace row line using folder visuals from the session pane.
pub fn workspace_line(app: &AppState, path: &Path, width: u16) -> Line<'static> {
    let count = session_count_for_workspace(path, &app.session_terminals);
    folder_row_line(FolderRowLineConfig {
        path,
        current_session_count: count,
        total_session_count: count,
        is_collapsed: false,
        has_running_session: folder_has_running_session(path, &app.session_terminals),
        loader_tick: app.loader_tick,
        width,
        show_full_path: false,
        is_active_expo_folder: app.selected_workspace_path.as_deref() == Some(path),
        is_selected: app.selected_workspace_path.as_deref() == Some(path),
        is_dragging: app.workspace_drag.as_deref() == Some(path),
    })
}
