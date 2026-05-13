use crate::app::nexus_demo_state::NexusDemo;

/// Returns selected sessions as stable id/title pairs in sidebar order.
pub fn selected_delete_targets(app: &NexusDemo) -> Vec<(String, String)> {
    app.session_terminals
        .iter()
        .filter(|entry| app.selected_conversation_ids.contains(&entry.session.id))
        .map(|entry| (entry.session.id.clone(), entry.session.title.clone()))
        .collect()
}
