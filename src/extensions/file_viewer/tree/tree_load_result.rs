use std::path::PathBuf;

use ratkit::widgets::file_system_tree::FileSystemTree;

/// Root file tree loaded by a background worker.
pub struct TreeLoadResult {
    pub root: PathBuf,
    pub tree: std::io::Result<FileSystemTree<'static>>,
}
