use ratatui::layout::Rect;
use ratatui::style::{Color, Modifier, Style};
use ratatui::text::{Line, Span};
use ratatui::widgets::{Block, BorderType, Borders, Paragraph};
use ratatui::Frame;

use crate::expo::conversation_observation_preview::ConversationObservationPreview;
use crate::expo::observation_preview_lines::observation_preview_lines;
use crate::left_panel::truncate_text_to_width::truncate_text_to_width;
use crate::rendering::default_border_color::default_border_color;
use crate::rendering::left_focused_border_color::left_focused_border_color;

/// Renders one Expo conversation card with connected title and child observations.
pub fn render_conversation_card(
    frame: &mut Frame,
    area: Rect,
    title: &str,
    preview: &ConversationObservationPreview,
) {
    render_title(frame, area, title);
    render_observations(frame, area, preview);
}

/// Renders the card title inside the focused left-pane border color.
fn render_title(frame: &mut Frame, area: Rect, title: &str) {
    let title_area = Rect::new(area.x, area.y, area.width, area.height.min(3));
    let title_width = usize::from(title_area.width.saturating_sub(2));
    let title_line = Line::from(Span::styled(
        truncate_text_to_width(title, title_width),
        Style::default()
            .fg(Color::White)
            .add_modifier(Modifier::BOLD),
    ));
    let title = Paragraph::new(title_line).block(
        Block::default()
            .borders(Borders::ALL)
            .border_type(BorderType::Rounded)
            .border_style(Style::default().fg(left_focused_border_color())),
    );
    frame.render_widget(title, title_area);
}

/// Renders observations as a padded child block connected below the title border.
fn render_observations(frame: &mut Frame, area: Rect, preview: &ConversationObservationPreview) {
    if area.height <= 3 {
        return;
    }
    let observation_area = Rect::new(
        area.x.saturating_add(1),
        area.y.saturating_add(3),
        area.width.saturating_sub(2),
        area.height.saturating_sub(3),
    );
    if observation_area.width == 0 || observation_area.height == 0 {
        return;
    }
    let lines = observation_preview_lines(
        preview,
        usize::from(observation_area.width.saturating_sub(2)),
    );
    if lines.is_empty() {
        return;
    }
    let observations = Paragraph::new(lines).block(
        Block::default()
            .borders(Borders::LEFT | Borders::RIGHT | Borders::BOTTOM)
            .border_style(Style::default().fg(default_border_color())),
    );
    frame.render_widget(observations, observation_area);
}
