use std::fs;
use std::path::Path;

/// Returns markdown preview content for a selected file-system path.
pub fn file_preview_markdown_for_path(path: &Path, is_dir: bool) -> String {
    if is_dir {
        return directory_preview_markdown(path);
    }
    match fs::read_to_string(path) {
        Ok(content) if content.is_empty() => empty_file_preview_markdown(path),
        Ok(content) => content,
        Err(error) => unreadable_file_preview_markdown(path, &error),
    }
}

/// Returns markdown shown when the selected path is a directory.
fn directory_preview_markdown(path: &Path) -> String {
    format!(
        "# {}\n\nDirectory selected.\n\nPath: `{}`",
        path_title(path),
        path.display()
    )
}

/// Returns markdown shown when the selected file has no content.
fn empty_file_preview_markdown(path: &Path) -> String {
    format!(
        "# {}\n\nThis file is empty.\n\nPath: `{}`",
        path_title(path),
        path.display()
    )
}

/// Returns markdown shown when the selected file cannot be read as text.
fn unreadable_file_preview_markdown(path: &Path, error: &std::io::Error) -> String {
    format!(
        "# {}\n\nUnable to preview this file.\n\nPath: `{}`\n\nError: `{error}`",
        path_title(path),
        path.display()
    )
}

/// Returns a compact display title for a file-system path.
fn path_title(path: &Path) -> String {
    path.file_name()
        .and_then(|name| name.to_str())
        .filter(|name| !name.is_empty())
        .map(ToString::to_string)
        .unwrap_or_else(|| path.display().to_string())
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::file_preview_markdown_for_path;

    /// File preview content should come from the selected text file.
    #[test]
    fn reads_text_file_content() -> anyhow::Result<()> {
        let path = std::env::temp_dir().join(format!("ratsus-preview-{}.md", uuid::Uuid::new_v4()));
        fs::write(&path, "# Hello\n\nPreview")?;

        let content = file_preview_markdown_for_path(&path, false);

        let _ = fs::remove_file(&path);
        assert_eq!(content, "# Hello\n\nPreview");
        Ok(())
    }

    /// Directory preview should identify the selected directory path.
    #[test]
    fn describes_directory_selection() {
        let content = file_preview_markdown_for_path(std::path::Path::new("/tmp"), true);

        assert!(content.contains("Directory selected"));
        assert!(content.contains("/tmp"));
    }
}
