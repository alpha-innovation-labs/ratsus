use std::collections::BTreeMap;

use ratkit::primitives::resizable_grid::PaneId;

use crate::ui::grid_layout::group::group_session_count::group_session_count;
use crate::ui::grid_layout::group::split_pane_session_group_state::SplitPaneSessionGroupState;

/// Removes split groups that no longer contain at least two pane-bound sessions.
pub fn compact_split_pane_session_groups(
    state: &mut SplitPaneSessionGroupState,
    pane_session_bundles: &BTreeMap<PaneId, Vec<String>>,
) {
    state
        .groups
        .retain(|_, group| group_session_count(group, pane_session_bundles) >= 2);
}
