use std::path::Path;

use ratkit::widgets::code_widget::CodeState;

use super::configured_code_state::configured_code_state;
use super::directory_preview_text::directory_preview_text;
use super::empty_file_preview_text::empty_file_preview_text;
use super::unreadable_file_preview_text::unreadable_file_preview_text;

/// Builds a Ratkit code-widget state for the selected file-system path.
pub fn code_state_for_path(path: &Path, is_dir: bool) -> CodeState {
    let mut state = configured_code_state();
    if is_dir {
        state.source.set_source_string(directory_preview_text(path));
        return state;
    }
    match state.source.set_source_file(path) {
        Ok(()) if state.source.content().is_empty() => {
            state
                .source
                .set_source_string(empty_file_preview_text(path));
        }
        Ok(()) => {}
        Err(error) => state
            .source
            .set_source_string(unreadable_file_preview_text(path, &error)),
    }
    state
}

#[cfg(test)]
mod tests {
    use std::fs;

    use super::code_state_for_path;

    /// File preview code state should load content from the selected text file.
    #[test]
    fn reads_text_file_into_code_state() -> anyhow::Result<()> {
        let path = std::env::temp_dir().join(format!("ratsus-preview-{}.rs", uuid::Uuid::new_v4()));
        fs::write(&path, "fn main() {}")?;

        let state = code_state_for_path(&path, false);

        let _ = fs::remove_file(&path);
        assert_eq!(state.source.content(), "fn main() {}");
        assert_eq!(state.source.source_path(), Some(path.as_path()));
        Ok(())
    }

    /// Directory preview should identify the selected directory path as plain text.
    #[test]
    fn describes_directory_selection_as_code_state() {
        let state = code_state_for_path(std::path::Path::new("/tmp"), true);

        assert!(state.source.content().contains("Directory selected"));
        assert!(state.source.content().contains("/tmp"));
        assert_eq!(state.source.source_path(), None);
    }

    /// Empty file preview should render a visible plain-text message.
    #[test]
    fn describes_empty_file_as_code_state() -> anyhow::Result<()> {
        let path =
            std::env::temp_dir().join(format!("ratsus-preview-{}.txt", uuid::Uuid::new_v4()));
        fs::write(&path, "")?;

        let state = code_state_for_path(&path, false);

        let _ = fs::remove_file(&path);
        assert!(state.source.content().contains("This file is empty"));
        assert_eq!(state.source.source_path(), None);
        Ok(())
    }
}
