use ratatui::style::{Color, Style};

/// Returns the style for the git status dot marker.
pub fn git_status_dot_style() -> Style {
    Style::default().fg(Color::Rgb(255, 119, 132))
}
