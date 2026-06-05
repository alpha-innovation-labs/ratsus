use crate::app::navigation::reorder_session_to_index::reorder_session_to_index;
use crate::app::state::app_state::AppState;
use crate::extensions::history_modal::data::item::{
    ConversationPickerItem, ConversationPickerItemKind,
};
use crate::extensions::history_modal::data::items::conversation_picker_items;
use crate::extensions::history_modal::selection::focus_session::focus_conversation_picker_session;

/// Reorders the selected picker conversation by moving it to the next visible session target.
pub fn reorder_selected_conversation(app: &mut AppState, direction: isize) {
    let items = current_items(app);
    let selected_position = app.conversation_picker.selected_position;
    let Some(from_index) = selected_session_index(&items, selected_position) else {
        return;
    };
    let Some(session_id) = session_id_at_index(app, from_index) else {
        return;
    };
    let Some(target_index) = target_session_index(&items, selected_position, direction) else {
        return;
    };
    if reorder_session_to_index(app, from_index, target_index) {
        focus_conversation_picker_session(app, &session_id);
    }
}

/// Builds the current picker item snapshot.
fn current_items(app: &AppState) -> Vec<ConversationPickerItem> {
    conversation_picker_items(
        &app.session_terminals,
        &app.folder_order,
        &app.conversation_picker.query,
        app.active_index,
        &app.selected_conversation_ids,
        app.conversation_picker.folder_filter.as_deref(),
        &app.collapsed_folders,
    )
}

/// Returns the session vector index for a picker item position.
fn selected_session_index(items: &[ConversationPickerItem], position: usize) -> Option<usize> {
    let item = items.get(position)?;
    session_index_for_item(item)
}

/// Returns a stable session id for a session vector index.
fn session_id_at_index(app: &AppState, index: usize) -> Option<String> {
    app.session_terminals
        .get(index)
        .map(|entry| entry.session.id.clone())
}

/// Finds the nearest visible session item in the requested direction.
fn target_session_index(
    items: &[ConversationPickerItem],
    selected_position: usize,
    direction: isize,
) -> Option<usize> {
    let selected_position = selected_position.min(items.len());
    if direction < 0 {
        return items[..selected_position]
            .iter()
            .rev()
            .find_map(session_index_for_item);
    }
    items
        .iter()
        .skip(selected_position.saturating_add(1))
        .find_map(session_index_for_item)
}

/// Returns the session vector index for a session picker item.
fn session_index_for_item(item: &ConversationPickerItem) -> Option<usize> {
    match &item.kind {
        ConversationPickerItemKind::Session { index, .. } => Some(*index),
        ConversationPickerItemKind::Folder { .. } => None,
    }
}
