use ratkit::CoordinatorAction;

use crate::app::state::app_state::AppState;
use crate::extensions::file_viewer::tabs::tab::MainPaneTab;
use crate::extensions::file_viewer::tabs::tab_at_position::main_pane_tab_at_position;
use crate::ui::grid_layout::persistence::persist_multiplexer_state::persist_multiplexer_state;
use crate::ui::layout::focus::focused_pane::FocusedPane;

/// Handles clicks on the main pane tabs and returns the consumed action.
pub fn handle_main_pane_tab_click(
    app: &mut AppState,
    mouse: ratkit::MouseEvent,
) -> Option<CoordinatorAction> {
    if !mouse.is_click() {
        return None;
    }

    match main_pane_tab_at_position(app.last_main_pane_area, mouse.column, mouse.row)? {
        MainPaneTab::Chat => {
            app.active_main_pane_tab = MainPaneTab::Chat;
            persist_multiplexer_state(app);
            Some(CoordinatorAction::Redraw)
        }
        MainPaneTab::Files => {
            app.active_main_pane_tab = MainPaneTab::Files;
            app.left_pane_visible = true;
            app.focused_pane = FocusedPane::Left;
            persist_multiplexer_state(app);
            Some(CoordinatorAction::Redraw)
        }
        MainPaneTab::Diff => {
            app.active_main_pane_tab = MainPaneTab::Diff;
            persist_multiplexer_state(app);
            Some(CoordinatorAction::Redraw)
        }
        MainPaneTab::Expo => {
            app.active_main_pane_tab = MainPaneTab::Expo;
            persist_multiplexer_state(app);
            Some(CoordinatorAction::Redraw)
        }
    }
}
