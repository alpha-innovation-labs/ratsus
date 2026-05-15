use std::path::Path;

use crate::app::state::app_state::AppState;

/// Returns the first session index belonging to one workspace folder.
pub fn first_session_index_for_workspace(app: &AppState, path: &Path) -> Option<usize> {
    app.session_terminals
        .iter()
        .position(|entry| entry.session.working_dir.as_path() == path)
}
