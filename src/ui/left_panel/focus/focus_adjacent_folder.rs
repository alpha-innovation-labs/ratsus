use crate::app::state::app_state::AppState;
use crate::ui::left_panel::focus::focus_row::focus_left_panel_row;
use crate::ui::left_panel::folder::row_index_after::folder_row_index_after;

/// Focuses the next or previous visible folder row in the left pane.
pub fn focus_adjacent_folder(app: &mut AppState, direction: isize) {
    let rows = app.visible_rows();
    let Some(row_index) = folder_row_index_after(&rows, app.focused_row, direction) else {
        return;
    };
    focus_left_panel_row(app, row_index);
}
