use ratkit::CoordinatorAction;

use crate::app::state::app_state::AppState;
use crate::extensions::file_viewer::tabs::tab::MainPaneTab;
use crate::ui::layout::focus::focused_pane::FocusedPane;
use crate::ui::menu_bar::state::tab_for_menu_index::main_pane_tab_for_menu_index;

/// Handles mouse events inside the top menu bar.
pub fn handle_menu_bar_mouse(
    app: &mut AppState,
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
                if tab == MainPaneTab::Files {
                    app.left_pane_visible = true;
                    app.focused_pane = FocusedPane::Left;
                }
            }
        }
    }
    Some(CoordinatorAction::Redraw)
}
