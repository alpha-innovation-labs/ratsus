use crate::app::state::app_state::AppState;
use crate::ui::left_panel::session::list_row::SessionListRow;

/// Returns the currently focused visible row in the left panel.
pub fn focused_left_row(app: &AppState) -> Option<SessionListRow> {
    app.visible_rows().get(app.focused_row).cloned()
}
