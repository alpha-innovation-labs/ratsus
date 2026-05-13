use crate::app::nexus_demo_state::NexusDemo;
use crate::conversation_picker::conversation_picker_item::ConversationPickerItemKind;
use crate::conversation_picker::conversation_picker_items::conversation_picker_items;
use crate::conversation_picker::conversation_picker_mode::ConversationPickerMode;
use crate::nexus_sessions::start_new_nexus_chat_in_dir::start_new_nexus_chat_in_dir;
use crate::notifications::show_failed_to_start_new_chat_toast::show_failed_to_start_new_chat_toast;
use crate::session_panes::place_existing_session_in_active_terminal_pane::place_existing_session_in_active_terminal_pane;

/// Activates the conversation or folder action highlighted in the picker.
pub fn activate_selected_conversation(app: &mut NexusDemo) {
    let items = conversation_picker_items(
        &app.session_terminals,
        &app.folder_order,
        &app.conversation_picker.query,
        app.active_index,
        &app.selected_conversation_ids,
        app.conversation_picker.folder_filter.as_deref(),
        &app.collapsed_folders,
    );
    let Some(item) = items.get(app.conversation_picker.selected_position) else {
        return;
    };

    match app.conversation_picker.mode {
        ConversationPickerMode::Open => match item.kind.clone() {
            ConversationPickerItemKind::Folder { path, .. } => activate_folder(app, path),
            ConversationPickerItemKind::Session { index, .. } => activate_session(app, index),
        },
        ConversationPickerMode::PlaceInActiveSplit => match item.kind {
            ConversationPickerItemKind::Session { index, .. } => place_session(app, index),
            ConversationPickerItemKind::Folder { .. } => {}
        },
    }
}

/// Starts a new chat in the selected folder and closes the picker on success.
fn activate_folder(app: &mut NexusDemo, path: std::path::PathBuf) {
    if let Err(error) = start_new_nexus_chat_in_dir(app, &path) {
        show_failed_to_start_new_chat_toast(&mut app.toast_manager, &error);
        return;
    }
    close_picker(app);
}

/// Focuses an existing selected conversation and closes the picker.
fn activate_session(app: &mut NexusDemo, index: usize) {
    app.focused_index = index;
    app.activate_focused_session();
    close_picker(app);
}

/// Places an existing conversation into the active split and closes the picker.
fn place_session(app: &mut NexusDemo, index: usize) {
    place_existing_session_in_active_terminal_pane(app, index);
    close_picker(app);
}

/// Closes the picker and restores normal activation mode.
fn close_picker(app: &mut NexusDemo) {
    app.conversation_picker.is_open = false;
    app.conversation_picker.folder_filter = None;
    app.conversation_picker.mode = ConversationPickerMode::Open;
}

#[cfg(test)]
mod tests {
    use super::activate_selected_conversation;
    use crate::conversation_picker::conversation_picker_mode::ConversationPickerMode;
    use crate::layout::pane_ids::TERMINAL_PANE_ID;
    use crate::test_support::dormant_session::dormant_session;
    use crate::test_support::nexus_demo_fixture::nexus_demo_fixture;

    /// Placement mode should add an existing selected session to the active split bundle.
    #[test]
    fn place_mode_bundles_existing_session_without_creating_one() -> anyhow::Result<()> {
        let mut app = nexus_demo_fixture(vec![
            dormant_session("Alpha", "a", "/tmp/project"),
            dormant_session("Beta", "b", "/tmp/project"),
        ])?;
        app.conversation_picker.is_open = true;
        app.conversation_picker.mode = ConversationPickerMode::PlaceInActiveSplit;
        app.conversation_picker.selected_position = 2;
        app.terminal_pane_sessions
            .insert(TERMINAL_PANE_ID, "a".to_string());
        app.terminal_pane_session_bundles
            .insert(TERMINAL_PANE_ID, vec!["a".to_string()]);

        activate_selected_conversation(&mut app);

        assert_eq!(app.session_terminals.len(), 2);
        assert_eq!(
            app.terminal_pane_sessions
                .get(&TERMINAL_PANE_ID)
                .map(String::as_str),
            Some("b")
        );
        assert_eq!(
            app.terminal_pane_session_bundles.get(&TERMINAL_PANE_ID),
            Some(&vec!["a".to_string(), "b".to_string()])
        );
        assert!(!app.conversation_picker.is_open);
        Ok(())
    }
}
