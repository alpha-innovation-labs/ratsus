use crossterm::event::{KeyCode, KeyModifiers};
use ratkit::{CoordinatorAction, KeyboardEvent};

use crate::app::state::app_state::AppState;
use crate::extensions::harness::conversation_picker::actions::toggle_scope::toggle_conversation_picker_scope;
use crate::extensions::harness::conversation_picker::input::key_behavior::ConversationPickerKeyBehavior;
use crate::ui::keyboard::list::behavior::ListKeyBehavior;
use crate::ui::keyboard::list::outcome::ListKeyOutcome;

/// Handles keyboard input while the conversation picker modal is open.
pub fn handle_conversation_picker_keyboard(
    app: &mut AppState,
    keyboard: KeyboardEvent,
) -> CoordinatorAction {
    if should_toggle_picker_scope(&keyboard) {
        return redraw_if(toggle_conversation_picker_scope(app));
    }
    let mut behavior = ConversationPickerKeyBehavior::new(app);
    coordinator_action_for_list_outcome(behavior.handle_list_keyboard(keyboard))
}

/// Returns true when Tab should toggle between workspace and all picker scopes.
fn should_toggle_picker_scope(keyboard: &KeyboardEvent) -> bool {
    keyboard.key_code == KeyCode::Tab && keyboard.modifiers == KeyModifiers::empty()
}

/// Converts a state-change flag into a coordinator action.
fn redraw_if(changed: bool) -> CoordinatorAction {
    if changed {
        CoordinatorAction::Redraw
    } else {
        CoordinatorAction::Continue
    }
}

/// Converts shared list-key outcomes into coordinator actions.
fn coordinator_action_for_list_outcome(outcome: ListKeyOutcome) -> CoordinatorAction {
    match outcome {
        ListKeyOutcome::Handled => CoordinatorAction::Redraw,
        ListKeyOutcome::Continue => CoordinatorAction::Continue,
        ListKeyOutcome::Quit => CoordinatorAction::Quit,
    }
}
