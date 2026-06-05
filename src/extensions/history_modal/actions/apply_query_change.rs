use crate::app::state::app_state::AppState;
use crate::extensions::history_modal::data::items::history_modal_items;
use crate::extensions::history_modal::selection::clamp_selection::clamp_history_modal_selection;

/// Re-clamps picker selection after the filter query changes.
pub fn apply_history_modal_query_change(app: &mut AppState) {
    let item_count = history_modal_items(
        &app.session_terminals,
        &app.folder_order,
        &app.history_modal.query,
        app.active_index,
        &app.selected_conversation_ids,
        app.history_modal.folder_filter.as_deref(),
        &app.collapsed_folders,
    )
    .len();
    clamp_history_modal_selection(&mut app.history_modal, item_count);
}
