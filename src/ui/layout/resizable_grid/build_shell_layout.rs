use ratkit::primitives::resizable_grid::ResizableGrid;

use crate::ui::layout::resizable_grid::pane_ids::{LEFT_PANE_ID, WORKSPACE_PANE_ID};
use crate::ui::layout::resizable_grid::split_indices::{SHELL_SPLIT_INDEX, WORKSPACE_SPLIT_INDEX};

/// Builds the three-pane app shell layout with workspace, left, and main panes.
pub fn build_shell_layout(shell_split_percent: u16, workspace_split_percent: u16) -> ResizableGrid {
    let mut layout = ResizableGrid::new(LEFT_PANE_ID);
    let _ = layout.split_pane_vertically(LEFT_PANE_ID);
    let _ = layout.split_pane_vertically(LEFT_PANE_ID);
    let _ = layout.move_pane(LEFT_PANE_ID, WORKSPACE_PANE_ID);
    let _ = layout.resize_split(SHELL_SPLIT_INDEX, shell_split_percent);
    let _ = layout.resize_split(WORKSPACE_SPLIT_INDEX, workspace_split_percent);
    layout
}

#[cfg(test)]
mod tests {
    use ratatui::layout::Rect;

    use super::build_shell_layout;
    use crate::ui::layout::resizable_grid::pane_area_by_id::pane_area_by_id;
    use crate::ui::layout::resizable_grid::pane_ids::{
        LEFT_PANE_ID, TERMINAL_PANE_ID, WORKSPACE_PANE_ID,
    };

    /// Verifies the shell layout orders workspace, left, and terminal panes left to right.
    #[test]
    fn orders_workspace_left_and_terminal_panes() {
        let layout = build_shell_layout(40, 25);
        let panes = layout.layout_panes(Rect::new(0, 0, 100, 20));

        let workspace = pane_area_by_id(&panes, WORKSPACE_PANE_ID);
        let left = pane_area_by_id(&panes, LEFT_PANE_ID);
        let terminal = pane_area_by_id(&panes, TERMINAL_PANE_ID);

        assert!(workspace.x < left.x);
        assert!(left.x < terminal.x);
    }
}
