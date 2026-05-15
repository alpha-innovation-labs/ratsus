use ratatui::layout::Rect;
use ratatui::Frame;
use ratkit::primitives::resizable_grid::ResizableGridWidget;

use crate::app::state::app_state::AppState;
use crate::ui::layout::resizable_grid::visible_layout_widget_state::visible_layout_widget_state;

/// Renders the active resizable grid divider overlay for the app layout.
pub fn render_resizable_grid_overlay(app: &mut AppState, frame: &mut Frame, area: Rect) {
    let visible_state =
        visible_layout_widget_state(app.layout_widget_state, app.workspace_view_enabled);
    let widget = ResizableGridWidget::new(app.layout.clone())
        .with_state(visible_state)
        .with_pane_borders(false);
    app.layout_widget_state = widget.state();
    frame.render_widget(widget, area);
}
