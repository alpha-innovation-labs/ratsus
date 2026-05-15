use std::collections::BTreeMap;

use ratkit::primitives::resizable_grid::PaneId;

use crate::ui::grid_layout::group::split_pane_session_group::SplitPaneSessionGroup;

/// Counts session ids assigned to the panes in one split-session group.
pub fn group_session_count(
    group: &SplitPaneSessionGroup,
    pane_session_bundles: &BTreeMap<PaneId, Vec<String>>,
) -> usize {
    group
        .panes
        .iter()
        .filter_map(|pane_id| pane_session_bundles.get(pane_id))
        .map(Vec::len)
        .sum()
}
