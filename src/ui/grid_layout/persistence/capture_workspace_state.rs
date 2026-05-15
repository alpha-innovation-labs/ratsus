use crate::app::state::app_state::AppState;
use crate::ui::grid_layout::persistence::persisted_workspace_state::PersistedWorkspaceState;

/// Captures workspace view mode and visible workspace ordering for persistence.
pub fn capture_workspace_state(app: &AppState) -> PersistedWorkspaceState {
    PersistedWorkspaceState {
        workspace_view_enabled: app.workspace_view_enabled,
        workspace_order: app.folder_order.clone(),
        selected_workspace_path: app.selected_workspace_path.clone(),
    }
}
