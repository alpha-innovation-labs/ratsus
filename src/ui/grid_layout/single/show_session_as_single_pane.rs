use ratkit::primitives::resizable_grid::ResizableGrid;

use crate::app::state::app_state::AppState;
use crate::ui::grid_layout::group::default_split_pane_session_group_state::default_split_pane_session_group_state;
use crate::ui::grid_layout::persistence::capture_multiplexer_state::capture_multiplexer_state;
use crate::ui::grid_layout::persistence::persist_multiplexer_state::persist_multiplexer_state;
use crate::ui::grid_layout::split::set_active_session::set_active_terminal_pane_session;
use crate::ui::layout::resizable_grid::pane_ids::TERMINAL_PANE_ID;

/// Displays one session by itself without swapping it into an existing split group.
pub fn show_session_as_single_pane(app: &mut AppState, session_id: String) {
    suspend_current_multiplexer_state(app);
    app.terminal_layout = ResizableGrid::new(TERMINAL_PANE_ID);
    app.active_terminal_pane_id = TERMINAL_PANE_ID;
    app.terminal_pane_sessions.clear();
    app.terminal_pane_session_bundles.clear();
    app.split_pane_session_groups = default_split_pane_session_group_state();
    set_active_terminal_pane_session(app, session_id);
}

/// Saves the current split layout before switching to a single standalone session.
fn suspend_current_multiplexer_state(app: &mut AppState) {
    if app
        .terminal_layout
        .layout_panes(app.last_terminal_area)
        .len()
        <= 1
    {
        return;
    }
    app.suspended_multiplexer_state = Some(capture_multiplexer_state(app));
    persist_multiplexer_state(app);
}
