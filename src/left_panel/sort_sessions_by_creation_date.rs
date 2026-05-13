use std::cmp::Reverse;

use crate::left_panel::session_created_timestamp::session_created_timestamp;
use crate::terminal::session_terminal::SessionTerminal;

/// Sorts session entries from newest to oldest creation date during initial loading.
pub fn sort_sessions_by_creation_date(entries: &mut [SessionTerminal]) {
    entries.sort_by_key(|entry| Reverse(session_created_timestamp(&entry.session)));
}

#[cfg(test)]
mod tests {
    use crate::left_panel::sort_sessions_by_creation_date::sort_sessions_by_creation_date;
    use crate::nexus_sessions::session_info::NexusSession;
    use crate::terminal::session_terminal::SessionTerminal;

    /// Builds a dormant session entry with separate created and modified dates.
    fn session_entry(created_at: &str, modified_at: &str, id: &str) -> SessionTerminal {
        SessionTerminal::dormant(NexusSession::new_with_created(
            modified_at,
            created_at,
            id,
            id,
            "/tmp/project",
        ))
    }

    /// Verifies initial session sorting uses creation date instead of modification date.
    #[test]
    fn sorts_sessions_newest_creation_first() {
        let mut entries = vec![
            session_entry(
                "2026-05-11T10:00:00Z",
                "2026-05-13T10:00:00Z",
                "older-created",
            ),
            session_entry(
                "2026-05-12T10:00:00Z",
                "2026-05-12T10:00:00Z",
                "newer-created",
            ),
        ];

        sort_sessions_by_creation_date(&mut entries);

        assert_eq!(entries[0].session.id, "newer-created");
        assert_eq!(entries[1].session.id, "older-created");
    }
}
