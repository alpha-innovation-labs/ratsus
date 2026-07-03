use crate::app::state::app_state::AppState;
use crate::extensions::history_modal::data::item::HistoryModalItemKind;
use crate::extensions::history_modal::data::items::history_modal_items;
use crate::extensions::history_modal::data::mode::HistoryModalMode;

/// Opens the conversation picker and highlights the active conversation when visible.
pub fn open_history_modal(app: &mut AppState) {
    app.history_modal.is_open = true;
    app.history_modal.query.clear();
    app.history_modal.is_filtering = false;
    app.history_modal.pending_g = false;
    app.history_modal.mouse_down_position = None;
    app.history_modal.mouse_drag_moved = false;
    app.history_modal.folder_filter = None;
    app.history_modal.mode = HistoryModalMode::Open;

    let items = history_modal_items(
        &app.session_terminals,
        &app.folder_order,
        "",
        app.active_index,
        &app.selected_conversation_ids,
        app.history_modal.folder_filter.as_deref(),
        &app.collapsed_folders,
    );
    app.history_modal.selected_position = items
        .iter()
        .position(|item| matches!(item.kind, HistoryModalItemKind::Session { index, .. } if index == app.active_index))
        .unwrap_or(0);
}
