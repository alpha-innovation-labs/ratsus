use crossterm::event::{MouseButton, MouseEventKind};

use crate::app::nexus_demo_state::NexusDemo;
use crate::left_panel::session_list_row::SessionListRow;
use crate::left_panel::session_row_for_click::session_row_for_click;
use crate::left_panel::should_toggle_folder_on_drop::should_toggle_folder_on_drop;
use crate::left_panel::toggle_session_folder::toggle_session_folder;

/// Handles left-pane drag-and-drop session reordering mouse input.
pub fn handle_session_drag_mouse(app: &mut NexusDemo, mouse: ratkit::MouseEvent) -> bool {
    match mouse.kind {
        MouseEventKind::Down(MouseButton::Left) => start_drag_from_mouse(app, mouse.row),
        MouseEventKind::Drag(MouseButton::Left) => move_drag_from_mouse(app, mouse.row),
        MouseEventKind::Up(MouseButton::Left) => finish_drag_from_mouse(app, mouse.row),
        _ => false,
    }
}

/// Starts a session or folder drag from the row under the mouse.
fn start_drag_from_mouse(app: &mut NexusDemo, row: u16) -> bool {
    let Some(list_row) = left_panel_row_for_mouse(app, row) else {
        return false;
    };
    match list_row {
        SessionListRow::Folder { path, .. } => app.start_folder_drag(path),
        SessionListRow::Session { index } => app.start_session_drag(index),
        SessionListRow::FolderMore { .. } => return false,
    }
    true
}

/// Moves the dragged session or folder to the row under the mouse.
fn move_drag_from_mouse(app: &mut NexusDemo, row: u16) -> bool {
    if app.session_drag.is_none() && app.folder_drag.is_none() {
        return false;
    }
    app.mark_folder_drag_moved();
    move_drag_to_mouse_row(app, row);
    true
}

/// Applies the final drop target and clears the active drag state.
fn finish_drag_from_mouse(app: &mut NexusDemo, row: u16) -> bool {
    if app.session_drag.is_none() && app.folder_drag.is_none() {
        return false;
    }
    let folder_click = folder_click_for_drop(app, row);
    if folder_click.is_none() {
        move_drag_to_mouse_row(app, row);
    }
    app.finish_left_panel_drag();
    if let Some(path) = folder_click {
        toggle_session_folder(app, path);
    }
    true
}

/// Returns a folder path when a folder drag ended as a click.
fn folder_click_for_drop(app: &NexusDemo, row: u16) -> Option<std::path::PathBuf> {
    let source = app.folder_drag.as_ref()?;
    let SessionListRow::Folder { path, .. } = left_panel_row_for_mouse(app, row)? else {
        return None;
    };
    should_toggle_folder_on_drop(source, &path, app.folder_drag_moved).then_some(path)
}

/// Moves an active drag operation onto the row under the mouse.
fn move_drag_to_mouse_row(app: &mut NexusDemo, row: u16) {
    let Some(list_row) = left_panel_row_for_mouse(app, row) else {
        return;
    };
    match list_row {
        SessionListRow::Folder { path, .. } if app.folder_drag.is_some() => {
            app.move_dragged_folder(path);
        }
        SessionListRow::Session { index } if app.session_drag.is_some() => {
            app.move_dragged_session(index);
        }
        _ => {}
    }
}

/// Resolves the visible left-panel row for a mouse row.
fn left_panel_row_for_mouse(app: &NexusDemo, row: u16) -> Option<SessionListRow> {
    let rows = app.visible_rows();
    session_row_for_click(row, app.last_session_list_area, app.session_scroll, &rows)
}
