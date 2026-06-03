use ratkit::{CoordinatorAction, KeyboardEvent};

use crate::app::state::app_state::AppState;
use crate::ui::grid_layout::persistence::persist_multiplexer_state::{
    persist_multiplexer_state, persist_multiplexer_state_now,
};
use crate::ui::left_panel::active_content::ActiveLeftPaneContent;
use crate::ui::left_panel::input::dispatch_left_pane_keyboard::dispatch_left_pane_keyboard;
use crate::ui::left_panel::outcome::LeftPaneActionOutcome;

/// Handles keyboard input while the left pane is focused through active content.
pub fn handle_left_keyboard(
    app: &mut AppState,
    keyboard: KeyboardEvent,
) -> ratkit::LayoutResult<CoordinatorAction> {
    let outcome = {
        let mut content = ActiveLeftPaneContent::for_app(app);
        dispatch_left_pane_keyboard(&mut content, keyboard)
    };
    match outcome {
        LeftPaneActionOutcome::Handled => persist_multiplexer_state(app),
        LeftPaneActionOutcome::Quit => persist_multiplexer_state_now(app),
        LeftPaneActionOutcome::Continue => {}
    }
    Ok(coordinator_action_for_left_pane_outcome(outcome))
}

/// Converts left-pane action outcomes into coordinator actions.
fn coordinator_action_for_left_pane_outcome(outcome: LeftPaneActionOutcome) -> CoordinatorAction {
    match outcome {
        LeftPaneActionOutcome::Handled => CoordinatorAction::Redraw,
        LeftPaneActionOutcome::Continue => CoordinatorAction::Continue,
        LeftPaneActionOutcome::Quit => CoordinatorAction::Quit,
    }
}
