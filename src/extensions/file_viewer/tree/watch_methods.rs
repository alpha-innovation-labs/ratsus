use std::path::Path;
#[cfg(test)]
use std::path::PathBuf;
use std::sync::mpsc::TryRecvError;

#[cfg(test)]
use crate::extensions::file_viewer::preview::file_preview_state::FilePreviewState;
#[cfg(test)]
use crate::extensions::file_viewer::preview::loading_preview_state_for_path::loading_preview_state_for_path;
use crate::extensions::file_viewer::preview::preview_state_for_content::preview_state_for_content;
#[cfg(test)]
use crate::extensions::file_viewer::tree::spawn_preview_load_worker::spawn_preview_load_worker;
#[cfg(test)]
use crate::extensions::file_viewer::tree::spawn_tree_load_worker::spawn_tree_load_worker;
use crate::extensions::file_viewer::tree::view::FileSystemTreeView;
use crate::ui::left_panel::outcome::LeftPaneActionOutcome;

impl FileSystemTreeView {
    /// Drains asynchronous file-preview and tree-load workers without filesystem watching.
    pub(crate) fn poll_watchers(&mut self) -> bool {
        let loaded_preview_changed = self.drain_preview_load_receiver();
        let loaded_tree_changed = self.drain_tree_load_receiver();
        loaded_preview_changed || loaded_tree_changed
    }

    /// Refreshes the preview when changed paths include the selected file.
    #[cfg(test)]
    pub(crate) fn refresh_preview_for_changed_paths(&mut self, changed_paths: &[PathBuf]) -> bool {
        let Some(path) = self.selected_preview_path.clone() else {
            return false;
        };
        if self.selected_preview_is_dir || !changed_paths.iter().any(|changed| changed == &path) {
            return false;
        }
        self.preview_state = loading_preview_state_for_path(&path);
        self.preview_load_receiver = Some(spawn_preview_load_worker(path));
        true
    }

    /// Rebuilds the root tree when root-level changed paths can affect visible rows.
    #[cfg(test)]
    pub(crate) fn refresh_tree_for_changed_paths(&mut self, changed_paths: &[PathBuf]) -> bool {
        if !changed_paths
            .iter()
            .any(|changed| root_level_change(changed, &self.root_path))
        {
            return false;
        }
        if self.tree_load_receiver.is_none() {
            self.tree_load_receiver = Some(spawn_tree_load_worker(self.root_path.clone()));
        }
        true
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

    /// Applies a completed preview load when it still matches the selected path.
    fn drain_preview_load_receiver(&mut self) -> bool {
        let Some(receiver) = self.preview_load_receiver.as_ref() else {
            return false;
        };
        match receiver.try_recv() {
            Ok(result) => {
                self.preview_load_receiver = None;
                if self.selected_preview_path.as_ref() != Some(&result.path) {
                    return false;
                }
                self.preview_state = preview_state_for_content(&result.path, result.content);
                true
            }
            Err(TryRecvError::Empty) => false,
            Err(TryRecvError::Disconnected) => {
                self.preview_load_receiver = None;
                false
            }
        }
    }

    /// Applies a completed root tree load while preserving matching selection.
    fn drain_tree_load_receiver(&mut self) -> bool {
        let Some(receiver) = self.tree_load_receiver.as_ref() else {
            return false;
        };
        match receiver.try_recv() {
            Ok(result) => {
                self.tree_load_receiver = None;
                if result.root != self.root_path {
                    return false;
                }
                let Ok(tree) = result.tree else {
                    return false;
                };
                self.tree = tree;
                self.restore_selection_after_tree_load();
                true
            }
            Err(TryRecvError::Empty) => false,
            Err(TryRecvError::Disconnected) => {
                self.tree_load_receiver = None;
                false
            }
        }
    }

    /// Restores selected path after an asynchronous root tree load.
    fn restore_selection_after_tree_load(&mut self) {
        let selected_path = self.selected_preview_path.clone();
        let selected_tree_path = selected_path
            .as_deref()
            .and_then(|path| tree_path_for_path(self, path))
            .unwrap_or_else(|| vec![0]);
        self.state.select(selected_tree_path);
        let _ = self.refresh_selection(LeftPaneActionOutcome::Handled);
    }
}

/// Returns whether a changed path can affect root-level rows.
#[cfg(test)]
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
