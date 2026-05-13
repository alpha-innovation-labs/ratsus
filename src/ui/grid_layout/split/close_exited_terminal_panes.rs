use ratkit::primitives::resizable_grid::PaneId;

use crate::app::state::app_state::AppState;
use crate::ui::grid_layout::split::close_terminal_pane::close_terminal_pane;

/// Closes split panes whose displayed session exited, leaving the last pane for fallback logic.
pub fn close_exited_terminal_panes(app: &mut AppState, pane_ids: &[PaneId]) -> bool {
    let mut changed = false;
    for pane_id in pane_ids {
        if terminal_pane_count(app) <= 1 {
            break;
        }
        changed |= close_terminal_pane(app, *pane_id);
    }
    changed
}

/// Returns the number of currently visible terminal panes.
fn terminal_pane_count(app: &AppState) -> usize {
    app.terminal_layout
        .layout_panes(app.last_terminal_area)
        .len()
}
