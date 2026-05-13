use crate::extensions::terminal::persistence::persisted_from_chat_session::persisted_normal_terminal_session_from_chat_session;
use crate::extensions::terminal::persistence::persisted_normal_terminal_session::PersistedNormalTerminalSession;
use crate::extensions::terminal::session::is_normal_terminal_session::is_normal_terminal_session;
use crate::extensions::terminal::session::session_terminal::SessionTerminal;

/// Extracts persistable normal terminal metadata from current session entries.
pub fn normal_terminal_sessions_from_session_terminals(
    sessions: &[SessionTerminal],
) -> Vec<PersistedNormalTerminalSession> {
    sessions
        .iter()
        .filter(|entry| is_normal_terminal_session(&entry.session))
        .map(|entry| persisted_normal_terminal_session_from_chat_session(&entry.session))
        .collect()
}
