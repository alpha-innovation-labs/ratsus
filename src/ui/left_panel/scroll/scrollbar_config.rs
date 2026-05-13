use ratatui::style::{Color, Style};
use ratkit::widgets::markdown_preview::ScrollbarConfig;

/// Returns the Ratkit markdown scrollbar styling used by the left session pane.
pub fn left_panel_scrollbar_config() -> ScrollbarConfig {
    ScrollbarConfig {
        width: 1,
        track_char: '░',
        thumb_char: '█',
        track_style: Style::default().fg(Color::Rgb(50, 55, 65)),
        thumb_style: Style::default().fg(Color::Rgb(120, 130, 145)),
        percentage_style: Style::default().fg(Color::Rgb(70, 75, 85)),
        min_thumb_height: 1,
    }
}
