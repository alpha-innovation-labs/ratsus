use crate::app::state::app_state::AppState;
use crate::ui::left_panel::session::list_row::SessionListRow;
use crate::ui::left_panel::session::visible_rows::visible_session_rows_with_folders;
use crate::ui::workspace_pane::selected_folder_order::selected_folder_order;

impl AppState {
    /// Returns the currently visible left-panel tree rows from the row cache.
    pub fn visible_rows(&self) -> Vec<SessionListRow> {
        if !self.workspace_view_enabled {
            return visible_session_rows_with_folders(
                &self.session_terminals,
                &self.collapsed_folders,
                &self.folder_order,
                Some(self.focused_index),
                &self.split_pane_session_groups,
                &self.terminal_pane_session_bundles,
            );
        }
        let folder_order = selected_workspace_folder_order(self);
        self.visible_rows_cache
            .borrow_mut()
            .rows(
                &self.session_terminals,
                &self.collapsed_folders,
                &folder_order,
                Some(self.active_index),
                &self.split_pane_session_groups,
                &self.terminal_pane_session_bundles,
            )
            .to_vec()
    }

    /// Returns the currently visible left-panel tree row count from the row cache.
    pub fn visible_row_count(&self) -> usize {
        if !self.workspace_view_enabled {
            return visible_session_rows_with_folders(
                &self.session_terminals,
                &self.collapsed_folders,
                &self.folder_order,
                Some(self.focused_index),
                &self.split_pane_session_groups,
                &self.terminal_pane_session_bundles,
            )
            .len();
        }
        let folder_order = selected_workspace_folder_order(self);
        self.visible_rows_cache.borrow_mut().row_count(
            &self.session_terminals,
            &self.collapsed_folders,
            &folder_order,
            Some(self.active_index),
            &self.split_pane_session_groups,
            &self.terminal_pane_session_bundles,
        )
    }

    /// Returns how many times the visible row cache has rebuilt rows in this test app.
    #[cfg(test)]
    pub fn visible_rows_rebuild_count(&self) -> usize {
        self.visible_rows_cache.borrow().rebuild_count()
    }
}

/// Returns folder order for the active left-pane view mode.
fn selected_workspace_folder_order(app: &AppState) -> Vec<std::path::PathBuf> {
    if app.workspace_view_enabled {
        return selected_folder_order(&app.folder_order, app.selected_workspace_path.as_ref());
    }
    app.folder_order.clone()
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::app::test_support::app_fixture::app_fixture;
    use crate::app::test_support::dormant_session::dormant_session;
    use crate::ui::left_panel::session::list_row::SessionListRow;

    /// Verifies legacy workspace view mode keeps all folder rows in the left pane.
    #[test]
    fn legacy_workspace_view_returns_all_folders() -> anyhow::Result<()> {
        let mut app = app_fixture(vec![
            dormant_session("Alpha", "a", "/workspace/alpha"),
            dormant_session("Beta", "b", "/workspace/beta"),
        ])?;
        app.folder_order = vec![
            PathBuf::from("/workspace/alpha"),
            PathBuf::from("/workspace/beta"),
        ];
        app.selected_workspace_path = Some(PathBuf::from("/workspace/beta"));
        app.workspace_view_enabled = false;

        let rows = app.visible_rows();

        assert!(rows.iter().any(|row| matches!(
            row,
            SessionListRow::Folder { path, .. } if path == &PathBuf::from("/workspace/alpha")
        )));
        assert!(rows.iter().any(|row| matches!(
            row,
            SessionListRow::Folder { path, .. } if path == &PathBuf::from("/workspace/beta")
        )));
        Ok(())
    }
}
