use crossterm::event::{MouseButton, MouseEventKind};

use crate::extensions::plans::data::plan_list_state::PlanListState;
use crate::extensions::plans::input::plan_row_for_mouse::plan_row_for_mouse;

/// Handles plan-pane drag-and-drop reordering mouse input.
pub fn handle_plan_drag_mouse(state: &mut PlanListState, mouse: ratkit::MouseEvent) -> bool {
    match mouse.kind {
        MouseEventKind::Down(MouseButton::Left) => start_drag_from_mouse(state, mouse.row),
        MouseEventKind::Drag(MouseButton::Left) => move_drag_from_mouse(state, mouse.row),
        MouseEventKind::Up(MouseButton::Left) => finish_drag_from_mouse(state, mouse.row),
        _ => false,
    }
}

/// Starts dragging the plan under the mouse.
fn start_drag_from_mouse(state: &mut PlanListState, row: u16) -> bool {
    let Some(index) = plan_row_for_mouse(state, row) else {
        return false;
    };
    state.start_drag(index);
    true
}

/// Moves the active drag to the plan under the mouse.
fn move_drag_from_mouse(state: &mut PlanListState, row: u16) -> bool {
    if state.drag.is_none() {
        return false;
    }
    if let Some(index) = plan_row_for_mouse(state, row) {
        state.move_dragged_plan(index);
    }
    true
}

/// Finishes any active plan drag operation.
fn finish_drag_from_mouse(state: &mut PlanListState, row: u16) -> bool {
    if state.drag.is_none() {
        return false;
    }
    if let Some(index) = plan_row_for_mouse(state, row) {
        state.move_dragged_plan(index);
    }
    state.finish_drag();
    true
}
