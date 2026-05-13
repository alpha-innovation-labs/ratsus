use crate::extensions::harness::core::chat_session::ChatSession;

/// Returns whether a chat session title should be shown for the current picker query.
pub fn matches_conversation_query(session: &ChatSession, query: &str) -> bool {
    let query = query.trim().to_lowercase();
    query.is_empty() || session.title.to_lowercase().contains(&query)
}

#[cfg(test)]
mod tests {
    use super::matches_conversation_query;
    use crate::extensions::harness::core::chat_session::ChatSession;

    /// Builds a reusable session fixture for filter tests.
    fn session() -> ChatSession {
        ChatSession::new("2026-01-01", "Build Rust modal", "abc123", "/tmp/project")
    }

    /// Empty queries should include every conversation.
    #[test]
    fn empty_query_matches() {
        assert!(matches_conversation_query(&session(), ""));
    }

    /// Title filtering should be case-insensitive.
    #[test]
    fn title_query_matches_case_insensitively() {
        assert!(matches_conversation_query(&session(), "rust"));
    }

    /// Id filtering should not match because picker search is title-only.
    #[test]
    fn id_query_does_not_match() {
        assert!(!matches_conversation_query(&session(), "abc123"));
    }

    /// Working directory filtering should not match because picker search is title-only.
    #[test]
    fn path_query_does_not_match() {
        assert!(!matches_conversation_query(&session(), "project"));
    }

    /// Non-contiguous fuzzy queries should not match unrelated title text.
    #[test]
    fn fuzzy_query_does_not_match() {
        assert!(!matches_conversation_query(&session(), "brm"));
    }

    /// Unrelated queries should exclude the conversation.
    #[test]
    fn unrelated_query_does_not_match() {
        assert!(!matches_conversation_query(&session(), "python"));
    }
}
