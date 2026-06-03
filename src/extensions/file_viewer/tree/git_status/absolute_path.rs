use std::path::{Path, PathBuf};

/// Returns an absolute path when the filesystem can resolve it, otherwise preserves the input path.
pub fn absolute_path(path: &Path) -> PathBuf {
    path.canonicalize().unwrap_or_else(|_| path.to_path_buf())
}
