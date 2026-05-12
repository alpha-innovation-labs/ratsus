use crossterm::event::{MouseButton, MouseEventKind};
use ratkit::primitives::resizable_grid::ResizableGridWidget;

use crate::app::nexus_demo_state::NexusDemo;

/// Updates the resizable grid from mouse input and reports whether it consumed the event.
pub fn handle_resizable_grid_mouse(app: &mut NexusDemo, mouse: ratkit::MouseEvent) -> bool {
    if !app.left_pane_visible {
        return false;
    }
    let was_dragging = app.layout_widget_state.dragging_divider.is_some();
    let crossterm_mouse = crossterm::event::MouseEvent {
        kind: mouse.kind,
        column: mouse.column,
        row: mouse.row,
        modifiers: mouse.modifiers,
    };

    let mut widget = ResizableGridWidget::new(app.layout.clone())
        .with_state(app.layout_widget_state)
        .with_pane_borders(false);
    widget.handle_mouse(crossterm_mouse, app.last_layout_area);
    app.layout_widget_state = widget.state();
    app.layout = widget.layout().clone();

    consumed_resizable_grid_mouse_event(
        was_dragging,
        app.layout_widget_state.dragging_divider.is_some(),
        mouse.kind,
    )
}

/// Determines whether a mouse event belongs to divider resizing.
fn consumed_resizable_grid_mouse_event(
    was_dragging: bool,
    is_dragging: bool,
    kind: MouseEventKind,
) -> bool {
    match kind {
        MouseEventKind::Down(MouseButton::Left) => is_dragging,
        MouseEventKind::Drag(MouseButton::Left) | MouseEventKind::Up(MouseButton::Left) => {
            was_dragging
        }
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use crossterm::event::{MouseButton, MouseEventKind};

    use super::consumed_resizable_grid_mouse_event;

    /// Verifies that a divider mouse down is consumed once dragging starts.
    #[test]
    fn consumes_down_when_dragging_starts() {
        assert!(consumed_resizable_grid_mouse_event(
            false,
            true,
            MouseEventKind::Down(MouseButton::Left)
        ));
    }

    /// Verifies that drag continuation is consumed from prior drag state.
    #[test]
    fn consumes_drag_when_already_dragging() {
        assert!(consumed_resizable_grid_mouse_event(
            true,
            true,
            MouseEventKind::Drag(MouseButton::Left)
        ));
    }

    /// Verifies that ordinary clicks are not consumed by resize handling.
    #[test]
    fn ignores_non_divider_clicks() {
        assert!(!consumed_resizable_grid_mouse_event(
            false,
            false,
            MouseEventKind::Down(MouseButton::Left)
        ));
    }
}
