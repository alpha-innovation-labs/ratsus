use std::path::Path;

/// Formats a folder path as the final path component for compact left-panel display.
pub fn folder_compact_display_name(path: &Path) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .filter(|name| !name.is_empty())
        .map(str::to_owned)
        .unwrap_or_else(|| path.display().to_string())
}

#[cfg(test)]
mod tests {
    use std::path::Path;

    use super::folder_compact_display_name;

    /// Verifies compact display uses only the last path component.
    #[test]
    fn uses_last_component() {
        assert_eq!(
            folder_compact_display_name(Path::new("/tmp/ratsus")),
            "ratsus"
        );
    }
}
