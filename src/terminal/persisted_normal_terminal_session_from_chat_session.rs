use crate::harness::chat_session::ChatSession;
use crate::terminal::persisted_normal_terminal_session::PersistedNormalTerminalSession;

/// Builds persisted normal terminal metadata from in-memory session metadata.
pub fn persisted_normal_terminal_session_from_chat_session(
    session: &ChatSession,
) -> PersistedNormalTerminalSession {
    PersistedNormalTerminalSession {
        date: session.date.clone(),
        created_at: session.created_at.clone(),
        title: session.title.clone(),
        id: session.id.clone(),
        working_dir: session.working_dir.clone(),
    }
}
