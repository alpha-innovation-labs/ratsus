use crate::app::navigation::toggle_conversation_selection_by_index::toggle_conversation_selection_by_index;
use crate::app::state::app_state::AppState;
use crate::extensions::history_modal::data::item::HistoryModalItemKind;
use crate::extensions::history_modal::selection::selected_item::selected_history_modal_item;

/// Toggles bulk-selection state for the selected conversation picker session row.
pub fn toggle_selected_history_modal_item(app: &mut AppState) {
    let Some(item) = selected_history_modal_item(app) else {
        return;
    };
    if let HistoryModalItemKind::Session { index, .. } = item.kind {
        toggle_conversation_selection_by_index(app, index);
    }
}
