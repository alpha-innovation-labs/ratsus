use ratkit::primitives::resizable_grid::ResizableGridWidgetState;

/// Returns true while the resizable grid is actively dragging a divider.
pub fn is_resizing_layout(state: &ResizableGridWidgetState) -> bool {
    state.dragging_divider.is_some()
}

#[cfg(test)]
mod tests {
    use ratkit::primitives::resizable_grid::ResizableGridWidgetState;

    use super::is_resizing_layout;

    /// Verifies resize mode is active only while a divider is being dragged.
    #[test]
    fn detects_active_dragging_divider() {
        assert!(!is_resizing_layout(&ResizableGridWidgetState::default()));
        assert!(is_resizing_layout(&ResizableGridWidgetState {
            hovered_divider: None,
            dragging_divider: Some(0),
        }));
    }
}
