use ratatui::layout::Rect;
use ratatui::Frame;
use ratkit::primitives::menu_bar::MenuBar;

use crate::ui::left_panel::mode::left_pane_mode::LeftPaneMode;
use crate::ui::menu_bar::render::style_border::style_app_menu_bar_border;
use crate::ui::menu_bar::state::active_tab_menu_index::active_left_pane_mode_menu_index;
use crate::ui::menu_bar::state::sync_selection::sync_app_menu_bar_selection;

/// Renders the top menu bar and keeps its selected item in sync with app state.
pub fn render_app_menu_bar(
    menu_bar: &mut MenuBar,
    selected_mode: LeftPaneMode,
    frame: &mut Frame,
    area: Rect,
) {
    sync_app_menu_bar_selection(menu_bar, active_left_pane_mode_menu_index(selected_mode));
    menu_bar.render(frame, area);
    if let Some(menu_area) = menu_bar.area {
        style_app_menu_bar_border(frame.buffer_mut(), menu_area);
    }
}
