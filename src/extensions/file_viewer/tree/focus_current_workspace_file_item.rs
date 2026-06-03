use std::path::PathBuf;

use crate::extensions::file_viewer::tree::view::FileSystemTreeView;
use crate::extensions::file_viewer::tree::workspace_file_row::WorkspaceFileRow;

impl FileSystemTreeView {
    /// Focuses the nth visible file item in the workspace represented by the selected file.
    pub(crate) fn focus_current_workspace_file_item(&mut self, item_index: usize) {
        let Some(workspace_root) = self.current_file_workspace_root() else {
            return;
        };
        let Some(row_index) = self.current_workspace_file_item_row(&workspace_root, item_index)
        else {
            return;
        };
        self.select_workspace_row(row_index);
    }

    /// Returns the workspace root represented by the currently selected file path.
    fn current_file_workspace_root(&self) -> Option<PathBuf> {
        let selected_path = self
            .workspace_selected_path
            .as_ref()
            .unwrap_or(&self.root_path);
        self.workspace_roots
            .iter()
            .find(|root| selected_path.starts_with(root))
            .cloned()
            .or_else(|| self.workspace_roots.first().cloned())
    }

    /// Returns the visible row for the nth non-root file item in one workspace.
    fn current_workspace_file_item_row(
        &self,
        workspace_root: &PathBuf,
        item_index: usize,
    ) -> Option<usize> {
        self.workspace_rows()
            .iter()
            .enumerate()
            .filter_map(|(row_index, row)| match row {
                WorkspaceFileRow::Entry { path, .. } if path.starts_with(workspace_root) => {
                    Some(row_index)
                }
                WorkspaceFileRow::WorkspaceFolder { .. } | WorkspaceFileRow::Entry { .. } => None,
            })
            .nth(item_index)
    }
}
