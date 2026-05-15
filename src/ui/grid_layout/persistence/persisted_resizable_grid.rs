use ratkit::primitives::resizable_grid::PaneId;
use serde::{Deserialize, Serialize};

/// Serializable copy of a Ratkit resizable-grid layout.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersistedResizableGrid {
    pub root_index: usize,
    pub nodes: Vec<PersistedLayoutNode>,
    pub next_pane_id: PaneId,
    pub hit_threshold: u16,
}

/// Serializable copy of one resizable-grid node.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum PersistedLayoutNode {
    Pane {
        id: PaneId,
    },
    Split {
        axis: PersistedSplitAxis,
        ratio: u16,
        first: usize,
        second: usize,
    },
}

/// Serializable copy of a Ratkit split axis.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum PersistedSplitAxis {
    Horizontal,
    Vertical,
}
