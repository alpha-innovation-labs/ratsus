use std::collections::HashMap;

use crate::nexus_sessions::is_new_nexus_chat_session::is_new_nexus_chat_session;
use crate::nexus_sessions::merge_registry_session_metadata::merge_registry_session_metadata;
use crate::nexus_sessions::session_info::NexusSession;
use crate::terminal::session_terminal::SessionTerminal;

/// Applies refreshed Nexus session metadata while preserving live terminal processes.
pub fn apply_session_refresh(
    session_terminals: &mut [SessionTerminal],
    refreshed_sessions: Vec<NexusSession>,
) -> bool {
    let running_by_id = running_status_by_id(&refreshed_sessions);
    let mut changed = apply_running_status(session_terminals, &running_by_id);
    for refreshed_session in refreshed_sessions {
        changed |= apply_refreshed_session(session_terminals, refreshed_session);
    }
    changed
}

/// Builds running status lookup values from refreshed registry metadata.
fn running_status_by_id(refreshed_sessions: &[NexusSession]) -> HashMap<String, bool> {
    refreshed_sessions
        .iter()
        .map(|session| (session.id.clone(), session.is_running))
        .collect()
}

/// Applies active process state from the registry to every known session.
fn apply_running_status(
    session_terminals: &mut [SessionTerminal],
    running_by_id: &HashMap<String, bool>,
) -> bool {
    let mut changed = false;
    for entry in session_terminals {
        let is_running = running_by_id
            .get(&entry.session.id)
            .copied()
            .unwrap_or(false);
        if entry.session.is_running != is_running {
            entry.session.is_running = is_running;
            changed = true;
        }
    }
    changed
}

/// Applies one refreshed registry session by id or local new-chat placeholder match.
fn apply_refreshed_session(
    session_terminals: &mut [SessionTerminal],
    refreshed_session: NexusSession,
) -> bool {
    if let Some(index) = session_index_by_id(session_terminals, &refreshed_session.id) {
        let merged =
            merge_registry_session_metadata(&session_terminals[index].session, refreshed_session);
        return replace_session_metadata(session_terminals, index, merged);
    }
    if let Some(index) = new_chat_placeholder_index(session_terminals, &refreshed_session) {
        return replace_session_metadata(session_terminals, index, refreshed_session);
    }
    false
}

/// Finds an existing session entry by stable Nexus session id.
fn session_index_by_id(session_terminals: &[SessionTerminal], session_id: &str) -> Option<usize> {
    session_terminals
        .iter()
        .position(|entry| entry.session.id == session_id)
}

/// Finds a local new-chat placeholder that likely became the refreshed Nexus session.
fn new_chat_placeholder_index(
    session_terminals: &[SessionTerminal],
    refreshed_session: &NexusSession,
) -> Option<usize> {
    session_terminals.iter().position(|entry| {
        is_new_nexus_chat_session(&entry.session)
            && entry.session.working_dir == refreshed_session.working_dir
    })
}

/// Replaces metadata for one entry and reports whether anything changed.
fn replace_session_metadata(
    session_terminals: &mut [SessionTerminal],
    index: usize,
    refreshed_session: NexusSession,
) -> bool {
    if session_terminals[index].session == refreshed_session {
        return false;
    }
    session_terminals[index].session = refreshed_session;
    true
}
