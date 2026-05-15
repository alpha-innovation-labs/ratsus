use std::io;
use std::path::PathBuf;
use std::sync::mpsc::Receiver;

use ratatui::layout::Rect;
use ratatui::Frame;
use ratkit::services::file_watcher::FileWatcher;
use ratkit::widgets::file_system_tree::{FileSystemTree, FileSystemTreeState};

use crate::extensions::file_viewer::preview::file_preview_state::FilePreviewState;
use crate::extensions::file_viewer::preview::loading_preview_state_for_path::loading_preview_state_for_path;
use crate::extensions::file_viewer::preview::preview_state_for_path::preview_state_for_path;
use crate::extensions::file_viewer::tree::preview_load_result::PreviewLoadResult;
use crate::extensions::file_viewer::tree::spawn_preview_load_worker::spawn_preview_load_worker;
use crate::extensions::file_viewer::tree::start_root_watcher::start_root_watcher;
use crate::extensions::file_viewer::tree::start_selected_file_watcher::start_selected_file_watcher;
use crate::extensions::file_viewer::tree::tree_load_result::TreeLoadResult;
use crate::ui::keyboard::list::wrapped_position::wrapped_list_position;
use crate::ui::left_panel::outcome::LeftPaneActionOutcome;

/// File-system tree state copied from the Ratkit file system tree demo.
pub struct FileSystemTreeView {
    pub(crate) tree: FileSystemTree<'static>,
    pub(crate) state: FileSystemTreeState,
    pub(crate) root_path: PathBuf,
    pub(crate) last_selection: String,
    pub(crate) last_tree_area: Rect,
    pub(crate) preview_state: FilePreviewState,
    pub(crate) selected_preview_path: Option<PathBuf>,
    pub(crate) selected_preview_is_dir: bool,
    pub(crate) selected_file_watcher: Option<FileWatcher>,
    pub(crate) root_watcher: Option<FileWatcher>,
    pub(crate) preview_load_receiver: Option<Receiver<PreviewLoadResult>>,
    pub(crate) tree_load_receiver: Option<Receiver<TreeLoadResult>>,
    pending_g: bool,
}

impl FileSystemTreeView {
    /// Builds the file-system tree from the current working directory.
    pub fn new() -> io::Result<Self> {
        let root = std::env::current_dir().unwrap_or_else(|_| PathBuf::from("."));
        Self::with_root(root)
    }

    /// Builds the file-system tree from an explicit root directory.
    pub fn with_root(root: PathBuf) -> io::Result<Self> {
        let tree = FileSystemTree::new(root.clone())
            .map_err(|err| io::Error::new(io::ErrorKind::Other, err))?;
        let mut state = FileSystemTreeState::new();
        state.select(vec![0]);
        let preview_state = preview_state_for_path(&root, true);

        Ok(Self {
            tree,
            state,
            root_path: root.clone(),
            last_selection: root.display().to_string(),
            last_tree_area: Rect::default(),
            preview_state,
            selected_preview_path: Some(root.clone()),
            selected_preview_is_dir: true,
            selected_file_watcher: None,
            root_watcher: start_root_watcher(&root),
            preview_load_receiver: None,
            tree_load_receiver: None,
            pending_g: false,
        })
    }

    /// Returns the last selected path text shown in the left-pane footer.
    pub(crate) fn selected_status(&self) -> String {
        self.last_selection.clone()
    }

    /// Returns whether the file tree is waiting for a second `g` key.
    pub(crate) fn pending_g(&self) -> bool {
        self.pending_g
    }

    /// Updates whether the file tree is waiting for a second `g` key.
    pub(crate) fn set_pending_g(&mut self, pending: bool) {
        self.pending_g = pending;
    }

    /// Returns true when the file tree is accepting filter text.
    pub(crate) fn is_filtering(&self) -> bool {
        self.tree.is_filter_mode(&self.state)
    }

    /// Returns the active filter text when one exists.
    #[cfg(test)]
    pub(crate) fn filter_text(&self) -> Option<&str> {
        self.tree.filter_text(&self.state)
    }

    /// Returns the currently selected tree path.
    #[cfg(test)]
    pub(crate) fn selected_path(&self) -> Option<Vec<usize>> {
        self.state.selected_path.clone()
    }

    /// Selects one tree path without exposing Ratkit state.
    pub(crate) fn select_path(&mut self, path: Vec<usize>) {
        self.state.select(path);
    }

    /// Records the tree body area without exposing layout state fields.
    pub(crate) fn set_tree_area(&mut self, area: Rect) {
        self.last_tree_area = area;
    }

    /// Renders the Ratkit file tree without exposing the Ratkit state fields.
    pub(crate) fn render_tree_body(&mut self, frame: &mut Frame, area: Rect) {
        self.set_tree_area(area);
        let tree = self.tree.clone();
        frame.render_stateful_widget(tree, area, &mut self.state);
    }

    /// Collapses the selected tree node when possible.
    pub(crate) fn collapse_selected(&mut self) {
        self.tree.collapse_selected(&mut self.state);
    }

    /// Appends one character to the active file-tree filter.
    pub(crate) fn push_filter_character(&mut self, character: char) {
        self.state.push_filter(character);
    }

    /// Removes one character from the active file-tree filter.
    pub(crate) fn pop_filter_character(&mut self) {
        self.state.pop_filter();
    }

    /// Moves tree selection by a signed visible-row delta, wrapping at list edges.
    pub(crate) fn move_by(&mut self, direction: isize) {
        let visible_paths = self.tree.get_visible_paths(&self.state);
        if visible_paths.is_empty() {
            return;
        }
        let current = self
            .state
            .selected_path
            .as_ref()
            .and_then(|selected| visible_paths.iter().position(|path| path == selected))
            .unwrap_or(0);
        let next = wrapped_list_position(current, direction, visible_paths.len());
        self.state.select(visible_paths[next].clone());
    }

    /// Selects the first visible tree path when one exists.
    pub(crate) fn focus_first(&mut self) {
        if let Some(path) = self.tree.get_visible_paths(&self.state).first() {
            self.state.select(path.clone());
        }
    }

    /// Selects the last visible tree path when one exists.
    pub(crate) fn focus_last(&mut self) {
        if let Some(path) = self.tree.get_visible_paths(&self.state).last() {
            self.state.select(path.clone());
        }
    }

    /// Expands the selection or moves into its first visible child.
    pub(crate) fn expand_or_enter_child(&mut self) {
        if self.tree.expand_selected(&mut self.state).unwrap_or(false) {
            return;
        }
        if let Some(path) = self.state.selected_path.clone() {
            let mut first_child = path;
            first_child.push(0);
            if self.tree.get_entry_at_path(&first_child).is_some() {
                self.state.select(first_child);
            }
        }
    }

    /// Starts filter mode when the tree is not already filtering.
    pub(crate) fn start_filtering(&mut self) {
        if !self.tree.is_filter_mode(&self.state) {
            self.tree.enter_filter_mode(&mut self.state);
        }
    }

    /// Exits filter mode when filtering, otherwise asks the app to quit.
    pub(crate) fn quit_or_close_filter(&mut self) -> LeftPaneActionOutcome {
        if self.tree.is_filter_mode(&self.state) {
            self.state.exit_filter_mode();
            return LeftPaneActionOutcome::Handled;
        }
        LeftPaneActionOutcome::Quit
    }

    /// Refreshes selected path text and preview after tree state changes.
    pub(crate) fn refresh_selection(
        &mut self,
        outcome: LeftPaneActionOutcome,
    ) -> LeftPaneActionOutcome {
        if outcome == LeftPaneActionOutcome::Continue {
            return outcome;
        }
        let Some(entry) = self.tree.get_selected_entry(&self.state) else {
            return outcome;
        };
        let path = entry.path.clone();
        let is_dir = entry.is_dir;
        self.last_selection = path.display().to_string();
        self.selected_preview_path = Some(path.clone());
        self.selected_preview_is_dir = is_dir;
        self.selected_file_watcher = start_selected_file_watcher(&path, is_dir);
        if is_dir {
            self.preview_state = preview_state_for_path(&path, true);
            self.preview_load_receiver = None;
        } else {
            self.preview_state = loading_preview_state_for_path(&path);
            self.preview_load_receiver = Some(spawn_preview_load_worker(path));
        }
        outcome
    }
}
