use crate::app::state::app_state::AppState;
use crate::extensions::history_modal::data::item::HistoryModalItem;
use crate::extensions::history_modal::data::items::history_modal_items;

/// Returns the currently selected conversation picker item.
pub fn selected_history_modal_item(app: &AppState) -> Option<HistoryModalItem> {
    history_modal_items(
        &app.session_terminals,
        &app.folder_order,
        &app.history_modal.query,
        app.active_index,
        &app.selected_conversation_ids,
        app.history_modal.folder_filter.as_deref(),
        &app.collapsed_folders,
    )
    .get(app.history_modal.selected_position)
    .cloned()
}
