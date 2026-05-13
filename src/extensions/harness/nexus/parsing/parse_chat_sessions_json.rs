use std::path::PathBuf;

use serde::Deserialize;

use crate::extensions::harness::core::chat_session::ChatSession;
use crate::extensions::harness::nexus::parsing::sanitize_chat_sessions_json::sanitize_chat_sessions_json;

#[derive(Debug, Deserialize)]
struct ChatSessionJson {
    id: String,
    title: String,
    cwd: PathBuf,
    created: Option<String>,
    modified: String,
}

/// Parses `nexus --sessions-all --json` output into session metadata rows.
pub fn parse_chat_sessions_json(output: &str) -> serde_json::Result<Vec<ChatSession>> {
    let sanitized_output = sanitize_chat_sessions_json(output);
    let sessions: Vec<ChatSessionJson> = serde_json::from_str(&sanitized_output)?;
    Ok(sessions
        .into_iter()
        .map(|session| {
            ChatSession::new_with_created(
                session.modified.clone(),
                session.created.unwrap_or(session.modified),
                session.title,
                session.id,
                session.cwd,
            )
        })
        .collect())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::parse_chat_sessions_json;

    /// Verifies that Nexus JSON sessions become session metadata values.
    #[test]
    fn parses_chat_sessions_json() {
        let output = r#"[{"id":"abc","title":"Build UI","cwd":"/tmp/project","created":"2026-05-11T10:00:00.000Z","modified":"2026-05-12T10:00:00.000Z"}]"#;

        let sessions = parse_chat_sessions_json(output).unwrap();

        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0].id, "abc");
        assert_eq!(sessions[0].title, "Build UI");
        assert_eq!(sessions[0].date, "2026-05-12T10:00:00.000Z");
        assert_eq!(sessions[0].created_at, "2026-05-11T10:00:00.000Z");
        assert_eq!(sessions[0].working_dir, PathBuf::from("/tmp/project"));
    }

    /// Verifies Nexus JSON with raw control characters inside strings is accepted.
    #[test]
    fn parses_sessions_with_raw_control_character_in_title() {
        let output = "[{\"id\":\"abc\",\"title\":\"Build\nUI\",\"cwd\":\"/tmp/project\",\"created\":\"2026-05-11T10:00:00.000Z\",\"modified\":\"2026-05-12T10:00:00.000Z\"}]";

        let sessions = parse_chat_sessions_json(output).unwrap();

        assert_eq!(sessions[0].title, "Build\nUI");
    }
}
