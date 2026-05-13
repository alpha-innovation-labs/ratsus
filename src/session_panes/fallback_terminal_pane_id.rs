use ratkit::primitives::resizable_grid::PaneId;

use crate::app::nexus_demo_state::NexusDemo;

/// Returns a remaining terminal pane id after a pane is removed.
pub fn fallback_terminal_pane_id(app: &NexusDemo) -> Option<PaneId> {
    app.terminal_layout
        .layout_panes(app.last_terminal_area)
        .first()
        .map(|pane| pane.pane_id())
}
