use crate::extensions::plans::data::plan_list_row::PlanListRow;
use crate::extensions::plans::data::plan_list_state::PlanListState;

/// Resolves the plan index rendered under a mouse row.
pub fn plan_row_for_mouse(state: &PlanListState, row: u16) -> Option<usize> {
    if row < state.last_area.y || row >= state.last_area.y.saturating_add(state.last_area.height) {
        return None;
    }
    let local_row = usize::from(row.saturating_sub(state.last_area.y));
    let visible_row = state.scroll + local_row;
    match state.visible_rows().get(visible_row) {
        Some(PlanListRow::Plan { index }) => Some(*index),
        Some(PlanListRow::Folder { .. }) | None => None,
    }
}
