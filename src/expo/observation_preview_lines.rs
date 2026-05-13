use ratatui::style::{Modifier, Style};
use ratatui::text::{Line, Span};

use crate::expo::conversation_observation_preview::ConversationObservationPreview;
use crate::left_panel::truncate_text_to_width::truncate_text_to_width;
use ratatui::style::Color;

/// Builds one-line truncated observation preview lines below a card title.
pub fn observation_preview_lines(
    preview: &ConversationObservationPreview,
    width: usize,
) -> Vec<Line<'static>> {
    let mut lines = Vec::new();
    if preview.has_more {
        lines.push(Line::from(Span::styled(
            truncate_text_to_width("+ more", width),
            Style::default()
                .fg(Color::White)
                .add_modifier(Modifier::ITALIC),
        )));
    }
    lines.extend(preview.observations.iter().map(|observation| {
        Line::from(Span::styled(
            truncate_text_to_width(&format!("• {observation}"), width),
            Style::default().fg(Color::White),
        ))
    }));
    lines
}

#[cfg(test)]
mod tests {
    use super::observation_preview_lines;
    use crate::expo::conversation_observation_preview::ConversationObservationPreview;

    /// Verifies the more indicator is rendered before observation bullets.
    #[test]
    fn puts_more_before_observations() {
        let preview = ConversationObservationPreview {
            has_more: true,
            observations: vec!["latest".to_string()],
        };

        let lines = observation_preview_lines(&preview, 20);

        assert_eq!(lines.len(), 2);
    }

    /// Verifies long observations remain one line and end with an ellipsis.
    #[test]
    fn truncates_long_observations() {
        let preview = ConversationObservationPreview {
            has_more: false,
            observations: vec!["abcdef".to_string()],
        };

        let lines = observation_preview_lines(&preview, 5);

        assert_eq!(lines[0].spans[0].content.as_ref(), "• ab…");
    }
}
