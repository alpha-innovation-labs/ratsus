/// Clamps a session index to an available item after list removals.
pub fn clamp_session_index(index: usize, session_count: usize) -> usize {
    if session_count == 0 {
        return 0;
    }
    index.min(session_count - 1)
}

#[cfg(test)]
mod tests {
    use super::clamp_session_index;

    /// Empty lists always clamp to zero.
    #[test]
    fn empty_list_clamps_to_zero() {
        assert_eq!(clamp_session_index(3, 0), 0);
    }

    /// Indexes beyond the last session clamp to the last item.
    #[test]
    fn clamps_to_last_session() {
        assert_eq!(clamp_session_index(3, 2), 1);
    }
}
