use crate::harness::chat_session::ChatSession;

/// Returns true when the session was created by Ctrl+N in the Nexus demo.
pub fn is_new_nexus_chat_session(session: &ChatSession) -> bool {
    session.title == "New Nexus chat" && session.id.starts_with("new-")
}
