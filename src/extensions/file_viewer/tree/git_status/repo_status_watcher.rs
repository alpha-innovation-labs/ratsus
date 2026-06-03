use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use ratkit::services::repo_watcher::{GitFileStatus, RepoWatcher};

use crate::extensions::file_viewer::tree::git_status::absolute_path::absolute_path;
use crate::extensions::file_viewer::tree::git_status::change_set_status_map::change_set_status_map;
use crate::extensions::file_viewer::tree::git_status::path_status::path_status;

/// Tracks one repository watcher and its latest absolute git status map.
pub struct RepoStatusWatcher {
    pub(crate) repo_root: PathBuf,
    pub(crate) watcher: RepoWatcher,
    pub(crate) statuses: BTreeMap<PathBuf, GitFileStatus>,
}

impl RepoStatusWatcher {
    /// Builds a repository status watcher from an initialized Ratkit repo watcher.
    pub(crate) fn new(repo_root: PathBuf, mut watcher: RepoWatcher) -> Self {
        let change_set = watcher.get_change_set();
        let statuses = change_set_status_map(&repo_root, &change_set);
        Self {
            repo_root,
            watcher,
            statuses,
        }
    }

    /// Refreshes the cached status map when Ratkit reports repository changes.
    pub(crate) fn refresh(&mut self) -> bool {
        if !self.watcher.check_for_changes() {
            return false;
        }
        let change_set = self.watcher.get_change_set();
        self.statuses = change_set_status_map(&self.repo_root, &change_set);
        true
    }

    /// Returns the visible git status for one path when it belongs to this repository.
    pub(crate) fn status_for_path(&self, path: &Path, is_dir: bool) -> Option<GitFileStatus> {
        let path = absolute_path(path);
        if !path.starts_with(&self.repo_root) {
            return None;
        }
        path_status(&path, is_dir, &self.statuses)
    }
}
