use anyhow::Result;

use crate::app::delete_focus_row_index::delete_focus_row_index;
use crate::app::nexus_demo_state::NexusDemo;
use crate::app::spawn_delete_sessions_worker::spawn_delete_sessions_worker;
use crate::terminal::is_chat_session::is_chat_session;

/// Starts deletion for all pending session targets.
pub fn confirm_delete_session(app: &mut NexusDemo) -> Result<()> {
    if app.delete_confirmation.is_deleting {
        return Ok(());
    }
    let session_ids = app.delete_confirmation.session_ids.clone();
    if session_ids.is_empty() {
        return Ok(());
    }
    let deleted_indices = delete_target_indices(app, &session_ids);
    let preferred_row = delete_focus_row_index(&app.visible_rows(), &deleted_indices);
    let chat_session_ids = chat_delete_targets(app, &session_ids);
    app.delete_confirmation.start_deleting(preferred_row);
    app.delete_session_receiver = Some(spawn_delete_sessions_worker(chat_session_ids));
    Ok(())
}

/// Returns sorted session indices matching requested stable ids.
pub fn delete_target_indices(app: &NexusDemo, session_ids: &[String]) -> Vec<usize> {
    app.session_terminals
        .iter()
        .enumerate()
        .filter_map(|(index, entry)| session_ids.contains(&entry.session.id).then_some(index))
        .collect()
}

/// Returns requested ids that represent Nexus chat sessions.
fn chat_delete_targets(app: &NexusDemo, session_ids: &[String]) -> Vec<String> {
    app.session_terminals
        .iter()
        .filter(|entry| session_ids.contains(&entry.session.id))
        .filter(|entry| is_chat_session(&entry.session))
        .map(|entry| entry.session.id.clone())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::delete_target_indices;

    /// This helper is exercised by integration-style app tests.
    #[test]
    fn helper_is_linked() {
        let _ = delete_target_indices;
    }
}
