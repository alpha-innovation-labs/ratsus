use crossterm::event::{MouseButton, MouseEventKind};

use crate::app::state::app_state::AppState;
use crate::ui::workspace_pane::select_workspace::select_workspace;
use crate::ui::workspace_pane::workspace_path_for_mouse_row::workspace_path_for_mouse_row;

/// Handles workspace-pane drag-and-drop folder reordering mouse input.
pub fn handle_workspace_drag_mouse(app: &mut AppState, mouse: ratkit::MouseEvent) -> bool {
    match mouse.kind {
        MouseEventKind::Down(MouseButton::Left) => start_drag_from_mouse(app, mouse.row),
        MouseEventKind::Drag(MouseButton::Left) => move_drag_from_mouse(app, mouse.row),
        MouseEventKind::Up(MouseButton::Left) => finish_drag_from_mouse(app, mouse.row),
        _ => false,
    }
}

/// Starts a workspace drag from the row under the mouse.
fn start_drag_from_mouse(app: &mut AppState, row: u16) -> bool {
    let Some(path) = workspace_path_for_mouse_row(app, row, app.last_workspace_list_area) else {
        return false;
    };
    app.start_workspace_drag(path);
    true
}

/// Moves the dragged workspace to the row under the mouse.
fn move_drag_from_mouse(app: &mut AppState, row: u16) -> bool {
    if app.workspace_drag.is_none() {
        return false;
    }
    app.mark_workspace_drag_moved();
    move_drag_to_mouse_row(app, row);
    true
}

/// Applies the final workspace drop target and clears drag state.
fn finish_drag_from_mouse(app: &mut AppState, row: u16) -> bool {
    let Some(source) = app.workspace_drag.clone() else {
        return false;
    };
    if let Some(target) = workspace_path_for_mouse_row(app, row, app.last_workspace_list_area) {
        if app.workspace_drag_moved {
            app.move_dragged_workspace(target);
        } else {
            let _ = select_workspace(app, target);
        }
    } else if !app.workspace_drag_moved {
        let _ = select_workspace(app, source);
    }
    app.finish_workspace_drag();
    true
}

/// Moves an active workspace drag operation onto the row under the mouse.
fn move_drag_to_mouse_row(app: &mut AppState, row: u16) {
    let Some(path) = workspace_path_for_mouse_row(app, row, app.last_workspace_list_area) else {
        return;
    };
    app.move_dragged_workspace(path);
}
