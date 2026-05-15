use ratkit::primitives::resizable_grid::PaneId;

use crate::app::state::app_state::AppState;

/// Returns whether a pane id is present in the current terminal layout tree.
pub fn pane_exists_in_terminal_layout(app: &AppState, pane_id: PaneId) -> bool {
    app.terminal_layout
        .layout_panes(app.last_terminal_area)
        .into_iter()
        .any(|pane| pane.pane_id() == pane_id)
}
