use crate::nexus_sessions::session_info::NexusSession;
use crate::terminal::is_normal_terminal_session::is_normal_terminal_session;

/// Icon shown next to a session row in the left panel.
pub fn session_icon(session: &NexusSession) -> &'static str {
    if is_normal_terminal_session(session) {
        return "󰆍";
    }
    "󰀘"
}

#[cfg(test)]
mod tests {
    use super::session_icon;
    use crate::nexus_sessions::session_info::NexusSession;

    /// Chat sessions should use the chat icon.
    #[test]
    fn chat_session_uses_chat_icon() {
        let session = NexusSession::new("now", "Chat", "chat-123", "/tmp/project");
        assert_eq!(session_icon(&session), "󰀘");
    }

    /// Normal terminal sessions should use the terminal icon.
    #[test]
    fn terminal_session_uses_terminal_icon() {
        let session = NexusSession::new("now", "Terminal", "terminal-123", "/tmp/project");
        assert_eq!(session_icon(&session), "󰆍");
    }
}
