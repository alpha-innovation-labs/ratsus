use crate::app::app_state::AppState;
use crate::conversation_picker::clamp_conversation_picker_selection::clamp_conversation_picker_selection;
use crate::conversation_picker::conversation_picker_items::conversation_picker_items;

/// Re-clamps picker selection after the filter query changes.
pub fn apply_conversation_picker_query_change(app: &mut AppState) {
    let item_count = conversation_picker_items(
        &app.session_terminals,
        &app.folder_order,
        &app.conversation_picker.query,
        app.active_index,
        &app.selected_conversation_ids,
        app.conversation_picker.folder_filter.as_deref(),
        &app.collapsed_folders,
    )
    .len();
    clamp_conversation_picker_selection(&mut app.conversation_picker, item_count);
}
