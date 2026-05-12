use std::path::PathBuf;

use serde::Deserialize;

use crate::nexus_sessions::session_info::NexusSession;

#[derive(Debug, Deserialize)]
struct NexusSessionRegistryJson {
    entries: Vec<NexusSessionRegistryEntryJson>,
}

#[derive(Debug, Deserialize)]
struct NexusSessionRegistryEntryJson {
    #[serde(rename = "sessionId")]
    session_id: String,
    #[serde(rename = "sessionTitle")]
    session_title: Option<String>,
    #[serde(rename = "updatedAt")]
    updated_at: String,
    cwd: PathBuf,
    pid: u32,
}

/// Parses Nexus cmux registry JSON into session metadata values.
pub fn parse_nexus_session_registry<F>(
    output: &str,
    is_pid_alive: F,
) -> serde_json::Result<Vec<NexusSession>>
where
    F: Fn(u32) -> bool,
{
    let registry: NexusSessionRegistryJson = serde_json::from_str(output)?;
    Ok(registry
        .entries
        .into_iter()
        .map(|entry| registry_entry_to_session(entry, &is_pid_alive))
        .collect())
}

/// Converts one registry entry into session metadata.
fn registry_entry_to_session<F>(
    entry: NexusSessionRegistryEntryJson,
    is_pid_alive: &F,
) -> NexusSession
where
    F: Fn(u32) -> bool,
{
    NexusSession::new(
        entry.updated_at,
        entry
            .session_title
            .unwrap_or_else(|| "New Session".to_string()),
        entry.session_id,
        entry.cwd,
    )
    .with_running(is_pid_alive(entry.pid))
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::parse_nexus_session_registry;

    /// Verifies registry entries provide title, update time, session id, cwd, and running status.
    #[test]
    fn parses_registry_session_fields() {
        let output = r#"{"entries":[{"sessionId":"abc","sessionTitle":"Live title","updatedAt":"2026-05-12T10:00:00.000Z","cwd":"/tmp/project","pid":42}]}"#;

        let sessions = parse_nexus_session_registry(output, |pid| pid == 42).unwrap();

        assert_eq!(sessions.len(), 1);
        assert_eq!(sessions[0].id, "abc");
        assert_eq!(sessions[0].title, "Live title");
        assert_eq!(sessions[0].date, "2026-05-12T10:00:00.000Z");
        assert_eq!(sessions[0].working_dir, PathBuf::from("/tmp/project"));
        assert!(sessions[0].is_running);
    }

    /// Verifies missing titles are kept displayable while pid liveness controls running state.
    #[test]
    fn falls_back_for_missing_title_and_marks_dead_pid_inactive() {
        let output = r#"{"entries":[{"sessionId":"abc","updatedAt":"2026-05-12T10:00:00.000Z","cwd":"/tmp/project","pid":99}]}"#;

        let sessions = parse_nexus_session_registry(output, |_| false).unwrap();

        assert_eq!(sessions[0].title, "New Session");
        assert!(!sessions[0].is_running);
    }
}
