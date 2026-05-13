use ratatui::layout::Rect;
use ratatui::Frame;

use crate::app::state::app_state::AppState;
use crate::extensions::expo::render::hotkey_footer::expo_hotkey_footer;
use crate::extensions::expo::render::render_filter_bar::render_expo_filter_bar;

/// Renders Expo footer hotkeys or active filter input.
pub fn render_expo_footer(app: &AppState, frame: &mut Frame, area: Rect) {
    if area.height == 0 {
        return;
    }
    if app.expo_filtering {
        render_expo_filter_bar(frame, area, &app.expo_filter_query);
    } else {
        frame.render_widget(expo_hotkey_footer(), area);
    }
}
