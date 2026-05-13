use crate::app::nexus_demo_state::NexusDemo;

/// Returns the insertion point directly after the active terminal pane bundle.
pub fn active_terminal_bundle_insert_index(app: &NexusDemo) -> usize {
    let Some(bundle) = app
        .terminal_pane_session_bundles
        .get(&app.active_terminal_pane_id)
    else {
        return app
            .active_index
            .saturating_add(1)
            .min(app.session_terminals.len());
    };
    app.session_terminals
        .iter()
        .enumerate()
        .filter(|(_, entry)| bundle.iter().any(|id| id == &entry.session.id))
        .map(|(index, _)| index + 1)
        .max()
        .unwrap_or_else(|| {
            app.active_index
                .saturating_add(1)
                .min(app.session_terminals.len())
        })
}
