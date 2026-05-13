use ratkit::primitives::resizable_grid::PaneId;

use crate::app::nexus_demo_state::NexusDemo;

/// Returns all session ids bundled into a terminal pane.
pub fn terminal_pane_session_ids(app: &NexusDemo, pane_id: PaneId) -> &[String] {
    app.terminal_pane_session_bundles
        .get(&pane_id)
        .map(Vec::as_slice)
        .unwrap_or(&[])
}
