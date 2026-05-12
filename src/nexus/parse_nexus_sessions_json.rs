use std::path::PathBuf;

use serde::Deserialize;

use crate::session_info::NexusSession;

#[derive(Debug, Deserialize)]
struct NexusSessionJson {
    id: String,
    title: String,
    cwd: PathBuf,
    modified: String,
}

/// Parses `nexus --sessions-all --json` output into session metadata rows.
pub fn parse_nexus_sessions_json(output: &str) -> serde_json::Result<Vec<NexusSession>> {
    let sessions: Vec<NexusSessionJson> = serde_json::from_str(output)?;
    Ok(sessions
        .into_iter()
        .map(|session| NexusSession::new(session.modified, session.title, session.id, session.cwd))
        .collect())
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::parse_nexus_sessions_json;

    /// Verifies that Nexus JSON sessions become session metadata values.
    #[test]
    fn parses_nexus_sessions_json() {
        let output = r#"[{"id":"abc","title":"Build UI","cwd":"/tmp/project","modified":"2026-05-12T10:00:00.000Z"}]"#;

        let sessions = parse_nexus_sessions_json(output).unwrap();

        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0].id, "abc");
        assert_eq!(sessions[0].title, "Build UI");
        assert_eq!(sessions[0].date, "2026-05-12T10:00:00.000Z");
        assert_eq!(sessions[0].working_dir, PathBuf::from("/tmp/project"));
    }
}
