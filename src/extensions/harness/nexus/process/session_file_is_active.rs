use std::fs::File;
use std::io::{Read, Seek, SeekFrom};
use std::path::Path;

use serde_json::Value;

const SESSION_FILE_TAIL_CHUNK_SIZE: u64 = 8192;

/// Returns true when a Nexus session file ends in unfinished assistant work.
pub fn session_file_is_active(path: &Path) -> bool {
    let Ok(file) = File::open(path) else {
        return false;
    };
    last_message_from_session_tail(file)
        .as_ref()
        .is_some_and(message_is_active)
}

/// Returns the newest message object by scanning the session file from the end.
fn last_message_from_session_tail(mut file: File) -> Option<Value> {
    let mut cursor = file.seek(SeekFrom::End(0)).ok()?;
    let mut carry = Vec::new();
    while cursor > 0 {
        let chunk_len = cursor.min(SESSION_FILE_TAIL_CHUNK_SIZE) as usize;
        cursor = cursor.saturating_sub(chunk_len as u64);
        let mut chunk = read_session_tail_chunk(&mut file, cursor, chunk_len)?;
        chunk.extend_from_slice(&carry);
        let complete_lines = complete_lines_from_tail_chunk(&chunk, cursor, &mut carry)?;
        if let Some(message) = newest_message_from_lines(complete_lines) {
            return Some(message);
        }
    }
    None
}

/// Reads a fixed byte range from the session file.
fn read_session_tail_chunk(file: &mut File, start: u64, len: usize) -> Option<Vec<u8>> {
    file.seek(SeekFrom::Start(start)).ok()?;
    let mut chunk = vec![0; len];
    file.read_exact(&mut chunk).ok()?;
    Some(chunk)
}

/// Returns only complete newline-delimited records from a reverse-scanned chunk.
fn complete_lines_from_tail_chunk<'a>(
    chunk: &'a [u8],
    cursor: u64,
    carry: &mut Vec<u8>,
) -> Option<&'a [u8]> {
    if cursor == 0 {
        carry.clear();
        return Some(chunk);
    }
    let first_newline = chunk.iter().position(|byte| *byte == b'\n')?;
    carry.clear();
    carry.extend_from_slice(&chunk[..first_newline]);
    Some(&chunk[first_newline.saturating_add(1)..])
}

/// Returns the newest JSON line whose type is `message`.
fn newest_message_from_lines(lines: &[u8]) -> Option<Value> {
    lines
        .split(|byte| *byte == b'\n')
        .rev()
        .filter(|line| !line_is_blank(line))
        .filter_map(|line| serde_json::from_slice::<Value>(trim_carriage_return(line)).ok())
        .find(|value| value.get("type").and_then(Value::as_str) == Some("message"))
}

/// Returns true when a line has no visible bytes.
fn line_is_blank(line: &[u8]) -> bool {
    line.iter().all(u8::is_ascii_whitespace)
}

/// Removes a trailing carriage return from a JSONL record.
fn trim_carriage_return(line: &[u8]) -> &[u8] {
    line.strip_suffix(b"\r").unwrap_or(line)
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
    use std::fs;
    use std::path::PathBuf;

    use serde_json::json;

    use super::{message_is_active, session_file_is_active};

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

    /// Verifies active detection uses the newest message near the end of a large JSONL file.
    #[test]
    fn detects_newest_message_from_large_file_tail() {
        let path = unique_session_file_path("large-tail-active");
        let filler = format!("{}\n", json!({"type":"event","payload":"x".repeat(32_000)}));
        let active_message = format!("{}\n", json!({"type":"message","message":{"role":"user"}}));
        fs::write(&path, format!("{}{}", filler, active_message)).unwrap();

        assert!(session_file_is_active(&path));

        let _ = fs::remove_file(path);
    }

    /// Builds a unique temporary path for session-file tests.
    fn unique_session_file_path(name: &str) -> PathBuf {
        std::env::temp_dir().join(format!(
            "ratsus-{name}-{}-{}.jsonl",
            std::process::id(),
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_nanos()
        ))
    }
}
