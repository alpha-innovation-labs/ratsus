use std::path::PathBuf;

use crate::app::state::app_state::AppState;
use crate::extensions::file_viewer::tree::sync_workspace_root::sync_file_viewer_workspace_root;
use crate::ui::grid_layout::persistence::persist_multiplexer_state::persist_multiplexer_state;
use crate::ui::workspace_pane::focus_workspace_session::focus_workspace_session;
use crate::ui::workspace_pane::remember_active_workspace_session::remember_active_workspace_session;

/// Selects one workspace folder, restores its focused session, and resets folder-scoped viewport state.
pub fn select_workspace(app: &mut AppState, path: PathBuf) -> bool {
    if !app.folder_order.iter().any(|folder| folder == &path) {
        return false;
    }
    let changed = app.selected_workspace_path.as_ref() != Some(&path);
    if !changed {
        return false;
    }
    remember_active_workspace_session(app);
    app.selected_workspace_path = Some(path.clone());
    sync_file_viewer_workspace_root(app);
    app.session_scroll = 0;
    app.focused_row = 0;
    app.suppress_left_focus_scroll = false;
    let _ = focus_workspace_session(app, path.as_path());
    persist_multiplexer_state(app);
    true
}

#[cfg(test)]
mod file_viewer_tests {
    use std::fs;

    use uuid::Uuid;

    use super::select_workspace;
    use crate::app::test_support::app_fixture::app_fixture;
    use crate::extensions::file_viewer::tree::view::FileSystemTreeView;

    /// Selecting a workspace should point the Files tab at that workspace root.
    #[test]
    fn selecting_workspace_updates_file_viewer_root() -> anyhow::Result<()> {
        let alpha = temp_workspace("alpha")?;
        let beta = temp_workspace("beta")?;
        fs::write(beta.join("visible.txt"), "beta")?;
        let mut app = app_fixture(Vec::new())?;
        app.folder_order = vec![alpha.clone(), beta.clone()];
        app.selected_workspace_path = Some(alpha.clone());
        app.file_system_tree_view = FileSystemTreeView::with_root(alpha.clone())?;

        assert!(select_workspace(&mut app, beta.clone()));

        assert_eq!(app.file_system_tree_view.root_path(), beta.as_path());
        assert!(app
            .file_system_tree_view
            .root_child_names()
            .contains(&"visible.txt".to_string()));
        let _ = fs::remove_dir_all(alpha);
        let _ = fs::remove_dir_all(beta);
        Ok(())
    }

    /// Creates a test-owned workspace directory.
    fn temp_workspace(name: &str) -> anyhow::Result<std::path::PathBuf> {
        let root = std::env::temp_dir().join(format!(
            "ratsus_workspace_{name}_{}_{}",
            std::process::id(),
            Uuid::new_v4()
        ));
        fs::create_dir_all(&root)?;
        Ok(root)
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::select_workspace;
    use crate::app::test_support::app_fixture::app_fixture;
    use crate::app::test_support::dormant_session::dormant_session;
    use crate::ui::layout::focus::focused_pane::FocusedPane;

    /// Verifies switching workspaces restores the workspace's remembered focused session.
    #[test]
    fn restores_remembered_workspace_session() {
        let mut app = app_fixture(vec![
            dormant_session("a-one", "a-one", "/workspace/a"),
            dormant_session("a-two", "a-two", "/workspace/a"),
            dormant_session("b-one", "b-one", "/workspace/b"),
            dormant_session("b-two", "b-two", "/workspace/b"),
        ])
        .expect("app fixture");
        app.folder_order = vec![PathBuf::from("/workspace/a"), PathBuf::from("/workspace/b")];
        app.selected_workspace_path = Some(PathBuf::from("/workspace/a"));
        app.active_index = 1;
        app.focused_index = 1;
        app.workspace_focused_session_ids
            .insert(PathBuf::from("/workspace/b"), "b-two".to_string());

        assert!(select_workspace(&mut app, PathBuf::from("/workspace/b")));

        assert_eq!(app.active_index, 3);
        assert_eq!(app.focused_index, 3);
        assert_eq!(app.focused_pane, FocusedPane::Terminal);
        assert_eq!(
            app.workspace_focused_session_ids
                .get(&PathBuf::from("/workspace/a")),
            Some(&"a-two".to_string())
        );
    }

    /// Verifies switching workspaces focuses the first session when no remembered session exists.
    #[test]
    fn focuses_first_workspace_session_without_memory() {
        let mut app = app_fixture(vec![
            dormant_session("a-one", "a-one", "/workspace/a"),
            dormant_session("b-one", "b-one", "/workspace/b"),
            dormant_session("b-two", "b-two", "/workspace/b"),
        ])
        .expect("app fixture");
        app.folder_order = vec![PathBuf::from("/workspace/a"), PathBuf::from("/workspace/b")];
        app.selected_workspace_path = Some(PathBuf::from("/workspace/a"));

        assert!(select_workspace(&mut app, PathBuf::from("/workspace/b")));

        assert_eq!(app.active_index, 1);
        assert_eq!(app.focused_pane, FocusedPane::Terminal);
    }
}
