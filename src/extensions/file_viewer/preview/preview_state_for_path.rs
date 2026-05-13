use std::path::Path;

use super::code_state_for_path::code_state_for_path;
use super::file_preview_state::FilePreviewState;
use super::is_markdown_path::is_markdown_path;
use super::markdown_widget_for_path::markdown_widget_for_path;

/// Builds the correct preview widget state for the selected file-system path.
pub fn preview_state_for_path(path: &Path, is_dir: bool) -> FilePreviewState {
    if !is_dir && is_markdown_path(path) {
        return FilePreviewState::Markdown(Box::new(markdown_widget_for_path(path)));
    }
    FilePreviewState::Code(code_state_for_path(path, is_dir))
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::preview_state_for_path;
    use crate::extensions::file_viewer::preview::file_preview_state::FilePreviewState;

    /// Markdown files should be routed to Ratkit's Markdown widget.
    #[test]
    fn routes_markdown_files_to_markdown_widget() -> anyhow::Result<()> {
        let path = std::env::temp_dir().join(format!("ratsus-preview-{}.md", uuid::Uuid::new_v4()));
        fs::write(&path, "# Title")?;

        let preview = preview_state_for_path(&path, false);

        let _ = fs::remove_file(&path);
        assert!(matches!(preview, FilePreviewState::Markdown(_)));
        Ok(())
    }

    /// Non-markdown files should stay on Ratkit's Code widget.
    #[test]
    fn routes_code_files_to_code_widget() -> anyhow::Result<()> {
        let path = std::env::temp_dir().join(format!("ratsus-preview-{}.rs", uuid::Uuid::new_v4()));
        fs::write(&path, "fn main() {}")?;

        let preview = preview_state_for_path(&path, false);

        let _ = fs::remove_file(&path);
        assert!(matches!(preview, FilePreviewState::Code(_)));
        Ok(())
    }
}
