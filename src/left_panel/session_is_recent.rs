use chrono::{DateTime, Duration, Utc};

use crate::nexus_sessions::session_info::NexusSession;

/// Returns whether a session was modified within the recent-history window.
pub fn session_is_recent(session: &NexusSession, now: DateTime<Utc>) -> bool {
    if session.date == "now" {
        return true;
    }
    DateTime::parse_from_rfc3339(&session.date)
        .map(|modified| {
            now.signed_duration_since(modified.with_timezone(&Utc)) <= Duration::hours(24)
        })
        .unwrap_or(true)
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, TimeZone, Utc};

    use super::session_is_recent;
    use crate::nexus_sessions::session_info::NexusSession;

    /// Verifies sessions modified within twenty-four hours are visible.
    #[test]
    fn includes_recent_session() {
        let now = Utc.with_ymd_and_hms(2026, 5, 12, 12, 0, 0).unwrap();
        let modified = (now - Duration::hours(23)).to_rfc3339();
        let session = NexusSession::new(modified, "Recent", "id", "/tmp/project");

        assert!(session_is_recent(&session, now));
    }

    /// Verifies sessions older than twenty-four hours are hidden from history.
    #[test]
    fn excludes_old_session() {
        let now = Utc.with_ymd_and_hms(2026, 5, 12, 12, 0, 0).unwrap();
        let modified = (now - Duration::hours(25)).to_rfc3339();
        let session = NexusSession::new(modified, "Old", "id", "/tmp/project");

        assert!(!session_is_recent(&session, now));
    }
}
