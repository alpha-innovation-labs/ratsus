use crate::app::state::app_state::AppState;
use crate::extensions::history_modal::data::item::{
    HistoryModalItem, HistoryModalItemKind,
};
use crate::extensions::history_modal::data::items::history_modal_items;

/// Moves picker selection to the row for the given stable session id when visible.
pub fn focus_history_modal_session(app: &mut AppState, session_id: &str) {
    let items = history_modal_items(
        &app.session_terminals,
        &app.folder_order,
        &app.history_modal.query,
        app.active_index,
        &app.selected_conversation_ids,
        app.history_modal.folder_filter.as_deref(),
        &app.collapsed_folders,
    );
    if let Some(position) = items
        .iter()
        .position(|item| picker_item_has_session_id(app, item, session_id))
    {
        app.history_modal.selected_position = position;
    }
}

/// Returns whether a picker item points at the requested stable session id.
fn picker_item_has_session_id(
    app: &AppState,
    item: &HistoryModalItem,
    session_id: &str,
) -> bool {
    let HistoryModalItemKind::Session { index, .. } = &item.kind else {
        return false;
    };
    app.session_terminals
        .get(*index)
        .is_some_and(|entry| entry.session.id == session_id)
}
