use std::path::PathBuf;

use ratatui::layout::Rect;

use crate::app::nexus_demo_state::NexusDemo;
use crate::layout::should_resize_active_terminal::should_resize_active_terminal;
use crate::left_panel::clamp_visible_offset::clamp_visible_offset;
use crate::left_panel::move_folder_order::move_folder_order;
use crate::left_panel::persist_session_order_preferences::persist_session_order_preferences;
use crate::left_panel::reordered_index_after_move::reordered_index_after_move;
use crate::left_panel::session_drag_state::SessionDragState;
use crate::left_panel::session_list_row::SessionListRow;
use crate::left_panel::session_visible_row_index::session_visible_row_index;
use crate::left_panel::visible_session_rows::visible_session_rows;
use crate::terminal::nexus_terminal::NexusTerminal;
use crate::terminal::session_terminal::SessionTerminal;

impl NexusDemo {
    /// Returns the active terminal for immutable operations.
    pub fn active_terminal(&self) -> Option<&NexusTerminal> {
        self.session_terminals
            .get(self.active_index)
            .and_then(|entry| entry.terminal.as_ref())
    }

    /// Returns the active session terminal entry for mutable operations.
    pub fn active_session_terminal_mut(&mut self) -> Option<&mut SessionTerminal> {
        self.session_terminals.get_mut(self.active_index)
    }

    /// Returns the active session terminal entry for immutable operations.
    pub fn active_session_terminal(&self) -> Option<&SessionTerminal> {
        self.session_terminals.get(self.active_index)
    }

    /// Moves focus through visible folder/session rows.
    pub fn move_focused_left_row(&mut self, direction: isize) {
        let rows = self.visible_rows();
        if rows.is_empty() {
            return;
        }
        self.focused_row = self
            .focused_row
            .saturating_add_signed(direction)
            .min(rows.len() - 1);
        if let Some(SessionListRow::Session { index }) = rows.get(self.focused_row) {
            self.focused_index = *index;
            self.activate_focused_session();
        }
        self.keep_focused_row_visible();
    }

    /// Moves focus and immediately displays the newly focused session when applicable.
    pub fn select_relative_session(&mut self, direction: isize) {
        self.move_focused_left_row(direction);
    }

    /// Activates the focused session and displays its terminal.
    pub fn activate_focused_session(&mut self) {
        if self.focused_index >= self.session_terminals.len() {
            return;
        }
        self.active_index = self.focused_index;
        self.sync_focused_row_to_session();
        persist_session_order_preferences(self);
        let area = self.last_terminal_area;
        if let Some(entry) = self.session_terminals.get_mut(self.active_index) {
            let _ = entry.ensure_terminal(area.height, area.width);
        }
        self.active_terminal_area = area;
    }

    /// Starts dragging a session row and activates it for immediate feedback.
    pub fn start_session_drag(&mut self, index: usize) {
        if index >= self.session_terminals.len() {
            return;
        }
        self.session_drag = Some(SessionDragState::new(index));
        self.focused_index = index;
        self.sync_focused_row_to_session();
        self.activate_focused_session();
    }

    /// Moves the active dragged session to a target session row.
    pub fn move_dragged_session(&mut self, target_index: usize) {
        let Some(drag) = self.session_drag else {
            return;
        };
        let from_index = drag.current_index;
        if target_index >= self.session_terminals.len() || target_index == from_index {
            return;
        }
        let entry = self.session_terminals.remove(from_index);
        self.session_terminals.insert(target_index, entry);
        self.active_index = reordered_index_after_move(self.active_index, from_index, target_index);
        self.focused_index =
            reordered_index_after_move(self.focused_index, from_index, target_index);
        self.session_drag = Some(SessionDragState {
            source_index: drag.source_index,
            current_index: target_index,
        });
        self.keep_focused_session_visible();
        persist_session_order_preferences(self);
    }

    /// Starts dragging a folder row.
    pub fn start_folder_drag(&mut self, path: PathBuf) {
        self.folder_drag = Some(path);
        self.folder_drag_moved = false;
    }

    /// Marks the active folder drag as having moved beyond the initial click.
    pub fn mark_folder_drag_moved(&mut self) {
        if self.folder_drag.is_some() {
            self.folder_drag_moved = true;
        }
    }

    /// Moves the active dragged folder before the target folder row.
    pub fn move_dragged_folder(&mut self, target: PathBuf) {
        let Some(source) = self.folder_drag.clone() else {
            return;
        };
        if move_folder_order(&mut self.folder_order, &source, &target) {
            persist_session_order_preferences(self);
        }
    }

    /// Ends any active left-pane drag operation.
    pub fn finish_left_panel_drag(&mut self) {
        self.session_drag = None;
        self.folder_drag = None;
        self.folder_drag_moved = false;
    }

    /// Keeps the focused session visible in the left pane viewport.
    pub fn keep_focused_session_visible(&mut self) {
        self.sync_focused_row_to_session();
        self.keep_focused_row_visible();
    }

    /// Keeps the focused visible row in the left pane viewport.
    pub fn keep_focused_row_visible(&mut self) {
        self.session_scroll = clamp_visible_offset(
            self.focused_row,
            self.session_scroll,
            usize::from(self.last_session_list_area.height),
        );
    }

    /// Returns the currently visible left-panel tree rows.
    pub fn visible_rows(&self) -> Vec<SessionListRow> {
        visible_session_rows(
            &self.session_terminals,
            &self.collapsed_folders,
            &self.folder_order,
        )
    }

    /// Moves visible-row focus to the currently focused session when visible.
    pub fn sync_focused_row_to_session(&mut self) {
        let rows = self.visible_rows();
        if let Some(row_index) = session_visible_row_index(&rows, self.focused_index) {
            self.focused_row = row_index;
        }
    }

    /// Resizes only the active terminal, throttling drag-time PTY resizes.
    pub fn resize_active_terminal(&mut self, area: Rect) {
        self.last_terminal_area = area;
        if area == self.active_terminal_area {
            return;
        }
        let is_dragging = self.layout_widget_state.dragging_divider.is_some();
        if !should_resize_active_terminal(is_dragging) {
            return;
        }
        if let Some(entry) = self.session_terminals.get_mut(self.active_index) {
            if let Ok(terminal) = entry.ensure_terminal(area.height, area.width) {
                terminal.resize(area.height, area.width);
            }
        }
        self.active_terminal_area = area;
    }
}
