use crate::app::nexus_demo_state::NexusDemo;
use crate::terminal::is_chat_session::is_chat_session;

/// Returns selected sessions that can be removed after delete worker completion.
pub fn removable_delete_session_ids(app: &NexusDemo, deleted_chat_ids: &[String]) -> Vec<String> {
    app.session_terminals
        .iter()
        .filter(|entry| {
            app.delete_confirmation
                .session_ids
                .contains(&entry.session.id)
        })
        .filter(|entry| can_remove_session(app, &entry.session.id, deleted_chat_ids))
        .map(|entry| entry.session.id.clone())
        .collect()
}

/// Returns whether one selected session is removable now.
fn can_remove_session(app: &NexusDemo, session_id: &str, deleted_chat_ids: &[String]) -> bool {
    let Some(entry) = app
        .session_terminals
        .iter()
        .find(|entry| entry.session.id == session_id)
    else {
        return false;
    };
    !is_chat_session(&entry.session) || deleted_chat_ids.contains(&entry.session.id)
}
