/// Returns the first visible result index for a selected row and viewport height.
pub fn history_modal_start_index(
    selected_position: usize,
    item_count: usize,
    height: usize,
) -> usize {
    if item_count <= height || height == 0 {
        return 0;
    }
    selected_position
        .saturating_sub(height / 2)
        .min(item_count.saturating_sub(height))
}

#[cfg(test)]
mod tests {
    use super::history_modal_start_index;

    /// Selected rows near the top should start at zero.
    #[test]
    fn starts_at_zero_near_top() {
        assert_eq!(history_modal_start_index(1, 10, 5), 0);
    }

    /// Selected rows near the bottom should clamp to the last full page.
    #[test]
    fn clamps_to_last_full_page() {
        assert_eq!(history_modal_start_index(9, 10, 5), 5);
    }
}
