use std::path::PathBuf;

use crate::app::nexus_demo_state::NexusDemo;
use crate::conversation_picker::conversation_picker_item::ConversationPickerItemKind;
use crate::conversation_picker::selected_conversation_picker_item::selected_conversation_picker_item;

/// Returns the project folder for the currently selected picker row.
pub fn selected_conversation_picker_project_path(app: &NexusDemo) -> Option<PathBuf> {
    match selected_conversation_picker_item(app)?.kind {
        ConversationPickerItemKind::Folder { path, .. } => Some(path),
        ConversationPickerItemKind::Session { index, .. } => app
            .session_terminals
            .get(index)
            .map(|entry| entry.session.working_dir.clone()),
    }
}
