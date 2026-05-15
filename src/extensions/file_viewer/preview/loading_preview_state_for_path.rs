use std::path::Path;

use super::code_state_for_content::code_state_for_content;
use super::file_preview_state::FilePreviewState;
use super::loading_file_preview_text::loading_file_preview_text;

/// Builds a lightweight placeholder preview while file content loads asynchronously.
pub fn loading_preview_state_for_path(path: &Path) -> FilePreviewState {
    FilePreviewState::Code(Box::new(code_state_for_content(
        path,
        Ok(loading_file_preview_text(path)),
    )))
}
