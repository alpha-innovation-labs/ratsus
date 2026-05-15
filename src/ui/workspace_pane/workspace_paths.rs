use std::path::PathBuf;

use crate::app::state::app_state::AppState;

/// Returns workspace folder paths in their persisted visible order.
pub fn workspace_paths(app: &AppState) -> Vec<PathBuf> {
    app.folder_order.clone()
}
