use crossterm::event::{KeyEvent as CrosstermKeyEvent, KeyEventState};
use ratkit::widgets::code_widget::CodeWidget;
use ratkit::widgets::markdown_preview::MarkdownEvent;
use ratkit::{CoordinatorAction, KeyboardEvent};

use crate::extensions::file_viewer::preview::file_preview_state::FilePreviewState;
use crate::extensions::plans::data::plan_list_state::PlanListState;

/// Handles keyboard input for the active plan preview.
pub fn handle_plan_preview_key(
    state: &mut PlanListState,
    keyboard: &KeyboardEvent,
) -> CoordinatorAction {
    let event = CrosstermKeyEvent {
        code: keyboard.key_code,
        modifiers: keyboard.modifiers,
        kind: keyboard.kind,
        state: KeyEventState::NONE,
    };
    if handle_preview_event(state, event) {
        CoordinatorAction::Redraw
    } else {
        CoordinatorAction::Continue
    }
}

/// Sends one crossterm key event to the active preview widget.
fn handle_preview_event(state: &mut PlanListState, event: CrosstermKeyEvent) -> bool {
    let Some(preview_state) = &mut state.preview_state else {
        return false;
    };
    match preview_state {
        FilePreviewState::Code(code_state) => {
            let state = code_state.as_mut();
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
