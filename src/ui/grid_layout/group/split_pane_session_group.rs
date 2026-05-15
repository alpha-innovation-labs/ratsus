use ratkit::primitives::resizable_grid::PaneId;
use serde::{Deserialize, Serialize};

/// Stable identifier for one split-pane session group.
pub type SplitPaneSessionGroupId = u64;

/// References the terminal panes that belong to one visible split-session group.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SplitPaneSessionGroup {
    pub id: SplitPaneSessionGroupId,
    pub name: String,
    pub panes: Vec<PaneId>,
}
