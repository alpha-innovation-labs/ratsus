use ratkit::widgets::markdown_preview::ScrollState;

/// Builds a Ratkit markdown scroll state for the left session list viewport.
pub fn left_panel_scroll_state(
    scroll_offset: usize,
    viewport_height: usize,
    total_lines: usize,
) -> ScrollState {
    let clamped_offset = scroll_offset.min(total_lines.saturating_sub(viewport_height));
    ScrollState {
        offset: clamped_offset,
        scroll_offset: clamped_offset,
        viewport_height,
        total_lines,
        current_line: clamped_offset.saturating_add(1),
        filter: None,
        filter_mode: false,
    }
}

#[cfg(test)]
mod tests {
    use super::left_panel_scroll_state;

    /// Verifies the scroll state clamps offset to Ratkit's valid range.
    #[test]
    fn clamps_scroll_offset_to_maximum() {
        let state = left_panel_scroll_state(20, 5, 10);

        assert_eq!(state.offset, 5);
        assert_eq!(state.scroll_offset, 5);
        assert_eq!(state.viewport_height, 5);
        assert_eq!(state.total_lines, 10);
    }
}
