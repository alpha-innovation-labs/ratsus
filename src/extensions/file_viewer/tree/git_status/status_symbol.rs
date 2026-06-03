use ratkit::services::repo_watcher::GitFileStatus;

/// Returns the one-character git status label shown beside a file-tree row.
pub fn status_symbol(status: GitFileStatus) -> &'static str {
    match status {
        GitFileStatus::Added => "A",
        GitFileStatus::Modified => "M",
        GitFileStatus::Deleted => "D",
        GitFileStatus::Renamed => "R",
        GitFileStatus::Untracked => "U",
    }
}
