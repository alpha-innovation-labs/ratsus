use crate::app::state::app_state::AppState;
use crate::ui::workspace_pane::select_workspace::select_workspace;
use crate::ui::workspace_pane::select_workspace_first_session::select_workspace_first_session;
use crate::ui::workspace_pane::workspace_paths::workspace_paths;

/// Selects a workspace or grouped folder by zero-based visible order.
pub fn select_workspace_by_visible_index(app: &mut AppState, index: usize) -> bool {
    let Some(path) = workspace_paths(app).get(index).cloned() else {
        return false;
    };
    if app.workspace_view_enabled {
        return select_workspace(app, path);
    }
    select_workspace_first_session(app, path)
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::select_workspace_by_visible_index;
    use crate::app::test_support::app_fixture::app_fixture;
    use crate::app::test_support::dormant_session::dormant_session;
    use crate::ui::layout::focus::focused_pane::FocusedPane;

    /// Verifies workspace selection uses the persisted folder order.
    #[test]
    fn selects_workspace_by_visible_order() {
        let mut app = app_fixture(Vec::new()).expect("app fixture");
        app.folder_order = vec![PathBuf::from("/a"), PathBuf::from("/b")];

        assert!(select_workspace_by_visible_index(&mut app, 1));

        assert_eq!(app.selected_workspace_path, Some(PathBuf::from("/b")));
    }

    /// Verifies grouped folder mode selects the first session in the target folder.
    #[test]
    fn selects_first_folder_session_when_workspace_view_is_disabled() {
        let mut app = app_fixture(vec![
            dormant_session("a-one", "a-one", "/workspace/a"),
            dormant_session("b-one", "b-one", "/workspace/b"),
            dormant_session("b-two", "b-two", "/workspace/b"),
        ])
        .expect("app fixture");
        app.workspace_view_enabled = false;
        app.folder_order = vec![PathBuf::from("/workspace/a"), PathBuf::from("/workspace/b")];
        app.active_index = 0;
        app.focused_index = 0;

        assert!(select_workspace_by_visible_index(&mut app, 1));

        assert_eq!(
            app.selected_workspace_path,
            Some(PathBuf::from("/workspace/b"))
        );
        assert_eq!(app.active_index, 1);
        assert_eq!(app.focused_index, 1);
        assert_eq!(app.focused_pane, FocusedPane::Terminal);
    }
}
