use ratkit::primitives::resizable_grid::PaneId;

use crate::ui::grid_layout::group::split_pane_session_group::SplitPaneSessionGroupId;
use crate::ui::grid_layout::group::split_pane_session_group_state::SplitPaneSessionGroupState;

/// Finds the split-session group that currently owns a terminal pane.
pub fn pane_group_id(
    state: &SplitPaneSessionGroupState,
    pane_id: PaneId,
) -> Option<SplitPaneSessionGroupId> {
    state
        .groups
        .iter()
        .find_map(|(group_id, group)| group.panes.contains(&pane_id).then_some(*group_id))
}
