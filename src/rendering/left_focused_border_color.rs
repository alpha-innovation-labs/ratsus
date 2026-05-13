use ratatui::style::Color;

/// Returns the focused border color for the left pane.
pub fn left_focused_border_color() -> Color {
    Color::Rgb(255, 121, 162)
}

#[cfg(test)]
mod tests {
    use ratatui::style::Color;

    use super::left_focused_border_color;

    /// Verifies the focused left-pane border color matches #FF79A2.
    #[test]
    fn returns_left_focused_border_color() {
        assert_eq!(left_focused_border_color(), Color::Rgb(255, 121, 162));
    }
}
