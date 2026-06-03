use crate::extensions::file_viewer::preview::preview_state_for_path::preview_state_for_path;
use crate::extensions::plans::data::plan_entry::PlanEntry;
use crate::extensions::plans::data::plan_list_state::PlanListState;
use crate::extensions::plans::watch::start_selected_plan_watcher::start_selected_plan_watcher;

impl PlanListState {
    /// Returns the active plan entry when one is selected.
    pub fn active_plan(&self) -> Option<&PlanEntry> {
        self.active_index.and_then(|index| self.plans.get(index))
    }

    /// Activates the currently focused visible plan while preserving preview on folder rows.
    pub fn activate_focused(&mut self) {
        let Some(index) = self.focused_plan_index() else {
            if self.visible_rows().is_empty() {
                self.active_index = None;
                self.preview_state = None;
                self.selected_plan_watcher = None;
            }
            self.keep_focused_visible();
            return;
        };
        self.active_index = Some(index);
        self.preview_state = Some(preview_state_for_path(&self.plans[index].path, false));
        self.selected_plan_watcher = start_selected_plan_watcher(&self.plans[index].path);
        self.keep_focused_visible();
    }
}
