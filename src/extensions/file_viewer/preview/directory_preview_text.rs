use std::path::Path;

use super::path_title::path_title;

/// Returns plain text shown when the selected path is a directory.
pub(super) fn directory_preview_text(path: &Path) -> String {
    format!(
        "{}\n\nDirectory selected.\n\nPath: {}",
        path_title(path),
        path.display()
    )
}
