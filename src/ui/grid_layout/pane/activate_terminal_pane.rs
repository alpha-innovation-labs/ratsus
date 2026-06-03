use ratkit::primitives::resizable_grid::PaneId;

use crate::app::state::app_state::AppState;
use crate::extensions::file_viewer::tabs::tab::MainPaneTab;
use crate::ui::grid_layout::pane::session_index_for_pane::session_index_for_pane;
use crate::ui::grid_layout::persistence::persist_multiplexer_state::persist_multiplexer_state;
use crate::ui::layout::focus::focused_pane::FocusedPane;

/// Focuses a terminal pane and activates its assigned session.
pub fn activate_terminal_pane(app: &mut AppState, pane_id: PaneId) {
    let Some(index) = session_index_for_pane(app, pane_id) else {
        return;
    };
    app.active_terminal_pane_id = pane_id;
    if let Some(area) = app.terminal_pane_areas.get(&pane_id).copied() {
        app.active_terminal_area = area;
    }
    app.active_index = index;
    app.focused_index = index;
    app.focused_pane = FocusedPane::Terminal;
    app.active_main_pane_tab = MainPaneTab::Chat;
    app.sync_focused_row_to_session();
    app.keep_focused_row_visible();
    persist_multiplexer_state(app);
}
