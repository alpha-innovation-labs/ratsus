use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use ratkit::services::repo_watcher::{GitChangeSet, GitFileStatus};

/// Builds an absolute-path status map from one Ratkit repository change set.
pub fn change_set_status_map(
    repo_root: &Path,
    change_set: &GitChangeSet,
) -> BTreeMap<PathBuf, GitFileStatus> {
    let mut statuses = BTreeMap::new();
    insert_paths(
        &mut statuses,
        repo_root,
        &change_set.added,
        GitFileStatus::Added,
    );
    insert_paths(
        &mut statuses,
        repo_root,
        &change_set.modified,
        GitFileStatus::Modified,
    );
    insert_paths(
        &mut statuses,
        repo_root,
        &change_set.deleted,
        GitFileStatus::Deleted,
    );
    insert_paths(
        &mut statuses,
        repo_root,
        &change_set.renamed,
        GitFileStatus::Renamed,
    );
    insert_paths(
        &mut statuses,
        repo_root,
        &change_set.untracked,
        GitFileStatus::Untracked,
    );
    statuses
}

/// Inserts relative change-set paths into an absolute-path status map.
fn insert_paths(
    statuses: &mut BTreeMap<PathBuf, GitFileStatus>,
    repo_root: &Path,
    paths: &[PathBuf],
    status: GitFileStatus,
) {
    for path in paths {
        statuses.insert(repo_root.join(path), status);
    }
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use ratkit::services::repo_watcher::{GitChangeSet, GitFileStatus};

    use super::change_set_status_map;

    /// Change-set paths should become absolute paths keyed by repo root.
    #[test]
    fn converts_relative_change_set_paths_to_absolute_keys() {
        let repo_root = PathBuf::from("/repo");
        let change_set = GitChangeSet {
            modified: vec![PathBuf::from("src/main.rs")],
            untracked: vec![PathBuf::from("README.md")],
            ..GitChangeSet::default()
        };

        let statuses = change_set_status_map(&repo_root, &change_set);

        assert_eq!(
            statuses.get(&repo_root.join("src/main.rs")),
            Some(&GitFileStatus::Modified)
        );
        assert_eq!(
            statuses.get(&repo_root.join("README.md")),
            Some(&GitFileStatus::Untracked)
        );
    }
}
