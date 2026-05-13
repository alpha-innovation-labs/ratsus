use crate::terminal::is_chat_session::is_chat_session;
use crate::terminal::session_terminal::SessionTerminal;

/// Returns the next chat session index at or after a preferred position, wrapping once.
pub fn next_chat_index(sessions: &[SessionTerminal], preferred_index: usize) -> Option<usize> {
    if sessions.is_empty() {
        return None;
    }
    (preferred_index.min(sessions.len())..sessions.len())
        .chain(0..preferred_index.min(sessions.len()))
        .find(|index| is_chat_session(&sessions[*index].session))
}

#[cfg(test)]
mod tests {
    use super::next_chat_index;
    use crate::harness::chat_session::ChatSession;
    use crate::terminal::normal_terminal_session_info::normal_terminal_session_info;
    use crate::terminal::session_terminal::SessionTerminal;

    /// Builds a dormant chat fixture for focus selection tests.
    fn chat(id: &str) -> SessionTerminal {
        SessionTerminal::dormant(ChatSession::new("now", id, id, "/tmp/project"))
    }

    /// Builds a dormant terminal fixture for focus selection tests.
    fn terminal() -> SessionTerminal {
        SessionTerminal::dormant(normal_terminal_session_info("/tmp/project".as_ref()))
    }

    /// Selects the next chat at or after the preferred position.
    #[test]
    fn selects_next_chat_after_preferred_index() {
        let sessions = vec![terminal(), chat("chat-1"), chat("chat-2")];
        assert_eq!(next_chat_index(&sessions, 1), Some(1));
    }

    /// Wraps to the first chat when no later chat exists.
    #[test]
    fn wraps_to_first_chat() {
        let sessions = vec![chat("chat-1"), terminal()];
        assert_eq!(next_chat_index(&sessions, 1), Some(0));
    }

    /// Returns none when only terminals remain.
    #[test]
    fn returns_none_without_chats() {
        let sessions = vec![terminal()];
        assert_eq!(next_chat_index(&sessions, 0), None);
    }
}
