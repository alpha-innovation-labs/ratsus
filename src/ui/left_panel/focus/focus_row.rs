use crate::app::expo::activate_expo_folder::activate_expo_folder;
use crate::app::state::app_state::AppState;
use crate::ui::left_panel::session::activation::activate_split_group_child::activate_split_group_child;
use crate::ui::left_panel::session::activation::activate_split_group_parent::activate_split_group_parent;
use crate::ui::left_panel::session::list_row::SessionListRow;

/// Focuses a visible left-panel row and activates it when it is a session row.
pub fn focus_left_panel_row(app: &mut AppState, row_index: usize) {
    let rows = app.visible_rows();
    if rows.is_empty() {
        return;
    }
    app.suppress_left_focus_scroll = false;
    app.focused_row = row_index.min(rows.len() - 1);
    match rows.get(app.focused_row) {
        Some(SessionListRow::Folder { path, .. }) => activate_expo_folder(app, path.clone()),
        Some(SessionListRow::SplitGroup { group_id, .. }) => {
            activate_split_group_parent(app, *group_id);
        }
        Some(SessionListRow::SplitGroupChild { pane_id, index, .. }) => {
            activate_split_group_child(app, *pane_id, *index);
        }
        Some(SessionListRow::Session { index }) => {
            app.focused_index = *index;
            app.activate_focused_session();
        }
        None => {}
    }
    app.keep_focused_row_visible();
}
