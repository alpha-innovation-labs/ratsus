use crate::harness::chat_session::ChatSession;
use crate::terminal::chat_session_from_persisted_normal_terminal_session::chat_session_from_persisted_normal_terminal_session;
use crate::terminal::load_persisted_normal_terminal_sessions::load_persisted_normal_terminal_sessions;
use crate::terminal::normal_terminal_legacy_registry_path::normal_terminal_legacy_registry_path;
use crate::terminal::normal_terminal_registry_path::normal_terminal_registry_path;
use crate::terminal::persisted_normal_terminal_session::PersistedNormalTerminalSession;
use crate::terminal::save_normal_terminal_sessions::save_normal_terminal_sessions;

/// Loads persisted normal terminal sessions from the Ratsus data directory.
pub fn load_normal_terminal_sessions() -> Vec<ChatSession> {
    load_normal_terminal_entries()
        .into_iter()
        .map(chat_session_from_persisted_normal_terminal_session)
        .collect()
}

/// Loads normal terminal entries, migrating the previous Ratsus config path when needed.
fn load_normal_terminal_entries() -> Vec<PersistedNormalTerminalSession> {
    if let Some(entries) = normal_terminal_registry_path()
        .as_deref()
        .and_then(load_persisted_normal_terminal_sessions)
    {
        return entries;
    }
    let Some(entries) = normal_terminal_legacy_registry_path()
        .as_deref()
        .and_then(load_persisted_normal_terminal_sessions)
    else {
        return Vec::new();
    };
    let _ = save_normal_terminal_sessions(&entries);
    entries
}
