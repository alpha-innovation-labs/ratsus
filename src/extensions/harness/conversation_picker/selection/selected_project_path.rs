use std::path::PathBuf;

use crate::app::state::app_state::AppState;
use crate::extensions::history_modal::data::item::HistoryModalItemKind;
use crate::extensions::history_modal::selection::selected_item::selected_history_modal_item;

/// Returns the project folder for the currently selected picker row.
pub fn selected_history_modal_project_path(app: &AppState) -> Option<PathBuf> {
    match selected_history_modal_item(app)?.kind {
        HistoryModalItemKind::Folder { path, .. } => Some(path),
        HistoryModalItemKind::Session { index, .. } => app
            .session_terminals
            .get(index)
            .map(|entry| entry.session.working_dir.clone()),
    }
}
