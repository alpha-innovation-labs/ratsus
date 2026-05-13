use crossterm::event::KeyModifiers;
use ratkit::KeyboardEvent;

/// Returns true when the keyboard shortcut should open split-placement picker mode.
pub fn terminal_bundle_for_keyboard(keyboard: &KeyboardEvent) -> bool {
    if !keyboard.modifiers.contains(KeyModifiers::CONTROL) {
        return false;
    }
    let shifted_bracket = keyboard.modifiers.contains(KeyModifiers::SHIFT)
        && (keyboard.is_char('[') || keyboard.is_char(']'));
    let brace = keyboard.is_char('{') || keyboard.is_char('}');
    shifted_bracket || brace
}

#[cfg(test)]
mod tests {
    use crossterm::event::{KeyCode, KeyEventKind, KeyModifiers};
    use ratkit::KeyboardEvent;

    use super::terminal_bundle_for_keyboard;

    /// Builds a keyboard event for bundle shortcut tests.
    fn key(key_code: KeyCode, modifiers: KeyModifiers) -> KeyboardEvent {
        KeyboardEvent {
            key_code,
            modifiers,
            kind: KeyEventKind::Press,
        }
    }

    /// Ctrl+Shift+[ requests placement into the active split.
    #[test]
    fn maps_ctrl_shift_left_bracket_to_bundle() {
        assert!(terminal_bundle_for_keyboard(&key(
            KeyCode::Char('['),
            KeyModifiers::CONTROL | KeyModifiers::SHIFT,
        )));
    }

    /// Shifted bracket characters are accepted for terminals that emit braces.
    #[test]
    fn maps_ctrl_brace_to_bundle() {
        assert!(terminal_bundle_for_keyboard(&key(
            KeyCode::Char('{'),
            KeyModifiers::CONTROL,
        )));
    }

    /// Ctrl+[ without Shift remains a split shortcut, not a bundle shortcut.
    #[test]
    fn ignores_unshifted_ctrl_left_bracket() {
        assert!(!terminal_bundle_for_keyboard(&key(
            KeyCode::Char('['),
            KeyModifiers::CONTROL,
        )));
    }
}
