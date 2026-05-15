use std::io;

use crate::extensions::terminal::persistence::persisted_normal_terminal_session::PersistedNormalTerminalSession;
use crate::extensions::terminal::persistence::registry_path::normal_terminal_registry_path;
use crate::shared::async_persistence::save_job::SaveJob;
use crate::shared::async_persistence::worker::enqueue_save;

/// Saves normal terminal session metadata to disk.
pub fn save_normal_terminal_sessions(
    sessions: &[PersistedNormalTerminalSession],
) -> io::Result<()> {
    let Some(path) = normal_terminal_registry_path() else {
        return Ok(());
    };
    let content = serde_json::to_string_pretty(sessions)?;
    enqueue_save(SaveJob {
        key: "normal-terminal-sessions",
        path,
        content,
    });
    Ok(())
}
