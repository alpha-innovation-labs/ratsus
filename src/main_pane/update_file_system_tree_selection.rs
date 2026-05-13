use crate::main_pane::file_preview_markdown_for_path::file_preview_markdown_for_path;
use crate::main_pane::file_system_tree_view::FileSystemTreeView;
use crate::main_pane::markdown_widget_for_content::markdown_widget_for_content;

/// Updates the visible selected path text and markdown preview from the tree state.
pub fn update_file_system_tree_selection(view: &mut FileSystemTreeView) {
    let Some(entry) = view.tree.get_selected_entry(&view.state) else {
        return;
    };
    let path = entry.path.clone();
    let is_dir = entry.is_dir;

    view.last_selection = path.display().to_string();
    view.preview = markdown_widget_for_content(file_preview_markdown_for_path(&path, is_dir));
}
