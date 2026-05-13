use crate::app::state::app_state::AppState;
use crate::ui::layout::focus::focused_pane::FocusedPane;

/// Toggles focus between the session list and terminal pane.
pub fn toggle_focused_pane(app: &mut AppState) {
    if !app.left_pane_visible {
        app.focused_pane = FocusedPane::Terminal;
        return;
    }
    app.focused_pane = match app.focused_pane {
        FocusedPane::Terminal => FocusedPane::Left,
        FocusedPane::Left => FocusedPane::Terminal,
    };
}
