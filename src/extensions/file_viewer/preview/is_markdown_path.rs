use std::path::Path;

/// Returns true when a path should be previewed with the Markdown widget.
pub fn is_markdown_path(path: &Path) -> bool {
    path.extension()
        .and_then(|extension| extension.to_str())
        .map(|extension| matches!(extension.to_ascii_lowercase().as_str(), "md" | "markdown"))
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::is_markdown_path;

    /// Markdown extensions should use the Markdown preview widget.
    #[test]
    fn detects_markdown_extensions() {
        assert!(is_markdown_path(std::path::Path::new("README.md")));
        assert!(is_markdown_path(std::path::Path::new("guide.MARKDOWN")));
    }

    /// Rust source files should remain on the Code preview widget.
    #[test]
    fn rejects_non_markdown_extensions() {
        assert!(!is_markdown_path(std::path::Path::new("main.rs")));
    }
}
