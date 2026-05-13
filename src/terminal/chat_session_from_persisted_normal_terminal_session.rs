use crate::harness::chat_session::{ChatSession, ChatSessionKind};
use crate::terminal::persisted_normal_terminal_session::PersistedNormalTerminalSession;

/// Builds in-memory session metadata from a persisted normal terminal entry.
pub fn chat_session_from_persisted_normal_terminal_session(
    entry: PersistedNormalTerminalSession,
) -> ChatSession {
    ChatSession::new_with_created(
        entry.date,
        entry.created_at,
        entry.title,
        entry.id,
        entry.working_dir,
    )
    .with_kind(ChatSessionKind::NormalTerminal)
}
