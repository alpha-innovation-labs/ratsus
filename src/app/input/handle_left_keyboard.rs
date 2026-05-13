use ratkit::{CoordinatorAction, KeyboardEvent};

use crate::app::state::app_state::AppState;
use crate::ui::left_panel::active_content::ActiveLeftPaneContent;
use crate::ui::left_panel::input::dispatch_left_pane_keyboard::dispatch_left_pane_keyboard;
use crate::ui::left_panel::outcome::LeftPaneActionOutcome;

/// Handles keyboard input while the left pane is focused through active content.
pub fn handle_left_keyboard(
    app: &mut AppState,
    keyboard: KeyboardEvent,
) -> ratkit::LayoutResult<CoordinatorAction> {
    let mut content = ActiveLeftPaneContent::for_app(app);
    Ok(coordinator_action_for_left_pane_outcome(
        dispatch_left_pane_keyboard(&mut content, keyboard),
    ))
}

/// Converts left-pane action outcomes into coordinator actions.
fn coordinator_action_for_left_pane_outcome(outcome: LeftPaneActionOutcome) -> CoordinatorAction {
    match outcome {
        LeftPaneActionOutcome::Handled => CoordinatorAction::Redraw,
        LeftPaneActionOutcome::Continue => CoordinatorAction::Continue,
        LeftPaneActionOutcome::Quit => CoordinatorAction::Quit,
    }
}
