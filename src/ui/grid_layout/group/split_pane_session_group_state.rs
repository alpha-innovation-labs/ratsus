use std::collections::BTreeMap;

use serde::{Deserialize, Serialize};

use crate::ui::grid_layout::group::split_pane_session_group::{
    SplitPaneSessionGroup, SplitPaneSessionGroupId,
};

/// Owns stable split-pane groups and the next creation-order identifier.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SplitPaneSessionGroupState {
    pub groups: BTreeMap<SplitPaneSessionGroupId, SplitPaneSessionGroup>,
    pub next_id: SplitPaneSessionGroupId,
}
