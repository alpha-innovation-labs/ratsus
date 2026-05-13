/// Returns a nonzero fallback count for code paths that need viewport cardinality.
pub fn expo_visible_card_count(viewport_height: u16) -> usize {
    usize::from(viewport_height.max(1))
}

#[cfg(test)]
mod tests {
    use super::expo_visible_card_count;

    /// Verifies the fallback count never reaches zero.
    #[test]
    fn count_is_nonzero() {
        assert_eq!(expo_visible_card_count(0), 1);
    }
}
