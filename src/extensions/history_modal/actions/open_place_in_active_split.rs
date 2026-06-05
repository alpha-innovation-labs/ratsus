use crate::app::state::app_state::AppState;
use crate::extensions::history_modal::data::item::ConversationPickerItemKind;
use crate::extensions::history_modal::data::items::conversation_picker_items;
use crate::extensions::history_modal::data::mode::ConversationPickerMode;
use crate::ui::grid_layout::split::split_direction::TerminalSplitDirection;

/// Opens the conversation picker in placement mode for adding an existing chat to a new split.
pub fn open_place_in_active_split_picker(app: &mut AppState, direction: TerminalSplitDirection) {
    app.conversation_picker.is_open = true;
    app.conversation_picker.query.clear();
    app.conversation_picker.is_filtering = false;
    app.conversation_picker.pending_g = false;
    app.conversation_picker.folder_filter = None;
    app.conversation_picker.mode = ConversationPickerMode::PlaceInActiveSplit(direction);

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
        .position(|item| matches!(item.kind, ConversationPickerItemKind::Session { index, .. } if index != app.active_index))
        .unwrap_or(0);
}

#[cfg(test)]
mod tests {
    use super::open_place_in_active_split_picker;
    use crate::app::test_support::app_fixture::app_fixture;
    use crate::app::test_support::dormant_session::dormant_session;
    use crate::extensions::history_modal::data::mode::ConversationPickerMode;
    use crate::ui::grid_layout::split::split_direction::TerminalSplitDirection;

    /// Ctrl+Shift placement mode should open the picker without creating sessions.
    #[test]
    fn opens_picker_in_place_mode_without_creating_session() -> anyhow::Result<()> {
        let mut app = app_fixture(vec![
            dormant_session("Alpha", "a", "/tmp/project"),
            dormant_session("Beta", "b", "/tmp/project"),
        ])?;

        open_place_in_active_split_picker(&mut app, TerminalSplitDirection::Right);

        assert!(app.conversation_picker.is_open);
        assert_eq!(
            app.conversation_picker.mode,
            ConversationPickerMode::PlaceInActiveSplit(TerminalSplitDirection::Right)
        );
        assert_eq!(app.session_terminals.len(), 2);
        Ok(())
    }
}
