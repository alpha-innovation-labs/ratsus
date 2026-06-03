use std::path::Path;

use ratkit::services::repo_watcher::RepoWatcher;

use crate::extensions::file_viewer::tree::git_status::absolute_path::absolute_path;
use crate::extensions::file_viewer::tree::git_status::repo_root_for_path::repo_root_for_path;
use crate::extensions::file_viewer::tree::git_status::repo_status_watcher::RepoStatusWatcher;

/// Starts a Ratkit repository watcher for the git repo containing a workspace root.
pub fn start_repo_status_watcher(root: &Path) -> Option<RepoStatusWatcher> {
    let repo_root = repo_root_for_path(root)?;
    let repo_root = absolute_path(&repo_root);
    let mut watcher = RepoWatcher::new().ok()?;
    watcher.watch(&repo_root).ok()?;
    Some(RepoStatusWatcher::new(repo_root, watcher))
}
