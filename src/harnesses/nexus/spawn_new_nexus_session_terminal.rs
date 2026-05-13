use std::path::Path;

use anyhow::Result;

use crate::harnesses::nexus::new_nexus_chat_session::new_nexus_chat_session;
use crate::terminal::chat_terminal::ChatTerminal;
use crate::terminal::session_terminal::SessionTerminal;

/// Spawns a fresh Nexus chat terminal in the provided working directory.
pub fn spawn_new_nexus_session_terminal(
    working_dir: &Path,
    rows: u16,
    cols: u16,
) -> Result<SessionTerminal> {
    let session = new_nexus_chat_session(working_dir);
    let terminal = ChatTerminal::spawn_with_command_in_dir("nexus", &[], working_dir, rows, cols)?;
    Ok(SessionTerminal::with_terminal(session, terminal))
}
