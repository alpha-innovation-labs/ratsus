use crate::app::state::app_state::AppState;
use crate::extensions::file_viewer::tree::persisted_file_system_tree_state::persisted_file_system_tree_state;
use crate::ui::grid_layout::persistence::capture_workspace_state::capture_workspace_state;
use crate::ui::grid_layout::persistence::persisted_multiplexer_state::PersistedMultiplexerState;
use crate::ui::grid_layout::persistence::persisted_resizable_grid_from_layout::persisted_resizable_grid_from_layout;

const MULTIPLEXER_STATE_VERSION: u8 = 1;

/// Captures the current split-pane multiplexer state for persistence.
pub fn capture_multiplexer_state(app: &AppState) -> PersistedMultiplexerState {
    PersistedMultiplexerState {
        version: MULTIPLEXER_STATE_VERSION,
        terminal_layout: persisted_resizable_grid_from_layout(&app.terminal_layout),
        terminal_pane_sessions: app.terminal_pane_sessions.clone(),
        terminal_pane_session_bundles: app.terminal_pane_session_bundles.clone(),
        split_pane_session_groups: app.split_pane_session_groups.clone(),
        active_terminal_pane_id: app.active_terminal_pane_id,
        active_session_id: app
            .session_terminals
            .get(app.active_index)
            .map(|entry| entry.session.id.clone()),
        file_system_tree: persisted_file_system_tree_state(
            &app.file_system_tree_expanded_paths,
            &app.file_system_tree_view,
        ),
        workspace: capture_workspace_state(app),
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use uuid::Uuid;

    use super::capture_multiplexer_state;
    use crate::app::test_support::app_fixture::app_fixture;
    use crate::extensions::file_viewer::tree::view::FileSystemTreeView;

    /// Capturing app state should include currently expanded file-viewer folders.
    #[test]
    fn captures_current_file_viewer_expanded_folders() -> anyhow::Result<()> {
        let root = temp_workspace()?;
        let nested = root.join("nested");
        fs::create_dir_all(&nested)?;
        let mut app = app_fixture(Vec::new())?;
        app.file_system_tree_view = FileSystemTreeView::with_root(root.clone())?;
        app.file_system_tree_view.select_path(vec![0, 0]);
        app.file_system_tree_view.expand_or_enter_child();

        let state = capture_multiplexer_state(&app);

        assert!(state
            .file_system_tree
            .expanded_paths_by_root
            .get(&root)
            .is_some_and(|paths| paths.contains(&nested)));
        let _ = fs::remove_dir_all(root);
        Ok(())
    }

    /// Capturing app state should include workspace mode and order.
    #[test]
    fn captures_workspace_mode_and_order() -> anyhow::Result<()> {
        let mut app = app_fixture(Vec::new())?;
        app.workspace_view_enabled = false;
        app.folder_order = vec!["/workspace/b".into(), "/workspace/a".into()];
        app.selected_workspace_path = Some("/workspace/b".into());

        let state = capture_multiplexer_state(&app);

        assert!(!state.workspace.workspace_view_enabled);
        assert_eq!(state.workspace.workspace_order, app.folder_order);
        assert_eq!(
            state.workspace.selected_workspace_path,
            Some("/workspace/b".into())
        );
        Ok(())
    }

    /// Creates a test-owned workspace directory.
    fn temp_workspace() -> anyhow::Result<std::path::PathBuf> {
        let root = std::env::temp_dir().join(format!(
            "ratsus_capture_file_tree_{}_{}",
            std::process::id(),
            Uuid::new_v4()
        ));
        fs::create_dir_all(&root)?;
        Ok(root)
    }
}
