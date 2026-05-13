use crate::extensions::file_viewer::tree::view::FileSystemTreeView;
use crate::ui::left_panel::outcome::LeftPaneActionOutcome;

/// Updates the visible selected path text and code preview from the tree state.
pub fn update_file_system_tree_selection(view: &mut FileSystemTreeView) {
    let _ = view.refresh_selection(LeftPaneActionOutcome::Handled);
}
