use ratatui::layout::Rect;

use crate::app::nexus_demo_state::NexusDemo;

/// Reconstructs the last full frame area from the stored app body layout area.
pub fn conversation_picker_frame_area(app: &NexusDemo) -> Rect {
    Rect::new(
        app.last_layout_area.x,
        0,
        app.last_layout_area.width,
        app.last_layout_area
            .y
            .saturating_add(app.last_layout_area.height),
    )
}
