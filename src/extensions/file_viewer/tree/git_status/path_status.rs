use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use ratkit::services::repo_watcher::GitFileStatus;

use crate::extensions::file_viewer::tree::git_status::directory_status::directory_status;

/// Returns the git status that should be rendered for one file-tree row.
pub fn path_status(
    path: &Path,
    is_dir: bool,
    statuses: &BTreeMap<PathBuf, GitFileStatus>,
) -> Option<GitFileStatus> {
    if is_dir {
        return directory_status(path, statuses);
    }
    statuses.get(path).copied()
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::path::PathBuf;

    use ratkit::services::repo_watcher::GitFileStatus;

    use super::path_status;

    /// File status should require an exact path match.
    #[test]
    fn returns_exact_file_status() {
        let path = PathBuf::from("/repo/src/lib.rs");
        let mut statuses = BTreeMap::new();
        statuses.insert(path.clone(), GitFileStatus::Modified);

        assert_eq!(
            path_status(&path, false, &statuses),
            Some(GitFileStatus::Modified)
        );
    }
}
