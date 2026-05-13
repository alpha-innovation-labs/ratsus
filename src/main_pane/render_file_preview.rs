use ratatui::layout::Rect;
use ratatui::Frame;

use crate::main_pane::file_system_tree_view::FileSystemTreeView;

/// Renders the selected file-system item with Ratkit's markdown preview widget.
pub fn render_file_preview(view: &mut FileSystemTreeView, frame: &mut Frame, area: Rect) {
    frame.render_widget(&mut view.preview, area);
}
