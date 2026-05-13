use crate::app::app_state::AppState;
use crate::expo::expo_content_height::expo_content_height;

/// Scrolls the Expo masonry viewport without changing active conversation.
pub fn scroll_expo_view(app: &mut AppState, delta: isize) -> bool {
    let before = app.expo_scroll;
    let viewport_height = usize::from(app.last_terminal_area.height).max(1);
    let max_scroll =
        expo_content_height(app, app.last_terminal_area).saturating_sub(viewport_height);
    app.expo_scroll = app.expo_scroll.saturating_add_signed(delta).min(max_scroll);
    app.expo_scroll != before
}
