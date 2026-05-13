use crate::app::nexus_demo_state::NexusDemo;
use crate::left_panel::focus_left_panel_row::focus_left_panel_row;
use crate::left_panel::folder_row_index_after::folder_row_index_after;

/// Focuses the next or previous visible folder row in the left pane.
pub fn focus_adjacent_folder(app: &mut NexusDemo, direction: isize) {
    let rows = app.visible_rows();
    let Some(row_index) = folder_row_index_after(&rows, app.focused_row, direction) else {
        return;
    };
    focus_left_panel_row(app, row_index);
}
