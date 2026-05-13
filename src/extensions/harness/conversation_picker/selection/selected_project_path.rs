use std::path::PathBuf;

use crate::app::state::app_state::AppState;
use crate::extensions::harness::conversation_picker::data::item::ConversationPickerItemKind;
use crate::extensions::harness::conversation_picker::selection::selected_item::selected_conversation_picker_item;

/// Returns the project folder for the currently selected picker row.
pub fn selected_conversation_picker_project_path(app: &AppState) -> Option<PathBuf> {
    match selected_conversation_picker_item(app)?.kind {
        ConversationPickerItemKind::Folder { path, .. } => Some(path),
        ConversationPickerItemKind::Session { index, .. } => app
            .session_terminals
            .get(index)
            .map(|entry| entry.session.working_dir.clone()),
    }
}
