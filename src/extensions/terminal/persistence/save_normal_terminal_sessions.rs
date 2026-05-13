use std::io;

use crate::extensions::terminal::persistence::persisted_normal_terminal_session::PersistedNormalTerminalSession;
use crate::extensions::terminal::persistence::registry_path::normal_terminal_registry_path;

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
