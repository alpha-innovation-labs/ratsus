use std::path::Path;

use ratkit::widgets::markdown_preview::MarkdownWidget;

use super::empty_file_preview_text::empty_file_preview_text;
use super::markdown_widget_for_content::markdown_widget_for_content;
use super::unreadable_file_preview_text::unreadable_file_preview_text;

/// Builds a Markdown widget for a selected markdown file path.
pub fn markdown_widget_for_path(path: &Path) -> MarkdownWidget<'static> {
    let content = match std::fs::read_to_string(path) {
        Ok(content) if content.is_empty() => empty_file_preview_text(path),
        Ok(content) => content,
        Err(error) => unreadable_file_preview_text(path, &error),
    };
    markdown_widget_for_content(content)
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::markdown_widget_for_path;

    /// Markdown preview should load selected markdown file content.
    #[test]
    fn reads_markdown_file_into_widget() -> anyhow::Result<()> {
        let path = std::env::temp_dir().join(format!("ratsus-preview-{}.md", uuid::Uuid::new_v4()));
        fs::write(&path, "# Title")?;

        let widget = markdown_widget_for_path(&path);

        let _ = fs::remove_file(&path);
        assert!(widget.rendered_lines().is_empty());
        Ok(())
    }
}
