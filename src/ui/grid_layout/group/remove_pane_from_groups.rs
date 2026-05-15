use ratkit::primitives::resizable_grid::PaneId;

use crate::ui::grid_layout::group::split_pane_session_group_state::SplitPaneSessionGroupState;

/// Removes a terminal pane from every split-session group that references it.
pub fn remove_pane_from_groups(state: &mut SplitPaneSessionGroupState, pane_id: PaneId) {
    for group in state.groups.values_mut() {
        group.panes.retain(|candidate| *candidate != pane_id);
    }
}
