use crate::app::activate_expo_folder::activate_expo_folder;
use crate::app::nexus_demo_state::NexusDemo;
use crate::left_panel::session_list_row::SessionListRow;

/// Focuses a visible left-panel row and activates it when it is a session row.
pub fn focus_left_panel_row(app: &mut NexusDemo, row_index: usize) {
    let rows = app.visible_rows();
    if rows.is_empty() {
        return;
    }
    app.suppress_left_focus_scroll = false;
    app.focused_row = row_index.min(rows.len() - 1);
    match rows.get(app.focused_row) {
        Some(SessionListRow::Folder { path, .. }) => activate_expo_folder(app, path.clone()),
        Some(SessionListRow::Session { index }) => {
            app.focused_index = *index;
            app.activate_focused_session();
        }
        Some(SessionListRow::FolderMore { .. }) | None => {}
    }
    app.keep_focused_row_visible();
}
