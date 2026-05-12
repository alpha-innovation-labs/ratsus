use crate::app::nexus_demo_state::NexusDemo;

/// Closes the conversation picker without changing the active conversation.
pub fn close_conversation_picker(app: &mut NexusDemo) {
    app.conversation_picker.is_open = false;
    app.conversation_picker.folder_filter = None;
}
