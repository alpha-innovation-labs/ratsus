use crate::app::state::app_state::AppState;

/// Polls file-viewer Ratkit watchers and reports whether visible state changed.
pub fn poll_file_viewer_watchers(app: &mut AppState) -> bool {
    app.file_system_tree_view.poll_watchers()
}
