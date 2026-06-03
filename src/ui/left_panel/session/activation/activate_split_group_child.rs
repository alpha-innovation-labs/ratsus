use ratkit::primitives::resizable_grid::PaneId;

use crate::app::state::app_state::AppState;
use crate::extensions::file_viewer::tabs::tab::MainPaneTab;
use crate::ui::grid_layout::bundle::set_active_bundle_session::set_active_terminal_pane_bundle_session;
use crate::ui::grid_layout::pane::pane_exists_in_terminal_layout::pane_exists_in_terminal_layout;
use crate::ui::grid_layout::persistence::persist_multiplexer_state::persist_multiplexer_state;
use crate::ui::grid_layout::persistence::restore_available_multiplexer_state_into_app::restore_available_multiplexer_state_into_app;
use crate::ui::layout::focus::focused_pane::FocusedPane;

/// Activates the pane and session represented by a split-group child row.
pub fn activate_split_group_child(app: &mut AppState, pane_id: PaneId, index: usize) {
    let Some(session_id) = app
        .session_terminals
        .get(index)
        .map(|entry| entry.session.id.clone())
    else {
        return;
    };
    if !pane_exists_in_terminal_layout(app, pane_id) {
        restore_available_multiplexer_state_into_app(app);
    }
    app.active_terminal_pane_id = pane_id;
    app.active_index = index;
    app.focused_index = index;
    app.completed_unseen_session_ids.remove(&session_id);
    app.focused_pane = FocusedPane::Terminal;
    app.active_main_pane_tab = MainPaneTab::Chat;
    set_active_terminal_pane_bundle_session(app, pane_id, session_id);
    sync_active_pane_area(app, pane_id);
    app.keep_focused_row_visible();
    persist_multiplexer_state(app);
}

/// Updates active pane geometry from the most recent render pass.
fn sync_active_pane_area(app: &mut AppState, pane_id: PaneId) {
    if let Some(area) = app.terminal_pane_areas.get(&pane_id).copied() {
        app.active_terminal_area = area;
    }
}
