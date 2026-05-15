use ratkit::primitives::resizable_grid::types::LayoutNode;
use ratkit::primitives::resizable_grid::{ResizableGrid, SplitAxis};

use crate::ui::grid_layout::persistence::persisted_resizable_grid::{
    PersistedLayoutNode, PersistedResizableGrid, PersistedSplitAxis,
};

/// Converts a live Ratkit resizable grid into serializable state.
pub fn persisted_resizable_grid_from_layout(layout: &ResizableGrid) -> PersistedResizableGrid {
    PersistedResizableGrid {
        root_index: layout.root_index,
        nodes: layout.nodes.iter().map(persisted_layout_node).collect(),
        next_pane_id: layout.next_pane_id,
        hit_threshold: layout.hit_threshold,
    }
}

/// Converts one live Ratkit layout node into serializable state.
fn persisted_layout_node(node: &LayoutNode) -> PersistedLayoutNode {
    match node {
        LayoutNode::Pane { id } => PersistedLayoutNode::Pane { id: *id },
        LayoutNode::Split {
            axis,
            ratio,
            first,
            second,
        } => PersistedLayoutNode::Split {
            axis: persisted_split_axis(*axis),
            ratio: *ratio,
            first: *first,
            second: *second,
        },
    }
}

/// Converts one live Ratkit split axis into serializable state.
fn persisted_split_axis(axis: SplitAxis) -> PersistedSplitAxis {
    match axis {
        SplitAxis::Horizontal => PersistedSplitAxis::Horizontal,
        SplitAxis::Vertical => PersistedSplitAxis::Vertical,
    }
}
