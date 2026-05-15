use std::path::PathBuf;

/// File preview content loaded by a background worker.
pub struct PreviewLoadResult {
    pub path: PathBuf,
    pub content: std::io::Result<String>,
}
