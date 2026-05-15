use ratkit::CoordinatorAction;

use crate::app::state::app_state::AppState;
use crate::extensions::plans::input::focus_clicked_plan::focus_clicked_plan;
use crate::extensions::plans::input::handle_plan_drag_mouse::handle_plan_drag_mouse;
use crate::extensions::terminal::input::scroll_delta_for_mouse_kind::{
    scroll_delta_for_mouse_kind, TerminalScrollAction,
};

/// Handles mouse input routed to the plan list in the left pane.
pub fn handle_plan_left_mouse(app: &mut AppState, mouse: ratkit::MouseEvent) -> CoordinatorAction {
    match scroll_delta_for_mouse_kind(mouse.kind, 1) {
        TerminalScrollAction::Up(rows) => {
            app.plan_list.scroll_by(-(rows as isize));
            return CoordinatorAction::Continue;
        }
        TerminalScrollAction::Down(rows) => {
            app.plan_list.scroll_by(rows as isize);
            return CoordinatorAction::Continue;
        }
        TerminalScrollAction::None => {}
    }
    focus_clicked_plan(app, mouse);
    if handle_plan_drag_mouse(&mut app.plan_list, mouse) {
        return CoordinatorAction::Redraw;
    }
    CoordinatorAction::Redraw
}
