use crate::harness::chat_session::ChatSession;
use crate::terminal::is_normal_terminal_session::is_normal_terminal_session;

/// Returns whether session metadata represents a chat entry.
pub fn is_chat_session(session: &ChatSession) -> bool {
    !is_normal_terminal_session(session)
}

#[cfg(test)]
mod tests {
    use super::is_chat_session;
    use crate::harness::chat_session::{ChatSession, ChatSessionKind};

    /// Normal terminal sessions are not chat sessions.
    #[test]
    fn normal_terminal_is_not_chat() {
        let session = ChatSession::new("now", "Terminal", "terminal-1", "/tmp/project")
            .with_kind(ChatSessionKind::NormalTerminal);
        assert!(!is_chat_session(&session));
    }

    /// Chat session ids are chat sessions.
    #[test]
    fn chat_session_is_chat() {
        let session = ChatSession::new("now", "Chat", "session-1", "/tmp/project");
        assert!(is_chat_session(&session));
    }
}
