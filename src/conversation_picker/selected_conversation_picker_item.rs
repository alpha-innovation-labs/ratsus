use crate::app::app_state::AppState;
use crate::conversation_picker::conversation_picker_item::ConversationPickerItem;
use crate::conversation_picker::conversation_picker_items::conversation_picker_items;

/// Returns the currently selected conversation picker item.
pub fn selected_conversation_picker_item(app: &AppState) -> Option<ConversationPickerItem> {
    conversation_picker_items(
        &app.session_terminals,
        &app.folder_order,
        &app.conversation_picker.query,
        app.active_index,
        &app.selected_conversation_ids,
        app.conversation_picker.folder_filter.as_deref(),
        &app.collapsed_folders,
    )
    .get(app.conversation_picker.selected_position)
    .cloned()
}
