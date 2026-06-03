use std::path::Path;

use crate::extensions::file_viewer::tree::view::FileSystemTreeView;
use crate::extensions::file_viewer::tree::workspace_file_row::WorkspaceFileRow;

impl FileSystemTreeView {
    /// Restores grouped file-tree focus to a previously selected filesystem path.
    pub(crate) fn restore_workspace_selected_path(&mut self, selected_path: &Path) -> bool {
        let Some(row_index) = self.workspace_rows().iter().position(|row| match row {
            WorkspaceFileRow::WorkspaceFolder { path } | WorkspaceFileRow::Entry { path, .. } => {
                path == selected_path
            }
        }) else {
            return false;
        };
        self.workspace_focused_row = row_index;
        self.activate_workspace_focused_row();
        true
    }
}
