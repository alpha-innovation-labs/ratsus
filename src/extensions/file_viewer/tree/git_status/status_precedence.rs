use ratkit::services::repo_watcher::GitFileStatus;

/// Returns display precedence for a git status when multiple descendant states apply.
pub fn status_precedence(status: GitFileStatus) -> u8 {
    match status {
        GitFileStatus::Modified => 5,
        GitFileStatus::Added => 4,
        GitFileStatus::Renamed => 3,
        GitFileStatus::Deleted => 2,
        GitFileStatus::Untracked => 1,
    }
}
