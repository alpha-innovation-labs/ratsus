use std::collections::BTreeMap;

use ratatui::layout::Rect;
use ratsus::app::state::app_state::AppState;
use ratsus::ui::layout::resizable_grid::pane_ids::TERMINAL_PANE_ID;

/// Creates a side-by-side terminal grid using real layout APIs and Nexus sessions.
pub fn prepare_grid(app: &mut AppState) -> anyhow::Result<()> {
    let Some(left_session) = app
        .session_terminals
        .first()
        .map(|entry| entry.session.id.clone())
    else {
        return Ok(());
    };
    let right_session = app
        .session_terminals
        .get(1)
        .map(|entry| entry.session.id.clone())
        .unwrap_or_else(|| left_session.clone());
    if let Some(right_pane) = app.terminal_layout.split_pane_vertically(TERMINAL_PANE_ID) {
        app.terminal_pane_sessions = BTreeMap::from([
            (TERMINAL_PANE_ID, left_session.clone()),
            (right_pane, right_session.clone()),
        ]);
        app.terminal_pane_session_bundles = BTreeMap::from([
            (TERMINAL_PANE_ID, vec![left_session]),
            (right_pane, vec![right_session]),
        ]);
        app.terminal_pane_areas = BTreeMap::from([
            (TERMINAL_PANE_ID, Rect::new(18, 5, 29, 13)),
            (right_pane, Rect::new(49, 5, 29, 13)),
        ]);
    }
    Ok(())
}
