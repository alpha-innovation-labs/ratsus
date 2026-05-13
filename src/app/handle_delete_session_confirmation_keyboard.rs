use crossterm::event::KeyCode;
use ratkit::{CoordinatorAction, KeyboardEvent};

use crate::app::confirm_delete_session::confirm_delete_session;
use crate::app::nexus_demo_state::NexusDemo;
use crate::notifications::show_failed_to_delete_session_toast::show_failed_to_delete_session_toast;

/// Handles keyboard input while delete confirmation is open.
pub fn handle_delete_session_confirmation_keyboard(
    app: &mut NexusDemo,
    keyboard: KeyboardEvent,
) -> CoordinatorAction {
    if app.delete_confirmation.is_deleting {
        return CoordinatorAction::Continue;
    }
    match keyboard.key_code {
        KeyCode::Enter | KeyCode::Char('y') => {
            if let Err(error) = confirm_delete_session(app) {
                show_failed_to_delete_session_toast(&mut app.toast_manager, &error);
            }
            CoordinatorAction::Redraw
        }
        KeyCode::Esc | KeyCode::Char('n') => {
            app.delete_confirmation.close();
            CoordinatorAction::Redraw
        }
        _ => CoordinatorAction::Continue,
    }
}
