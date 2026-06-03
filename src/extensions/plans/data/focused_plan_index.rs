use crate::extensions::plans::data::plan_list_row::PlanListRow;
use crate::extensions::plans::data::plan_list_state::PlanListState;

impl PlanListState {
    /// Returns the plan index under the currently focused grouped row.
    pub fn focused_plan_index(&self) -> Option<usize> {
        match self.visible_rows().get(self.focused_row) {
            Some(PlanListRow::Plan { index }) => Some(*index),
            _ => None,
        }
    }
}
