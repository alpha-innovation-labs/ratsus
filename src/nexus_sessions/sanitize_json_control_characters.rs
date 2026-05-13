/// Escapes raw C0 control characters that appear inside JSON strings.
pub fn sanitize_json_control_characters(output: &str) -> String {
    let mut sanitized = String::with_capacity(output.len());
    let mut in_string = false;
    let mut escaped = false;

    for character in output.chars() {
        if escaped {
            sanitized.push(character);
            escaped = false;
            continue;
        }
        if character == '\\' && in_string {
            sanitized.push(character);
            escaped = true;
            continue;
        }
        if character == '"' {
            in_string = !in_string;
            sanitized.push(character);
            continue;
        }
        push_sanitized_character(&mut sanitized, character, in_string);
    }

    sanitized
}

/// Pushes one character, escaping invalid raw string controls when needed.
fn push_sanitized_character(sanitized: &mut String, character: char, in_string: bool) {
    if in_string && character.is_control() {
        sanitized.push_str(&format!("\\u{:04X}", character as u32));
    } else {
        sanitized.push(character);
    }
}

#[cfg(test)]
mod tests {
    use super::sanitize_json_control_characters;

    /// Verifies raw newlines in a JSON string are escaped for serde parsing.
    #[test]
    fn escapes_raw_newline_inside_string() {
        let output = "[{\"title\":\"hello\nworld\"}]";

        assert_eq!(
            sanitize_json_control_characters(output),
            "[{\"title\":\"hello\\u000Aworld\"}]"
        );
    }
}
