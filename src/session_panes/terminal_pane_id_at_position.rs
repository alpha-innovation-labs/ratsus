use ratkit::primitives::resizable_grid::PaneId;

use crate::app::app_state::AppState;

/// Finds the split terminal pane under a screen position.
pub fn terminal_pane_id_at_position(app: &AppState, column: u16, row: u16) -> Option<PaneId> {
    app.terminal_layout
        .layout_panes(app.last_terminal_area)
        .into_iter()
        .find(|pane| {
            let area = pane.area();
            column >= area.x
                && column < area.x.saturating_add(area.width)
                && row >= area.y
                && row < area.y.saturating_add(area.height)
        })
        .map(|pane| pane.pane_id())
}
