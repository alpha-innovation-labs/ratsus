use ratkit::primitives::resizable_grid::PaneId;

use crate::app::state::app_state::AppState;

/// Returns the split pane close button hit by a screen position.
pub fn terminal_pane_close_button_at_position(
    app: &AppState,
    column: u16,
    row: u16,
) -> Option<PaneId> {
    app.terminal_pane_close_buttons
        .iter()
        .find_map(|(pane_id, area)| {
            (column >= area.x
                && column < area.x.saturating_add(area.width)
                && row >= area.y
                && row < area.y.saturating_add(area.height))
            .then_some(*pane_id)
        })
}
