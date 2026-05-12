use crate::main_pane::file_system_tree_view::FileSystemTreeView;

/// Updates the visible selected path text from the file-system tree state.
pub fn update_file_system_tree_selection(view: &mut FileSystemTreeView) {
    if let Some(entry) = view.tree.get_selected_entry(&view.state) {
        view.last_selection = entry.path.display().to_string();
    }
}
