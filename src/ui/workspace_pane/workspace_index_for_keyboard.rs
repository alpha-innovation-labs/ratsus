use crossterm::event::{KeyCode, KeyModifiers};
use ratkit::KeyboardEvent;

/// Returns a zero-based workspace index for Ctrl+1 through Ctrl+9.
pub fn workspace_index_for_keyboard(keyboard: &KeyboardEvent) -> Option<usize> {
    if !keyboard.modifiers.contains(KeyModifiers::CONTROL) {
        return None;
    }
    let KeyCode::Char(character) = keyboard.key_code else {
        return None;
    };
    let digit = character.to_digit(10)?;
    if !(1..=9).contains(&digit) {
        return None;
    }
    Some(digit as usize - 1)
}

#[cfg(test)]
mod tests {
    use crossterm::event::{KeyCode, KeyEventKind, KeyModifiers};
    use ratkit::KeyboardEvent;

    use super::workspace_index_for_keyboard;

    /// Verifies Ctrl+number maps to zero-based workspace indexes.
    #[test]
    fn maps_control_number_to_workspace_index() {
        assert_eq!(
            workspace_index_for_keyboard(&key('3', KeyModifiers::CONTROL)),
            Some(2)
        );
    }

    /// Verifies Cmd+number is ignored by workspace shortcuts.
    #[test]
    fn ignores_super_number() {
        assert_eq!(
            workspace_index_for_keyboard(&key('3', KeyModifiers::SUPER)),
            None
        );
    }

    /// Verifies plain numbers are ignored by workspace shortcuts.
    #[test]
    fn ignores_plain_number() {
        assert_eq!(
            workspace_index_for_keyboard(&key('3', KeyModifiers::empty())),
            None
        );
    }

    /// Builds a keyboard event for shortcut tests.
    fn key(character: char, modifiers: KeyModifiers) -> KeyboardEvent {
        KeyboardEvent {
            key_code: KeyCode::Char(character),
            modifiers,
            kind: KeyEventKind::Press,
        }
    }
}
