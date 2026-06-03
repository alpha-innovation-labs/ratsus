use crossterm::event::{KeyCode, KeyModifiers};
use ratkit::KeyboardEvent;

/// Returns a zero-based visible left-pane row index for Cmd+1 through Cmd+9.
pub fn visible_row_index_for_keyboard(keyboard: &KeyboardEvent) -> Option<usize> {
    if !keyboard
        .modifiers
        .intersects(KeyModifiers::SUPER | KeyModifiers::META)
    {
        return None;
    }
    match keyboard.key_code {
        KeyCode::Char(digit @ '1'..='9') => digit.to_digit(10).map(|value| value as usize - 1),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use crossterm::event::{KeyCode, KeyEventKind, KeyModifiers};
    use ratkit::KeyboardEvent;

    use super::visible_row_index_for_keyboard;

    /// Verifies Command number shortcuts map to zero-based visible row indexes.
    #[test]
    fn maps_command_numbers_to_visible_row_indexes() {
        assert_eq!(
            visible_row_index_for_keyboard(&key(KeyCode::Char('1'), KeyModifiers::SUPER)),
            Some(0)
        );
        assert_eq!(
            visible_row_index_for_keyboard(&key(KeyCode::Char('9'), KeyModifiers::META)),
            Some(8)
        );
    }

    /// Verifies Ctrl numbers remain reserved for workspace selection.
    #[test]
    fn ignores_control_numbers() {
        assert_eq!(
            visible_row_index_for_keyboard(&key(KeyCode::Char('1'), KeyModifiers::CONTROL)),
            None
        );
    }

    /// Builds a keyboard event fixture for row-index shortcut tests.
    fn key(key_code: KeyCode, modifiers: KeyModifiers) -> KeyboardEvent {
        KeyboardEvent {
            key_code,
            modifiers,
            kind: KeyEventKind::Press,
        }
    }
}
