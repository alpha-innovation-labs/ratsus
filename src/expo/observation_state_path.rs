use std::fs;
use std::io;
use std::path::PathBuf;

use crate::expo::nexus_observations_dir::nexus_observations_dir;

/// Returns the persisted observation state path for a Nexus conversation id.
pub fn observation_state_path(conversation_id: &str) -> io::Result<PathBuf> {
    let suffix = format!("_{conversation_id}.state.json");
    fs::read_dir(nexus_observations_dir())?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .find(|path| path_matches_suffix(path, &suffix))
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, conversation_id.to_string()))
}

/// Returns true when an observation path belongs to the requested conversation id.
fn path_matches_suffix(path: &std::path::Path, suffix: &str) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.ends_with(suffix))
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::path_matches_suffix;

    /// Verifies UUID-only session ids match timestamp-prefixed observation state files.
    #[test]
    fn matches_timestamp_prefixed_state_file() {
        assert!(path_matches_suffix(
            Path::new("2026-04-20T15-18-32-751Z_abc.state.json"),
            "_abc.state.json"
        ));
    }
}
