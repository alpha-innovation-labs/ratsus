use crate::harnesses::nexus::sanitize_json_control_characters::sanitize_json_control_characters;

/// Escapes invalid raw control characters from Nexus sessions JSON output.
pub fn sanitize_chat_sessions_json(output: &str) -> String {
    sanitize_json_control_characters(output)
}

#[cfg(test)]
mod tests {
    use super::sanitize_chat_sessions_json;

    /// Verifies raw newlines in a JSON string are escaped for serde parsing.
    #[test]
    fn escapes_raw_newline_inside_string() {
        let output = "[{\"title\":\"hello\nworld\"}]";

        assert_eq!(
            sanitize_chat_sessions_json(output),
            "[{\"title\":\"hello\\u000Aworld\"}]"
        );
    }

    /// Verifies structural whitespace outside strings is left unchanged.
    #[test]
    fn preserves_whitespace_outside_strings() {
        let output = "[\n  {\"title\":\"hello\"}\n]";

        assert_eq!(sanitize_chat_sessions_json(output), output);
    }
}
