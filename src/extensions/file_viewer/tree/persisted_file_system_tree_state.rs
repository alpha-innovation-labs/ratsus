use std::collections::BTreeMap;
use std::path::PathBuf;

use serde::{Deserialize, Serialize};

use crate::extensions::file_viewer::tree::view::FileSystemTreeView;

/// Serializable file-viewer folder expansion state keyed by workspace root.
#[derive(Debug, Clone, Default, PartialEq, Eq, Serialize, Deserialize)]
pub struct PersistedFileSystemTreeState {
    #[serde(default)]
    pub expanded_paths_by_root: BTreeMap<PathBuf, Vec<PathBuf>>,
    #[serde(default)]
    pub workspace_expanded_paths: Vec<PathBuf>,
    #[serde(default)]
    pub workspace_collapsed_paths: Vec<PathBuf>,
    #[serde(default)]
    pub selected_path: Option<PathBuf>,
}

/// Captures persisted file-viewer state while merging the active tree view.
pub fn persisted_file_system_tree_state(
    stored_paths: &BTreeMap<PathBuf, Vec<PathBuf>>,
    view: &FileSystemTreeView,
) -> PersistedFileSystemTreeState {
    let mut expanded_paths_by_root = stored_paths.clone();
    expanded_paths_by_root.insert(
        view.root_path().to_path_buf(),
        view.expanded_directory_paths(),
    );
    PersistedFileSystemTreeState {
        expanded_paths_by_root,
        workspace_expanded_paths: view.workspace_expanded_directory_paths(),
        workspace_collapsed_paths: view.workspace_collapsed_directory_paths(),
        selected_path: view.selected_preview_path.clone(),
    }
}
