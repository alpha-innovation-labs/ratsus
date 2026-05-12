use crossterm::event::{MouseButton, MouseEventKind};

use crate::nexus_demo_state::NexusDemo;
use crate::session_row_for_click::session_row_for_click;
use crate::visible_session_rows::visible_session_rows;

/// Handles left-pane drag-and-drop session reordering mouse input.
pub fn handle_session_drag_mouse(app: &mut NexusDemo, mouse: ratkit::MouseEvent) -> bool {
    match mouse.kind {
        MouseEventKind::Down(MouseButton::Left) => start_drag_from_mouse(app, mouse.row),
        MouseEventKind::Drag(MouseButton::Left) => move_drag_from_mouse(app, mouse.row),
        MouseEventKind::Up(MouseButton::Left) => finish_drag_from_mouse(app, mouse.row),
        _ => false,
    }
}

/// Starts a session drag from the row under the mouse.
fn start_drag_from_mouse(app: &mut NexusDemo, row: u16) -> bool {
    let Some(index) = session_index_for_mouse_row(app, row) else {
        return false;
    };
    app.start_session_drag(index);
    true
}

/// Moves the dragged session to the row under the mouse.
fn move_drag_from_mouse(app: &mut NexusDemo, row: u16) -> bool {
    if app.session_drag.is_none() {
        return false;
    }
    if let Some(index) = session_index_for_mouse_row(app, row) {
        app.move_dragged_session(index);
    }
    true
}

/// Applies the final drop target and clears the active drag state.
fn finish_drag_from_mouse(app: &mut NexusDemo, row: u16) -> bool {
    if app.session_drag.is_none() {
        return false;
    }
    if let Some(index) = session_index_for_mouse_row(app, row) {
        app.move_dragged_session(index);
    }
    app.finish_session_drag();
    true
}

/// Resolves the visible session index for a mouse row.
fn session_index_for_mouse_row(app: &NexusDemo, row: u16) -> Option<usize> {
    let rows = visible_session_rows(&app.session_terminals, &app.collapsed_folders);
    session_row_for_click(row, app.last_session_list_area, app.session_scroll, &rows)
        .and_then(|row| row.session_index())
}
