use std::io;
use std::path::{Path, PathBuf};

use ratkit::widgets::file_system_tree::{FileSystemTree, FileSystemTreeNode, FileSystemTreeState};

use crate::extensions::file_viewer::preview::preview_state_for_path::preview_state_for_path;
use crate::extensions::file_viewer::tree::view::FileSystemTreeView;

impl FileSystemTreeView {
    /// Returns the root directory currently shown by the file tree.
    pub(crate) fn root_path(&self) -> &Path {
        &self.root_path
    }

    /// Returns expanded directory paths for persistence.
    pub(crate) fn expanded_directory_paths(&self) -> Vec<PathBuf> {
        let mut paths = self
            .state
            .expanded
            .iter()
            .filter_map(|tree_path| self.tree.get_entry_at_path(tree_path))
            .filter(|entry| entry.is_dir)
            .map(|entry| entry.path.clone())
            .collect::<Vec<_>>();
        paths.sort();
        paths.dedup();
        paths
    }

    /// Replaces the tree root and restores persisted expanded folders under it.
    pub(crate) fn replace_root(
        &mut self,
        root: PathBuf,
        expanded_paths: &[PathBuf],
    ) -> io::Result<()> {
        let tree = FileSystemTree::new(root.clone())
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
        self.tree = tree;
        self.state = FileSystemTreeState::new();
        self.state.select(vec![0]);
        self.root_path = root.clone();
        self.last_selection = root.display().to_string();
        self.preview_state = preview_state_for_path(&root, true);
        self.selected_preview_path = Some(root.clone());
        self.selected_preview_is_dir = true;
        self.preview_load_receiver = None;
        self.tree_load_receiver = None;
        self.workspace_roots = vec![root.clone()];
        self.apply_expanded_directory_paths(expanded_paths);
        Ok(())
    }

    /// Restores expanded folder paths into the current tree root.
    pub(crate) fn apply_expanded_directory_paths(&mut self, expanded_paths: &[PathBuf]) {
        self.state.expand(vec![0]);
        let mut paths = expanded_paths.to_vec();
        paths.sort_by_key(|path| path.components().count());
        for path in paths {
            let Some(tree_path) = tree_path_for_directory_path(self, &path) else {
                continue;
            };
            let _ = self.tree.expand_directory(&tree_path);
            self.state.expand(tree_path);
        }
    }
}

/// Finds the tree-index path for one directory path, loading parents as needed.
fn tree_path_for_directory_path(
    view: &mut FileSystemTreeView,
    target: &Path,
) -> Option<Vec<usize>> {
    if target == view.root_path {
        return Some(vec![0]);
    }
    let relative = target.strip_prefix(&view.root_path).ok()?;
    let mut tree_path = vec![0];
    let mut current_path = view.root_path.clone();
    for component in relative.components() {
        view.tree.expand_directory(&tree_path).ok()?;
        current_path.push(component.as_os_str());
        let parent = node_at_path(&view.tree.nodes, &tree_path)?;
        let child_index = parent
            .children
            .iter()
            .position(|child| child.data.path == current_path && child.data.is_dir)?;
        tree_path.push(child_index);
    }
    Some(tree_path)
}

/// Returns the file tree node at one tree-index path.
fn node_at_path<'a>(
    nodes: &'a [FileSystemTreeNode],
    path: &[usize],
) -> Option<&'a FileSystemTreeNode> {
    let (first, rest) = path.split_first()?;
    let node = nodes.get(*first)?;
    if rest.is_empty() {
        return Some(node);
    }
    node_at_path(&node.children, rest)
}
