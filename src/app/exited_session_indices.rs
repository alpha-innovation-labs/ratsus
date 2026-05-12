use crate::terminal::session_terminal::SessionTerminal;
use crate::terminal::session_terminal_has_exited::session_terminal_has_exited;

/// Returns indexes for entries whose backing terminal process has exited.
pub fn exited_session_indices(sessions: &mut [SessionTerminal]) -> Vec<usize> {
    sessions
        .iter_mut()
        .enumerate()
        .filter_map(|(index, entry)| session_terminal_has_exited(entry).then_some(index))
        .collect()
}
