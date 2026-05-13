use ratkit::CoordinatorAction;

use crate::extensions::file_viewer::tree::path_at_position::file_system_tree_path_at_position;
use crate::extensions::file_viewer::tree::update_selection::update_file_system_tree_selection;
use crate::extensions::file_viewer::tree::view::FileSystemTreeView;
use crate::extensions::terminal::input::scroll_delta_for_mouse_kind::{
    scroll_delta_for_mouse_kind, TerminalScrollAction,
};

/// Handles mouse selection and wheel navigation for the file-system tree.
pub fn handle_file_system_tree_mouse(
    view: &mut FileSystemTreeView,
    mouse: ratkit::MouseEvent,
) -> CoordinatorAction {
    match scroll_delta_for_mouse_kind(mouse.kind, 1) {
        TerminalScrollAction::Up(rows) => move_file_tree_selection(view, -(rows as isize)),
        TerminalScrollAction::Down(rows) => move_file_tree_selection(view, rows as isize),
        TerminalScrollAction::None => select_file_tree_row(view, mouse),
    }
}

/// Moves file tree selection by a relative row offset.
fn move_file_tree_selection(view: &mut FileSystemTreeView, direction: isize) -> CoordinatorAction {
    view.move_by(direction);
    update_file_system_tree_selection(view);
    CoordinatorAction::Redraw
}

/// Selects the clicked file tree row when the click lands on a visible row.
fn select_file_tree_row(
    view: &mut FileSystemTreeView,
    mouse: ratkit::MouseEvent,
) -> CoordinatorAction {
    if !mouse.is_click() {
        return CoordinatorAction::Continue;
    }
    let Some(path) = file_system_tree_path_at_position(view, mouse.column, mouse.row) else {
        return CoordinatorAction::Continue;
    };
    view.select_path(path);
    update_file_system_tree_selection(view);
    CoordinatorAction::Redraw
}
