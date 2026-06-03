use std::path::PathBuf;

use crate::extensions::plans::data::plan_list_row::PlanListRow;
use crate::extensions::plans::data::plan_list_state::PlanListState;

impl PlanListState {
    /// Focuses the nth visible plan item in the workspace represented by the selected plan.
    pub fn focus_visible_row(&mut self, item_index: usize) {
        let Some(workspace_path) = self.current_plan_workspace_path() else {
            return;
        };
        let Some(row_index) = self.current_workspace_plan_item_row(&workspace_path, item_index)
        else {
            return;
        };
        self.focused_row = row_index;
        self.activate_focused();
    }

    /// Returns the workspace represented by the active or focused plan-list row.
    fn current_plan_workspace_path(&self) -> Option<PathBuf> {
        self.active_index
            .and_then(|index| self.plans.get(index))
            .map(|plan| plan.folder.clone())
            .or_else(|| self.focused_row_workspace_path())
            .or_else(|| self.folder_order.first().cloned())
    }

    /// Returns the workspace represented by the currently focused visible row.
    fn focused_row_workspace_path(&self) -> Option<PathBuf> {
        match self.visible_rows().get(self.focused_row)? {
            PlanListRow::Folder { path, .. } => Some(path.clone()),
            PlanListRow::Plan { index } => self.plans.get(*index).map(|plan| plan.folder.clone()),
        }
    }

    /// Returns the visible row for the nth plan in one workspace.
    fn current_workspace_plan_item_row(
        &self,
        workspace_path: &PathBuf,
        item_index: usize,
    ) -> Option<usize> {
        self.visible_rows()
            .iter()
            .enumerate()
            .filter_map(|(row_index, row)| match row {
                PlanListRow::Plan { index }
                    if self
                        .plans
                        .get(*index)
                        .is_some_and(|plan| plan.folder == *workspace_path) =>
                {
                    Some(row_index)
                }
                PlanListRow::Folder { .. } | PlanListRow::Plan { .. } => None,
            })
            .nth(item_index)
    }
}
