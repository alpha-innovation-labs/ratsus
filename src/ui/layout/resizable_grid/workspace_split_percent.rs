use ratkit::primitives::resizable_grid::ResizableGrid;

use crate::ui::layout::resizable_grid::default_workspace_split_percent::default_workspace_split_percent;
use crate::ui::layout::resizable_grid::split_indices::WORKSPACE_SPLIT_INDEX;

/// Returns the persisted workspace split percent from a layout.
pub fn workspace_split_percent(layout: &ResizableGrid) -> u16 {
    layout
        .get_split_ratio(WORKSPACE_SPLIT_INDEX)
        .unwrap_or_else(default_workspace_split_percent)
}
