use crate::app::state::app_state::AppState;
use crate::ui::left_panel::mode::left_pane_mode::LeftPaneMode;

/// Returns the left-pane mode title toggle clicked by the mouse event.
pub fn clicked_left_pane_mode(app: &AppState, mouse: ratkit::MouseEvent) -> Option<LeftPaneMode> {
    if !mouse.is_click() {
        return None;
    }
    if mouse.is_inside(app.last_left_session_toggle_area) {
        return Some(LeftPaneMode::Sessions);
    }
    if mouse.is_inside(app.last_left_plan_toggle_area) {
        return Some(LeftPaneMode::Plans);
    }
    if mouse.is_inside(app.last_left_file_toggle_area) {
        return Some(LeftPaneMode::Files);
    }
    None
}
