use std::path::Path;

/// Returns true when a changed path is a Nexus observation state file.
pub fn path_is_observation_state(path: &Path) -> bool {
    path.file_name()
        .and_then(|name| name.to_str())
        .is_some_and(|name| name.ends_with(".state.json"))
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::path_is_observation_state;

    /// Verifies observation state file names are detected.
    #[test]
    fn detects_state_file() {
        assert!(path_is_observation_state(Path::new("abc.state.json")));
    }
}
