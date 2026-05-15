use std::path::Path;

/// Returns the display title for a Markdown plan path.
pub fn plan_title(path: &Path) -> String {
    path.file_stem()
        .and_then(|name| name.to_str())
        .filter(|name| !name.is_empty())
        .unwrap_or("Untitled plan")
        .to_string()
}
