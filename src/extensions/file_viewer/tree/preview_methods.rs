use crossterm::event::{KeyEvent as CrosstermKeyEvent, MouseEvent as CrosstermMouseEvent};
use ratatui::{layout::Rect, Frame};
use ratkit::widgets::code_widget::CodeWidget;
use ratkit::widgets::markdown_preview::MarkdownEvent;

use crate::extensions::file_viewer::preview::file_preview_state::FilePreviewState;
use crate::extensions::file_viewer::tree::view::FileSystemTreeView;

impl FileSystemTreeView {
    /// Renders the selected file preview without exposing the preview widget.
    pub(crate) fn render_preview(&mut self, frame: &mut Frame, area: Rect) {
        match &mut self.preview_state {
            FilePreviewState::Code(state) => {
                let state = state.as_mut();
                let widget = CodeWidget::from_state(state)
                    .show_line_numbers(true)
                    .show_outline(true);
                frame.render_stateful_widget(widget, area, state);
            }
            FilePreviewState::Markdown(widget) => {
                frame.render_widget(widget.as_mut(), area);
            }
        }
    }

    /// Handles one preview key event without exposing the preview widget.
    pub(crate) fn handle_preview_key(&mut self, event: CrosstermKeyEvent) -> bool {
        match &mut self.preview_state {
            FilePreviewState::Code(state) => {
                let state = state.as_mut();
                !matches!(
                    CodeWidget::from_state(state).handle_key(event, state),
                    ratkit::widgets::code_widget::CodeEvent::None
                )
            }
            FilePreviewState::Markdown(widget) => {
                !matches!(widget.handle_key(event), MarkdownEvent::None)
            }
        }
    }

    /// Handles one preview mouse event without exposing the preview widget.
    pub(crate) fn handle_preview_mouse(&mut self, event: CrosstermMouseEvent, area: Rect) -> bool {
        match &mut self.preview_state {
            FilePreviewState::Code(state) => {
                let state = state.as_mut();
                !matches!(
                    CodeWidget::from_state(state).handle_mouse(event, area, state),
                    ratkit::widgets::code_widget::CodeEvent::None
                )
            }
            FilePreviewState::Markdown(widget) => {
                !matches!(widget.handle_mouse(event, area), MarkdownEvent::None)
            }
        }
    }
}
