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
        left_pane_mode: app.left_pane_mode,
        active_main_pane_tab: app.active_main_pane_tab,
        selected_expo_folder: app.selected_expo_folder.clone(),
        active_plan_path: app.plan_list.active_plan().map(|plan| plan.path.clone()),
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
    use crate::extensions::file_viewer::tabs::tab::MainPaneTab;
    use crate::extensions::file_viewer::tree::view::FileSystemTreeView;
    use crate::ui::left_panel::mode::left_pane_mode::LeftPaneMode;

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
        app.file_system_tree_view
            .apply_workspace_open_state(std::slice::from_ref(&nested), std::slice::from_ref(&root));

        let state = capture_multiplexer_state(&app);

        assert!(state
            .file_system_tree
            .expanded_paths_by_root
            .get(&root)
            .is_some_and(|paths| paths.contains(&nested)));
        assert!(state
            .file_system_tree
            .workspace_expanded_paths
            .contains(&nested));
        assert!(state
            .file_system_tree
            .workspace_collapsed_paths
            .contains(&root));
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

    /// Capturing app state should include the last focused app surfaces.
    #[test]
    fn captures_current_focus_surfaces() -> anyhow::Result<()> {
        let root = temp_workspace()?;
        let file_path = root.join("visible.txt");
        fs::write(&file_path, "visible")?;
        fs::create_dir_all(root.join("plans"))?;
        let plan_path = root.join("plans/alpha.md");
        fs::write(&plan_path, "# Alpha")?;
        let mut app = app_fixture(Vec::new())?;
        app.folder_order = vec![root.clone()];
        app.left_pane_mode = LeftPaneMode::Files;
        app.active_main_pane_tab = MainPaneTab::Files;
        app.selected_expo_folder = Some(root.clone());
        app.plan_list.sync_workspace_folders(&app.folder_order)?;
        app.plan_list.active_index = Some(0);
        app.file_system_tree_view = FileSystemTreeView::with_root(root.clone())?;
        app.file_system_tree_view
            .sync_workspace_roots(&app.folder_order);
        app.file_system_tree_view.select_workspace_row(2);

        let state = capture_multiplexer_state(&app);

        assert_eq!(state.left_pane_mode, LeftPaneMode::Files);
        assert_eq!(state.active_main_pane_tab, MainPaneTab::Files);
        assert_eq!(state.selected_expo_folder, Some(root.clone()));
        assert_eq!(state.active_plan_path, Some(plan_path));
        assert_eq!(state.file_system_tree.selected_path, Some(file_path));
        let _ = fs::remove_dir_all(root);
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
