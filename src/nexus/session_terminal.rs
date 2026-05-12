use anyhow::Result;

use crate::nexus_terminal::NexusTerminal;
use crate::session_info::NexusSession;
use crate::terminal_copy_selection::TerminalCopySelection;

/// A Nexus session paired with its dedicated terminal process.
pub struct SessionTerminal {
    pub session: NexusSession,
    pub terminal: NexusTerminal,
    pub copy_selection: TerminalCopySelection,
}

impl SessionTerminal {
    /// Spawns a terminal that resumes a specific Nexus session.
    pub fn spawn(session: NexusSession, rows: u16, cols: u16) -> Result<Self> {
        let working_dir = session.working_dir.clone();
        let terminal = NexusTerminal::spawn_with_command_in_dir(
            "nexus",
            &["--resume", session.id.as_str()],
            &working_dir,
            rows,
            cols,
        )?;
        Ok(Self {
            session,
            terminal,
            copy_selection: TerminalCopySelection::default(),
        })
    }
}
