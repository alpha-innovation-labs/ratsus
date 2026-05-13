use crate::extensions::file_viewer::tree::view::FileSystemTreeView;

impl FileSystemTreeView {
    /// Returns the tree path rendered at a screen position.
    pub(crate) fn path_at_position(&self, column: u16, row: u16) -> Option<Vec<usize>> {
        if column < self.last_tree_area.x
            || column
                >= self
                    .last_tree_area
                    .x
                    .saturating_add(self.last_tree_area.width)
            || row < self.last_tree_area.y
            || row
                >= self
                    .last_tree_area
                    .y
                    .saturating_add(self.last_tree_area.height)
        {
            return None;
        }
        let visible_row = usize::from(row.saturating_sub(self.last_tree_area.y));
        self.tree
            .get_visible_paths(&self.state)
            .into_iter()
            .skip(self.state.offset())
            .nth(visible_row)
    }
}
