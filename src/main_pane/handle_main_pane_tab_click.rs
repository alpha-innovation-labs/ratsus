use ratkit::CoordinatorAction;

use crate::app::nexus_demo_state::NexusDemo;
use crate::main_pane::main_pane_tab::MainPaneTab;
use crate::main_pane::main_pane_tab_at_position::main_pane_tab_at_position;

/// Handles clicks on the main pane tabs and returns the consumed action.
pub fn handle_main_pane_tab_click(
    app: &mut NexusDemo,
    mouse: ratkit::MouseEvent,
) -> Option<CoordinatorAction> {
    if !mouse.is_click() {
        return None;
    }

    match main_pane_tab_at_position(app.last_main_pane_area, mouse.column, mouse.row)? {
        MainPaneTab::Chat => {
            app.active_main_pane_tab = MainPaneTab::Chat;
            Some(CoordinatorAction::Redraw)
        }
        MainPaneTab::Files => {
            app.active_main_pane_tab = MainPaneTab::Files;
            Some(CoordinatorAction::Redraw)
        }
        MainPaneTab::Diff => {
            app.active_main_pane_tab = MainPaneTab::Diff;
            Some(CoordinatorAction::Redraw)
        }
        MainPaneTab::Expo => {
            app.active_main_pane_tab = MainPaneTab::Expo;
            Some(CoordinatorAction::Redraw)
        }
    }
}
