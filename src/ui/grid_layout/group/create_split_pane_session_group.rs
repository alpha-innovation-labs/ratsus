use ratkit::primitives::resizable_grid::PaneId;

use crate::ui::grid_layout::group::split_pane_session_group::{
    SplitPaneSessionGroup, SplitPaneSessionGroupId,
};
use crate::ui::grid_layout::group::split_pane_session_group_state::SplitPaneSessionGroupState;

/// Creates a split-session group from the source pane and its first split child.
pub fn create_split_pane_session_group(
    state: &mut SplitPaneSessionGroupState,
    source_pane_id: PaneId,
    new_pane_id: PaneId,
) -> SplitPaneSessionGroupId {
    let id = state.next_id.max(1);
    state.next_id = id + 1;
    state.groups.insert(
        id,
        SplitPaneSessionGroup {
            id,
            name: format!("Group {id}"),
            panes: vec![source_pane_id, new_pane_id],
        },
    );
    id
}
