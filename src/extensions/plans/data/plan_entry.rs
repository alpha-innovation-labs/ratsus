use std::path::PathBuf;

/// One Markdown plan discovered under the workspace `plans` directory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanEntry {
    pub path: PathBuf,
    pub title: String,
}

impl PlanEntry {
    /// Builds a plan entry from its filesystem path and display title.
    pub fn new(path: PathBuf, title: String) -> Self {
        Self { path, title }
    }
}
