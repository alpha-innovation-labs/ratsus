use std::path::Path;

use super::path_title::path_title;

/// Returns plain text shown when the selected file is empty.
pub(super) fn empty_file_preview_text(path: &Path) -> String {
    format!(
        "{}\n\nThis file is empty.\n\nPath: {}",
        path_title(path),
        path.display()
    )
}
