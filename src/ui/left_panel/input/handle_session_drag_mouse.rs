use crossterm::event::{MouseButton, MouseEventKind};

use crate::app::expo::activate_expo_folder::activate_expo_folder;
use crate::app::state::app_state::AppState;
use crate::ui::left_panel::folder::click_hits_label::folder_click_hits_icon;
use crate::ui::left_panel::folder::toggle_session_folder::toggle_session_folder;
use crate::ui::left_panel::input::session_row_for_rendered_click::session_row_for_rendered_click;
use crate::ui::left_panel::input::should_toggle_folder_on_drop::should_toggle_folder_on_drop;
use crate::ui::left_panel::render::rendered_rows::rendered_left_panel_rows;
use crate::ui::left_panel::session::list_row::SessionListRow;

/// Handles left-pane drag-and-drop session reordering mouse input.
pub fn handle_session_drag_mouse(app: &mut AppState, mouse: ratkit::MouseEvent) -> bool {
    match mouse.kind {
        MouseEventKind::Down(MouseButton::Left) => start_drag_from_mouse(app, mouse.row),
        MouseEventKind::Drag(MouseButton::Left) => move_drag_from_mouse(app, mouse.row),
        MouseEventKind::Up(MouseButton::Left) => {
            finish_drag_from_mouse(app, mouse.row, mouse.column)
        }
        _ => false,
    }
}

/// Starts a session or folder drag from the row under the mouse.
fn start_drag_from_mouse(app: &mut AppState, row: u16) -> bool {
    let Some(list_row) = left_panel_row_for_mouse(app, row) else {
        return false;
    };
    match list_row {
        SessionListRow::Folder { path, .. } => app.start_folder_drag(path),
        SessionListRow::Session { index } | SessionListRow::SplitGroupChild { index, .. } => {
            app.start_session_drag(index);
        }
        SessionListRow::SplitGroup { .. } => return false,
    }
    true
}

/// Moves the dragged session or folder to the row under the mouse.
fn move_drag_from_mouse(app: &mut AppState, row: u16) -> bool {
    if app.session_drag.is_none() && app.folder_drag.is_none() {
        return false;
    }
    app.mark_folder_drag_moved();
    move_drag_to_mouse_row(app, row);
    true
}

/// Applies the final drop target and clears the active drag state.
fn finish_drag_from_mouse(app: &mut AppState, row: u16, column: u16) -> bool {
    if app.session_drag.is_none() && app.folder_drag.is_none() {
        return false;
    }
    let folder_click = folder_click_for_drop(app, row);
    if folder_click.is_none() {
        move_drag_to_mouse_row(app, row);
    }
    app.finish_left_panel_drag();
    if let Some(path) = folder_click {
        activate_expo_folder(app, path.clone());
        if folder_click_hits_icon(column, app.last_session_list_area.x) {
            toggle_session_folder(app, path);
        }
    }
    true
}

/// Returns a folder path when a folder drag ended as a click.
fn folder_click_for_drop(app: &AppState, row: u16) -> Option<std::path::PathBuf> {
    let source = app.folder_drag.as_ref()?;
    let SessionListRow::Folder { path, .. } = left_panel_row_for_mouse(app, row)? else {
        return None;
    };
    should_toggle_folder_on_drop(source, &path, app.folder_drag_moved).then_some(path)
}

/// Moves an active drag operation onto the row under the mouse.
fn move_drag_to_mouse_row(app: &mut AppState, row: u16) {
    let Some(list_row) = left_panel_row_for_mouse(app, row) else {
        return;
    };
    match list_row {
        SessionListRow::Folder { path, .. } if app.folder_drag.is_some() => {
            app.move_dragged_folder(path);
        }
        SessionListRow::Session { index } | SessionListRow::SplitGroupChild { index, .. }
            if app.session_drag.is_some() =>
        {
            app.move_dragged_session(index);
        }
        _ => {}
    }
}

/// Resolves the rendered left-panel row for a mouse row.
fn left_panel_row_for_mouse(app: &AppState, row: u16) -> Option<SessionListRow> {
    let rows = rendered_left_panel_rows(app);
    session_row_for_rendered_click(row, app.last_session_list_area, &rows).map(|(_, row)| row)
}
