use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use serde_json::Value;

/// Returns true when a Nexus session file ends in unfinished assistant work.
pub fn session_file_is_active(path: &Path) -> bool {
    let Ok(file) = File::open(path) else {
        return false;
    };
    let mut last_message = None;
    for line in BufReader::new(file).lines().map_while(Result::ok) {
        let Ok(value) = serde_json::from_str::<Value>(&line) else {
            continue;
        };
        if value.get("type").and_then(Value::as_str) == Some("message") {
            last_message = Some(value);
        }
    }
    last_message.as_ref().is_some_and(message_is_active)
}

/// Returns true when the latest message indicates pending assistant work.
fn message_is_active(value: &Value) -> bool {
    let Some(message) = value.get("message") else {
        return false;
    };
    match message.get("role").and_then(Value::as_str) {
        Some("user") => true,
        Some("toolResult") => true,
        Some("assistant") => message
            .get("stopReason")
            .and_then(Value::as_str)
            .is_some_and(|reason| reason != "stop"),
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use serde_json::json;

    use super::message_is_active;

    /// Verifies final assistant messages are treated as idle.
    #[test]
    fn final_assistant_message_is_idle() {
        let value = json!({"type":"message","message":{"role":"assistant","stopReason":"stop"}});

        assert!(!message_is_active(&value));
    }

    /// Verifies tool-use assistant messages are treated as active.
    #[test]
    fn tool_use_assistant_message_is_active() {
        let value = json!({"type":"message","message":{"role":"assistant","stopReason":"toolUse"}});

        assert!(message_is_active(&value));
    }

    /// Verifies user messages are treated as active until an assistant finishes.
    #[test]
    fn user_message_is_active() {
        let value = json!({"type":"message","message":{"role":"user"}});

        assert!(message_is_active(&value));
    }
}
