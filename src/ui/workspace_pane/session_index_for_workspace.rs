use std::path::Path;

use crate::app::state::app_state::AppState;
use crate::ui::workspace_pane::first_session_index_for_workspace::first_session_index_for_workspace;

/// Returns the remembered or first available session index for a workspace.
pub fn session_index_for_workspace(app: &AppState, path: &Path) -> Option<usize> {
    app.workspace_focused_session_ids
        .get(path)
        .and_then(|session_id| session_index_for_workspace_session_id(app, path, session_id))
        .or_else(|| first_session_index_for_workspace(app, path))
}

/// Returns the index for a specific session id when it still belongs to the workspace.
fn session_index_for_workspace_session_id(
    app: &AppState,
    path: &Path,
    session_id: &str,
) -> Option<usize> {
    app.session_terminals.iter().position(|entry| {
        entry.session.working_dir.as_path() == path && entry.session.id == session_id
    })
}
