use std::path::PathBuf;

use crate::app::state::app_state::AppState;
use crate::extensions::file_viewer::tree::sync_workspace_root::sync_file_viewer_workspace_root;
use crate::ui::grid_layout::persistence::persist_multiplexer_state::persist_multiplexer_state;
use crate::ui::layout::focus::focused_pane::FocusedPane;
use crate::ui::workspace_pane::first_session_index_for_workspace::first_session_index_for_workspace;

/// Selects a workspace folder by activating its first session.
pub fn select_workspace_first_session(app: &mut AppState, path: PathBuf) -> bool {
    if !app.folder_order.iter().any(|folder| folder == &path) {
        return false;
    }
    let Some(index) = first_session_index_for_workspace(app, path.as_path()) else {
        return false;
    };
    let changed = app.active_index != index || app.selected_workspace_path.as_ref() != Some(&path);
    app.selected_workspace_path = Some(path);
    sync_file_viewer_workspace_root(app);
    app.session_scroll = 0;
    app.focused_index = index;
    app.suppress_left_focus_scroll = false;
    app.activate_focused_session();
    app.focused_pane = FocusedPane::Terminal;
    persist_multiplexer_state(app);
    changed
}
