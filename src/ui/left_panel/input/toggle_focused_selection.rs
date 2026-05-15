use crate::app::navigation::toggle_conversation_selection_by_index::toggle_conversation_selection_by_index;
use crate::app::state::app_state::AppState;
use crate::ui::left_panel::focus::focused_row::focused_left_row;
use crate::ui::left_panel::session::list_row::SessionListRow;

/// Toggles bulk-selection state for the focused left-pane conversation row.
pub fn toggle_focused_left_conversation_selection(app: &mut AppState) {
    let Some(SessionListRow::Session { index } | SessionListRow::SplitGroupChild { index, .. }) =
        focused_left_row(app)
    else {
        return;
    };
    toggle_conversation_selection_by_index(app, index);
}
