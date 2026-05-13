use crate::app::nexus_demo_state::NexusDemo;
use crate::app::toggle_conversation_selection_by_index::toggle_conversation_selection_by_index;
use crate::left_panel::focused_left_row::focused_left_row;
use crate::left_panel::session_list_row::SessionListRow;

/// Toggles bulk-selection state for the focused left-pane conversation row.
pub fn toggle_focused_left_conversation_selection(app: &mut NexusDemo) {
    let Some(SessionListRow::Session { index }) = focused_left_row(app) else {
        return;
    };
    toggle_conversation_selection_by_index(app, index);
}
