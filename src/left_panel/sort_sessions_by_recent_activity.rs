use std::cmp::Reverse;

use crate::left_panel::session_modified_timestamp::session_modified_timestamp;
use crate::terminal::session_terminal::SessionTerminal;

/// Sorts session entries from newest to oldest once during initial loading.
pub fn sort_sessions_by_recent_activity(entries: &mut [SessionTerminal]) {
    entries.sort_by_key(|entry| Reverse(session_modified_timestamp(&entry.session)));
}

#[cfg(test)]
mod tests {
    use crate::left_panel::sort_sessions_by_recent_activity::sort_sessions_by_recent_activity;
    use crate::nexus_sessions::session_info::NexusSession;
    use crate::terminal::session_terminal::SessionTerminal;

    /// Builds a dormant session entry with a fixed modified date.
    fn session_entry(date: &str, id: &str) -> SessionTerminal {
        SessionTerminal::dormant(NexusSession::new(date, id, id, "/tmp/project"))
    }

    /// Verifies initial session sorting places the newest conversation first.
    #[test]
    fn sorts_sessions_newest_first() {
        let mut entries = vec![
            session_entry("2026-05-11T10:00:00Z", "older"),
            session_entry("2026-05-12T10:00:00Z", "newer"),
        ];

        sort_sessions_by_recent_activity(&mut entries);

        assert_eq!(entries[0].session.id, "newer");
        assert_eq!(entries[1].session.id, "older");
    }
}
