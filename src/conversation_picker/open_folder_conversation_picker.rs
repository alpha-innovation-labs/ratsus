use std::path::PathBuf;

use crate::app::app_state::AppState;
use crate::conversation_picker::conversation_picker_item::ConversationPickerItemKind;
use crate::conversation_picker::conversation_picker_items::conversation_picker_items;
use crate::conversation_picker::conversation_picker_mode::ConversationPickerMode;

/// Opens the conversation picker scoped to one project folder.
pub fn open_folder_conversation_picker(app: &mut AppState, folder: PathBuf) {
    app.conversation_picker.is_open = true;
    app.conversation_picker.query.clear();
    app.conversation_picker.is_filtering = false;
    app.conversation_picker.pending_g = false;
    app.conversation_picker.mouse_down_position = None;
    app.conversation_picker.mouse_drag_moved = false;
    app.conversation_picker.folder_filter = Some(folder);
    app.conversation_picker.mode = ConversationPickerMode::Open;

    let items = conversation_picker_items(
        &app.session_terminals,
        &app.folder_order,
        "",
        app.active_index,
        &app.selected_conversation_ids,
        app.conversation_picker.folder_filter.as_deref(),
        &app.collapsed_folders,
    );
    app.conversation_picker.selected_position = items
        .iter()
        .position(|item| {
            matches!(item.kind, ConversationPickerItemKind::Session { index, .. } if index == app.active_index)
        })
        .unwrap_or(0);
}
