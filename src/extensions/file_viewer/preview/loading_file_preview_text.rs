use std::path::Path;

/// Returns placeholder text while a selected file preview loads in the background.
pub fn loading_file_preview_text(path: &Path) -> String {
    format!("Loading preview…\n{}", path.display())
}
