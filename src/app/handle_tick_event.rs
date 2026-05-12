use ratkit::CoordinatorAction;

use crate::app::nexus_demo_state::NexusDemo;
use crate::app::redraw_action::redraw_action;
use crate::layout::is_resizing_layout::is_resizing_layout;
use crate::nexus_sessions::drain_session_refreshes::drain_session_refreshes;

/// Converts completed background refreshes and terminal redraw flags into a coordinator action.
pub fn handle_tick_event(app: &mut NexusDemo, tick_count: u64) -> CoordinatorAction {
    app.loader_tick = tick_count;
    let sessions_changed = drain_session_refreshes(app);
    if is_resizing_layout(&app.layout_widget_state) {
        return redraw_action(sessions_changed);
    }

    let terminal_changed = app
        .session_terminals
        .iter()
        .filter_map(|entry| entry.terminal.as_ref())
        .any(|terminal| terminal.take_needs_redraw());
    let loader_changed = app
        .session_terminals
        .iter()
        .any(|entry| entry.session.is_running);
    redraw_action(sessions_changed || terminal_changed || loader_changed)
}
