use crate::app::nexus_demo_state::NexusDemo;
use crate::app::toggle_conversation_selection_by_index::toggle_conversation_selection_by_index;
use crate::conversation_picker::conversation_picker_item::ConversationPickerItemKind;
use crate::conversation_picker::selected_conversation_picker_item::selected_conversation_picker_item;

/// Toggles bulk-selection state for the selected conversation picker session row.
pub fn toggle_selected_conversation_picker_item(app: &mut NexusDemo) {
    let Some(item) = selected_conversation_picker_item(app) else {
        return;
    };
    if let ConversationPickerItemKind::Session { index, .. } = item.kind {
        toggle_conversation_selection_by_index(app, index);
    }
}
