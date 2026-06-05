use crate::app::state::app_state::AppState;
use crate::extensions::history_modal::data::item::HistoryModalItemKind;
use crate::extensions::history_modal::data::items::history_modal_items;
use crate::extensions::history_modal::data::mode::HistoryModalMode;
use crate::extensions::harness::sessions::creation::start_new_chat_in_dir::start_new_chat_in_dir;
use crate::ui::grid_layout::split::place_existing_session_in_split::place_existing_session_in_terminal_split;
use crate::ui::grid_layout::split::split_direction::TerminalSplitDirection;
use crate::ui::notifications::toast::show_failed_to_start_new_chat::show_failed_to_start_new_chat_toast;

/// Activates the conversation or folder action highlighted in the picker.
pub fn activate_selected_conversation(app: &mut AppState) {
    let items = history_modal_items(
        &app.session_terminals,
        &app.folder_order,
        &app.history_modal.query,
        app.active_index,
        &app.selected_conversation_ids,
        app.history_modal.folder_filter.as_deref(),
        &app.collapsed_folders,
    );
    let Some(item) = items.get(app.history_modal.selected_position) else {
        return;
    };

    match app.history_modal.mode {
        HistoryModalMode::Open => match item.kind.clone() {
            HistoryModalItemKind::Folder { path, .. } => activate_folder(app, path),
            HistoryModalItemKind::Session { index, .. } => activate_session(app, index),
        },
        HistoryModalMode::PlaceInActiveSplit(direction) => match item.kind {
            HistoryModalItemKind::Session { index, .. } => {
                place_session(app, index, direction)
            }
            HistoryModalItemKind::Folder { .. } => {}
        },
    }
}

/// Starts a new chat in the selected folder and closes the picker on success.
fn activate_folder(app: &mut AppState, path: std::path::PathBuf) {
    if let Err(error) = start_new_chat_in_dir(app, &path) {
        show_failed_to_start_new_chat_toast(&mut app.toast_manager, &error);
        return;
    }
    close_picker(app);
}

/// Focuses an existing selected conversation and closes the picker.
fn activate_session(app: &mut AppState, index: usize) {
    app.focused_index = index;
    app.activate_focused_session();
    close_picker(app);
}

/// Places an existing conversation into a new split and closes the picker on success.
fn place_session(app: &mut AppState, index: usize, direction: TerminalSplitDirection) {
    if let Err(error) = place_existing_session_in_terminal_split(app, index, direction) {
        show_failed_to_start_new_chat_toast(&mut app.toast_manager, &error);
        return;
    }
    close_picker(app);
}

/// Closes the picker and restores normal activation mode.
fn close_picker(app: &mut AppState) {
    app.history_modal.is_open = false;
    app.history_modal.folder_filter = None;
    app.history_modal.mode = HistoryModalMode::Open;
}

#[cfg(test)]
mod tests {
    use super::activate_selected_conversation;
    use crate::app::test_support::app_fixture::app_fixture;
    use crate::app::test_support::dormant_session::dormant_session;
    use crate::extensions::history_modal::data::mode::HistoryModalMode;
    use crate::ui::grid_layout::split::split_direction::TerminalSplitDirection;
    use crate::ui::layout::resizable_grid::pane_ids::TERMINAL_PANE_ID;

    /// Placement mode should put an existing selected session in a grouped split pane.
    #[test]
    fn place_mode_splits_existing_session_without_creating_one() -> anyhow::Result<()> {
        let mut app = app_fixture(vec![
            dormant_session("Alpha", "a", "/tmp/project"),
            dormant_session("Beta", "b", "/tmp/project"),
        ])?;
        app.history_modal.is_open = true;
        app.history_modal.mode =
            HistoryModalMode::PlaceInActiveSplit(TerminalSplitDirection::Right);
        app.history_modal.selected_position = 2;
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
            Some("a")
        );
        assert!(app
            .terminal_pane_session_bundles
            .values()
            .any(|bundle| bundle == &vec!["b".to_string()]));
        assert_eq!(app.split_pane_session_groups.groups.len(), 1);
        assert!(!app.history_modal.is_open);
        Ok(())
    }
}
