/// Keeps the focused row visible inside a list viewport.
pub fn clamp_visible_offset(focused_index: usize, offset: usize, viewport_height: usize) -> usize {
    if viewport_height == 0 || focused_index < offset {
        return focused_index;
    }

    let last_visible = offset + viewport_height.saturating_sub(1);
    if focused_index > last_visible {
        return focused_index + 1 - viewport_height;
    }

    offset
}

#[cfg(test)]
mod tests {
    use super::clamp_visible_offset;

    /// Verifies that focus above the viewport moves the offset upward.
    #[test]
    fn moves_offset_up_to_focused_index() {
        assert_eq!(clamp_visible_offset(2, 5, 4), 2);
    }

    /// Verifies that focus below the viewport moves the offset downward.
    #[test]
    fn moves_offset_down_to_include_focus() {
        assert_eq!(clamp_visible_offset(8, 2, 4), 5);
    }

    /// Verifies that visible focus leaves the offset unchanged.
    #[test]
    fn keeps_offset_when_focus_is_visible() {
        assert_eq!(clamp_visible_offset(4, 2, 4), 2);
    }
}
