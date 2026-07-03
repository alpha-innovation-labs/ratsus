use std::path::PathBuf;

use crate::app::state::app_state::AppState;
use crate::extensions::history_modal::data::item::HistoryModalItemKind;
use crate::extensions::history_modal::data::items::history_modal_items;
use crate::extensions::history_modal::data::mode::HistoryModalMode;

/// Opens the conversation picker and highlights the active conversation when visible.
pub fn open_history_modal(app: &mut AppState) {
    app.history_modal.is_open = true;
    app.history_modal.query.clear();
    app.history_modal.is_filtering = false;
    app.history_modal.pending_g = false;
    app.history_modal.mouse_down_position = None;
    app.history_modal.mouse_drag_moved = false;
    app.history_modal.folder_filter = None;
    app.history_modal.mode = HistoryModalMode::Open;

    let items = history_modal_items(
        &app.session_terminals,
        &app.folder_order,
        "",
        app.active_index,
        &app.selected_conversation_ids,
        app.history_modal.folder_filter.as_deref(),
        &app.collapsed_folders,
    );
    app.history_modal.selected_position = items
        .iter()
        .position(|item| matches!(item.kind, HistoryModalItemKind::Session { index, .. } if index == app.active_index))
        .unwrap_or(0);
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::open_history_modal;
    use crate::app::test_support::app_fixture::app_fixture;
    use crate::app::test_support::dormant_session::dormant_session;
    use crate::extensions::history_modal::data::item::HistoryModalItemKind;
    use crate::extensions::history_modal::data::items::history_modal_items;

    /// Verifies Ctrl+H opens the picker without a folder filter by default.
    #[test]
    fn opens_without_folder_filter() -> anyhow::Result<()> {
        let mut app = app_fixture(vec![
            dormant_session("Alpha", "a", "/workspace/alpha"),
            dormant_session("Beta", "b", "/workspace/beta"),
        ])?;
        app.folder_order = vec![
            PathBuf::from("/workspace/alpha"),
            PathBuf::from("/workspace/beta"),
        ];

        open_history_modal(&mut app);

        let items = history_modal_items(
            &app.session_terminals,
            &app.folder_order,
            "",
            app.active_index,
            &app.selected_conversation_ids,
            app.history_modal.folder_filter.as_deref(),
            &app.collapsed_folders,
        );
        assert!(items.iter().any(|item| matches!(
            &item.kind,
            HistoryModalItemKind::Folder { path, .. } if path == &PathBuf::from("/workspace/alpha")
        )));
        assert!(items.iter().any(|item| matches!(
            &item.kind,
            HistoryModalItemKind::Folder { path, .. } if path == &PathBuf::from("/workspace/beta")
        )));
        Ok(())
    }
}
