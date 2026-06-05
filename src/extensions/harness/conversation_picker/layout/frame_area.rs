use ratatui::layout::Rect;

use crate::app::state::app_state::AppState;

/// Reconstructs the last full frame area from the stored app body layout area.
pub fn history_modal_frame_area(app: &AppState) -> Rect {
    Rect::new(
        app.last_layout_area.x,
        0,
        app.last_layout_area.width,
        app.last_layout_area
            .y
            .saturating_add(app.last_layout_area.height),
    )
}
