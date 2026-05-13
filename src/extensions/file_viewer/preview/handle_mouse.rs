use crossterm::event::MouseEvent as CrosstermMouseEvent;
use ratatui::layout::Rect;
use ratkit::widgets::markdown_preview::MarkdownEvent;
use ratkit::CoordinatorAction;

use crate::extensions::file_viewer::tree::view::FileSystemTreeView;

/// Handles mouse input for the selected file markdown preview.
pub fn handle_file_preview_mouse(
    view: &mut FileSystemTreeView,
    mouse: ratkit::MouseEvent,
    area: Rect,
) -> CoordinatorAction {
    let event = CrosstermMouseEvent {
        kind: mouse.kind,
        column: mouse.column,
        row: mouse.row,
        modifiers: mouse.modifiers,
    };
    if matches!(view.handle_preview_mouse(event, area), MarkdownEvent::None) {
        CoordinatorAction::Continue
    } else {
        CoordinatorAction::Redraw
    }
}
