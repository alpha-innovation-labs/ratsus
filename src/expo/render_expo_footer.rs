use ratatui::layout::Rect;
use ratatui::Frame;

use crate::app::nexus_demo_state::NexusDemo;
use crate::expo::expo_hotkey_footer::expo_hotkey_footer;
use crate::expo::render_expo_filter_bar::render_expo_filter_bar;

/// Renders Expo footer hotkeys or active filter input.
pub fn render_expo_footer(app: &NexusDemo, frame: &mut Frame, area: Rect) {
    if area.height == 0 {
        return;
    }
    if app.expo_filtering {
        render_expo_filter_bar(frame, area, &app.expo_filter_query);
    } else {
        frame.render_widget(expo_hotkey_footer(), area);
    }
}
