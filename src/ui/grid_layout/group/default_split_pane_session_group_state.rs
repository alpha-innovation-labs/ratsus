use std::collections::BTreeMap;

use crate::ui::grid_layout::group::split_pane_session_group_state::SplitPaneSessionGroupState;

/// Builds empty split-pane group state with human group numbering starting at one.
pub fn default_split_pane_session_group_state() -> SplitPaneSessionGroupState {
    SplitPaneSessionGroupState {
        groups: BTreeMap::new(),
        next_id: 1,
    }
}
