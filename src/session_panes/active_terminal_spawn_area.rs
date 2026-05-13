use ratatui::layout::Rect;

use crate::app::nexus_demo_state::NexusDemo;

/// Returns the best known size for spawning a terminal in the active split pane.
pub fn active_terminal_spawn_area(app: &NexusDemo) -> Rect {
    if app.active_terminal_area.width > 0 && app.active_terminal_area.height > 0 {
        return app.active_terminal_area;
    }
    app.last_terminal_area
}
