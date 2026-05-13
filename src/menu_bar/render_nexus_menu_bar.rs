use ratatui::layout::Rect;
use ratatui::Frame;
use ratkit::primitives::menu_bar::MenuBar;

use crate::main_pane::main_pane_tab::MainPaneTab;
use crate::menu_bar::active_main_pane_tab_menu_index::active_main_pane_tab_menu_index;
use crate::menu_bar::style_nexus_menu_bar_border::style_nexus_menu_bar_border;
use crate::menu_bar::sync_nexus_menu_bar_selection::sync_nexus_menu_bar_selection;

/// Renders the Nexus top menu bar and keeps its selected item in sync with app state.
pub fn render_nexus_menu_bar(
    menu_bar: &mut MenuBar,
    selected_tab: MainPaneTab,
    frame: &mut Frame,
    area: Rect,
) {
    sync_nexus_menu_bar_selection(menu_bar, active_main_pane_tab_menu_index(selected_tab));
    menu_bar.render(frame, area);
    if let Some(menu_area) = menu_bar.area {
        style_nexus_menu_bar_border(frame.buffer_mut(), menu_area);
    }
}
