/// Decides whether the active PTY should be resized on this frame.
pub fn should_resize_active_terminal(is_dragging: bool) -> bool {
    !is_dragging
}

#[cfg(test)]
mod tests {
    use super::should_resize_active_terminal;

    /// Verifies non-drag resize is allowed.
    #[test]
    fn allows_non_drag_resize() {
        assert!(should_resize_active_terminal(false));
    }

    /// Verifies drag resize is suppressed until mouse-up.
    #[test]
    fn suppresses_drag_resize() {
        assert!(!should_resize_active_terminal(true));
    }
}
