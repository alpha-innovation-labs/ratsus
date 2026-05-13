use ratatui::style::Color;

/// Returns the configured foreground color for session titles.
pub fn session_title_color() -> Color {
    Color::Rgb(114, 113, 131)
}

#[cfg(test)]
mod tests {
    use ratatui::style::Color;

    use super::session_title_color;

    /// Verifies session title color matches #727183.
    #[test]
    fn returns_configured_session_title_color() {
        assert_eq!(session_title_color(), Color::Rgb(114, 113, 131));
    }
}
