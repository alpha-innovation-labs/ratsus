use std::io;
use std::path::PathBuf;

use ratatui::layout::Rect;
use ratatui::Frame;
use ratkit::widgets::file_system_tree::{FileSystemTree, FileSystemTreeState};
use ratkit::widgets::markdown_preview::MarkdownWidget;

use crate::extensions::file_viewer::preview::markdown_for_path::file_preview_markdown_for_path;
use crate::extensions::file_viewer::preview::markdown_widget_for_content::markdown_widget_for_content;
use crate::ui::left_panel::outcome::LeftPaneActionOutcome;

/// File-system tree state copied from the Ratkit file system tree demo.
pub struct FileSystemTreeView {
    pub(super) tree: FileSystemTree<'static>,
    pub(super) state: FileSystemTreeState,
    last_selection: String,
    pub(super) last_tree_area: Rect,
    pub(super) preview: MarkdownWidget<'static>,
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
        let preview = markdown_widget_for_content(file_preview_markdown_for_path(&root, true));

        Ok(Self {
            tree,
            state,
            last_selection: root.display().to_string(),
            last_tree_area: Rect::default(),
            preview,
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

    /// Moves tree selection by repeatedly applying one-step movement.
    pub(crate) fn move_by(&mut self, direction: isize) {
        for _ in 0..direction.unsigned_abs() {
            if direction.is_positive() {
                self.tree.select_next(&mut self.state);
            } else {
                self.tree.select_previous(&mut self.state);
            }
        }
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
        self.preview = markdown_widget_for_content(file_preview_markdown_for_path(&path, is_dir));
        outcome
    }
}
