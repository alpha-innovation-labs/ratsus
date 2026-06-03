use std::path::PathBuf;

/// One Markdown plan discovered under the workspace `plans` directory.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PlanEntry {
    pub path: PathBuf,
    pub folder: PathBuf,
    pub title: String,
}

impl PlanEntry {
    /// Builds a plan entry from its filesystem path, grouping folder, and display title.
    pub fn new(path: PathBuf, folder: PathBuf, title: String) -> Self {
        Self {
            path,
            folder,
            title,
        }
    }
}
