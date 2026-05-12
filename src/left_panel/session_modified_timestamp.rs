use chrono::DateTime;

use crate::nexus_sessions::session_info::NexusSession;

/// Returns a comparable timestamp for ordering sessions by most recent modification first.
pub fn session_modified_timestamp(session: &NexusSession) -> i64 {
    if session.date == "now" {
        return i64::MAX;
    }
    DateTime::parse_from_rfc3339(&session.date)
        .map(|modified| modified.timestamp())
        .unwrap_or(i64::MIN)
}

#[cfg(test)]
mod tests {
    use super::session_modified_timestamp;
    use crate::nexus_sessions::session_info::NexusSession;

    /// Builds a timestamp test session.
    fn session(date: &str) -> NexusSession {
        NexusSession::new(date, "Title", date, "/tmp/project")
    }

    /// Verifies newer RFC3339 timestamps compare greater than older timestamps.
    #[test]
    fn newer_timestamp_is_greater() {
        assert!(
            session_modified_timestamp(&session("2026-05-12T10:00:00Z"))
                > session_modified_timestamp(&session("2026-05-11T10:00:00Z"))
        );
    }

    /// Verifies live placeholder sessions sort first.
    #[test]
    fn now_sorts_first() {
        assert_eq!(session_modified_timestamp(&session("now")), i64::MAX);
    }
}
