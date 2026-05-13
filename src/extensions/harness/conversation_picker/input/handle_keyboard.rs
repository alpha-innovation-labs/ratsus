use ratkit::{CoordinatorAction, KeyboardEvent};

use crate::app::state::app_state::AppState;
use crate::extensions::harness::conversation_picker::input::key_behavior::ConversationPickerKeyBehavior;
use crate::ui::keyboard::list::behavior::ListKeyBehavior;
use crate::ui::keyboard::list::outcome::ListKeyOutcome;

/// Handles keyboard input while the conversation picker modal is open.
pub fn handle_conversation_picker_keyboard(
    app: &mut AppState,
    keyboard: KeyboardEvent,
) -> CoordinatorAction {
    let mut behavior = ConversationPickerKeyBehavior::new(app);
    coordinator_action_for_list_outcome(behavior.handle_list_keyboard(keyboard))
}

/// Converts shared list-key outcomes into coordinator actions.
fn coordinator_action_for_list_outcome(outcome: ListKeyOutcome) -> CoordinatorAction {
    match outcome {
        ListKeyOutcome::Handled => CoordinatorAction::Redraw,
        ListKeyOutcome::Continue => CoordinatorAction::Continue,
        ListKeyOutcome::Quit => CoordinatorAction::Quit,
    }
}
