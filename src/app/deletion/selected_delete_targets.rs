use crate::app::state::app_state::AppState;

/// Returns selected sessions as stable id/title pairs in sidebar order.
pub fn selected_delete_targets(app: &AppState) -> Vec<(String, String)> {
    app.session_terminals
        .iter()
        .filter(|entry| app.selected_conversation_ids.contains(&entry.session.id))
        .map(|entry| (entry.session.id.clone(), entry.session.title.clone()))
        .collect()
}
