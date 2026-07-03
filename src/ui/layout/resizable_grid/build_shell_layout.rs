use ratkit::primitives::resizable_grid::ResizableGrid;

use crate::ui::layout::resizable_grid::pane_ids::LEFT_PANE_ID;
use crate::ui::layout::resizable_grid::split_indices::SHELL_SPLIT_INDEX;

/// Builds the two-pane app shell layout with left and main panes.
pub fn build_shell_layout(shell_split_percent: u16) -> ResizableGrid {
    let mut layout = ResizableGrid::new(LEFT_PANE_ID);
    let _ = layout.split_pane_vertically(LEFT_PANE_ID);
    let _ = layout.resize_split(SHELL_SPLIT_INDEX, shell_split_percent);
    layout
}

#[cfg(test)]
mod tests {
    use ratatui::layout::Rect;

    use super::build_shell_layout;
    use crate::ui::layout::resizable_grid::pane_area_by_id::pane_area_by_id;
    use crate::ui::layout::resizable_grid::pane_ids::{LEFT_PANE_ID, TERMINAL_PANE_ID};

    /// Verifies the shell layout orders left and terminal panes left to right.
    #[test]
    fn orders_left_and_terminal_panes() {
        let layout = build_shell_layout(40);
        let panes = layout.layout_panes(Rect::new(0, 0, 100, 20));

        let left = pane_area_by_id(&panes, LEFT_PANE_ID);
        let terminal = pane_area_by_id(&panes, TERMINAL_PANE_ID);

        assert!(left.x < terminal.x);
    }
}
