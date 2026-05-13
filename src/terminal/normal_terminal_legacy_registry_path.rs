use std::path::PathBuf;

/// Returns the previous file path used for normal terminal session metadata.
pub fn normal_terminal_legacy_registry_path() -> Option<PathBuf> {
    std::env::var_os("HOME").map(PathBuf::from).map(|home| {
        home.join(".config")
            .join("ratsus")
            .join("normal-terminals.json")
    })
}
