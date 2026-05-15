use crossterm::event::{MouseButton, MouseEventKind};

use crate::app::state::app_state::AppState;
use crate::extensions::plans::input::plan_row_for_mouse::plan_row_for_mouse;

/// Focuses and activates the plan row under a left-click.
pub fn focus_clicked_plan(app: &mut AppState, mouse: ratkit::MouseEvent) {
    if !matches!(mouse.kind, MouseEventKind::Down(MouseButton::Left)) {
        return;
    }
    let Some(plan_index) = plan_row_for_mouse(&app.plan_list, mouse.row) else {
        return;
    };
    let Some(visible_row) = app
        .plan_list
        .visible_indices()
        .iter()
        .position(|index| *index == plan_index)
    else {
        return;
    };
    app.plan_list.focused_row = visible_row;
    app.plan_list.activate_focused();
}
