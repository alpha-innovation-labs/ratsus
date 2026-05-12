use crate::app::nexus_demo_state::NexusDemo;
use crate::left_panel::session_list_row::SessionListRow;

/// Returns the currently focused visible row in the left panel.
pub fn focused_left_row(app: &NexusDemo) -> Option<SessionListRow> {
    app.visible_rows().get(app.focused_row).cloned()
}
