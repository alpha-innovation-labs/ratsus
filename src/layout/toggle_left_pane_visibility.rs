use ratkit::primitives::resizable_grid::ResizableGridWidgetState;

use crate::app::nexus_demo_state::NexusDemo;
use crate::layout::focused_pane_after_left_pane_toggle::focused_pane_after_left_pane_toggle;

/// Toggles the session list pane and focuses the pane that becomes active.
pub fn toggle_left_pane_visibility(app: &mut NexusDemo) {
    app.left_pane_visible = !app.left_pane_visible;
    app.focused_pane = focused_pane_after_left_pane_toggle(app.left_pane_visible);
    if !app.left_pane_visible {
        app.layout_widget_state = ResizableGridWidgetState::default();
    }
}
