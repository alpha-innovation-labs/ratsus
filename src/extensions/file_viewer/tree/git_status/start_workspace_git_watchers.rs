use std::path::PathBuf;

use crate::extensions::file_viewer::tree::git_status::repo_status_watcher::RepoStatusWatcher;
use crate::extensions::file_viewer::tree::git_status::start_repo_status_watcher::start_repo_status_watcher;

/// Starts repository status watchers for all workspace roots that belong to git repositories.
pub fn start_workspace_git_watchers(roots: &[PathBuf]) -> Vec<RepoStatusWatcher> {
    roots
        .iter()
        .filter_map(|root| start_repo_status_watcher(root))
        .collect()
}
