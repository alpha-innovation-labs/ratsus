use std::path::PathBuf;

/// Returns the file path used to persist normal terminal session metadata.
pub fn normal_terminal_registry_path() -> Option<PathBuf> {
    std::env::var_os("HOME").map(PathBuf::from).map(|home| {
        home.join(".local")
            .join("share")
            .join("nexus")
            .join("terminal-sessions.json")
    })
}

#[cfg(test)]
mod tests {
    use super::normal_terminal_registry_path;

    /// Normal terminal sessions should persist in the Nexus data directory.
    #[test]
    fn uses_nexus_local_share_path() {
        let path = normal_terminal_registry_path().expect("HOME is set for tests");

        assert!(path.ends_with(".local/share/nexus/terminal-sessions.json"));
    }
}
