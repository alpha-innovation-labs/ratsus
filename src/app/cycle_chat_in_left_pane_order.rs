use crate::app::adjacent_chat_index_in_left_pane_order::adjacent_chat_index_in_left_pane_order;
use crate::app::app_state::AppState;

/// Activates the next or previous chat according to visible left-pane row order.
pub fn cycle_chat_in_left_pane_order(app: &mut AppState, direction: isize) -> bool {
    let rows = app.visible_rows();
    let Some(index) = adjacent_chat_index_in_left_pane_order(
        &rows,
        &app.session_terminals,
        app.active_index,
        direction,
    ) else {
        return false;
    };
    app.focused_index = index;
    app.activate_focused_session();
    true
}
