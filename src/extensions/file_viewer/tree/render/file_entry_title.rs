use std::path::Path;

/// Returns the display title for a file-tree path.
pub fn file_entry_title(path: &Path) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .map(ToString::to_string)
        .unwrap_or_else(|| path.to_string_lossy().to_string())
}
