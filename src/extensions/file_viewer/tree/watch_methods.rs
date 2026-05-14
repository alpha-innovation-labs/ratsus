use std::path::{Path, PathBuf};

use ratkit::widgets::file_system_tree::FileSystemTree;

#[cfg(test)]
use crate::extensions::file_viewer::preview::file_preview_state::FilePreviewState;
use crate::extensions::file_viewer::preview::preview_state_for_path::preview_state_for_path;
use crate::extensions::file_viewer::tree::view::FileSystemTreeView;
use crate::ui::left_panel::outcome::LeftPaneActionOutcome;

impl FileSystemTreeView {
    /// Polls Ratkit file watchers and refreshes visible file-viewer state after changes.
    pub(crate) fn poll_watchers(&mut self) -> bool {
        let preview_changed = self.poll_selected_file_watcher();
        let tree_changed = self.poll_root_watcher();
        preview_changed || tree_changed
    }

    /// Refreshes the preview when changed paths include the selected file.
    pub(crate) fn refresh_preview_for_changed_paths(&mut self, changed_paths: &[PathBuf]) -> bool {
        let Some(path) = self.selected_preview_path.clone() else {
            return false;
        };
        if self.selected_preview_is_dir || !changed_paths.iter().any(|changed| changed == &path) {
            return false;
        }
        self.preview_state = preview_state_for_path(&path, false);
        true
    }

    /// Rebuilds the root tree when root-level changed paths can affect visible rows.
    pub(crate) fn refresh_tree_for_changed_paths(&mut self, changed_paths: &[PathBuf]) -> bool {
        if !changed_paths
            .iter()
            .any(|changed| root_level_change(changed, &self.root_path))
        {
            return false;
        }
        self.refresh_tree_from_disk()
    }

    /// Returns the active code preview source text for tests.
    #[cfg(test)]
    pub(crate) fn code_preview_content(&self) -> Option<&str> {
        match &self.preview_state {
            FilePreviewState::Code(state) => Some(state.source.content()),
            FilePreviewState::Markdown(_) => None,
        }
    }

    /// Returns the root child names currently loaded in the tree for tests.
    #[cfg(test)]
    pub(crate) fn root_child_names(&self) -> Vec<String> {
        self.tree
            .nodes
            .first()
            .map(|node| {
                node.children
                    .iter()
                    .map(|child| child.data.name.clone())
                    .collect()
            })
            .unwrap_or_default()
    }

    /// Polls the selected-file watcher and refreshes the preview on changes.
    fn poll_selected_file_watcher(&mut self) -> bool {
        let Some(watcher) = self.selected_file_watcher.as_mut() else {
            return false;
        };
        if !watcher.check_for_changes() {
            return false;
        }
        let changed_paths = watcher.get_changed_paths();
        self.refresh_preview_for_changed_paths(&changed_paths)
    }

    /// Polls the root directory watcher and rebuilds the visible tree on root changes.
    fn poll_root_watcher(&mut self) -> bool {
        let Some(watcher) = self.root_watcher.as_mut() else {
            return false;
        };
        if !watcher.check_for_changes() {
            return false;
        }
        let changed_paths = watcher.get_changed_paths();
        self.refresh_tree_for_changed_paths(&changed_paths)
    }

    /// Rebuilds the tree from disk while preserving a matching selected path when possible.
    fn refresh_tree_from_disk(&mut self) -> bool {
        let selected_path = self.selected_preview_path.clone();
        let Ok(tree) = FileSystemTree::new(self.root_path.clone()) else {
            return false;
        };
        self.tree = tree;
        let selected_tree_path = selected_path
            .as_deref()
            .and_then(|path| tree_path_for_path(self, path))
            .unwrap_or_else(|| vec![0]);
        self.state.select(selected_tree_path);
        let _ = self.refresh_selection(LeftPaneActionOutcome::Handled);
        true
    }
}

/// Returns whether a changed path can affect root-level rows.
fn root_level_change(changed: &Path, root: &Path) -> bool {
    changed == root || changed.parent().is_some_and(|parent| parent == root)
}

/// Finds the loaded tree path for a file-system path.
fn tree_path_for_path(view: &FileSystemTreeView, target: &Path) -> Option<Vec<usize>> {
    view.tree.nodes.first().and_then(|node| {
        if node.data.path == target {
            return Some(vec![0]);
        }
        node.children
            .iter()
            .position(|child| child.data.path == target)
            .map(|index| vec![0, index])
    })
}

#[cfg(test)]
mod tests {
    use std::fs;

    use crate::extensions::file_viewer::tree::view::FileSystemTreeView;
    use crate::ui::left_panel::outcome::LeftPaneActionOutcome;

    /// Selected file changes should refresh preview content from disk.
    #[test]
    fn refreshes_preview_for_changed_selected_file() -> anyhow::Result<()> {
        let root = temp_root("selected_file_refresh")?;
        let file_path = root.join("alpha.txt");
        fs::write(&file_path, "before")?;
        let mut view = FileSystemTreeView::with_root(root.clone())?;
        view.select_path(vec![0, 0]);
        let _ = view.refresh_selection(LeftPaneActionOutcome::Handled);
        fs::write(&file_path, "after")?;

        let changed = view.refresh_preview_for_changed_paths(std::slice::from_ref(&file_path));

        assert!(changed);
        assert_eq!(view.code_preview_content(), Some("after"));
        let _ = fs::remove_dir_all(root);
        Ok(())
    }

    /// Root-level file changes should rebuild visible root rows.
    #[test]
    fn rebuilds_tree_for_root_level_changes() -> anyhow::Result<()> {
        let root = temp_root("root_tree_refresh")?;
        fs::write(root.join("alpha.txt"), "alpha")?;
        let mut view = FileSystemTreeView::with_root(root.clone())?;
        fs::write(root.join("beta.txt"), "beta")?;

        let changed = view.refresh_tree_for_changed_paths(&[root.join("beta.txt")]);

        assert!(changed);
        assert!(view.root_child_names().contains(&"beta.txt".to_string()));
        let _ = fs::remove_dir_all(root);
        Ok(())
    }

    /// Nested file changes should not rebuild root rows.
    #[test]
    fn ignores_nested_changes_for_root_rows() -> anyhow::Result<()> {
        let root = temp_root("nested_tree_refresh")?;
        fs::create_dir_all(root.join("nested"))?;
        let nested_file = root.join("nested").join("leaf.txt");
        fs::write(&nested_file, "leaf")?;
        let mut view = FileSystemTreeView::with_root(root.clone())?;

        let changed = view.refresh_tree_for_changed_paths(&[nested_file]);

        assert!(!changed);
        let _ = fs::remove_dir_all(root);
        Ok(())
    }

    /// Creates an isolated temporary tree root for watcher refresh tests.
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
