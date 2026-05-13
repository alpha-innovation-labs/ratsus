use crate::main_pane::file_system_tree_view::FileSystemTreeView;

/// Returns the file-system tree path rendered at a screen position.
pub fn file_system_tree_path_at_position(
    view: &FileSystemTreeView,
    column: u16,
    row: u16,
) -> Option<Vec<usize>> {
    let area = view.last_tree_area;
    if column < area.x
        || column >= area.x.saturating_add(area.width)
        || row < area.y
        || row >= area.y.saturating_add(area.height)
    {
        return None;
    }
    let visible_row = usize::from(row.saturating_sub(area.y));
    view.tree
        .get_visible_paths(&view.state)
        .into_iter()
        .skip(view.state.offset())
        .nth(visible_row)
}
