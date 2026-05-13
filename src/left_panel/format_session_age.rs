use chrono::{DateTime, Utc};

use crate::harness::chat_session::ChatSession;

/// Formats how long ago a session was modified for compact left-panel display.
pub fn format_session_age(session: &ChatSession, now: DateTime<Utc>) -> String {
    if session.date == "now" {
        return "0m".to_string();
    }
    let Ok(modified) = DateTime::parse_from_rfc3339(&session.date) else {
        return String::new();
    };
    let elapsed_seconds = now
        .signed_duration_since(modified.with_timezone(&Utc))
        .num_seconds()
        .max(0);
    format_elapsed_seconds(elapsed_seconds)
}

/// Formats elapsed seconds using minutes, hours, days, weeks, or months.
fn format_elapsed_seconds(seconds: i64) -> String {
    let minutes = seconds / 60;
    if minutes < 60 {
        return format!("{}m", minutes);
    }
    let hours = minutes / 60;
    if hours < 24 {
        return format!("{}h", hours);
    }
    let days = hours / 24;
    if days < 7 {
        return format!("{}d", days);
    }
    let weeks = days / 7;
    if weeks < 4 {
        return format!("{}w", weeks);
    }
    format!("{}M", days / 30)
}

#[cfg(test)]
mod tests {
    use chrono::{Duration, TimeZone, Utc};

    use super::format_session_age;
    use crate::harness::chat_session::ChatSession;

    /// Builds a session whose modified time is relative to a fixed clock.
    fn session_with_age(duration: Duration) -> (ChatSession, chrono::DateTime<Utc>) {
        let now = Utc.with_ymd_and_hms(2026, 5, 12, 12, 0, 0).unwrap();
        let modified = (now - duration).to_rfc3339();
        (
            ChatSession::new(modified, "Title", "id", "/tmp/project"),
            now,
        )
    }

    /// Verifies minute display.
    #[test]
    fn formats_minutes() {
        let (session, now) = session_with_age(Duration::minutes(3));

        assert_eq!(format_session_age(&session, now), "3m");
    }

    /// Verifies hour display.
    #[test]
    fn formats_hours() {
        let (session, now) = session_with_age(Duration::hours(2));

        assert_eq!(format_session_age(&session, now), "2h");
    }

    /// Verifies day display.
    #[test]
    fn formats_days() {
        let (session, now) = session_with_age(Duration::days(3));

        assert_eq!(format_session_age(&session, now), "3d");
    }

    /// Verifies week display.
    #[test]
    fn formats_weeks() {
        let (session, now) = session_with_age(Duration::weeks(3));

        assert_eq!(format_session_age(&session, now), "3w");
    }

    /// Verifies month display.
    #[test]
    fn formats_months() {
        let (session, now) = session_with_age(Duration::days(90));

        assert_eq!(format_session_age(&session, now), "3M");
    }
}
