use std::path::PathBuf;

/// Returns the Nexus observations directory from the user's local data path.
pub fn nexus_observations_dir() -> PathBuf {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".local/share/nexus/agent/observations")
}
