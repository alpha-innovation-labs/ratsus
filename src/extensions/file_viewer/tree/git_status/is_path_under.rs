use std::path::Path;

/// Returns true when `path` is equal to `ancestor` or lives below it.
pub fn is_path_under(path: &Path, ancestor: &Path) -> bool {
    path == ancestor || path.starts_with(ancestor)
}
