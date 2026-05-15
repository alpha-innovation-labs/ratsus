use std::path::PathBuf;

const MULTIPLEXER_STATE_FILE_ENV: &str = "NEXUS_MULTIPLEXER_STATE_FILE";
const MULTIPLEXER_STATE_FILE_NAME: &str = "split-pane-state.json";

/// Returns the app-owned Nexus multiplexer state file path.
pub fn multiplexer_state_path() -> Option<PathBuf> {
    if let Some(path) = std::env::var_os(MULTIPLEXER_STATE_FILE_ENV) {
        return Some(PathBuf::from(path));
    }
    #[cfg(test)]
    {
        None
    }
    #[cfg(not(test))]
    {
        std::env::var_os("HOME").map(PathBuf::from).map(|home| {
            home.join(".local")
                .join("share")
                .join("nexus")
                .join("multiplexer")
                .join(MULTIPLEXER_STATE_FILE_NAME)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::MULTIPLEXER_STATE_FILE_NAME;

    /// Multiplexer state should use the recommended Nexus data filename.
    #[test]
    fn uses_recommended_multiplexer_file_name() {
        assert_eq!(MULTIPLEXER_STATE_FILE_NAME, "split-pane-state.json");
    }
}
