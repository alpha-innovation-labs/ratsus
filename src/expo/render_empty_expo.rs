use ratatui::layout::Rect;
use ratatui::text::Line;
use ratatui::widgets::Paragraph;
use ratatui::Frame;

/// Renders Expo placeholder content inside the existing main pane.
pub fn render_empty_expo(frame: &mut Frame, area: Rect, message: impl Into<String>) {
    frame.render_widget(Paragraph::new(Line::from(message.into())), area);
}
