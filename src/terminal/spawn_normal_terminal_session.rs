use std::path::Path;

use anyhow::Result;

use crate::copy_mode::terminal_copy_selection::TerminalCopySelection;
use crate::terminal::default_shell_command::default_shell_command;
use crate::terminal::nexus_terminal::NexusTerminal;
use crate::terminal::normal_terminal_session_info::normal_terminal_session_info;
use crate::terminal::session_terminal::SessionTerminal;

/// Spawns a normal shell terminal session in the provided working directory.
pub fn spawn_normal_terminal_session(
    working_dir: &Path,
    rows: u16,
    cols: u16,
) -> Result<SessionTerminal> {
    let shell = default_shell_command();
    let terminal = NexusTerminal::spawn_with_command_in_dir(&shell, &[], working_dir, rows, cols)?;
    Ok(SessionTerminal {
        session: normal_terminal_session_info(working_dir),
        terminal: Some(terminal),
        copy_selection: TerminalCopySelection::default(),
    })
}
