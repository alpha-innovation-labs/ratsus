use chrono::DateTime;

use crate::harness::chat_session::ChatSession;

/// Returns a comparable creation timestamp for ordering sessions by creation date.
pub fn session_created_timestamp(session: &ChatSession) -> i64 {
    if session.created_at == "now" {
        return i64::MAX;
    }
    DateTime::parse_from_rfc3339(&session.created_at)
        .map(|created| created.timestamp())
        .unwrap_or(i64::MIN)
}

#[cfg(test)]
mod tests {
    use super::session_created_timestamp;
    use crate::harness::chat_session::ChatSession;

    /// Builds a timestamp test session.
    fn session(created_at: &str) -> ChatSession {
        ChatSession::new_with_created("updated", created_at, "Title", created_at, "/tmp/project")
    }

    /// Verifies newer creation timestamps compare greater than older timestamps.
    #[test]
    fn newer_creation_timestamp_is_greater() {
        assert!(
            session_created_timestamp(&session("2026-05-12T10:00:00Z"))
                > session_created_timestamp(&session("2026-05-11T10:00:00Z"))
        );
    }

    /// Verifies live placeholder sessions sort first.
    #[test]
    fn now_sorts_first() {
        assert_eq!(session_created_timestamp(&session("now")), i64::MAX);
    }
}
