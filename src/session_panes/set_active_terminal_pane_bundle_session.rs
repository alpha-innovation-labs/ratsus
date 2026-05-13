use ratkit::primitives::resizable_grid::PaneId;

use crate::app::nexus_demo_state::NexusDemo;

/// Activates one session inside an existing terminal pane bundle.
pub fn set_active_terminal_pane_bundle_session(
    app: &mut NexusDemo,
    pane_id: PaneId,
    session_id: String,
) {
    app.active_terminal_pane_id = pane_id;
    app.terminal_pane_sessions
        .insert(pane_id, session_id.clone());
    app.terminal_pane_session_bundles
        .entry(pane_id)
        .or_default()
        .push(session_id);
    deduplicate_bundle(app, pane_id);
}

/// Removes duplicate session ids from one pane bundle while preserving order.
fn deduplicate_bundle(app: &mut NexusDemo, pane_id: PaneId) {
    let Some(bundle) = app.terminal_pane_session_bundles.get_mut(&pane_id) else {
        return;
    };
    let mut seen = std::collections::BTreeSet::new();
    bundle.retain(|session_id| seen.insert(session_id.clone()));
}
