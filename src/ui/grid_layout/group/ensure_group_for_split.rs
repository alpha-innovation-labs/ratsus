use ratkit::primitives::resizable_grid::PaneId;

use crate::ui::grid_layout::group::add_pane_to_group::add_pane_to_group;
use crate::ui::grid_layout::group::create_split_pane_session_group::create_split_pane_session_group;
use crate::ui::grid_layout::group::pane_group_id::pane_group_id;
use crate::ui::grid_layout::group::split_pane_session_group::SplitPaneSessionGroupId;
use crate::ui::grid_layout::group::split_pane_session_group_state::SplitPaneSessionGroupState;

/// Ensures a split creates or extends the source pane's stable session group.
pub fn ensure_group_for_split(
    state: &mut SplitPaneSessionGroupState,
    source_pane_id: PaneId,
    new_pane_id: PaneId,
) -> SplitPaneSessionGroupId {
    if let Some(group_id) = pane_group_id(state, source_pane_id) {
        add_pane_to_group(state, group_id, new_pane_id);
        return group_id;
    }
    create_split_pane_session_group(state, source_pane_id, new_pane_id)
}
