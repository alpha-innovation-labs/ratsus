/// Returns the session vector index where a new normal terminal should be inserted.
pub fn normal_terminal_insert_index(session_count: usize, focused_index: usize) -> usize {
    focused_index.saturating_add(1).min(session_count)
}

#[cfg(test)]
mod tests {
    use super::normal_terminal_insert_index;

    /// Inserts directly after the focused session when it exists.
    #[test]
    fn inserts_after_focused_session() {
        assert_eq!(normal_terminal_insert_index(5, 2), 3);
    }

    /// Appends when the focused session index is outside the current session vector.
    #[test]
    fn appends_without_valid_focused_session() {
        assert_eq!(normal_terminal_insert_index(5, 99), 5);
    }
}
