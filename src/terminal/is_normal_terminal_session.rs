use crate::harness::chat_session::{ChatSession, ChatSessionKind};

/// Returns whether session metadata represents a normal shell terminal.
pub fn is_normal_terminal_session(session: &ChatSession) -> bool {
    session.kind == ChatSessionKind::NormalTerminal
}

#[cfg(test)]
mod tests {
    use super::is_normal_terminal_session;
    use crate::harness::chat_session::{ChatSession, ChatSessionKind};

    /// Terminal ids should identify normal shell terminal sessions.
    #[test]
    fn terminal_id_is_normal_terminal() {
        let session = ChatSession::new("now", "Terminal", "terminal-123", "/tmp/project")
            .with_kind(ChatSessionKind::NormalTerminal);
        assert!(is_normal_terminal_session(&session));
    }

    /// chat ids should not identify normal shell terminal sessions.
    #[test]
    fn nexus_chat_id_is_not_normal_terminal() {
        let session = ChatSession::new("now", "Chat", "chat-123", "/tmp/project");
        assert!(!is_normal_terminal_session(&session));
    }
}
