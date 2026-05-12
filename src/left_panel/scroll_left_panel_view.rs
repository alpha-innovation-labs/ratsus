use crate::app::nexus_demo_state::NexusDemo;

/// Scrolls the left-panel viewport without changing the focused or active session.
pub fn scroll_left_panel_view(app: &mut NexusDemo, delta: isize) {
    let row_count = app.visible_rows().len();
    let visible_height = usize::from(app.last_session_list_area.height).max(1);
    let max_scroll = row_count.saturating_sub(visible_height);
    app.session_scroll = app
        .session_scroll
        .saturating_add_signed(delta)
        .min(max_scroll);
}
