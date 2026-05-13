use std::io;
use std::path::Path;

use super::path_title::path_title;

/// Returns plain text shown when the selected file cannot be read as text.
pub(super) fn unreadable_file_preview_text(path: &Path, error: &io::Error) -> String {
    format!(
        "{}\n\nUnable to preview this file as UTF-8 text.\n\nPath: {}\n\nError: {}",
        path_title(path),
        path.display(),
        error
    )
}
