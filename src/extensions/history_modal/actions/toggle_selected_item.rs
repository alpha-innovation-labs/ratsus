use crate::app::navigation::toggle_conversation_selection_by_index::toggle_conversation_selection_by_index;
use crate::app::state::app_state::AppState;
use crate::extensions::history_modal::data::item::ConversationPickerItemKind;
use crate::extensions::history_modal::selection::selected_item::selected_conversation_picker_item;

/// Toggles bulk-selection state for the selected conversation picker session row.
pub fn toggle_selected_conversation_picker_item(app: &mut AppState) {
    let Some(item) = selected_conversation_picker_item(app) else {
        return;
    };
    if let ConversationPickerItemKind::Session { index, .. } = item.kind {
        toggle_conversation_selection_by_index(app, index);
    }
}
