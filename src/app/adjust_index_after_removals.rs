/// Returns an index adjusted after sorted removals, or none when the original item was removed.
pub fn adjust_index_after_removals(index: usize, removed_indices: &[usize]) -> Option<usize> {
    if removed_indices.binary_search(&index).is_ok() {
        return None;
    }
    let removed_before = removed_indices
        .iter()
        .take_while(|removed_index| **removed_index < index)
        .count();
    Some(index.saturating_sub(removed_before))
}

#[cfg(test)]
mod tests {
    use super::adjust_index_after_removals;

    /// Removed indexes should return no adjusted position.
    #[test]
    fn returns_none_for_removed_index() {
        assert_eq!(adjust_index_after_removals(2, &[1, 2]), None);
    }

    /// Indexes after removals shift left by the number of earlier removals.
    #[test]
    fn shifts_index_left_after_prior_removals() {
        assert_eq!(adjust_index_after_removals(4, &[1, 3]), Some(2));
    }
}
