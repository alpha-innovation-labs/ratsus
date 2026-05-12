use crate::nexus_sessions::session_info::NexusSession;
use crate::terminal::is_normal_terminal_session::is_normal_terminal_session;

/// Returns whether session metadata represents a Nexus chat entry.
pub fn is_chat_session(session: &NexusSession) -> bool {
    !is_normal_terminal_session(session)
}

#[cfg(test)]
mod tests {
    use super::is_chat_session;
    use crate::nexus_sessions::session_info::NexusSession;

    /// Normal terminal sessions are not chat sessions.
    #[test]
    fn normal_terminal_is_not_chat() {
        let session = NexusSession::new("now", "Terminal", "terminal-1", "/tmp/project");
        assert!(!is_chat_session(&session));
    }

    /// Nexus session ids are chat sessions.
    #[test]
    fn nexus_session_is_chat() {
        let session = NexusSession::new("now", "Chat", "session-1", "/tmp/project");
        assert!(is_chat_session(&session));
    }
}
