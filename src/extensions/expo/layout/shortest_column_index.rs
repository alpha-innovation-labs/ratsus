/// Returns the index of the shortest Expo masonry column.
pub fn shortest_expo_column_index(heights: &[usize]) -> usize {
    heights
        .iter()
        .enumerate()
        .min_by_key(|(_, height)| **height)
        .map(|(index, _)| index)
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::shortest_expo_column_index;

    /// Verifies masonry placement picks the current shortest column.
    #[test]
    fn finds_shortest_column() {
        assert_eq!(shortest_expo_column_index(&[8, 3, 5]), 1);
    }
}
