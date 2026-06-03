use std::path::Path;

use anyhow::Result;

use crate::extensions::terminal::process::default_shell_command::default_shell_command;
use crate::extensions::terminal::process::normal_terminal_shell_argv::normal_terminal_shell_argv;
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
    let argv = normal_terminal_shell_argv(&shell, working_dir);
    let terminal = ChatTerminal::spawn_argv_in_dir(argv, working_dir, rows, cols)?;
    Ok(SessionTerminal::with_terminal(
        normal_terminal_session_info(working_dir),
        terminal,
    ))
}
