use std::path::PathBuf;

use crate::extensions::file_viewer::preview::loading_preview_state_for_path::loading_preview_state_for_path;
use crate::extensions::file_viewer::preview::preview_state_for_path::preview_state_for_path;
use crate::extensions::file_viewer::tree::spawn_preview_load_worker::spawn_preview_load_worker;
use crate::extensions::file_viewer::tree::view::FileSystemTreeView;
use crate::extensions::file_viewer::tree::workspace_file_row::WorkspaceFileRow;
use crate::ui::keyboard::list::wrapped_position::wrapped_list_position;
use crate::ui::left_panel::outcome::LeftPaneActionOutcome;

impl FileSystemTreeView {
    /// Returns expanded grouped workspace directory paths for persistence.
    pub(crate) fn workspace_expanded_directory_paths(&self) -> Vec<PathBuf> {
        sorted_paths(&self.workspace_expanded_paths)
    }

    /// Returns collapsed grouped workspace root paths for persistence.
    pub(crate) fn workspace_collapsed_directory_paths(&self) -> Vec<PathBuf> {
        sorted_paths(&self.workspace_collapsed_paths)
    }

    /// Restores grouped workspace directory expansion and collapse state.
    pub(crate) fn apply_workspace_open_state(
        &mut self,
        expanded_paths: &[PathBuf],
        collapsed_paths: &[PathBuf],
    ) {
        self.workspace_expanded_paths = expanded_paths.iter().cloned().collect();
        self.workspace_collapsed_paths = collapsed_paths.iter().cloned().collect();
        self.workspace_focused_row = self
            .workspace_focused_row
            .min(self.workspace_rows().len().saturating_sub(1));
    }

    /// Synchronizes the workspace roots shown in the grouped file tree.
    pub(crate) fn sync_workspace_roots(&mut self, roots: &[PathBuf]) {
        if self.workspace_roots == roots {
            return;
        }
        self.workspace_roots = roots.to_vec();
        self.workspace_focused_row = 0;
        self.workspace_scroll = 0;
        self.activate_workspace_focused_row();
    }

    /// Moves grouped file-tree focus by a signed visible-row delta.
    pub(crate) fn move_workspace_by(&mut self, direction: isize) {
        let rows = self.workspace_rows();
        if rows.is_empty() {
            self.workspace_focused_row = 0;
            return;
        }
        self.workspace_focused_row =
            wrapped_list_position(self.workspace_focused_row, direction, rows.len());
        self.activate_workspace_focused_row();
    }

    /// Focuses the first visible grouped file-tree row.
    pub(crate) fn focus_workspace_first(&mut self) {
        self.workspace_focused_row = 0;
        self.activate_workspace_focused_row();
    }

    /// Focuses the last visible grouped file-tree row.
    pub(crate) fn focus_workspace_last(&mut self) {
        self.workspace_focused_row = self.workspace_rows().len().saturating_sub(1);
        self.activate_workspace_focused_row();
    }

    /// Collapses the focused grouped directory row.
    pub(crate) fn collapse_workspace_selected(&mut self) {
        match self.focused_workspace_path() {
            Some((path, true)) if self.workspace_roots.contains(&path) => {
                self.workspace_collapsed_paths.insert(path);
            }
            Some((path, true)) => {
                self.workspace_expanded_paths.remove(&path);
            }
            _ => {}
        }
    }

    /// Expands the focused directory or activates the focused file.
    pub(crate) fn expand_or_enter_workspace_child(&mut self) {
        match self.focused_workspace_path() {
            Some((path, true)) if self.workspace_roots.contains(&path) => {
                self.workspace_collapsed_paths.remove(&path);
            }
            Some((path, true)) => {
                self.workspace_expanded_paths.insert(path);
            }
            Some((_, false)) => self.activate_workspace_focused_row(),
            None => {}
        }
    }

    /// Starts grouped file-tree filtering.
    pub(crate) fn start_workspace_filtering(&mut self) {
        self.workspace_filtering = true;
    }

    /// Appends one character to the grouped file-tree filter.
    pub(crate) fn push_workspace_filter_character(&mut self, character: char) {
        self.workspace_filter_query.push(character);
        self.workspace_focused_row = 0;
        self.activate_workspace_focused_row();
    }

    /// Removes one character from the grouped file-tree filter.
    pub(crate) fn pop_workspace_filter_character(&mut self) {
        self.workspace_filter_query.pop();
        self.workspace_focused_row = 0;
        self.activate_workspace_focused_row();
    }

    /// Exits grouped file filtering, otherwise asks the app to quit.
    pub(crate) fn quit_or_close_workspace_filter(&mut self) -> LeftPaneActionOutcome {
        if self.workspace_filtering {
            self.workspace_filtering = false;
            return LeftPaneActionOutcome::Handled;
        }
        LeftPaneActionOutcome::Quit
    }

    /// Selects the grouped file row at a visible row offset.
    pub(crate) fn select_workspace_row(&mut self, row: usize) {
        if row >= self.workspace_rows().len() {
            return;
        }
        self.workspace_focused_row = row;
        self.activate_workspace_focused_row();
    }

    /// Returns the grouped file row index at one terminal row.
    pub(crate) fn workspace_row_at_position(&self, row: u16) -> Option<usize> {
        if row < self.last_tree_area.y
            || row
                >= self
                    .last_tree_area
                    .y
                    .saturating_add(self.last_tree_area.height)
        {
            return None;
        }
        let index = self.workspace_scroll + usize::from(row - self.last_tree_area.y);
        (index < self.workspace_rows().len()).then_some(index)
    }

    /// Returns active status text for the grouped file tree.
    pub(crate) fn workspace_selected_status(&self) -> String {
        self.workspace_selected_path
            .as_ref()
            .unwrap_or(&self.root_path)
            .display()
            .to_string()
    }

    /// Scrolls the grouped file-tree viewport without changing focus.
    pub(crate) fn scroll_workspace_by(&mut self, delta: isize) {
        let max_scroll = self
            .workspace_rows()
            .len()
            .saturating_sub(usize::from(self.last_tree_area.height).max(1));
        self.workspace_scroll = self
            .workspace_scroll
            .saturating_add_signed(delta)
            .min(max_scroll);
    }

    /// Activates the focused grouped row and updates the shared file preview.
    pub(crate) fn activate_workspace_focused_row(&mut self) {
        let Some((path, is_dir)) = self.focused_workspace_path() else {
            return;
        };
        self.workspace_selected_path = Some(path.clone());
        self.last_selection = path.display().to_string();
        self.selected_preview_path = Some(path.clone());
        self.selected_preview_is_dir = is_dir;
        if is_dir {
            self.preview_state = preview_state_for_path(&path, true);
            self.preview_load_receiver = None;
        } else {
            self.preview_state = loading_preview_state_for_path(&path);
            self.preview_load_receiver = Some(spawn_preview_load_worker(path));
        }
        self.keep_workspace_focused_visible();
    }

    /// Keeps focused grouped file row visible in the current viewport.
    fn keep_workspace_focused_visible(&mut self) {
        let height = usize::from(self.last_tree_area.height).max(1);
        if self.workspace_focused_row < self.workspace_scroll {
            self.workspace_scroll = self.workspace_focused_row;
        } else if self.workspace_focused_row >= self.workspace_scroll + height {
            self.workspace_scroll = self.workspace_focused_row + 1 - height;
        }
    }

    /// Returns focused grouped path and directory flag.
    fn focused_workspace_path(&self) -> Option<(PathBuf, bool)> {
        match self.workspace_rows().get(self.workspace_focused_row)? {
            WorkspaceFileRow::WorkspaceFolder { path } => Some((path.clone(), true)),
            WorkspaceFileRow::Entry { path, is_dir, .. } => Some((path.clone(), *is_dir)),
        }
    }
}

/// Returns sorted unique paths copied from a set.
fn sorted_paths(paths: &std::collections::BTreeSet<PathBuf>) -> Vec<PathBuf> {
    paths.iter().cloned().collect()
}
