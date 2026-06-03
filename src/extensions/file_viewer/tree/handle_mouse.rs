use ratkit::CoordinatorAction;

use crate::extensions::file_viewer::tree::view::FileSystemTreeView;
use crate::extensions::terminal::input::scroll_delta_for_mouse_kind::{
    scroll_delta_for_mouse_kind, TerminalScrollAction,
};

/// Handles mouse selection and wheel navigation for the grouped file-system tree.
pub fn handle_file_system_tree_mouse(
    view: &mut FileSystemTreeView,
    mouse: ratkit::MouseEvent,
) -> CoordinatorAction {
    match scroll_delta_for_mouse_kind(mouse.kind, 1) {
        TerminalScrollAction::Up(rows) => {
            view.scroll_workspace_by(-(rows as isize));
            CoordinatorAction::Continue
        }
        TerminalScrollAction::Down(rows) => {
            view.scroll_workspace_by(rows as isize);
            CoordinatorAction::Continue
        }
        TerminalScrollAction::None => select_file_tree_row(view, mouse),
    }
}

/// Selects the clicked grouped file-tree row when the click lands on a visible row.
fn select_file_tree_row(
    view: &mut FileSystemTreeView,
    mouse: ratkit::MouseEvent,
) -> CoordinatorAction {
    if !mouse.is_click() {
        return CoordinatorAction::Continue;
    }
    let Some(row) = view.workspace_row_at_position(mouse.row) else {
        return CoordinatorAction::Continue;
    };
    view.select_workspace_row(row);
    CoordinatorAction::Redraw
}
