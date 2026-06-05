use chrono::{DateTime, Utc};

use crate::extensions::harness::core::chat_session::ChatSession;

/// Returns the day-based group label used for left-panel session separators.
pub fn session_day_group_label(session: &ChatSession, now: DateTime<Utc>) -> Option<String> {
    if session.date == "now" {
        return Some("today".to_string());
    }
    let modified = DateTime::parse_from_rfc3339(&session.date).ok()?;
    let elapsed_days = now
        .signed_duration_since(modified.with_timezone(&Utc))
        .num_days()
        .max(0);
    if elapsed_days == 0 {
        Some("today".to_string())
    } else {
        Some(format!("{elapsed_days}d"))
    }
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, TimeZone, Utc};

    use super::session_day_group_label;
    use crate::extensions::harness::core::chat_session::ChatSession;

    /// Sessions modified in the last day are grouped under today.
    #[test]
    fn labels_recent_sessions_as_today() {
        let now = Utc.with_ymd_and_hms(2026, 5, 12, 12, 0, 0).unwrap();
        let session = ChatSession::new(
            (now - Duration::hours(2)).to_rfc3339(),
            "Title",
            "id",
            "/tmp",
        );

        assert_eq!(
            session_day_group_label(&session, now).as_deref(),
            Some("today")
        );
    }

    /// Older sessions are grouped by elapsed days.
    #[test]
    fn labels_older_sessions_by_day_count() {
        let now = Utc.with_ymd_and_hms(2026, 5, 12, 12, 0, 0).unwrap();
        let session = ChatSession::new(
            (now - Duration::days(2)).to_rfc3339(),
            "Title",
            "id",
            "/tmp",
        );

        assert_eq!(
            session_day_group_label(&session, now).as_deref(),
            Some("2d")
        );
    }
}
