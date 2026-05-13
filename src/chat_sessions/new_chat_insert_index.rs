use std::path::Path;

use crate::terminal::session_terminal::SessionTerminal;

/// Returns the session vector index where a new chat should appear newest-first.
pub fn new_chat_insert_index(sessions: &[SessionTerminal], working_dir: &Path) -> usize {
    sessions
        .iter()
        .position(|entry| entry.session.working_dir == working_dir)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::new_chat_insert_index;
    use crate::test_support::dormant_session::dormant_session;

    /// New chats insert before older chats in the same folder.
    #[test]
    fn inserts_before_existing_folder_sessions() {
        let sessions = vec![
            dormant_session("Old A", "a", "/tmp/a"),
            dormant_session("Old B", "b", "/tmp/b"),
        ];

        assert_eq!(new_chat_insert_index(&sessions, Path::new("/tmp/b")), 1);
    }

    /// New folders insert at the top of the global list.
    #[test]
    fn inserts_new_folder_at_top() {
        let sessions = vec![dormant_session("Old A", "a", "/tmp/a")];

        assert_eq!(new_chat_insert_index(&sessions, Path::new("/tmp/new")), 0);
    }
}
