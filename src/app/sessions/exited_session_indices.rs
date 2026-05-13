use crate::extensions::terminal::process::session_terminal_has_exited::session_terminal_has_exited;
use crate::extensions::terminal::session::session_terminal::SessionTerminal;

/// Returns indexes for entries whose backing terminal process has exited.
pub fn exited_session_indices(sessions: &mut [SessionTerminal]) -> Vec<usize> {
    sessions
        .iter_mut()
        .enumerate()
        .filter_map(|(index, entry)| session_terminal_has_exited(entry).then_some(index))
        .collect()
}
