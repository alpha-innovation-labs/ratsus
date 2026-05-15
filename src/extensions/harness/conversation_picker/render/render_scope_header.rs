use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::app::state::app_state::AppState;
use crate::extensions::harness::conversation_picker::data::scope_label::conversation_picker_scope_label;
use crate::extensions::harness::conversation_picker::layout::dialog_body_area::conversation_picker_dialog_area;

/// Renders the current picker scope into the top-right dialog header.
pub fn render_conversation_picker_scope_header(app: &AppState, frame: &mut Frame) {
    let label = conversation_picker_scope_label(app);
    let text = format!(" {label} ");
    let dialog_area = conversation_picker_dialog_area(frame.area());
    let width = text.chars().count() as u16;
    if dialog_area.width <= width.saturating_add(2) {
        return;
    }
    let area = Rect::new(
        dialog_area
            .x
            .saturating_add(dialog_area.width.saturating_sub(width).saturating_sub(2)),
        dialog_area.y,
        width,
        1,
    );
    frame.render_widget(
        Paragraph::new(text).style(Style::default().fg(Color::Cyan)),
        area,
    );
}
