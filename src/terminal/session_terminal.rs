use anyhow::Result;

use crate::copy_mode::terminal_copy_selection::TerminalCopySelection;
use crate::nexus_sessions::session_info::NexusSession;
use crate::terminal::default_shell_command::default_shell_command;
use crate::terminal::is_normal_terminal_session::is_normal_terminal_session;
use crate::terminal::nexus_terminal::NexusTerminal;

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
            self.terminal = Some(spawn_terminal_for_session(&self.session, rows, cols)?);
        }
        Ok(self.terminal.as_mut().expect("terminal is initialized"))
    }
}

/// Spawns the correct PTY command for a chat or normal terminal session.
fn spawn_terminal_for_session(
    session: &NexusSession,
    rows: u16,
    cols: u16,
) -> Result<NexusTerminal> {
    let working_dir = session.working_dir.clone();
    if is_normal_terminal_session(session) {
        let shell = default_shell_command();
        return NexusTerminal::spawn_with_command_in_dir(
            &shell,
            &[],
            &working_dir,
            rows.max(1),
            cols.max(1),
        );
    }
    NexusTerminal::spawn_with_command_in_dir(
        "nexus",
        &["--resume", session.id.as_str()],
        &working_dir,
        rows.max(1),
        cols.max(1),
    )
}
