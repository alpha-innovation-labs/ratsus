use crate::app::app_state::AppState;
use crate::conversation_picker::conversation_picker_items::conversation_picker_items;

/// Returns the number of rows currently visible in the conversation picker.
pub fn current_conversation_picker_item_count(app: &AppState) -> usize {
    conversation_picker_items(
        &app.session_terminals,
        &app.folder_order,
        &app.conversation_picker.query,
        app.active_index,
        &app.selected_conversation_ids,
        app.conversation_picker.folder_filter.as_deref(),
        &app.collapsed_folders,
    )
    .len()
}
