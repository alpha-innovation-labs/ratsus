use std::io;
use std::path::{Path, PathBuf};

use crate::extensions::terminal::session::session_terminal::SessionTerminal;

/// Returns the working directory for a new normal terminal, preferring the selected workspace.
pub fn normal_terminal_working_dir(
    sessions: &[SessionTerminal],
    active_index: usize,
    focused_index: usize,
    selected_workspace_path: Option<&Path>,
) -> io::Result<PathBuf> {
    if let Some(workspace_path) = selected_workspace_path {
        return Ok(workspace_path.to_path_buf());
    }
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

    /// New terminals should open in the selected workspace before active row fallback.
    #[test]
    fn prefers_selected_workspace_folder() {
        let sessions = vec![session("/tmp/active"), session("/tmp/focused")];

        let working_dir = normal_terminal_working_dir(
            &sessions,
            0,
            1,
            Some(std::path::Path::new("/tmp/workspace")),
        )
        .unwrap();

        assert_eq!(working_dir, std::path::PathBuf::from("/tmp/workspace"));
    }

    /// Active row is used only when no selected workspace exists.
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
