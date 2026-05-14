use ratkit::services::hotkey_service::{Hotkey, HotkeyRegistry};

use crate::app::input::hotkeys::scopes::GLOBAL_SCOPE;

/// Builds the Ratkit registry for global Ratsus shortcuts and help metadata.
pub fn app_hotkey_registry() -> HotkeyRegistry {
    let mut registry = HotkeyRegistry::new();
    for hotkey in global_hotkeys() {
        registry.register(hotkey);
    }
    registry
}

/// Returns the global hotkeys registered for top-level command resolution.
fn global_hotkeys() -> Vec<Hotkey> {
    vec![
        Hotkey::new("tab", "Next chat").scope(GLOBAL_SCOPE),
        Hotkey::new("k", "Open conversations").scope(GLOBAL_SCOPE),
        Hotkey::new("e", "Open Expo").scope(GLOBAL_SCOPE),
        Hotkey::new("[", "Split down or place").scope(GLOBAL_SCOPE),
        Hotkey::new("]", "Split right or place").scope(GLOBAL_SCOPE),
        Hotkey::new("5", "Split right").scope(GLOBAL_SCOPE),
        Hotkey::new("{", "Place in split").scope(GLOBAL_SCOPE),
        Hotkey::new("}", "Place in split").scope(GLOBAL_SCOPE),
        Hotkey::new("n", "New chat").scope(GLOBAL_SCOPE),
        Hotkey::new("t", "New terminal").scope(GLOBAL_SCOPE),
        Hotkey::new("l", "Toggle left pane").scope(GLOBAL_SCOPE),
        Hotkey::new("x", "Toggle focus").scope(GLOBAL_SCOPE),
        Hotkey::new("q", "Quit").scope(GLOBAL_SCOPE),
    ]
}
