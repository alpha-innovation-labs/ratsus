use crate::extensions::file_viewer::tree::view::FileSystemTreeView;

/// Returns the file-system tree path rendered at a screen position.
pub fn file_system_tree_path_at_position(
    view: &FileSystemTreeView,
    column: u16,
    row: u16,
) -> Option<Vec<usize>> {
    view.path_at_position(column, row)
}
