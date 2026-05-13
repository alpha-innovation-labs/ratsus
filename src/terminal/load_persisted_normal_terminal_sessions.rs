use std::path::Path;

use crate::terminal::persisted_normal_terminal_session::PersistedNormalTerminalSession;

/// Loads persisted normal terminal session metadata from one registry file.
pub fn load_persisted_normal_terminal_sessions(
    path: &Path,
) -> Option<Vec<PersistedNormalTerminalSession>> {
    let content = std::fs::read_to_string(path).ok()?;
    serde_json::from_str::<Vec<PersistedNormalTerminalSession>>(&content).ok()
}
