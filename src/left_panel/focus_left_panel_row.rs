use crate::app::nexus_demo_state::NexusDemo;
use crate::left_panel::session_list_row::SessionListRow;

/// Focuses a visible left-panel row and activates it when it is a session row.
pub fn focus_left_panel_row(app: &mut NexusDemo, row_index: usize) {
    let rows = app.visible_rows();
    if rows.is_empty() {
        return;
    }
    app.focused_row = row_index.min(rows.len() - 1);
    if let Some(SessionListRow::Session { index }) = rows.get(app.focused_row) {
        app.focused_index = *index;
        app.activate_focused_session();
    }
    app.keep_focused_row_visible();
}
