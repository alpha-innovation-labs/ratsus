use std::path::Path;

use crate::terminal::session_terminal::SessionTerminal;

/// Returns indexes for conversations that belong to the selected folder.
pub fn folder_conversation_indices(sessions: &[SessionTerminal], folder: &Path) -> Vec<usize> {
    sessions
        .iter()
        .enumerate()
        .filter_map(|(index, entry)| (entry.session.working_dir == folder).then_some(index))
        .collect()
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::folder_conversation_indices;
    use crate::harness::chat_session::ChatSession;
    use crate::terminal::session_terminal::SessionTerminal;

    /// Verifies that only sessions from the selected folder are included.
    #[test]
    fn filters_sessions_by_folder() {
        let sessions = vec![
            SessionTerminal::dormant(ChatSession::new("now", "A", "a", "/tmp/a")),
            SessionTerminal::dormant(ChatSession::new("now", "B", "b", "/tmp/b")),
            SessionTerminal::dormant(ChatSession::new("now", "C", "c", "/tmp/a")),
        ];

        assert_eq!(
            folder_conversation_indices(&sessions, Path::new("/tmp/a")),
            vec![0, 2]
        );
    }
}
