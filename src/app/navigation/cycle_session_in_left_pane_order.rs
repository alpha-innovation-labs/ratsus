use crate::app::navigation::adjacent_session_index_in_left_pane_order::adjacent_session_index_in_left_pane_order;
use crate::app::state::app_state::AppState;

/// Activates the next or previous session according to visible left-pane row order.
pub fn cycle_session_in_left_pane_order(app: &mut AppState, direction: isize) -> bool {
    let rows = app.visible_rows();
    let Some(index) = adjacent_session_index_in_left_pane_order(
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
