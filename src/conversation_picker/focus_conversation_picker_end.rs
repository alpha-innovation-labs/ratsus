use crate::app::nexus_demo_state::NexusDemo;
use crate::conversation_picker::current_conversation_picker_item_count::current_conversation_picker_item_count;

/// Moves conversation picker selection to the last visible row.
pub fn focus_conversation_picker_end(app: &mut NexusDemo) {
    app.conversation_picker.selected_position =
        current_conversation_picker_item_count(app).saturating_sub(1);
}
