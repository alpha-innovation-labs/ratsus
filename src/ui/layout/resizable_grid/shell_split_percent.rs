use ratkit::primitives::resizable_grid::ResizableGrid;

use crate::ui::layout::resizable_grid::default_shell_split_percent::default_shell_split_percent;
use crate::ui::layout::resizable_grid::split_indices::SHELL_SPLIT_INDEX;

/// Returns the persisted shell split percent from a layout.
pub fn shell_split_percent(layout: &ResizableGrid) -> u16 {
    layout
        .get_split_ratio(SHELL_SPLIT_INDEX)
        .unwrap_or_else(default_shell_split_percent)
}
