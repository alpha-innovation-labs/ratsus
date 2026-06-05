use ratkit::services::hotkey_service::HotkeyScope;

use crate::app::state::app_state::AppState;
use crate::ui::layout::focus::focused_pane::FocusedPane;

/// Ratkit scope name used when no narrower context is active.
pub const GLOBAL_SCOPE: HotkeyScope = HotkeyScope::Global;
/// Ratkit scope name used for terminal-pane shortcuts.
pub const TERMINAL_SCOPE: HotkeyScope = HotkeyScope::Custom("terminal");
/// Ratkit scope name used for left-panel shortcuts.
pub const LEFT_PANEL_SCOPE: HotkeyScope = HotkeyScope::Custom("left-panel");
/// Ratkit scope name used for conversation picker shortcuts.
pub const PICKER_SCOPE: HotkeyScope = HotkeyScope::Modal("conversation-picker");
/// Ratkit scope name used for confirmation modal shortcuts.
pub const MODAL_SCOPE: HotkeyScope = HotkeyScope::Modal("modal");

/// Returns the currently active Ratkit hotkey scope for top-level resolution.
pub fn active_hotkey_scope(app: &AppState) -> HotkeyScope {
    if app.delete_confirmation.is_open() {
        return MODAL_SCOPE;
    }
    if app.history_modal.is_open {
        return PICKER_SCOPE;
    }
    match app.focused_pane {
        FocusedPane::Left => LEFT_PANEL_SCOPE,
        FocusedPane::Terminal => TERMINAL_SCOPE,
    }
}
