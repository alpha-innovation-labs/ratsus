use ratatui::layout::Rect;
use ratkit::primitives::resizable_grid::PaneId;

use crate::app::app_state::AppState;
use crate::session_panes::session_index_for_pane::session_index_for_pane;

/// Resizes the terminal assigned to one split pane when its visible area changes.
pub fn resize_terminal_pane_session(app: &mut AppState, pane_id: PaneId, area: Rect) {
    if app
        .terminal_pane_areas
        .get(&pane_id)
        .is_some_and(|last| *last == area)
    {
        return;
    }
    let Some(index) = session_index_for_pane(app, pane_id) else {
        return;
    };
    if let Some(entry) = app.session_terminals.get_mut(index) {
        if let Ok(terminal) = entry.ensure_terminal(area.height.max(1), area.width.max(1)) {
            terminal.resize(area.height.max(1), area.width.max(1));
        }
    }
    app.terminal_pane_areas.insert(pane_id, area);
    if pane_id == app.active_terminal_pane_id {
        app.active_terminal_area = area;
    }
}
