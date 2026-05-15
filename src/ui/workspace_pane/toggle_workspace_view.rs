use crate::app::state::app_state::AppState;
use crate::ui::grid_layout::persistence::persist_multiplexer_state::persist_multiplexer_state;

/// Toggles between workspace-pane mode and legacy all-folders left-pane mode.
pub fn toggle_workspace_view(app: &mut AppState) -> bool {
    app.workspace_view_enabled = !app.workspace_view_enabled;
    app.session_scroll = 0;
    app.focused_row = 0;
    persist_multiplexer_state(app);
    true
}
