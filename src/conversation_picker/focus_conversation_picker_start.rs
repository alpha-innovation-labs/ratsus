use crate::app::nexus_demo_state::NexusDemo;

/// Moves conversation picker selection to the first visible row.
pub fn focus_conversation_picker_start(app: &mut NexusDemo) {
    app.conversation_picker.selected_position = 0;
}
