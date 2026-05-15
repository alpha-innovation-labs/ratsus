use crate::app::state::app_state::AppState;
use crate::extensions::harness::conversation_picker::data::item::ConversationPickerItemKind;
use crate::extensions::harness::conversation_picker::data::items::conversation_picker_items;
use crate::extensions::harness::conversation_picker::data::mode::ConversationPickerMode;

/// Opens the conversation picker and highlights the active conversation when visible.
pub fn open_conversation_picker(app: &mut AppState) {
    app.conversation_picker.is_open = true;
    app.conversation_picker.query.clear();
    app.conversation_picker.is_filtering = false;
    app.conversation_picker.pending_g = false;
    app.conversation_picker.mouse_down_position = None;
    app.conversation_picker.mouse_drag_moved = false;
    app.conversation_picker.folder_filter = app.selected_workspace_path.clone();
    app.conversation_picker.mode = ConversationPickerMode::Open;

    let items = conversation_picker_items(
        &app.session_terminals,
        &app.folder_order,
        "",
        app.active_index,
        &app.selected_conversation_ids,
        app.conversation_picker.folder_filter.as_deref(),
        &app.collapsed_folders,
    );
    app.conversation_picker.selected_position = items
        .iter()
        .position(|item| matches!(item.kind, ConversationPickerItemKind::Session { index, .. } if index == app.active_index))
        .unwrap_or(0);
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::open_conversation_picker;
    use crate::app::test_support::app_fixture::app_fixture;
    use crate::app::test_support::dormant_session::dormant_session;
    use crate::extensions::harness::conversation_picker::data::item::ConversationPickerItemKind;
    use crate::extensions::harness::conversation_picker::data::items::conversation_picker_items;

    /// Verifies Ctrl+H opens the picker scoped to the selected workspace folder.
    #[test]
    fn opens_scoped_to_selected_workspace() -> anyhow::Result<()> {
        let mut app = app_fixture(vec![
            dormant_session("Alpha", "a", "/workspace/alpha"),
            dormant_session("Beta", "b", "/workspace/beta"),
        ])?;
        app.folder_order = vec![
            PathBuf::from("/workspace/alpha"),
            PathBuf::from("/workspace/beta"),
        ];
        app.selected_workspace_path = Some(PathBuf::from("/workspace/beta"));

        open_conversation_picker(&mut app);

        let items = conversation_picker_items(
            &app.session_terminals,
            &app.folder_order,
            "",
            app.active_index,
            &app.selected_conversation_ids,
            app.conversation_picker.folder_filter.as_deref(),
            &app.collapsed_folders,
        );
        assert!(items.iter().any(|item| matches!(
            &item.kind,
            ConversationPickerItemKind::Folder { path, .. } if path == &PathBuf::from("/workspace/beta")
        )));
        assert!(!items.iter().any(|item| matches!(
            &item.kind,
            ConversationPickerItemKind::Folder { path, .. } if path == &PathBuf::from("/workspace/alpha")
        )));
        Ok(())
    }
}
