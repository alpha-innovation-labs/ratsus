use ratkit::primitives::resizable_grid::ResizableGridWidgetState;

use crate::ui::layout::resizable_grid::split_indices::WORKSPACE_SPLIT_INDEX;

/// Returns a resize widget state that ignores hidden workspace-pane dividers.
pub fn visible_layout_widget_state(
    state: ResizableGridWidgetState,
    workspace_view_enabled: bool,
) -> ResizableGridWidgetState {
    if workspace_view_enabled {
        return state;
    }
    ResizableGridWidgetState {
        hovered_divider: visible_divider_index(state.hovered_divider),
        dragging_divider: visible_divider_index(state.dragging_divider),
    }
}

/// Returns a divider index only when it belongs to a visible pane boundary.
fn visible_divider_index(index: Option<usize>) -> Option<usize> {
    index.filter(|divider| *divider != WORKSPACE_SPLIT_INDEX)
}

#[cfg(test)]
mod tests {
    use ratkit::primitives::resizable_grid::ResizableGridWidgetState;

    use super::visible_layout_widget_state;
    use crate::ui::layout::resizable_grid::split_indices::{
        SHELL_SPLIT_INDEX, WORKSPACE_SPLIT_INDEX,
    };

    /// Hidden workspace mode clears hover state for the workspace divider.
    #[test]
    fn clears_hidden_workspace_hover_divider() {
        let state = ResizableGridWidgetState {
            hovered_divider: Some(WORKSPACE_SPLIT_INDEX),
            dragging_divider: None,
        };

        let visible = visible_layout_widget_state(state, false);

        assert_eq!(visible.hovered_divider, None);
    }

    /// Hidden workspace mode keeps the shell divider available for resizing.
    #[test]
    fn keeps_shell_divider_when_workspace_is_hidden() {
        let state = ResizableGridWidgetState {
            hovered_divider: Some(SHELL_SPLIT_INDEX),
            dragging_divider: Some(SHELL_SPLIT_INDEX),
        };

        let visible = visible_layout_widget_state(state, false);

        assert_eq!(visible.hovered_divider, Some(SHELL_SPLIT_INDEX));
        assert_eq!(visible.dragging_divider, Some(SHELL_SPLIT_INDEX));
    }
}
