use ratkit::primitives::resizable_grid::PaneId;

use crate::ui::grid_layout::group::split_pane_session_group::SplitPaneSessionGroupId;
use crate::ui::grid_layout::group::split_pane_session_group_state::SplitPaneSessionGroupState;

/// Adds a terminal pane to a split group while preserving first-seen pane order.
pub fn add_pane_to_group(
    state: &mut SplitPaneSessionGroupState,
    group_id: SplitPaneSessionGroupId,
    pane_id: PaneId,
) {
    let Some(group) = state.groups.get_mut(&group_id) else {
        return;
    };
    if !group.panes.contains(&pane_id) {
        group.panes.push(pane_id);
    }
}
