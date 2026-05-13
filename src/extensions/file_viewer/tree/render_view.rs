use ratatui::{layout::Rect, Frame};

use crate::extensions::file_viewer::tree::view::FileSystemTreeView;

/// Renders only the file-system tree body inside the shared left-pane shell.
pub fn render_file_system_tree_view(view: &mut FileSystemTreeView, frame: &mut Frame, area: Rect) {
    view.render_tree_body(frame, area);
}
