use std::path::PathBuf;

use ratkit::services::file_watcher::FileWatcher;

use crate::extensions::plans::watch::start_plan_directory_watcher::start_plan_directory_watcher;

/// Starts recursive Ratkit watchers for every workspace `plans` directory.
pub fn start_plan_directory_watchers(
    root_path: &std::path::Path,
    workspace_folders: &[PathBuf],
) -> Vec<FileWatcher> {
    plan_directories(root_path, workspace_folders)
        .into_iter()
        .filter_map(|plans_dir| start_plan_directory_watcher(&plans_dir))
        .collect()
}

/// Returns watched `plans` directories for the current plan-list scope.
fn plan_directories(root_path: &std::path::Path, workspace_folders: &[PathBuf]) -> Vec<PathBuf> {
    if workspace_folders.is_empty() {
        return vec![root_path.join("plans")];
    }
    workspace_folders
        .iter()
        .map(|workspace_folder| workspace_folder.join("plans"))
        .collect()
}
