use std::path::Path;

use crate::extensions::terminal::session::session_terminal::SessionTerminal;

/// Counts sessions that belong to one workspace folder.
pub fn session_count_for_workspace(path: &Path, sessions: &[SessionTerminal]) -> usize {
    sessions
        .iter()
        .filter(|entry| entry.session.working_dir == path)
        .count()
}
