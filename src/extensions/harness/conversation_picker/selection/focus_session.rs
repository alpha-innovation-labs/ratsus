use crate::app::state::app_state::AppState;
use crate::extensions::harness::conversation_picker::data::item::{
    ConversationPickerItem, ConversationPickerItemKind,
};
use crate::extensions::harness::conversation_picker::data::items::conversation_picker_items;

/// Moves picker selection to the row for the given stable session id when visible.
pub fn focus_conversation_picker_session(app: &mut AppState, session_id: &str) {
    let items = conversation_picker_items(
        &app.session_terminals,
        &app.folder_order,
        &app.conversation_picker.query,
        app.active_index,
        &app.selected_conversation_ids,
        app.conversation_picker.folder_filter.as_deref(),
        &app.collapsed_folders,
    );
    if let Some(position) = items
        .iter()
        .position(|item| picker_item_has_session_id(app, item, session_id))
    {
        app.conversation_picker.selected_position = position;
    }
}

/// Returns whether a picker item points at the requested stable session id.
fn picker_item_has_session_id(
    app: &AppState,
    item: &ConversationPickerItem,
    session_id: &str,
) -> bool {
    let ConversationPickerItemKind::Session { index, .. } = &item.kind else {
        return false;
    };
    app.session_terminals
        .get(*index)
        .is_some_and(|entry| entry.session.id == session_id)
}
