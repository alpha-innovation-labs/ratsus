use std::path::{Path, PathBuf};
use std::process::Command;

/// Returns the top-level git repository root for a path when git can resolve one.
pub fn repo_root_for_path(path: &Path) -> Option<PathBuf> {
    let output = Command::new("git")
        .args(["-C", path.to_str()?, "rev-parse", "--show-toplevel"])
        .output()
        .ok()?;
    if !output.status.success() {
        return None;
    }
    let root = String::from_utf8(output.stdout).ok()?;
    let root = root.trim();
    (!root.is_empty()).then(|| PathBuf::from(root))
}

#[cfg(test)]
mod tests {
    use std::path::PathBuf;

    use super::repo_root_for_path;

    /// Non-repository paths should not produce a git repo root.
    #[test]
    fn returns_none_for_non_repository_path() {
        let path = PathBuf::from("/definitely/not/a/ratsus/git/repository/path");

        assert_eq!(repo_root_for_path(&path), None);
    }
}
