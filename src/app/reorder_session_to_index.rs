use crate::app::app_state::AppState;
use crate::left_panel::persist_session_order_preferences::persist_session_order_preferences;
use crate::left_panel::reordered_index_after_move::reordered_index_after_move;

/// Moves one session entry to another index while preserving active and focused references.
pub fn reorder_session_to_index(
    app: &mut AppState,
    from_index: usize,
    target_index: usize,
) -> bool {
    if from_index >= app.session_terminals.len()
        || target_index >= app.session_terminals.len()
        || from_index == target_index
    {
        return false;
    }
    let entry = app.session_terminals.remove(from_index);
    app.session_terminals.insert(target_index, entry);
    app.active_index = reordered_index_after_move(app.active_index, from_index, target_index);
    app.focused_index = reordered_index_after_move(app.focused_index, from_index, target_index);
    persist_session_order_preferences(app);
    true
}
