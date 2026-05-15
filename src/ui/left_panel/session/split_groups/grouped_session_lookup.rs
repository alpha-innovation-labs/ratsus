use std::collections::BTreeMap;

use crate::ui::grid_layout::group::split_pane_session_group::SplitPaneSessionGroupId;
use crate::ui::left_panel::session::split_groups::grouped_session_child::GroupedSessionChild;

/// Lookup tables used to replace flat session rows with split-group rows.
#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct GroupedSessionLookup {
    pub group_id_by_session_index: BTreeMap<usize, SplitPaneSessionGroupId>,
    pub children_by_group_id: BTreeMap<SplitPaneSessionGroupId, Vec<GroupedSessionChild>>,
}
