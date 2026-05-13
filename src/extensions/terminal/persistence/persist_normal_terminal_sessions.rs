use crate::extensions::terminal::persistence::save_normal_terminal_sessions::save_normal_terminal_sessions;
use crate::extensions::terminal::session::normal_terminal_sessions_from_session_terminals::normal_terminal_sessions_from_session_terminals;
use crate::extensions::terminal::session::session_terminal::SessionTerminal;

/// Persists current normal terminal metadata and ignores storage failures.
pub fn persist_normal_terminal_sessions(sessions: &[SessionTerminal]) {
    let entries = normal_terminal_sessions_from_session_terminals(sessions);
    let _ = save_normal_terminal_sessions(&entries);
}
