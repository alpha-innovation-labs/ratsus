use ratatui::layout::Rect;
use ratatui::widgets::Paragraph;
use ratatui::Frame;
use ratkit::widgets::code_widget::CodeWidget;

use crate::extensions::file_viewer::preview::file_preview_state::FilePreviewState;
use crate::extensions::plans::data::plan_list_state::PlanListState;

/// Renders the currently active Markdown plan in the main pane.
pub fn render_plan_preview(state: &mut PlanListState, frame: &mut Frame, area: Rect) {
    let Some(preview_state) = &mut state.preview_state else {
        frame.render_widget(Paragraph::new("No markdown plan selected"), area);
        return;
    };
    match preview_state {
        FilePreviewState::Code(code_state) => {
            let state = code_state.as_mut();
            let widget = CodeWidget::from_state(state)
                .show_line_numbers(true)
                .show_outline(true);
            frame.render_stateful_widget(widget, area, state);
        }
        FilePreviewState::Markdown(widget) => frame.render_widget(widget.as_mut(), area),
    }
}
