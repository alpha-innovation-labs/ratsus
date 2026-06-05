use crate::app::state::app_state::AppState;
use crate::extensions::history_modal::data::items::history_modal_items;

/// Returns the number of rows currently visible in the conversation picker.
pub fn current_history_modal_item_count(app: &AppState) -> usize {
    history_modal_items(
        &app.session_terminals,
        &app.folder_order,
        &app.history_modal.query,
        app.active_index,
        &app.selected_conversation_ids,
        app.history_modal.folder_filter.as_deref(),
        &app.collapsed_folders,
    )
    .len()
}
