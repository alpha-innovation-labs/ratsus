use crossterm::event::KeyModifiers;
use ratkit::KeyboardEvent;

use crate::ui::grid_layout::split::split_direction::TerminalSplitDirection;

/// Returns the split direction requested by an existing-session placement shortcut.
pub fn terminal_bundle_for_keyboard(keyboard: &KeyboardEvent) -> Option<TerminalSplitDirection> {
    if !keyboard.modifiers.contains(KeyModifiers::CONTROL) {
        return None;
    }
    if keyboard.modifiers.contains(KeyModifiers::SHIFT) && keyboard.is_char('[')
        || keyboard.is_char('{')
    {
        return Some(TerminalSplitDirection::Bottom);
    }
    if keyboard.modifiers.contains(KeyModifiers::SHIFT) && keyboard.is_char(']')
        || keyboard.is_char('}')
    {
        return Some(TerminalSplitDirection::Right);
    }
    None
}

#[cfg(test)]
mod tests {
    use crossterm::event::{KeyCode, KeyEventKind, KeyModifiers};
    use ratkit::KeyboardEvent;

    use super::terminal_bundle_for_keyboard;
    use crate::ui::grid_layout::split::split_direction::TerminalSplitDirection;

    /// Builds a keyboard event for bundle shortcut tests.
    fn key(key_code: KeyCode, modifiers: KeyModifiers) -> KeyboardEvent {
        KeyboardEvent {
            key_code,
            modifiers,
            kind: KeyEventKind::Press,
        }
    }

    /// Ctrl+Shift+[ requests bottom placement into a new split.
    #[test]
    fn maps_ctrl_shift_left_bracket_to_bottom_placement() {
        assert_eq!(
            terminal_bundle_for_keyboard(&key(
                KeyCode::Char('['),
                KeyModifiers::CONTROL | KeyModifiers::SHIFT,
            )),
            Some(TerminalSplitDirection::Bottom)
        );
    }

    /// Ctrl+Shift+] requests right placement into a new split.
    #[test]
    fn maps_ctrl_shift_right_bracket_to_right_placement() {
        assert_eq!(
            terminal_bundle_for_keyboard(&key(
                KeyCode::Char(']'),
                KeyModifiers::CONTROL | KeyModifiers::SHIFT,
            )),
            Some(TerminalSplitDirection::Right)
        );
    }

    /// Shifted bracket characters are accepted for terminals that emit braces.
    #[test]
    fn maps_ctrl_brace_to_bundle() {
        assert_eq!(
            terminal_bundle_for_keyboard(&key(KeyCode::Char('{'), KeyModifiers::CONTROL,)),
            Some(TerminalSplitDirection::Bottom)
        );
    }

    /// Ctrl+[ without Shift remains a split shortcut, not a bundle shortcut.
    #[test]
    fn ignores_unshifted_ctrl_left_bracket() {
        assert_eq!(
            terminal_bundle_for_keyboard(&key(KeyCode::Char('['), KeyModifiers::CONTROL,)),
            None
        );
    }
}
