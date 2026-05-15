use std::path::Path;

use super::code_state_for_content::code_state_for_content;
use super::file_preview_state::FilePreviewState;
use super::is_markdown_path::is_markdown_path;
use super::markdown_widget_for_content::markdown_widget_for_content;
use super::{
    empty_file_preview_text::empty_file_preview_text,
    unreadable_file_preview_text::unreadable_file_preview_text,
};

/// Builds preview state from file content loaded outside the UI thread.
pub fn preview_state_for_content(
    path: &Path,
    content: std::io::Result<String>,
) -> FilePreviewState {
    if is_markdown_path(path) {
        let content = match content {
            Ok(content) if content.is_empty() => empty_file_preview_text(path),
            Ok(content) => content,
            Err(error) => unreadable_file_preview_text(path, &error),
        };
        return FilePreviewState::Markdown(Box::new(markdown_widget_for_content(content)));
    }
    FilePreviewState::Code(Box::new(code_state_for_content(path, content)))
}
