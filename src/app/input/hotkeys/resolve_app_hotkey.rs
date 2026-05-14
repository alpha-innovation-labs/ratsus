use crossterm::event::KeyModifiers;
use ratkit::services::hotkey_service::{HotkeyRegistry, HotkeyScope};
use ratkit::KeyboardEvent;

use crate::app::input::chat_cycle_direction_for_keyboard::chat_cycle_direction_for_keyboard;
use crate::app::input::hotkeys::app_hotkey::AppHotkey;
use crate::ui::grid_layout::bundle::terminal_bundle_for_keyboard::terminal_bundle_for_keyboard;
use crate::ui::grid_layout::split::split_direction_for_keyboard::terminal_split_direction_for_keyboard;

/// Resolves one keyboard event into a top-level Ratsus shortcut command.
pub fn resolve_app_hotkey(
    registry: &HotkeyRegistry,
    keyboard: &KeyboardEvent,
    scope: &HotkeyScope,
) -> Option<AppHotkey> {
    let _ = registry.lookup(&keyboard.key_code, scope)?;
    if let Some(direction) = chat_cycle_direction_for_keyboard(keyboard) {
        return Some(AppHotkey::CycleChat(direction));
    }
    if control_char(keyboard, 'k') {
        return Some(AppHotkey::OpenConversationPicker);
    }
    if control_char(keyboard, 'e') {
        return Some(AppHotkey::OpenFocusedConversationExpo);
    }
    if terminal_bundle_for_keyboard(keyboard) {
        return Some(AppHotkey::PlaceConversationInActiveSplit);
    }
    if let Some(direction) = terminal_split_direction_for_keyboard(keyboard) {
        return Some(AppHotkey::SplitTerminal(direction));
    }
    if control_char(keyboard, 'n') {
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
    if control_char(keyboard, 'q') {
        return Some(AppHotkey::Quit);
    }
    None
}

/// Returns true when a keyboard event is a Ctrl-modified character command.
fn control_char(keyboard: &KeyboardEvent, character: char) -> bool {
    keyboard.is_char(character) && keyboard.modifiers.contains(KeyModifiers::CONTROL)
}

#[cfg(test)]
mod tests {
    use crossterm::event::{KeyCode, KeyEventKind, KeyModifiers};
    use ratkit::KeyboardEvent;

    use super::resolve_app_hotkey;
    use crate::app::input::hotkeys::app_hotkey::AppHotkey;
    use crate::app::input::hotkeys::app_hotkey_registry::app_hotkey_registry;
    use crate::app::input::hotkeys::scopes::TERMINAL_SCOPE;

    /// Ctrl+K should resolve through the Ratkit registry into the picker command.
    #[test]
    fn resolves_control_k_from_ratkit_registry() {
        let registry = app_hotkey_registry();

        let hotkey = resolve_app_hotkey(
            &registry,
            &key(KeyCode::Char('k'), KeyModifiers::CONTROL),
            &TERMINAL_SCOPE,
        );

        assert_eq!(hotkey, Some(AppHotkey::OpenConversationPicker));
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
