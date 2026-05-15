use ratkit::primitives::resizable_grid::PaneId;

use crate::ui::grid_layout::group::split_pane_session_group_state::SplitPaneSessionGroupState;

/// Returns the visible group name for a terminal pane when it belongs to a split group.
pub fn pane_group_name(state: &SplitPaneSessionGroupState, pane_id: PaneId) -> Option<&str> {
    state
        .groups
        .values()
        .find(|group| group.panes.contains(&pane_id))
        .map(|group| group.name.as_str())
}
