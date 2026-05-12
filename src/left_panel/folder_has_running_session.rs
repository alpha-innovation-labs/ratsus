use std::path::Path;

use crate::terminal::session_terminal::SessionTerminal;

/// Returns true when a folder contains at least one running session.
pub fn folder_has_running_session(folder: &Path, session_terminals: &[SessionTerminal]) -> bool {
    session_terminals
        .iter()
        .any(|entry| entry.session.working_dir == folder && entry.session.is_running)
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::left_panel::folder_has_running_session::folder_has_running_session;
    use crate::nexus_sessions::session_info::NexusSession;
    use crate::terminal::session_terminal::SessionTerminal;

    /// Verifies folders report running state when any child session is running.
    #[test]
    fn returns_true_when_folder_contains_running_session() {
        let entries = vec![SessionTerminal::dormant(
            NexusSession::new("now", "Chat", "id", "/tmp/project").with_running(true),
        )];

        assert!(folder_has_running_session(
            Path::new("/tmp/project"),
            &entries
        ));
    }

    /// Verifies folders without running child sessions report idle state.
    #[test]
    fn returns_false_when_folder_has_no_running_session() {
        let entries = vec![SessionTerminal::dormant(NexusSession::new(
            "now",
            "Chat",
            "id",
            "/tmp/project",
        ))];

        assert!(!folder_has_running_session(
            Path::new("/tmp/project"),
            &entries
        ));
    }
}
