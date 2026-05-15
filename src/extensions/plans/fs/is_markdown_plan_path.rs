use std::path::Path;

/// Returns true when a path is a supported Markdown plan file.
pub fn is_markdown_plan_path(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .is_some_and(|extension| {
            matches!(extension.to_ascii_lowercase().as_str(), "md" | "markdown")
        })
}
