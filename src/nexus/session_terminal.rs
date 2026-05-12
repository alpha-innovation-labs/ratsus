use anyhow::Result;

use crate::nexus_terminal::NexusTerminal;
use crate::session_info::NexusSession;
use crate::terminal_copy_selection::TerminalCopySelection;

/// A Nexus session paired with its lazily spawned terminal process.
pub struct SessionTerminal {
    pub session: NexusSession,
    pub terminal: Option<NexusTerminal>,
    pub copy_selection: TerminalCopySelection,
}

impl SessionTerminal {
    /// Creates session state without starting the backing PTY.
    pub fn dormant(session: NexusSession) -> Self {
        Self {
            session,
            terminal: None,
            copy_selection: TerminalCopySelection::default(),
        }
    }

    /// Starts the backing PTY when it has not been started yet.
    pub fn ensure_terminal(&mut self, rows: u16, cols: u16) -> Result<&mut NexusTerminal> {
        if self.terminal.is_none() {
            let working_dir = self.session.working_dir.clone();
            self.terminal = Some(NexusTerminal::spawn_with_command_in_dir(
                "nexus",
                &["--resume", self.session.id.as_str()],
                &working_dir,
                rows.max(1),
                cols.max(1),
            )?);
        }
        Ok(self.terminal.as_mut().expect("terminal is initialized"))
    }
}
