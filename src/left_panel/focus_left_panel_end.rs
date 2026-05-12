use crate::app::nexus_demo_state::NexusDemo;
use crate::left_panel::focus_left_panel_row::focus_left_panel_row;

/// Moves left-panel focus to the last visible row.
pub fn focus_left_panel_end(app: &mut NexusDemo) {
    let last_index = app.visible_rows().len().saturating_sub(1);
    focus_left_panel_row(app, last_index);
}
