use crossterm::event::{KeyCode, KeyModifiers};
use ratkit::{CoordinatorAction, KeyboardEvent};

use crate::app::state::app_state::AppState;
use crate::extensions::terminal::copy_mode::clipboard::copy_text_to_clipboard::copy_text_to_clipboard;
use crate::extensions::terminal::copy_mode::selection::clear_selection::clear_terminal_copy_selection;
use crate::extensions::terminal::copy_mode::selection::finish_selection::finish_terminal_copy_selection;
use crate::ui::notifications::toast::show_copied_to_clipboard::show_copied_to_clipboard_toast;

/// Handles keyboard commands while terminal copy selection owns input.
pub fn handle_terminal_copy_keyboard(
    app: &mut AppState,
    keyboard: &KeyboardEvent,
) -> Option<CoordinatorAction> {
    let entry = app.active_session_terminal_mut()?;
    entry.copy_selection.snapshot.as_ref()?;

    if keyboard.key_code == KeyCode::Esc {
        clear_terminal_copy_selection(&mut entry.copy_selection);
        return Some(CoordinatorAction::Redraw);
    }

    if keyboard.key_code == KeyCode::Char('c') && keyboard.modifiers == KeyModifiers::empty() {
        let copied_text = finish_terminal_copy_selection(&mut entry.copy_selection);
        if let Some(text) = copied_text {
            if !text.is_empty() {
                copy_text_to_clipboard(&text);
                show_copied_to_clipboard_toast(&mut app.toast_manager);
            }
        }
        return Some(CoordinatorAction::Redraw);
    }

    Some(CoordinatorAction::Continue)
}
