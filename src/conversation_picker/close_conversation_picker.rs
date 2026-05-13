use crate::app::app_state::AppState;
use crate::conversation_picker::conversation_picker_mode::ConversationPickerMode;

/// Closes the conversation picker without changing the active conversation.
pub fn close_conversation_picker(app: &mut AppState) {
    app.conversation_picker.is_open = false;
    app.conversation_picker.is_filtering = false;
    app.conversation_picker.pending_g = false;
    app.conversation_picker.folder_filter = None;
    app.conversation_picker.mode = ConversationPickerMode::Open;
}
