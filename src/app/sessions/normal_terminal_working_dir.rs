use std::io;
use std::path::PathBuf;

use crate::extensions::terminal::session::session_terminal::SessionTerminal;

/// Returns the working directory for a new normal terminal, preferring the active session folder.
pub fn normal_terminal_working_dir(
    sessions: &[SessionTerminal],
    active_index: usize,
    focused_index: usize,
    _selected_workspace_path: Option<&std::path::Path>,
) -> io::Result<PathBuf> {
    if let Some(entry) = sessions.get(active_index) {
        return Ok(entry.session.working_dir.clone());
    }
    if let Some(entry) = sessions.get(focused_index) {
        return Ok(entry.session.working_dir.clone());
    }
    std::env::current_dir()
}

#[cfg(test)]
mod tests {
    use super::normal_terminal_working_dir;
    use crate::extensions::harness::core::chat_session::ChatSession;
    use crate::extensions::terminal::session::session_terminal::SessionTerminal;

    /// Builds a dormant session fixture with a specific working directory.
    fn session(path: &str) -> SessionTerminal {
        SessionTerminal::dormant(ChatSession::new("now", path, path, path))
    }

    /// Active row is used when no selected workspace exists.
    #[test]
    fn falls_back_to_active_session_folder() {
        let sessions = vec![session("/tmp/active"), session("/tmp/focused")];

        let working_dir = normal_terminal_working_dir(&sessions, 0, 1, None).unwrap();

        assert_eq!(working_dir, std::path::PathBuf::from("/tmp/active"));
    }

    /// Focused row is used only when active index is unavailable.
    #[test]
    fn falls_back_to_focused_session_folder() {
        let sessions = vec![session("/tmp/focused")];

        let working_dir = normal_terminal_working_dir(&sessions, 99, 0, None).unwrap();

        assert_eq!(working_dir, std::path::PathBuf::from("/tmp/focused"));
    }
}
