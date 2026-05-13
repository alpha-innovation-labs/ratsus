use std::path::Path;

use anyhow::Result;

use crate::extensions::terminal::process::default_shell_command::default_shell_command;
use crate::extensions::terminal::session::chat_terminal::ChatTerminal;
use crate::extensions::terminal::session::normal_terminal_session_info::normal_terminal_session_info;
use crate::extensions::terminal::session::session_terminal::SessionTerminal;

/// Spawns a normal shell terminal session in the provided working directory.
pub fn spawn_normal_terminal_session(
    working_dir: &Path,
    rows: u16,
    cols: u16,
) -> Result<SessionTerminal> {
    let shell = default_shell_command();
    let terminal = ChatTerminal::spawn_with_command_in_dir(&shell, &[], working_dir, rows, cols)?;
    Ok(SessionTerminal::with_terminal(
        normal_terminal_session_info(working_dir),
        terminal,
    ))
}
