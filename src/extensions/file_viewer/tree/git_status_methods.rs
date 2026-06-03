use std::path::Path;

use ratkit::services::repo_watcher::GitFileStatus;

use crate::extensions::file_viewer::tree::view::FileSystemTreeView;

impl FileSystemTreeView {
    /// Returns no git status because file trees do not maintain repository watchers.
    pub(crate) fn git_status_for_path(&self, _path: &Path, _is_dir: bool) -> Option<GitFileStatus> {
        None
    }
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::extensions::file_viewer::tree::view::FileSystemTreeView;

    /// File-tree git status should stay disabled because file panes do not watch repositories.
    #[test]
    fn does_not_report_git_status_without_file_watchers() -> anyhow::Result<()> {
        let root = temp_root("git_status_disabled")?;
        let file_path = root.join("new.txt");
        fs::write(&file_path, "new")?;
        let view = FileSystemTreeView::with_root(root.clone())?;

        assert_eq!(view.git_status_for_path(&file_path, false), None);
        let _ = fs::remove_dir_all(root);
        Ok(())
    }

    /// Creates a unique temporary root for git status tests.
    fn temp_root(name: &str) -> anyhow::Result<std::path::PathBuf> {
        let root = std::env::temp_dir().join(format!(
            "ratsus_{name}_{}_{}",
            std::process::id(),
            uuid::Uuid::new_v4()
        ));
        fs::create_dir_all(&root)?;
        Ok(root)
    }
}
