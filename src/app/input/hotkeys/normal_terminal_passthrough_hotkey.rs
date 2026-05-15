use crossterm::event::{KeyCode, KeyModifiers};
use ratkit::KeyboardEvent;

use crate::app::state::app_state::AppState;
use crate::extensions::file_viewer::tabs::tab::MainPaneTab;
use crate::extensions::terminal::session::is_normal_terminal_session::is_normal_terminal_session;
use crate::ui::layout::focus::focused_pane::FocusedPane;

/// Returns whether a key should bypass app hotkeys inside normal shell terminals.
pub fn normal_terminal_passthrough_hotkey(app: &AppState, keyboard: &KeyboardEvent) -> bool {
    normal_terminal_is_focused(app) && keyboard_is_control_e(keyboard)
}

/// Returns whether the focused main pane is a normal terminal session.
fn normal_terminal_is_focused(app: &AppState) -> bool {
    app.focused_pane == FocusedPane::Terminal
        && app.active_main_pane_tab == MainPaneTab::Chat
        && app
            .active_session_terminal()
            .is_some_and(|entry| is_normal_terminal_session(&entry.session))
}

/// Returns whether the keyboard event is Ctrl+E.
fn keyboard_is_control_e(keyboard: &KeyboardEvent) -> bool {
    keyboard.key_code == KeyCode::Char('e') && keyboard.modifiers.contains(KeyModifiers::CONTROL)
}
