use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::widgets::{Paragraph, Widget};

/// Renders lightweight placeholder content while panes are being resized.
pub fn render_resize_placeholder(area: Rect, buf: &mut ratatui::buffer::Buffer) {
    Paragraph::new("Resizing…")
        .style(Style::default().fg(Color::DarkGray))
        .render(area, buf);
}
