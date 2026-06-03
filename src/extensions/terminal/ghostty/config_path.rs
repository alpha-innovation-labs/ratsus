use std::path::PathBuf;

/// Returns the Ghostty config path for the current platform.
pub fn ghostty_config_path() -> PathBuf {
    if cfg!(target_os = "macos") {
        return home_dir()
            .join("Library")
            .join("Application Support")
            .join("com.mitchellh.ghostty")
            .join("config");
    }
    xdg_config_home().join("ghostty").join("config")
}

/// Returns the active home directory, or the current directory when unavailable.
fn home_dir() -> PathBuf {
    std::env::var_os("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| PathBuf::from("."))
}

/// Returns the XDG config root for non-macOS Ghostty config lookup.
fn xdg_config_home() -> PathBuf {
    std::env::var_os("XDG_CONFIG_HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|| home_dir().join(".config"))
}

#[cfg(test)]
mod tests {
    use super::ghostty_config_path;

    /// Verifies the Ghostty path points at a config file.
    #[test]
    fn returns_config_file_path() {
        let path = ghostty_config_path();

        assert_eq!(
            path.file_name().and_then(|name| name.to_str()),
            Some("config")
        );
    }
}
