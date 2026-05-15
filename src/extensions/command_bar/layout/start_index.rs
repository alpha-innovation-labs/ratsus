/// Returns the first visible command row needed to keep selection on screen.
pub fn command_bar_start_index(
    selected_position: usize,
    item_count: usize,
    height: usize,
) -> usize {
    if height == 0 || item_count <= height || selected_position < height {
        return 0;
    }
    selected_position.saturating_add(1).saturating_sub(height)
}

#[cfg(test)]
mod tests {
    use super::command_bar_start_index;

    /// Verifies the visible window scrolls after selection leaves the first page.
    #[test]
    fn scrolls_to_keep_selection_visible() {
        assert_eq!(command_bar_start_index(6, 10, 5), 2);
    }
}
