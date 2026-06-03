use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use ratkit::services::repo_watcher::GitFileStatus;

use crate::extensions::file_viewer::tree::git_status::is_path_under::is_path_under;
use crate::extensions::file_viewer::tree::git_status::status_precedence::status_precedence;

/// Returns the highest-priority git status for a directory or any changed descendant.
pub fn directory_status(
    path: &Path,
    statuses: &BTreeMap<PathBuf, GitFileStatus>,
) -> Option<GitFileStatus> {
    statuses
        .iter()
        .filter(|(changed_path, _)| is_path_under(changed_path, path))
        .map(|(_, status)| *status)
        .max_by_key(|status| status_precedence(*status))
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeMap;
    use std::path::PathBuf;

    use ratkit::services::repo_watcher::GitFileStatus;

    use super::directory_status;

    /// Directory status should aggregate matching descendant statuses.
    #[test]
    fn returns_highest_priority_descendant_status() {
        let mut statuses = BTreeMap::new();
        statuses.insert(PathBuf::from("/repo/src/new.rs"), GitFileStatus::Untracked);
        statuses.insert(PathBuf::from("/repo/src/lib.rs"), GitFileStatus::Modified);

        let status = directory_status(&PathBuf::from("/repo/src"), &statuses);

        assert_eq!(status, Some(GitFileStatus::Modified));
    }
}
