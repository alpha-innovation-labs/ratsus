use ratatui::layout::Rect;
use ratatui::Frame;

use crate::app::state::app_state::AppState;
use crate::ui::grid_layout::render::render_pane_session::render_pane_session;
use crate::ui::grid_layout::render::render_split_chrome::{
    content_area, render_split_chrome, visible_close_button_area,
};
use crate::ui::grid_layout::split::ensure_active_session::ensure_active_terminal_pane_session;
use crate::ui::grid_layout::split::resize_session::resize_terminal_pane_session;

/// Renders every visible chat split and updates terminal sizes to match pane areas.
pub fn render_chat_sessions(app: &mut AppState, frame: &mut Frame, area: Rect) {
    ensure_active_terminal_pane_session(app);
    let panes = app.terminal_layout.layout_panes(area);
    app.terminal_pane_close_buttons.clear();
    let use_inner_borders = panes.len() > 1;
    for pane in panes {
        let pane_id = pane.pane_id();
        let pane_area = pane.area();
        let content_area = content_area_for_pane(pane_area, use_inner_borders);
        resize_terminal_pane_session(app, pane_id, content_area);
        if use_inner_borders {
            register_close_button(app, pane_id, pane_area);
            render_split_chrome(app, frame, pane_id, pane_area);
        }
        render_pane_session(app, frame, pane_id, content_area);
    }
}

/// Returns the content area for a pane according to chrome visibility.
fn content_area_for_pane(area: Rect, use_inner_borders: bool) -> Rect {
    if use_inner_borders {
        content_area(area)
    } else {
        area
    }
}

/// Stores the close-button hit target for one split pane.
fn register_close_button(app: &mut AppState, pane_id: u32, pane_area: Rect) {
    if let Some(button_area) = visible_close_button_area(pane_area) {
        app.terminal_pane_close_buttons.insert(pane_id, button_area);
    }
}
