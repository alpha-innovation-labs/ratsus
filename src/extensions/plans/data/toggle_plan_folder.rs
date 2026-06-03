use std::path::Path;

use crate::extensions::plans::data::plan_list_state::PlanListState;

impl PlanListState {
    /// Toggles whether a grouped plan folder is collapsed.
    pub fn toggle_folder(&mut self, path: &Path) {
        if !self.collapsed_folders.insert(path.to_path_buf()) {
            self.collapsed_folders.remove(path);
        }
        self.keep_focused_visible();
    }

    /// Collapses the focused folder or the focused plan's parent folder.
    pub fn collapse_focused_folder(&mut self) {
        let Some(path) = self.focused_folder_path() else {
            return;
        };
        self.collapsed_folders.insert(path);
        self.keep_focused_visible();
    }

    /// Expands the focused folder or the focused plan's parent folder.
    pub fn expand_focused_folder(&mut self) {
        let Some(path) = self.focused_folder_path() else {
            return;
        };
        self.collapsed_folders.remove(&path);
        self.keep_focused_visible();
    }

    /// Returns the folder represented by the focused grouped row.
    fn focused_folder_path(&self) -> Option<std::path::PathBuf> {
        match self.visible_rows().get(self.focused_row) {
            Some(crate::extensions::plans::data::plan_list_row::PlanListRow::Folder {
                path,
                ..
            }) => Some(path.clone()),
            Some(crate::extensions::plans::data::plan_list_row::PlanListRow::Plan { index }) => {
                self.plans.get(*index).map(|plan| plan.folder.clone())
            }
            None => None,
        }
    }
}
