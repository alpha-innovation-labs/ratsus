use std::fs;
use std::io;
use std::path::PathBuf;

use crate::extensions::harness::observations::nexus_observations_dir::nexus_observations_dir;

/// Returns the persisted observation JSON path for a Nexus conversation id.
pub fn observation_state_path(conversation_id: &str) -> io::Result<PathBuf> {
    fs::read_dir(nexus_observations_dir())?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .find(|path| path_matches_conversation_id(path, conversation_id))
        .ok_or_else(|| io::Error::new(io::ErrorKind::NotFound, conversation_id.to_string()))
}

/// Returns true when an observation JSON path belongs to the requested conversation id.
fn path_matches_conversation_id(path: &std::path::Path, conversation_id: &str) -> bool {
    let Some(name) = path.file_name().and_then(|name| name.to_str()) else {
        return false;
    };
    let Some(stem) = name.strip_suffix(".json") else {
        return false;
    };
    stem == conversation_id || stem.ends_with(&format!("_{conversation_id}"))
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::path_matches_conversation_id;

    /// Verifies UUID-only session ids match timestamp-prefixed observation JSON files.
    #[test]
    fn matches_timestamp_prefixed_json_file() {
        assert!(path_matches_conversation_id(
            Path::new("2026-04-20T15-18-32-751Z_abc.json"),
            "abc"
        ));
    }

    /// Verifies full timestamp-prefixed session ids match the consolidated JSON file.
    #[test]
    fn matches_full_conversation_json_file() {
        assert!(path_matches_conversation_id(
            Path::new("2026-04-20T15-18-32-751Z_abc.json"),
            "2026-04-20T15-18-32-751Z_abc"
        ));
    }
}
