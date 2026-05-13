/// Truncates text to a character width, reserving the final cell for an ellipsis when needed.
pub fn truncate_text_to_width(text: &str, max_width: usize) -> String {
    let char_count = text.chars().count();
    if char_count <= max_width {
        return text.to_string();
    }
    if max_width == 0 {
        return String::new();
    }
    if max_width == 1 {
        return "…".to_string();
    }
    let prefix = text.chars().take(max_width - 1).collect::<String>();
    format!("{}…", prefix)
}

#[cfg(test)]
mod tests {
    use super::truncate_text_to_width;

    /// Keeps text that already fits inside the requested width.
    #[test]
    fn keeps_fitting_text() {
        assert_eq!(truncate_text_to_width("chat", 4), "chat");
    }

    /// Truncates long text with an ellipsis inside the requested width.
    #[test]
    fn truncates_long_text() {
        assert_eq!(truncate_text_to_width("chat title", 6), "chat …");
    }

    /// Returns empty text when no width is available.
    #[test]
    fn handles_zero_width() {
        assert_eq!(truncate_text_to_width("chat", 0), "");
    }
}
