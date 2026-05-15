use std::path::PathBuf;

/// Deferred text write identified by a stable coalescing key.
pub struct SaveJob {
    pub key: &'static str,
    pub path: PathBuf,
    pub content: String,
}
