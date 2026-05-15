use std::path::Path;

/// Returns true when a changed path is a consolidated Nexus observation JSON file.
pub fn path_is_observation_state(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(is_consolidated_observation_json)
}

/// Returns true for the new one-file observation JSON format.
fn is_consolidated_observation_json(name: &str) -> bool {
    name.ends_with(".json")
        && !name.ends_with(".state.json")
        && !name.ends_with(".messages.json")
        && !name.ends_with(".observations.json")
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::path_is_observation_state;

    /// Verifies consolidated observation JSON file names are detected.
    #[test]
    fn detects_observation_json_file() {
        assert!(path_is_observation_state(Path::new("abc.json")));
    }

    /// Verifies legacy split observation state files are ignored.
    #[test]
    fn ignores_legacy_state_file() {
        assert!(!path_is_observation_state(Path::new("abc.state.json")));
    }
}
