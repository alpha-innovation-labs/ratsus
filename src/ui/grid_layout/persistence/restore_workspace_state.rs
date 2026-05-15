use std::collections::BTreeSet;
use std::path::PathBuf;

use crate::app::state::app_state::AppState;
use crate::ui::grid_layout::persistence::persisted_workspace_state::PersistedWorkspaceState;
use crate::ui::workspace_pane::sync_selected_workspace::sync_selected_workspace;

/// Restores persisted workspace mode and order into app state.
pub fn restore_workspace_state(app: &mut AppState, persisted: &PersistedWorkspaceState) {
    app.workspace_view_enabled = persisted.workspace_view_enabled;
    if !persisted.workspace_order.is_empty() {
        app.folder_order = restored_folder_order(&persisted.workspace_order, &app.folder_order);
    }
    if let Some(path) = &persisted.selected_workspace_path {
        app.selected_workspace_path = Some(path.clone());
    }
    sync_selected_workspace(app);
}

/// Merges persisted workspace order with currently known session folders.
fn restored_folder_order(persisted: &[PathBuf], current: &[PathBuf]) -> Vec<PathBuf> {
    let current_set = current.iter().collect::<BTreeSet<_>>();
    let mut restored = persisted
        .iter()
        .filter(|path| current_set.is_empty() || current_set.contains(path))
        .cloned()
        .collect::<Vec<_>>();
    for path in current {
        if !restored.contains(path) {
            restored.push(path.clone());
        }
    }
    restored
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::restored_folder_order;

    /// Restored folder order keeps known persisted folders first and appends new folders.
    #[test]
    fn restored_order_appends_new_current_folders() {
        let restored = restored_folder_order(
            &[PathBuf::from("/b"), PathBuf::from("/a")],
            &[
                PathBuf::from("/a"),
                PathBuf::from("/b"),
                PathBuf::from("/c"),
            ],
        );

        assert_eq!(
            restored,
            vec![
                PathBuf::from("/b"),
                PathBuf::from("/a"),
                PathBuf::from("/c")
            ]
        );
    }
}
