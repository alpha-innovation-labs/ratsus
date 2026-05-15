use crate::extensions::file_viewer::preview::preview_state_for_path::preview_state_for_path;
use crate::extensions::plans::data::plan_entry::PlanEntry;
use crate::extensions::plans::data::plan_list_state::PlanListState;

impl PlanListState {
    /// Returns the active plan entry when one is selected.
    pub fn active_plan(&self) -> Option<&PlanEntry> {
        self.active_index.and_then(|index| self.plans.get(index))
    }

    /// Activates the currently focused visible plan.
    pub fn activate_focused(&mut self) {
        let visible = self.visible_indices();
        let Some(index) = visible.get(self.focused_row).copied() else {
            self.active_index = None;
            self.preview_state = None;
            return;
        };
        self.active_index = Some(index);
        self.preview_state = Some(preview_state_for_path(&self.plans[index].path, false));
        self.keep_focused_visible();
    }
}
