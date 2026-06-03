use std::path::PathBuf;

use ratatui::layout::Rect;

use crate::app::expo::activate_expo_folder::activate_expo_folder;
use crate::app::navigation::reorder_session_to_index::reorder_session_to_index;
use crate::app::state::app_state::AppState;
use crate::extensions::file_viewer::tabs::tab::MainPaneTab;
use crate::extensions::file_viewer::tree::sync_workspace_root::sync_file_viewer_workspace_root;
use crate::extensions::terminal::session::chat_terminal::ChatTerminal;
use crate::extensions::terminal::session::session_terminal::SessionTerminal;
use crate::ui::grid_layout::bundle::set_active_bundle_session::set_active_terminal_pane_bundle_session;
use crate::ui::grid_layout::pane::pane_exists_in_terminal_layout::pane_exists_in_terminal_layout;
use crate::ui::grid_layout::pane::pane_id_for_session::pane_id_for_session;
use crate::ui::grid_layout::persistence::pane_id_for_session_in_available_multiplexer_state::pane_id_for_session_in_available_multiplexer_state;
use crate::ui::grid_layout::persistence::persist_multiplexer_state::persist_multiplexer_state;
use crate::ui::grid_layout::persistence::restore_available_multiplexer_state_into_app::restore_available_multiplexer_state_into_app;
use crate::ui::grid_layout::single::show_session_as_single_pane::show_session_as_single_pane;
use crate::ui::keyboard::list::wrapped_position::wrapped_list_position;
use crate::ui::layout::resizable_grid::should_resize_active_terminal::should_resize_active_terminal;
use crate::ui::left_panel::focus::session_visible_row_index::session_visible_row_index;
use crate::ui::left_panel::input::session_drag_state::SessionDragState;
use crate::ui::left_panel::order::move_folder_order::move_folder_order;
use crate::ui::left_panel::order::persist_preferences::persist_session_order_preferences;
use crate::ui::left_panel::scroll::clamp_visible_offset::clamp_visible_offset;
use crate::ui::left_panel::session::activation::activate_split_group_child::activate_split_group_child;
use crate::ui::left_panel::session::activation::activate_split_group_parent::activate_split_group_parent;
use crate::ui::left_panel::session::list_row::SessionListRow;
use crate::ui::workspace_pane::remember_active_workspace_session::remember_active_workspace_session;

impl AppState {
    /// Returns the active terminal for immutable operations.
    pub fn active_terminal(&self) -> Option<&ChatTerminal> {
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
        self.suppress_left_focus_scroll = false;
        let rows = self.visible_rows();
        if rows.is_empty() {
            return;
        }
        self.focused_row = wrapped_list_position(self.focused_row, direction, rows.len());
        match rows.get(self.focused_row) {
            Some(SessionListRow::Folder { path, .. }) => activate_expo_folder(self, path.clone()),
            Some(SessionListRow::SplitGroup { group_id, .. }) => {
                activate_split_group_parent(self, *group_id);
            }
            Some(SessionListRow::SplitGroupChild { pane_id, index, .. }) => {
                activate_split_group_child(self, *pane_id, *index);
            }
            Some(SessionListRow::Session { index }) => {
                self.focused_index = *index;
                self.activate_focused_session();
            }
            Some(SessionListRow::FolderMore { .. }) | None => {}
        }
        self.keep_focused_row_visible();
    }

    /// Moves focus and immediately displays the newly focused session when applicable.
    pub fn select_relative_session(&mut self, direction: isize) {
        self.move_focused_left_row(direction);
    }

    /// Activates the focused session and displays its terminal.
    pub fn activate_focused_session(&mut self) {
        self.suppress_left_focus_scroll = false;
        if self.focused_index >= self.session_terminals.len() {
            return;
        }
        self.active_index = self.focused_index;
        sync_selected_workspace_to_active_session(self);
        remember_active_workspace_session(self);
        self.active_main_pane_tab = MainPaneTab::Chat;
        self.sync_focused_row_to_session();
        persist_session_order_preferences(self);
        let session_id = self
            .session_terminals
            .get(self.active_index)
            .map(|entry| entry.session.id.clone());
        if let Some(session_id) = session_id {
            self.completed_unseen_session_ids.remove(&session_id);
            if let Some(pane_id) = pane_id_for_session(self, &session_id) {
                activate_session_in_pane(self, pane_id, session_id);
            } else if let Some(pane_id) =
                pane_id_for_session_in_available_multiplexer_state(self, &session_id)
            {
                restore_available_multiplexer_state_into_app(self);
                activate_session_in_pane(self, pane_id, session_id);
            } else {
                show_session_as_single_pane(self, session_id);
            }
        }
        let area = self
            .terminal_pane_areas
            .get(&self.active_terminal_pane_id)
            .copied()
            .unwrap_or(self.last_terminal_area);
        if let Some(entry) = self.session_terminals.get_mut(self.active_index) {
            let _ = entry.ensure_terminal(area.height, area.width);
        }
        self.active_terminal_area = area;
        persist_multiplexer_state(self);
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
        if !reorder_session_to_index(self, from_index, target_index) {
            return;
        }
        self.session_drag = Some(SessionDragState {
            source_index: drag.source_index,
            current_index: target_index,
        });
        self.keep_focused_session_visible();
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

    /// Starts dragging a workspace row.
    pub fn start_workspace_drag(&mut self, path: PathBuf) {
        self.workspace_drag = Some(path);
        self.workspace_drag_moved = false;
    }

    /// Marks the active workspace drag as having moved beyond the initial click.
    pub fn mark_workspace_drag_moved(&mut self) {
        if self.workspace_drag.is_some() {
            self.workspace_drag_moved = true;
        }
    }

    /// Moves the active dragged workspace before the target workspace row.
    pub fn move_dragged_workspace(&mut self, target: PathBuf) {
        let Some(source) = self.workspace_drag.clone() else {
            return;
        };
        if move_folder_order(&mut self.folder_order, &source, &target) {
            persist_session_order_preferences(self);
            persist_multiplexer_state(self);
        }
    }

    /// Ends any active workspace drag operation.
    pub fn finish_workspace_drag(&mut self) {
        self.workspace_drag = None;
        self.workspace_drag_moved = false;
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
        if self.suppress_left_focus_scroll {
            return;
        }
        self.session_scroll = clamp_visible_offset(
            self.focused_row,
            self.session_scroll,
            usize::from(self.last_session_list_area.height),
        );
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

/// Syncs selected workspace and file tree to the active session workspace.
fn sync_selected_workspace_to_active_session(app: &mut AppState) {
    let Some(workspace) = app
        .session_terminals
        .get(app.active_index)
        .map(|entry| entry.session.working_dir.clone())
    else {
        return;
    };
    if !app.folder_order.iter().any(|folder| folder == &workspace) {
        return;
    }
    if app.selected_workspace_path.as_ref() != Some(&workspace) {
        app.selected_workspace_path = Some(workspace);
        app.session_scroll = 0;
        app.suppress_left_focus_scroll = false;
    }
    sync_file_viewer_workspace_root(app);
}

/// Activates a session inside an existing pane, or falls back to standalone display.
fn activate_session_in_pane(app: &mut AppState, pane_id: u32, session_id: String) {
    if !pane_exists_in_terminal_layout(app, pane_id) {
        restore_available_multiplexer_state_into_app(app);
    }
    if pane_exists_in_terminal_layout(app, pane_id) {
        set_active_terminal_pane_bundle_session(app, pane_id, session_id.clone());
        if let Some(index) = app
            .session_terminals
            .iter()
            .position(|entry| entry.session.id == session_id)
        {
            app.active_index = index;
            app.focused_index = index;
        }
    } else {
        show_session_as_single_pane(app, session_id);
    }
}
