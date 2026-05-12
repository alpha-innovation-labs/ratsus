use crate::terminal::session_terminal::SessionTerminal;

/// Returns whether a session entry has a terminal process that has exited.
pub fn session_terminal_has_exited(entry: &mut SessionTerminal) -> bool {
    entry
        .terminal
        .as_mut()
        .is_some_and(|terminal| terminal.has_exited())
}
