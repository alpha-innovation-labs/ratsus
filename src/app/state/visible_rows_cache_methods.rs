use crate::app::state::app_state::AppState;
use crate::ui::left_panel::session::list_row::SessionListRow;
use crate::ui::left_panel::session::visible_rows::visible_session_rows_with_folders;

impl AppState {
    /// Returns the currently visible left-panel tree rows.
    pub fn visible_rows(&self) -> Vec<SessionListRow> {
        visible_session_rows_with_folders(
            &self.session_terminals,
            &self.collapsed_folders,
            &self.folder_order,
            Some(self.focused_index),
            &self.split_pane_session_groups,
            &self.terminal_pane_session_bundles,
        )
    }

    /// Returns the currently visible left-panel tree row count.
    pub fn visible_row_count(&self) -> usize {
        self.visible_rows().len()
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use crate::app::test_support::app_fixture::app_fixture;
    use crate::app::test_support::dormant_session::dormant_session;
    use crate::ui::left_panel::session::list_row::SessionListRow;

    /// Verifies the consolidated view returns folder headers followed by all sessions.
    #[test]
    fn consolidated_view_returns_folder_headers_and_all_sessions() -> anyhow::Result<()> {
        let mut app = app_fixture(vec![
            dormant_session("Alpha One", "alpha-1", "/workspace/alpha"),
            dormant_session("Alpha Two", "alpha-2", "/workspace/alpha"),
            dormant_session("Beta One", "beta-1", "/workspace/beta"),
        ])?;
        app.folder_order = vec![
            PathBuf::from("/workspace/alpha"),
            PathBuf::from("/workspace/beta"),
        ];

        let rows = app.visible_rows();

        assert!(matches!(
            rows[0],
            SessionListRow::Folder { path, .. } if path == &PathBuf::from("/workspace/alpha")
        ));
        assert_eq!(rows[1], SessionListRow::Session { index: 0 });
        assert_eq!(rows[2], SessionListRow::Session { index: 1 });
        assert!(matches!(
            rows[3],
            SessionListRow::Folder { path, .. } if path == &PathBuf::from("/workspace/beta")
        ));
        assert_eq!(rows[4], SessionListRow::Session { index: 2 });
        assert!(!rows
            .iter()
            .any(|row| matches!(row, SessionListRow::Folder { path: _, .. } if false)));
        Ok(())
    }
}
