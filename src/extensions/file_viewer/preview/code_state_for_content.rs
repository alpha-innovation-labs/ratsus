use std::path::Path;

use ratkit::widgets::code_widget::CodeState;

use super::configured_code_state::configured_code_state;
use super::empty_file_preview_text::empty_file_preview_text;
use super::unreadable_file_preview_text::unreadable_file_preview_text;

/// Builds code preview state from content that was loaded off the UI thread.
pub fn code_state_for_content(path: &Path, content: std::io::Result<String>) -> CodeState {
    let mut state = configured_code_state();
    match content {
        Ok(content) if content.is_empty() => {
            state
                .source
                .set_source_string(empty_file_preview_text(path));
        }
        Ok(content) => {
            state.source.set_source_string(content);
        }
        Err(error) => state
            .source
            .set_source_string(unreadable_file_preview_text(path, &error)),
    }
    state
}
