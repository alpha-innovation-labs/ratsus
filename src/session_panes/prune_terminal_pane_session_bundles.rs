use std::collections::BTreeSet;

use crate::app::app_state::AppState;

/// Removes deleted session ids from terminal pane bundle state.
pub fn prune_terminal_pane_session_bundles(app: &mut AppState) {
    let valid_ids = app
        .session_terminals
        .iter()
        .map(|entry| entry.session.id.clone())
        .collect::<BTreeSet<_>>();
    app.terminal_pane_session_bundles.retain(|_, bundle| {
        bundle.retain(|session_id| valid_ids.contains(session_id));
        !bundle.is_empty()
    });
    app.terminal_pane_sessions
        .retain(|_, session_id| valid_ids.contains(session_id));
}
