use std::path::{Path, PathBuf};

/// Formats a folder path for display, replacing the home directory with `~`.
pub fn folder_display_name(path: &Path) -> String {
    let path_text = path.display().to_string();
    let Some(home) = home_path() else {
        return path_text;
    };
    if path == home {
        return "~".to_string();
    }
    path.strip_prefix(&home)
        .ok()
        .map(home_relative_path)
        .unwrap_or(path_text)
}

/// Returns the current user's home path when available.
fn home_path() -> Option<PathBuf> {
    std::env::var_os("HOME").map(PathBuf::from)
}

/// Formats a home-relative path with a leading tilde.
fn home_relative_path(relative: &Path) -> String {
    format!("~/{}", relative.display())
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::folder_display_name;

    /// Verifies non-home paths are shown as full paths.
    #[test]
    fn uses_full_path() {
        assert_eq!(folder_display_name(Path::new("/tmp/ratsus")), "/tmp/ratsus");
    }

    /// Verifies paths under HOME use a leading tilde.
    #[test]
    fn uses_tilde_for_home_path() {
        let home = std::env::var("HOME").expect("HOME is set for tests");
        let path = Path::new(&home).join("workspace/ratsus");

        assert_eq!(folder_display_name(&path), "~/workspace/ratsus");
    }
}
