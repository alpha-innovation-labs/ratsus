use ratkit::CoordinatorAction;

use crate::app::state::app_state::AppState;
use crate::extensions::terminal::input::scroll_delta_for_mouse_kind::{
    scroll_delta_for_mouse_kind, TerminalScrollAction,
};
use crate::ui::workspace_pane::handle_workspace_drag_mouse::handle_workspace_drag_mouse;
use crate::ui::workspace_pane::scroll_workspace_view::scroll_workspace_view;

/// Handles mouse input routed to the workspace pane.
pub fn handle_workspace_mouse(app: &mut AppState, mouse: ratkit::MouseEvent) -> CoordinatorAction {
    if handle_workspace_drag_mouse(app, mouse) {
        return CoordinatorAction::Redraw;
    }
    match scroll_delta_for_mouse_kind(mouse.kind, 1) {
        TerminalScrollAction::Up(rows) => redraw_if(scroll_workspace_view(app, -(rows as isize))),
        TerminalScrollAction::Down(rows) => redraw_if(scroll_workspace_view(app, rows as isize)),
        TerminalScrollAction::None => CoordinatorAction::Continue,
    }
}

/// Converts a state-change flag to a coordinator action.
fn redraw_if(changed: bool) -> CoordinatorAction {
    if changed {
        CoordinatorAction::Redraw
    } else {
        CoordinatorAction::Continue
    }
}
