use crossterm::event::{KeyCode, KeyModifiers};
use ratkit::KeyboardEvent;

use crate::ui::grid_layout::split::split_direction::TerminalSplitDirection;

/// Returns the requested terminal split direction for supported keyboard shortcuts.
pub fn terminal_split_direction_for_keyboard(
    keyboard: &KeyboardEvent,
) -> Option<TerminalSplitDirection> {
    if keyboard.key_code == KeyCode::Char('\u{1d}') {
        return Some(TerminalSplitDirection::Right);
    }
    if !keyboard.modifiers.contains(KeyModifiers::CONTROL)
        || keyboard.modifiers.contains(KeyModifiers::SHIFT)
    {
        return None;
    }
    if keyboard.is_char(']') || keyboard.is_char('5') {
        return Some(TerminalSplitDirection::Right);
    }
    if keyboard.is_char('[') {
        return Some(TerminalSplitDirection::Bottom);
    }
    None
}

#[cfg(test)]
mod tests {
    use crossterm::event::{KeyCode, KeyEventKind, KeyModifiers};
    use ratkit::KeyboardEvent;

    use super::terminal_split_direction_for_keyboard;
    use crate::ui::grid_layout::split::split_direction::TerminalSplitDirection;

    /// Builds a keyboard event for split shortcut mapping tests.
    fn key(key_code: KeyCode, modifiers: KeyModifiers) -> KeyboardEvent {
        KeyboardEvent {
            key_code,
            modifiers,
            kind: KeyEventKind::Press,
        }
    }

    /// Ctrl+] should request a right-side split.
    #[test]
    fn maps_ctrl_right_bracket_to_right_split() {
        let direction =
            terminal_split_direction_for_keyboard(&key(KeyCode::Char(']'), KeyModifiers::CONTROL));

        assert_eq!(direction, Some(TerminalSplitDirection::Right));
    }

    /// Crossterm maps Unix Ctrl+] byte 0x1D to Ctrl+5.
    #[test]
    fn maps_unix_ctrl_right_bracket_to_right_split() {
        let direction =
            terminal_split_direction_for_keyboard(&key(KeyCode::Char('5'), KeyModifiers::CONTROL));

        assert_eq!(direction, Some(TerminalSplitDirection::Right));
    }

    /// Raw Ctrl+] control code should request a right-side split.
    #[test]
    fn maps_raw_ctrl_right_bracket_to_right_split() {
        let direction = terminal_split_direction_for_keyboard(&key(
            KeyCode::Char('\u{1d}'),
            KeyModifiers::empty(),
        ));

        assert_eq!(direction, Some(TerminalSplitDirection::Right));
    }

    /// Ctrl+[ should request a bottom split.
    #[test]
    fn maps_ctrl_left_bracket_to_bottom_split() {
        let direction =
            terminal_split_direction_for_keyboard(&key(KeyCode::Char('['), KeyModifiers::CONTROL));

        assert_eq!(direction, Some(TerminalSplitDirection::Bottom));
    }

    /// Bracket keys without Ctrl are normal terminal input.
    #[test]
    fn ignores_unmodified_brackets() {
        let direction =
            terminal_split_direction_for_keyboard(&key(KeyCode::Char(']'), KeyModifiers::empty()));

        assert_eq!(direction, None);
    }

    /// Ctrl+Shift+[ is reserved for bundling into the active split.
    #[test]
    fn ignores_shifted_ctrl_brackets() {
        let direction = terminal_split_direction_for_keyboard(&key(
            KeyCode::Char('['),
            KeyModifiers::CONTROL | KeyModifiers::SHIFT,
        ));

        assert_eq!(direction, None);
    }
}
