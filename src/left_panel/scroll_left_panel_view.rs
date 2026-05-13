use crate::app::app_state::AppState;

/// Scrolls the left-panel viewport without changing the focused or active session.
pub fn scroll_left_panel_view(app: &mut AppState, delta: isize) -> bool {
    let before = app.session_scroll;
    let row_count = app.visible_row_count();
    let visible_height = usize::from(app.last_session_list_area.height).max(1);
    let max_scroll = row_count.saturating_sub(visible_height);
    app.session_scroll = app
        .session_scroll
        .saturating_add_signed(delta)
        .min(max_scroll);
    let changed = app.session_scroll != before;
    if changed {
        app.suppress_left_focus_scroll = true;
    }
    changed
}

#[cfg(test)]
mod tests {
    use super::scroll_left_panel_view;

    /// This module is covered by integration-style scroll tests in the left_panel module.
    #[test]
    fn scroll_function_is_linked() {
        let _ = scroll_left_panel_view;
    }
}
