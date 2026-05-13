use ratatui::layout::Rect;
use ratatui::style::{Color, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

/// Renders the Expo search bar with the current filter query.
pub fn render_expo_filter_bar(frame: &mut Frame, area: Rect, query: &str) {
    let line = Line::from(vec![
        Span::styled("/ ", Style::default().fg(Color::Cyan)),
        Span::styled(query.to_string(), Style::default().fg(Color::White)),
    ]);
    frame.render_widget(Paragraph::new(line), area);
}
