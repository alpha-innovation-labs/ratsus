use crossterm::event::MouseEvent as CrosstermMouseEvent;
use ratatui::layout::Rect;
use ratkit::widgets::code_widget::CodeWidget;
use ratkit::widgets::markdown_preview::MarkdownEvent;
use ratkit::CoordinatorAction;

use crate::extensions::file_viewer::preview::file_preview_state::FilePreviewState;
use crate::extensions::plans::data::plan_list_state::PlanListState;

/// Handles mouse input for the active plan preview.
pub fn handle_plan_preview_mouse(
    state: &mut PlanListState,
    mouse: ratkit::MouseEvent,
    area: Rect,
) -> CoordinatorAction {
    let event = CrosstermMouseEvent {
        kind: mouse.kind,
        column: mouse.column,
        row: mouse.row,
        modifiers: mouse.modifiers,
    };
    if handle_preview_event(state, event, area) {
        CoordinatorAction::Redraw
    } else {
        CoordinatorAction::Continue
    }
}

/// Sends one crossterm mouse event to the active preview widget.
fn handle_preview_event(state: &mut PlanListState, event: CrosstermMouseEvent, area: Rect) -> bool {
    let Some(preview_state) = &mut state.preview_state else {
        return false;
    };
    match preview_state {
        FilePreviewState::Code(code_state) => {
            let state = code_state.as_mut();
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
