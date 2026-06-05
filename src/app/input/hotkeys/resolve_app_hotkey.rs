use crossterm::event::{KeyCode, KeyModifiers};
use ratkit::services::hotkey_service::{HotkeyRegistry, HotkeyScope};
use ratkit::KeyboardEvent;

use crate::app::input::hotkeys::app_hotkey::AppHotkey;
use crate::app::input::session_cycle_direction_for_keyboard::session_cycle_direction_for_keyboard;
use crate::ui::grid_layout::bundle::terminal_bundle_for_keyboard::terminal_bundle_for_keyboard;
use crate::ui::grid_layout::split::split_direction_for_keyboard::terminal_split_direction_for_keyboard;

/// Resolves one keyboard event into a top-level Ratsus shortcut command.
pub fn resolve_app_hotkey(
    registry: &HotkeyRegistry,
    keyboard: &KeyboardEvent,
    scope: &HotkeyScope,
) -> Option<AppHotkey> {
    if let Some(direction) = session_cycle_direction_for_keyboard(keyboard) {
        return Some(AppHotkey::CycleSession(direction));
    }
    if control_char(keyboard, '`') {
        return Some(AppHotkey::CycleLeftPaneMode);
    }
    if control_char(keyboard, 'q') {
        return Some(AppHotkey::Quit);
    }
    let _ = registry.lookup(&keyboard.key_code, scope)?;
    if control_char(keyboard, 'k') {
        return Some(AppHotkey::OpenCommandBar);
    }
    if control_char(keyboard, 'h') {
        return Some(AppHotkey::OpenHistoryModal);
    }
    if control_char(keyboard, 'e') {
        return Some(AppHotkey::OpenFocusedConversationExpo);
    }
    if let Some(direction) = terminal_bundle_for_keyboard(keyboard) {
        return Some(AppHotkey::PlaceConversationInActiveSplit(direction));
    }
    if let Some(direction) = terminal_split_direction_for_keyboard(keyboard) {
        return Some(AppHotkey::SplitTerminal(direction));
    }
    if command_char(keyboard, 'n') {
        return Some(AppHotkey::StartChat);
    }
    if control_char(keyboard, 't') {
        return Some(AppHotkey::StartTerminal);
    }
    if control_char(keyboard, 'l') {
        return Some(AppHotkey::ToggleLeftPane);
    }
    if control_char(keyboard, 'x') {
        return Some(AppHotkey::ToggleFocusedPane);
    }
    None
}

/// Returns true when a keyboard event is a Ctrl-modified character command.
fn control_char(keyboard: &KeyboardEvent, character: char) -> bool {
    keyboard_char_eq_ignore_ascii_case(keyboard, character)
        && keyboard.modifiers.contains(KeyModifiers::CONTROL)
}

/// Returns true when a keyboard event is a Cmd/Super-modified character command.
fn command_char(keyboard: &KeyboardEvent, character: char) -> bool {
    keyboard_char_eq_ignore_ascii_case(keyboard, character)
        && keyboard
            .modifiers
            .intersects(KeyModifiers::SUPER | KeyModifiers::META)
        && !keyboard
            .modifiers
            .intersects(KeyModifiers::CONTROL | KeyModifiers::ALT)
}

/// Returns whether a keyboard event matches a character regardless of shift casing.
fn keyboard_char_eq_ignore_ascii_case(keyboard: &KeyboardEvent, character: char) -> bool {
    matches!(keyboard.key_code, KeyCode::Char(ch) if ch.eq_ignore_ascii_case(&character))
}

#[cfg(test)]
mod tests {
    use crossterm::event::{KeyCode, KeyEventKind, KeyModifiers};
    use ratkit::KeyboardEvent;

    use super::resolve_app_hotkey;
    use crate::app::input::hotkeys::app_hotkey::AppHotkey;
    use crate::app::input::hotkeys::app_hotkey_registry::app_hotkey_registry;
    use crate::app::input::hotkeys::scopes::TERMINAL_SCOPE;

    /// Ctrl+K should resolve through the Ratkit registry into the command bar command.
    #[test]
    fn resolves_control_k_from_ratkit_registry() {
        let registry = app_hotkey_registry();

        let hotkey = resolve_app_hotkey(
            &registry,
            &key(KeyCode::Char('k'), KeyModifiers::CONTROL),
            &TERMINAL_SCOPE,
        );

        assert_eq!(hotkey, Some(AppHotkey::OpenCommandBar));
    }

    /// Ctrl+H should resolve through the Ratkit registry into the picker command.
    #[test]
    fn resolves_control_h_from_ratkit_registry() {
        let registry = app_hotkey_registry();

        let hotkey = resolve_app_hotkey(
            &registry,
            &key(KeyCode::Char('h'), KeyModifiers::CONTROL),
            &TERMINAL_SCOPE,
        );

        assert_eq!(hotkey, Some(AppHotkey::OpenHistoryModal));
    }

    /// Ctrl+Tab should resolve into next-session cycling.
    #[test]
    fn resolves_control_tab_to_next_session() {
        let registry = app_hotkey_registry();

        let hotkey = resolve_app_hotkey(
            &registry,
            &key(KeyCode::Tab, KeyModifiers::CONTROL),
            &TERMINAL_SCOPE,
        );

        assert_eq!(hotkey, Some(AppHotkey::CycleSession(1)));
    }

    /// Ctrl+Shift+Tab should resolve into previous-session cycling.
    #[test]
    fn resolves_control_shift_tab_to_previous_session() {
        let registry = app_hotkey_registry();

        let hotkey = resolve_app_hotkey(
            &registry,
            &key(
                KeyCode::BackTab,
                KeyModifiers::CONTROL | KeyModifiers::SHIFT,
            ),
            &TERMINAL_SCOPE,
        );

        assert_eq!(hotkey, Some(AppHotkey::CycleSession(-1)));
    }

    /// Cmd+N should resolve into starting a new chat.
    #[test]
    fn resolves_super_n_to_start_chat() {
        let registry = app_hotkey_registry();

        let hotkey = resolve_app_hotkey(
            &registry,
            &key(KeyCode::Char('n'), KeyModifiers::SUPER),
            &TERMINAL_SCOPE,
        );

        assert_eq!(hotkey, Some(AppHotkey::StartChat));
    }

    /// Ctrl+N should not start a new chat.
    #[test]
    fn ignores_control_n_for_start_chat() {
        let registry = app_hotkey_registry();

        let hotkey = resolve_app_hotkey(
            &registry,
            &key(KeyCode::Char('n'), KeyModifiers::CONTROL),
            &TERMINAL_SCOPE,
        );

        assert_eq!(hotkey, None);
    }

    /// Alt+N should resolve into a right-side vertical split.
    #[test]
    fn resolves_alt_n_to_vertical_split() {
        let registry = app_hotkey_registry();

        let hotkey = resolve_app_hotkey(
            &registry,
            &key(KeyCode::Char('n'), KeyModifiers::ALT),
            &TERMINAL_SCOPE,
        );

        assert_eq!(
            hotkey,
            Some(AppHotkey::SplitTerminal(
                crate::ui::grid_layout::split::split_direction::TerminalSplitDirection::Right
            ))
        );
    }

    /// Alt+Shift+N should resolve into a bottom horizontal split.
    #[test]
    fn resolves_alt_shift_n_to_horizontal_split() {
        let registry = app_hotkey_registry();

        let hotkey = resolve_app_hotkey(
            &registry,
            &key(KeyCode::Char('N'), KeyModifiers::ALT | KeyModifiers::SHIFT),
            &TERMINAL_SCOPE,
        );

        assert_eq!(
            hotkey,
            Some(AppHotkey::SplitTerminal(
                crate::ui::grid_layout::split::split_direction::TerminalSplitDirection::Bottom
            ))
        );
    }

    /// Ctrl+` should cycle the shared left pane mode.
    #[test]
    fn resolves_control_backtick_to_cycle_left_pane_mode() {
        let registry = app_hotkey_registry();

        let hotkey = resolve_app_hotkey(
            &registry,
            &key(KeyCode::Char('`'), KeyModifiers::CONTROL),
            &TERMINAL_SCOPE,
        );

        assert_eq!(hotkey, Some(AppHotkey::CycleLeftPaneMode));
    }

    /// Plain ` should not cycle the shared left pane mode.
    #[test]
    fn ignores_plain_backtick_for_cycle_left_pane_mode() {
        let registry = app_hotkey_registry();

        let hotkey = resolve_app_hotkey(
            &registry,
            &key(KeyCode::Char('`'), KeyModifiers::empty()),
            &TERMINAL_SCOPE,
        );

        assert_eq!(hotkey, None);
    }

    /// Ctrl+Q should resolve into quit even though display metadata includes the modifier.
    #[test]
    fn resolves_control_q_to_quit() {
        let registry = app_hotkey_registry();

        let hotkey = resolve_app_hotkey(
            &registry,
            &key(KeyCode::Char('q'), KeyModifiers::CONTROL),
            &TERMINAL_SCOPE,
        );

        assert_eq!(hotkey, Some(AppHotkey::Quit));
    }

    /// Plain q should not resolve into quit.
    #[test]
    fn ignores_plain_q_for_quit() {
        let registry = app_hotkey_registry();

        let hotkey = resolve_app_hotkey(
            &registry,
            &key(KeyCode::Char('q'), KeyModifiers::empty()),
            &TERMINAL_SCOPE,
        );

        assert_eq!(hotkey, None);
    }

    /// Unregistered character shortcuts should not resolve even with Ctrl held.
    #[test]
    fn ignores_unregistered_control_shortcuts() {
        let registry = app_hotkey_registry();

        let hotkey = resolve_app_hotkey(
            &registry,
            &key(KeyCode::Char('z'), KeyModifiers::CONTROL),
            &TERMINAL_SCOPE,
        );

        assert_eq!(hotkey, None);
    }

    /// Builds a keyboard event for hotkey resolver tests.
    fn key(key_code: KeyCode, modifiers: KeyModifiers) -> KeyboardEvent {
        KeyboardEvent {
            key_code,
            modifiers,
            kind: KeyEventKind::Press,
        }
    }
}
