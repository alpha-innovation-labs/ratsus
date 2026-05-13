use std::io;

use crate::terminal::normal_terminal_registry_path::normal_terminal_registry_path;
use crate::terminal::persisted_normal_terminal_session::PersistedNormalTerminalSession;

/// Saves normal terminal session metadata to disk.
pub fn save_normal_terminal_sessions(
    sessions: &[PersistedNormalTerminalSession],
) -> io::Result<()> {
    let Some(path) = normal_terminal_registry_path() else {
        return Ok(());
    };
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let content = serde_json::to_string_pretty(sessions)?;
    std::fs::write(path, content)
}
