use std::path::PathBuf;

/// Returns the Nexus cmux session registry path from env or the default user data path.
pub fn nexus_cmux_session_registry_path() -> PathBuf {
    if let Ok(path) = std::env::var("NEXUS_CMUX_SESSION_REGISTRY") {
        if !path.is_empty() {
            return PathBuf::from(path);
        }
    }

    std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".local/share/nexus/agent/cmux-session-registry.json")
}
