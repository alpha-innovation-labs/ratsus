use crate::app::app_state::AppState;

/// Toggles bulk-selection state for one conversation by session index.
pub fn toggle_conversation_selection_by_index(app: &mut AppState, index: usize) {
    let Some(session_id) = app
        .session_terminals
        .get(index)
        .map(|entry| entry.session.id.clone())
    else {
        return;
    };
    if !app.selected_conversation_ids.remove(&session_id) {
        app.selected_conversation_ids.insert(session_id);
    }
}
