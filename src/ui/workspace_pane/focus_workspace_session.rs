use std::path::Path;

use crate::app::state::app_state::AppState;
use crate::ui::layout::focus::focused_pane::FocusedPane;
use crate::ui::workspace_pane::session_index_for_workspace::session_index_for_workspace;

/// Activates the remembered or first session for a workspace and moves focus to the main pane.
pub fn focus_workspace_session(app: &mut AppState, path: &Path) -> bool {
    let Some(index) = session_index_for_workspace(app, path) else {
        app.focused_pane = FocusedPane::Terminal;
        return false;
    };
    app.focused_index = index;
    app.activate_focused_session();
    app.focused_pane = FocusedPane::Terminal;
    true
}
