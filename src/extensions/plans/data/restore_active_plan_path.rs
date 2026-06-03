use std::path::Path;

use crate::extensions::plans::data::plan_list_state::PlanListState;

/// Restores plan-list focus to a previously active plan file path.
pub fn restore_active_plan_path(state: &mut PlanListState, active_plan_path: &Path) -> bool {
    let Some(index) = state
        .plans
        .iter()
        .position(|plan| plan.path == active_plan_path)
    else {
        return false;
    };
    state.active_index = Some(index);
    state.focus_row_for_plan_index(index);
    state.activate_focused();
    true
}
