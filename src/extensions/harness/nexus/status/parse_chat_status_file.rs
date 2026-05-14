use std::path::PathBuf;

use serde::Deserialize;

use crate::extensions::harness::core::chat_session::ChatSession;
use crate::extensions::harness::nexus::parsing::sanitize_json_control_characters::sanitize_json_control_characters;

#[derive(Debug, Deserialize)]
struct ChatStatusFileJson {
    entries: Vec<ChatStatusFileEntryJson>,
}

#[derive(Debug, Deserialize)]
struct ChatStatusFileEntryJson {
    #[serde(rename = "sessionId")]
    session_id: String,
    #[serde(rename = "sessionTitle")]
    session_title: Option<String>,
    #[serde(rename = "updatedAt")]
    updated_at: String,
    cwd: PathBuf,
    pid: u32,
    #[serde(rename = "sessionFile")]
    session_file: PathBuf,
}

/// Parses Nexus chat status JSON into session metadata values.
pub fn parse_nexus_chat_status_file<F, G>(
    output: &str,
    is_pid_alive: F,
    is_session_active: G,
) -> serde_json::Result<Vec<ChatSession>>
where
    F: Fn(u32) -> bool,
    G: Fn(&std::path::Path) -> bool,
{
    let sanitized_output = sanitize_json_control_characters(output);
    let status: ChatStatusFileJson = serde_json::from_str(&sanitized_output)?;
    Ok(status
        .entries
        .into_iter()
        .map(|entry| chat_status_entry_to_session(entry, &is_pid_alive, &is_session_active))
        .collect())
}

/// Converts one chat status entry into session metadata.
fn chat_status_entry_to_session<F, G>(
    entry: ChatStatusFileEntryJson,
    is_pid_alive: &F,
    is_session_active: &G,
) -> ChatSession
where
    F: Fn(u32) -> bool,
    G: Fn(&std::path::Path) -> bool,
{
    ChatSession::new(
        entry.updated_at,
        entry
            .session_title
            .unwrap_or_else(|| "New Session".to_string()),
        entry.session_id,
        entry.cwd,
    )
    .with_running(is_pid_alive(entry.pid) && is_session_active(&entry.session_file))
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::parse_nexus_chat_status_file;

    /// Verifies chat status entries provide title, update time, session id, cwd, and active status.
    #[test]
    fn parses_chat_status_session_fields() {
        let output = r#"{"version":1,"entries":[{"id":"42:abc","sessionId":"abc","sessionTitle":"Live title","updatedAt":"2026-05-12T10:00:00.000Z","startedAt":"2026-05-12T09:00:00.000Z","cwd":"/tmp/project","pid":42,"sessionFile":"/tmp/session.jsonl"}]}"#;

        let sessions = parse_nexus_chat_status_file(output, |pid| pid == 42, |_| true).unwrap();

        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0].id, "abc");
        assert_eq!(sessions[0].title, "Live title");
        assert_eq!(sessions[0].date, "2026-05-12T10:00:00.000Z");
        assert_eq!(sessions[0].working_dir, PathBuf::from("/tmp/project"));
        assert!(sessions[0].is_running);
    }

    /// Verifies missing titles are kept displayable while pid liveness controls active state.
    #[test]
    fn falls_back_for_missing_title_and_marks_dead_pid_inactive() {
        let output = r#"{"entries":[{"sessionId":"abc","updatedAt":"2026-05-12T10:00:00.000Z","cwd":"/tmp/project","pid":99,"sessionFile":"/tmp/session.jsonl"}]}"#;

        let sessions = parse_nexus_chat_status_file(output, |_| false, |_| true).unwrap();

        assert_eq!(sessions[0].title, "New Session");
        assert!(!sessions[0].is_running);
    }

    /// Verifies idle session files suppress the loader even when pid is alive.
    #[test]
    fn marks_alive_but_idle_session_inactive() {
        let output = r#"{"entries":[{"sessionId":"abc","updatedAt":"2026-05-12T10:00:00.000Z","cwd":"/tmp/project","pid":42,"sessionFile":"/tmp/session.jsonl"}]}"#;

        let sessions = parse_nexus_chat_status_file(output, |pid| pid == 42, |_| false).unwrap();

        assert!(!sessions[0].is_running);
    }

    /// Verifies chat status titles with raw newlines are sanitized before parsing.
    #[test]
    fn parses_chat_status_title_with_raw_newline() {
        let output = "{\"entries\":[{\"sessionId\":\"abc\",\"sessionTitle\":\"hello\nworld\",\"updatedAt\":\"2026-05-12T10:00:00.000Z\",\"cwd\":\"/tmp/project\",\"pid\":42,\"sessionFile\":\"/tmp/session.jsonl\"}]}";

        let sessions = parse_nexus_chat_status_file(output, |pid| pid == 42, |_| false).unwrap();

        assert_eq!(sessions[0].title, "hello\nworld");
    }
}
