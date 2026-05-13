use ratkit::{CoordinatorAction, KeyboardEvent};

use crate::app::app_state::AppState;
use crate::conversation_picker::conversation_picker_key_behavior::ConversationPickerKeyBehavior;
use crate::keyboard::list_key_behavior::ListKeyBehavior;
use crate::keyboard::list_key_outcome::ListKeyOutcome;

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
