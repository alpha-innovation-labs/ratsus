use crate::app::state::app_state::AppState;
use crate::extensions::file_viewer::tree::sync_workspace_root::sync_file_viewer_workspace_root;

/// Ensures the selected workspace exists, falling back to the active session folder or first folder.
pub fn sync_selected_workspace(app: &mut AppState) {
    if app
        .selected_workspace_path
        .as_ref()
        .is_some_and(|selected| app.folder_order.iter().any(|folder| folder == selected))
    {
        sync_file_viewer_workspace_root(app);
        return;
    }
    app.selected_workspace_path = app
        .session_terminals
        .get(app.active_index)
        .map(|entry| entry.session.working_dir.clone())
        .or_else(|| app.folder_order.first().cloned());
    sync_file_viewer_workspace_root(app);
}
