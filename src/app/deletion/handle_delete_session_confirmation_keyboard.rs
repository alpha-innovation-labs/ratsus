use crossterm::event::KeyCode;
use ratkit::{CoordinatorAction, KeyboardEvent};

use crate::app::deletion::confirm_delete_session::confirm_delete_session;
use crate::app::state::app_state::AppState;
use crate::ui::notifications::toast::show_failed_to_delete_session::show_failed_to_delete_session_toast;

/// Handles keyboard input while delete confirmation is open.
pub fn handle_delete_session_confirmation_keyboard(
    app: &mut AppState,
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
