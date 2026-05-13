use crate::terminal::session_terminal::SessionTerminal;

/// Returns all Nexus conversation ids represented by session terminals.
pub fn observation_conversation_ids(sessions: &[SessionTerminal]) -> Vec<String> {
    sessions
        .iter()
        .map(|entry| entry.session.id.clone())
        .collect()
}
