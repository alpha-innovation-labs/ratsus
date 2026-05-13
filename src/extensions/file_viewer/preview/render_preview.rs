use ratatui::layout::Rect;
use ratatui::Frame;

use crate::extensions::file_viewer::tree::view::FileSystemTreeView;

/// Renders the selected file-system item with Ratkit's markdown preview widget.
pub fn render_file_preview(view: &mut FileSystemTreeView, frame: &mut Frame, area: Rect) {
    view.render_preview(frame, area);
}
