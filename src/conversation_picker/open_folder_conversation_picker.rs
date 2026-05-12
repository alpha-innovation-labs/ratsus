use std::path::PathBuf;

use crate::app::nexus_demo_state::NexusDemo;
use crate::conversation_picker::conversation_picker_item::ConversationPickerItemKind;
use crate::conversation_picker::conversation_picker_items::conversation_picker_items;

/// Opens the conversation picker scoped to one project folder.
pub fn open_folder_conversation_picker(app: &mut NexusDemo, folder: PathBuf) {
    app.conversation_picker.is_open = true;
    app.conversation_picker.query.clear();
    app.conversation_picker.folder_filter = Some(folder);

    let items = conversation_picker_items(
        &app.session_terminals,
        &app.folder_order,
        "",
        app.active_index,
        app.conversation_picker.folder_filter.as_deref(),
    );
    app.conversation_picker.selected_position = items
        .iter()
        .position(|item| {
            matches!(item.kind, ConversationPickerItemKind::Session { index } if index == app.active_index)
        })
        .unwrap_or(0);
}
