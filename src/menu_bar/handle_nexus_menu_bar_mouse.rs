use ratkit::CoordinatorAction;

use crate::app::nexus_demo_state::NexusDemo;
use crate::menu_bar::main_pane_tab_for_menu_index::main_pane_tab_for_menu_index;

/// Handles mouse events inside the Nexus top menu bar.
pub fn handle_nexus_menu_bar_mouse(
    app: &mut NexusDemo,
    mouse: ratkit::MouseEvent,
) -> Option<CoordinatorAction> {
    let area = app.menu_bar.area?;
    if !mouse.is_inside(area) {
        return None;
    }

    app.menu_bar.update_hover(mouse.column, mouse.row);
    if mouse.is_click() {
        if let Some(index) = app.menu_bar.handle_click(mouse.column, mouse.row) {
            if let Some(tab) = main_pane_tab_for_menu_index(index) {
                app.active_main_pane_tab = tab;
            }
        }
    }
    Some(CoordinatorAction::Redraw)
}
