use crate::app::state::app_state::AppState;

/// Remembers the active session as the last focused session for its workspace.
pub fn remember_active_workspace_session(app: &mut AppState) {
    let Some(entry) = app.session_terminals.get(app.active_index) else {
        return;
    };
    app.workspace_focused_session_ids
        .insert(entry.session.working_dir.clone(), entry.session.id.clone());
}
