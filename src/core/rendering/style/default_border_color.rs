use ratatui::style::Color;

/// Returns the default border color used by app chrome.
pub fn default_border_color() -> Color {
    Color::Rgb(72, 69, 90)
}

#[cfg(test)]
mod tests {
    use ratatui::style::Color;

    use super::default_border_color;

    /// Verifies the default border color matches #48455a.
    #[test]
    fn returns_default_border_color() {
        assert_eq!(default_border_color(), Color::Rgb(72, 69, 90));
    }
}
