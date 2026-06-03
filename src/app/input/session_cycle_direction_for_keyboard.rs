use crossterm::event::{KeyCode, KeyModifiers};
use ratkit::KeyboardEvent;

/// Returns the session-cycle direction for Ctrl+Tab shortcuts.
pub fn session_cycle_direction_for_keyboard(keyboard: &KeyboardEvent) -> Option<isize> {
    let has_control = keyboard.modifiers.contains(KeyModifiers::CONTROL);
    let has_shift = keyboard.modifiers.contains(KeyModifiers::SHIFT);
    match keyboard.key_code {
        KeyCode::Tab if has_control && has_shift => Some(-1),
        KeyCode::Char('\t') if has_control && has_shift => Some(-1),
        KeyCode::BackTab if has_control => Some(-1),
        KeyCode::Tab if has_control => Some(1),
        KeyCode::Char('\t') if has_control => Some(1),
        KeyCode::Char('i') if has_control && !has_shift => Some(1),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use crossterm::event::{KeyCode, KeyEventKind, KeyModifiers};
    use ratkit::KeyboardEvent;

    use super::session_cycle_direction_for_keyboard;

    /// Verifies Ctrl+Tab moves to the next session.
    #[test]
    fn maps_control_tab_to_next_session() {
        assert_eq!(
            session_cycle_direction_for_keyboard(&key(KeyCode::Tab, KeyModifiers::CONTROL)),
            Some(1)
        );
    }

    /// Verifies Ctrl+Shift+Tab moves to the previous session.
    #[test]
    fn maps_control_shift_tab_to_previous_session() {
        assert_eq!(
            session_cycle_direction_for_keyboard(&key(
                KeyCode::Tab,
                KeyModifiers::CONTROL | KeyModifiers::SHIFT,
            )),
            Some(-1)
        );
    }

    /// Verifies Ctrl+I moves forward when terminals encode Ctrl+Tab as Ctrl+I.
    #[test]
    fn maps_control_i_to_next_session() {
        assert_eq!(
            session_cycle_direction_for_keyboard(&key(KeyCode::Char('i'), KeyModifiers::CONTROL)),
            Some(1)
        );
    }

    /// Verifies Ctrl+BackTab also moves to the previous session.
    #[test]
    fn maps_control_backtab_to_previous_session() {
        assert_eq!(
            session_cycle_direction_for_keyboard(&key(KeyCode::BackTab, KeyModifiers::CONTROL)),
            Some(-1)
        );
    }

    /// Verifies plain Tab is ignored so it can reach the focused pane.
    #[test]
    fn ignores_plain_tab() {
        assert_eq!(
            session_cycle_direction_for_keyboard(&key(KeyCode::Tab, KeyModifiers::empty())),
            None
        );
    }

    /// Verifies Shift+BackTab is ignored without Ctrl.
    #[test]
    fn ignores_shift_backtab() {
        assert_eq!(
            session_cycle_direction_for_keyboard(&key(KeyCode::BackTab, KeyModifiers::SHIFT)),
            None
        );
    }

    /// Verifies plain BackTab is ignored without Ctrl.
    #[test]
    fn ignores_plain_backtab() {
        assert_eq!(
            session_cycle_direction_for_keyboard(&key(KeyCode::BackTab, KeyModifiers::empty())),
            None
        );
    }

    /// Verifies unrelated shortcuts are ignored.
    #[test]
    fn ignores_non_tab_shortcuts() {
        assert_eq!(
            session_cycle_direction_for_keyboard(&key(KeyCode::Char('x'), KeyModifiers::CONTROL)),
            None
        );
    }

    /// Builds a keyboard event fixture for shortcut direction tests.
    fn key(key_code: KeyCode, modifiers: KeyModifiers) -> KeyboardEvent {
        KeyboardEvent {
            key_code,
            modifiers,
            kind: KeyEventKind::Press,
        }
    }
}
