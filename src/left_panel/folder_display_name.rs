use std::path::Path;

/// Formats a folder path for compact display in the session tree.
pub fn folder_display_name(path: &Path) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .filter(|name| !name.is_empty())
        .map(str::to_owned)
        .unwrap_or_else(|| path.display().to_string())
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::folder_display_name;

    /// Verifies that the last path component is used as the visible folder label.
    #[test]
    fn uses_last_component() {
        assert_eq!(folder_display_name(Path::new("/tmp/ratsus")), "ratsus");
    }
}
