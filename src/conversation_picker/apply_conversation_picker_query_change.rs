use crate::app::nexus_demo_state::NexusDemo;
use crate::conversation_picker::clamp_conversation_picker_selection::clamp_conversation_picker_selection;
use crate::conversation_picker::conversation_picker_items::conversation_picker_items;

/// Re-clamps picker selection after the filter query changes.
pub fn apply_conversation_picker_query_change(app: &mut NexusDemo) {
    let item_count = conversation_picker_items(
        &app.session_terminals,
        &app.folder_order,
        &app.conversation_picker.query,
        app.active_index,
        app.conversation_picker.folder_filter.as_deref(),
    )
    .len();
    clamp_conversation_picker_selection(&mut app.conversation_picker, item_count);
}
