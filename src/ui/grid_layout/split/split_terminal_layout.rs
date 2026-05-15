use ratkit::primitives::resizable_grid::{PaneId, ResizableGrid};

use crate::ui::grid_layout::split::split_direction::TerminalSplitDirection;

/// Splits one terminal pane in the requested direction.
pub fn split_terminal_layout(
    layout: &mut ResizableGrid,
    pane_id: PaneId,
    direction: TerminalSplitDirection,
) -> Option<PaneId> {
    match direction {
        TerminalSplitDirection::Right => layout.split_pane_vertically(pane_id),
        TerminalSplitDirection::Bottom => layout.split_pane_horizontally(pane_id),
    }
}
