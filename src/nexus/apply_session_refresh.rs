use crate::is_new_nexus_chat_session::is_new_nexus_chat_session;
use crate::session_info::NexusSession;
use crate::session_terminal::SessionTerminal;

/// Applies refreshed Nexus session metadata while preserving live terminal processes.
pub fn apply_session_refresh(
    session_terminals: &mut Vec<SessionTerminal>,
    refreshed_sessions: Vec<NexusSession>,
) -> bool {
    let mut changed = false;
    for refreshed_session in refreshed_sessions {
        changed |= apply_refreshed_session(session_terminals, refreshed_session);
    }
    changed
}

/// Applies one refreshed session by id, placeholder match, or append.
fn apply_refreshed_session(
    session_terminals: &mut Vec<SessionTerminal>,
    refreshed_session: NexusSession,
) -> bool {
    if let Some(index) = session_index_by_id(session_terminals, &refreshed_session.id) {
        return replace_session_metadata(session_terminals, index, refreshed_session);
    }
    if let Some(index) = new_chat_placeholder_index(session_terminals, &refreshed_session) {
        return replace_session_metadata(session_terminals, index, refreshed_session);
    }
    session_terminals.push(SessionTerminal::dormant(refreshed_session));
    true
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

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::apply_session_refresh;
    use crate::new_nexus_chat_session::new_nexus_chat_session;
    use crate::session_info::NexusSession;
    use crate::session_terminal::SessionTerminal;

    /// Verifies refreshed metadata updates a session by stable id.
    #[test]
    fn updates_existing_session_title_by_id() {
        let mut entries = vec![SessionTerminal::dormant(NexusSession::new(
            "old-date",
            "Old title",
            "session-1",
            "/tmp/project",
        ))];

        let changed = apply_session_refresh(
            &mut entries,
            vec![NexusSession::new(
                "new-date",
                "Current title",
                "session-1",
                "/tmp/project",
            )],
        );

        assert!(changed);
        assert_eq!(entries[0].session.title, "Current title");
        assert_eq!(entries[0].session.date, "new-date");
    }

    /// Verifies a locally spawned new chat adopts its real Nexus id and title.
    #[test]
    fn replaces_new_chat_placeholder_by_working_dir() {
        let working_dir = PathBuf::from("/tmp/project");
        let mut entries = vec![SessionTerminal::dormant(new_nexus_chat_session(
            &working_dir,
        ))];

        let changed = apply_session_refresh(
            &mut entries,
            vec![NexusSession::new(
                "new-date",
                "Generated title",
                "real-session-id",
                &working_dir,
            )],
        );

        assert!(changed);
        assert_eq!(entries.len(), 1);
        assert_eq!(entries[0].session.id, "real-session-id");
        assert_eq!(entries[0].session.title, "Generated title");
    }

    /// Verifies unseen refreshed sessions are appended without disturbing existing indexes.
    #[test]
    fn appends_unseen_refreshed_session() {
        let mut entries = vec![SessionTerminal::dormant(NexusSession::new(
            "date-1",
            "Existing",
            "session-1",
            "/tmp/project",
        ))];

        let changed = apply_session_refresh(
            &mut entries,
            vec![NexusSession::new(
                "date-2",
                "New session",
                "session-2",
                "/tmp/project",
            )],
        );

        assert!(changed);
        assert_eq!(entries.len(), 2);
        assert_eq!(entries[0].session.id, "session-1");
        assert_eq!(entries[1].session.id, "session-2");
    }
}
