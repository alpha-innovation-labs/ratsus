use crossterm::event::{KeyEvent as CrosstermKeyEvent, MouseEvent as CrosstermMouseEvent};
use ratatui::{layout::Rect, Frame};
use ratkit::widgets::markdown_preview::MarkdownEvent;

use crate::extensions::file_viewer::tree::view::FileSystemTreeView;

impl FileSystemTreeView {
    /// Renders the selected file preview without exposing the preview widget.
    pub(crate) fn render_preview(&mut self, frame: &mut Frame, area: Rect) {
        frame.render_widget(&mut self.preview, area);
    }

    /// Handles one preview key event without exposing the preview widget.
    pub(crate) fn handle_preview_key(&mut self, event: CrosstermKeyEvent) -> MarkdownEvent {
        self.preview.handle_key(event)
    }

    /// Handles one preview mouse event without exposing the preview widget.
    pub(crate) fn handle_preview_mouse(
        &mut self,
        event: CrosstermMouseEvent,
        area: Rect,
    ) -> MarkdownEvent {
        self.preview.handle_mouse(event, area)
    }
}
