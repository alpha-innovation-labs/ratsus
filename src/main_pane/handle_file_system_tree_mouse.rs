use ratkit::CoordinatorAction;

use crate::main_pane::file_system_tree_path_at_position::file_system_tree_path_at_position;
use crate::main_pane::file_system_tree_view::FileSystemTreeView;
use crate::main_pane::update_file_system_tree_selection::update_file_system_tree_selection;
use crate::terminal::scroll_delta_for_mouse_kind::{
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
    for _ in 0..direction.unsigned_abs() {
        if direction.is_negative() {
            view.tree.select_previous(&mut view.state);
        } else {
            view.tree.select_next(&mut view.state);
        }
    }
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
    view.state.select(path);
    update_file_system_tree_selection(view);
    CoordinatorAction::Redraw
}
