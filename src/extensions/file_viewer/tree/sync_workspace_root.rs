use std::path::PathBuf;

use crate::app::state::app_state::AppState;

/// Stores the active file-viewer folder expansion state in app memory.
pub fn store_current_file_viewer_expansion(app: &mut AppState) {
    let root = app.file_system_tree_view.root_path().to_path_buf();
    let expanded_paths = app.file_system_tree_view.expanded_directory_paths();
    app.file_system_tree_expanded_paths
        .insert(root, expanded_paths);
}

/// Points the Files tree at the selected workspace and restores its expansion state.
pub fn sync_file_viewer_workspace_root(app: &mut AppState) {
    let Some(root) = app.selected_workspace_path.clone() else {
        return;
    };
    replace_file_viewer_root(app, root);
}

/// Applies persisted expansion state to the current file-viewer root.
pub fn restore_file_viewer_expansion(app: &mut AppState) {
    let root = app.file_system_tree_view.root_path().to_path_buf();
    let expanded_paths = app
        .file_system_tree_expanded_paths
        .get(&root)
        .cloned()
        .unwrap_or_default();
    app.file_system_tree_view
        .apply_expanded_directory_paths(&expanded_paths);
}

/// Replaces the active file-viewer root when it differs from the requested root.
fn replace_file_viewer_root(app: &mut AppState, root: PathBuf) {
    if app.file_system_tree_view.root_path() == root.as_path() {
        return;
    }
    store_current_file_viewer_expansion(app);
    let expanded_paths = app
        .file_system_tree_expanded_paths
        .get(&root)
        .cloned()
        .unwrap_or_default();
    let _ = app
        .file_system_tree_view
        .replace_root(root, &expanded_paths);
}
