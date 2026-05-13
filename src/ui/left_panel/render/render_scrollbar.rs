use ratatui::Frame;
use ratkit::widgets::markdown_preview::CustomScrollbar;

use crate::app::state::app_state::AppState;
use crate::ui::left_panel::scroll::scroll_state::left_panel_scroll_state;
use crate::ui::left_panel::scroll::scrollbar_area::left_panel_scrollbar_area;
use crate::ui::left_panel::scroll::scrollbar_config::left_panel_scrollbar_config;

/// Renders Ratkit's markdown scrollbar extension over the left session list.
pub fn render_left_panel_scrollbar(app: &AppState, frame: &mut Frame) {
    let total_rows = app.visible_row_count();
    let Some(area) = left_panel_scrollbar_area(app.last_session_list_area, total_rows) else {
        return;
    };
    let scroll = left_panel_scroll_state(
        app.session_scroll,
        usize::from(app.last_session_list_area.height),
        total_rows,
    );
    let scrollbar = CustomScrollbar::new(&scroll)
        .config(left_panel_scrollbar_config())
        .show_percentage(false);
    frame.render_widget(scrollbar, area);
}
