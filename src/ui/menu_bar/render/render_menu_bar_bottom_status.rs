use ratatui::style::{Color, Style};
use ratatui::widgets::Paragraph;
use ratatui::Frame;

use crate::ui::menu_bar::render::menu_bar_bottom_status_area::menu_bar_bottom_status_area;

/// Renders a compact status string on the bottom row of the menu bar.
pub fn render_menu_bar_bottom_status(frame: &mut Frame, area: ratatui::layout::Rect, status: &str) {
    let Some(status_area) = menu_bar_bottom_status_area(area, status) else {
        return;
    };
    let visible_status = status
        .chars()
        .rev()
        .take(status_area.width as usize)
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
        .collect::<String>();
    let paragraph = Paragraph::new(visible_status).style(Style::default().fg(Color::DarkGray));
    frame.render_widget(paragraph, status_area);
}
