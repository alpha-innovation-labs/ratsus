use ratkit::primitives::resizable_grid::types::LayoutNode;
use ratkit::primitives::resizable_grid::{ResizableGrid, SplitAxis};

use crate::ui::grid_layout::persistence::persisted_resizable_grid::{
    PersistedLayoutNode, PersistedResizableGrid, PersistedSplitAxis,
};

/// Converts persisted layout state into a live Ratkit resizable grid.
pub fn resizable_grid_from_persisted(persisted: &PersistedResizableGrid) -> Option<ResizableGrid> {
    if persisted.nodes.is_empty() || persisted.root_index >= persisted.nodes.len() {
        return None;
    }
    let mut grid = ResizableGrid::new(0);
    grid.root_index = persisted.root_index;
    grid.nodes = persisted
        .nodes
        .iter()
        .map(layout_node_from_persisted)
        .collect();
    grid.next_pane_id = persisted.next_pane_id;
    grid.hovered_split = None;
    grid.dragging_split = None;
    grid.hit_threshold = persisted.hit_threshold;
    Some(grid)
}

/// Converts one persisted layout node into a live Ratkit layout node.
fn layout_node_from_persisted(persisted: &PersistedLayoutNode) -> LayoutNode {
    match persisted {
        PersistedLayoutNode::Pane { id } => LayoutNode::Pane { id: *id },
        PersistedLayoutNode::Split {
            axis,
            ratio,
            first,
            second,
        } => LayoutNode::Split {
            axis: split_axis_from_persisted(*axis),
            ratio: *ratio,
            first: *first,
            second: *second,
        },
    }
}

/// Converts one persisted split axis into a live Ratkit split axis.
fn split_axis_from_persisted(axis: PersistedSplitAxis) -> SplitAxis {
    match axis {
        PersistedSplitAxis::Horizontal => SplitAxis::Horizontal,
        PersistedSplitAxis::Vertical => SplitAxis::Vertical,
    }
}
