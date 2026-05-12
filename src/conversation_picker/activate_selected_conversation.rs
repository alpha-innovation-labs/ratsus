use crate::app::nexus_demo_state::NexusDemo;
use crate::conversation_picker::conversation_picker_item::ConversationPickerItemKind;
use crate::conversation_picker::conversation_picker_items::conversation_picker_items;
use crate::nexus_sessions::start_new_nexus_chat_in_dir::start_new_nexus_chat_in_dir;
use crate::notifications::show_failed_to_start_new_chat_toast::show_failed_to_start_new_chat_toast;

/// Activates the conversation or folder action highlighted in the picker.
pub fn activate_selected_conversation(app: &mut NexusDemo) {
    let items = conversation_picker_items(
        &app.session_terminals,
        &app.folder_order,
        &app.conversation_picker.query,
        app.active_index,
        app.conversation_picker.folder_filter.as_deref(),
    );
    let Some(item) = items.get(app.conversation_picker.selected_position) else {
        return;
    };

    match item.kind.clone() {
        ConversationPickerItemKind::Folder { path } => activate_folder(app, path),
        ConversationPickerItemKind::Session { index } => activate_session(app, index),
    }
}

/// Starts a new chat in the selected folder and closes the picker on success.
fn activate_folder(app: &mut NexusDemo, path: std::path::PathBuf) {
    if let Err(error) = start_new_nexus_chat_in_dir(app, &path) {
        show_failed_to_start_new_chat_toast(&mut app.toast_manager, &error);
        return;
    }
    app.conversation_picker.is_open = false;
    app.conversation_picker.folder_filter = None;
}

/// Focuses an existing selected conversation and closes the picker.
fn activate_session(app: &mut NexusDemo, index: usize) {
    app.focused_index = index;
    app.activate_focused_session();
    app.conversation_picker.is_open = false;
    app.conversation_picker.folder_filter = None;
}
