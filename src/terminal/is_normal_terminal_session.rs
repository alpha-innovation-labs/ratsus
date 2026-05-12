use crate::nexus_sessions::session_info::NexusSession;

/// Returns whether session metadata represents a normal shell terminal.
pub fn is_normal_terminal_session(session: &NexusSession) -> bool {
    session.id.starts_with("terminal-")
}

#[cfg(test)]
mod tests {
    use super::is_normal_terminal_session;
    use crate::nexus_sessions::session_info::NexusSession;

    /// Terminal ids should identify normal shell terminal sessions.
    #[test]
    fn terminal_id_is_normal_terminal() {
        let session = NexusSession::new("now", "Terminal", "terminal-123", "/tmp/project");
        assert!(is_normal_terminal_session(&session));
    }

    /// Nexus chat ids should not identify normal shell terminal sessions.
    #[test]
    fn nexus_chat_id_is_not_normal_terminal() {
        let session = NexusSession::new("now", "Chat", "chat-123", "/tmp/project");
        assert!(!is_normal_terminal_session(&session));
    }
}
