use crossterm::event::{KeyCode, KeyModifiers};
use ratkit::KeyboardEvent;

/// Returns the chat-cycle direction for Tab and Ctrl+Tab shortcuts.
pub fn chat_cycle_direction_for_keyboard(keyboard: &KeyboardEvent) -> Option<isize> {
    match keyboard.key_code {
        KeyCode::Tab if keyboard.modifiers.contains(KeyModifiers::CONTROL) => Some(-1),
        KeyCode::Tab if keyboard.modifiers.is_empty() => Some(1),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use crossterm::event::{KeyCode, KeyEventKind, KeyModifiers};
    use ratkit::KeyboardEvent;

    use super::chat_cycle_direction_for_keyboard;

    /// Builds a keyboard event fixture for shortcut direction tests.
    fn key(key_code: KeyCode, modifiers: KeyModifiers) -> KeyboardEvent {
        KeyboardEvent {
            key_code,
            modifiers,
            kind: KeyEventKind::Press,
        }
    }

    /// Verifies plain Tab moves to the next chat.
    #[test]
    fn maps_tab_to_next_chat() {
        assert_eq!(
            chat_cycle_direction_for_keyboard(&key(KeyCode::Tab, KeyModifiers::empty())),
            Some(1)
        );
    }

    /// Verifies Ctrl+Tab moves to the previous chat.
    #[test]
    fn maps_control_tab_to_previous_chat() {
        assert_eq!(
            chat_cycle_direction_for_keyboard(&key(KeyCode::Tab, KeyModifiers::CONTROL)),
            Some(-1)
        );
    }

    /// Verifies unrelated shortcuts are ignored.
    #[test]
    fn ignores_non_tab_shortcuts() {
        assert_eq!(
            chat_cycle_direction_for_keyboard(&key(KeyCode::Char('x'), KeyModifiers::CONTROL)),
            None
        );
    }
}
