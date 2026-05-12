use std::collections::BTreeSet;
use std::path::PathBuf;

use anyhow::Result;
use ratatui::layout::Rect;
use ratkit::primitives::resizable_grid::{ResizableGrid, ResizableGridWidgetState};
use ratkit::primitives::toast::ToastManager;

use crate::clamp_visible_offset::clamp_visible_offset;
use crate::focused_pane::FocusedPane;
use crate::load_nexus_sessions::load_nexus_sessions;
use crate::nexus_terminal::NexusTerminal;
use crate::reordered_index_after_move::reordered_index_after_move;
use crate::session_drag_state::SessionDragState;
use crate::session_terminal::SessionTerminal;
use crate::session_visible_row_index::session_visible_row_index;
use crate::should_resize_active_terminal::should_resize_active_terminal;
use crate::visible_session_rows::visible_session_rows;

/// Pane id for the left session list pane.
pub const LEFT_PANE_ID: u32 = 0;

/// Pane id for the active terminal pane.
pub const TERMINAL_PANE_ID: u32 = 1;

/// Demo app state for Nexus session terminals.
pub struct NexusDemo {
    pub layout: ResizableGrid,
    pub layout_widget_state: ResizableGridWidgetState,
    pub toast_manager: ToastManager,
    pub session_terminals: Vec<SessionTerminal>,
    pub active_index: usize,
    pub focused_index: usize,
    pub session_scroll: usize,
    pub session_drag: Option<SessionDragState>,
    pub collapsed_folders: BTreeSet<PathBuf>,
    pub last_layout_area: Rect,
    pub last_terminal_area: Rect,
    pub active_terminal_area: Rect,
    pub last_left_area: Rect,
    pub last_session_list_area: Rect,
    pub left_pane_visible: bool,
    pub focused_pane: FocusedPane,
}

impl NexusDemo {
    /// Builds the demo by loading Nexus sessions and spawning their terminals.
    pub fn new() -> Result<Self> {
        let mut layout = ResizableGrid::new(LEFT_PANE_ID);
        let _ = layout.split_pane_vertically(LEFT_PANE_ID);
        layout.set_split_percent(20);

        let sessions = load_nexus_sessions()?;
        let mut session_terminals = Vec::new();
        for session in sessions {
            session_terminals.push(SessionTerminal::spawn(session, 24, 80)?);
        }

        Ok(Self {
            layout,
            layout_widget_state: ResizableGridWidgetState::default(),
            toast_manager: ToastManager::new(),
            session_terminals,
            active_index: 0,
            focused_index: 0,
            session_scroll: 0,
            session_drag: None,
            collapsed_folders: BTreeSet::new(),
            last_layout_area: Rect::default(),
            last_terminal_area: Rect::default(),
            active_terminal_area: Rect::default(),
            last_left_area: Rect::default(),
            last_session_list_area: Rect::default(),
            left_pane_visible: true,
            focused_pane: FocusedPane::Terminal,
        })
    }

    /// Returns the active terminal for immutable operations.
    pub fn active_terminal(&self) -> Option<&NexusTerminal> {
        self.session_terminals
            .get(self.active_index)
            .map(|entry| &entry.terminal)
    }

    /// Returns the active terminal for mutable operations.
    pub fn active_terminal_mut(&mut self) -> Option<&mut NexusTerminal> {
        self.session_terminals
            .get_mut(self.active_index)
            .map(|entry| &mut entry.terminal)
    }

    /// Returns the active session terminal entry for mutable operations.
    pub fn active_session_terminal_mut(&mut self) -> Option<&mut SessionTerminal> {
        self.session_terminals.get_mut(self.active_index)
    }

    /// Returns the active session terminal entry for immutable operations.
    pub fn active_session_terminal(&self) -> Option<&SessionTerminal> {
        self.session_terminals.get(self.active_index)
    }

    /// Moves the focused session item up or down by one row.
    pub fn move_focused_session(&mut self, direction: isize) {
        if self.session_terminals.is_empty() {
            return;
        }
        let last_index = self.session_terminals.len() - 1;
        self.focused_index = self
            .focused_index
            .saturating_add_signed(direction)
            .min(last_index);
        self.keep_focused_session_visible();
    }

    /// Moves focus and immediately displays the newly focused session.
    pub fn select_relative_session(&mut self, direction: isize) {
        self.move_focused_session(direction);
        self.activate_focused_session();
    }

    /// Toggles whether sessions under a folder are visible.
    pub fn toggle_folder(&mut self, folder: PathBuf) {
        if !self.collapsed_folders.remove(&folder) {
            self.collapsed_folders.insert(folder);
        }
    }

    /// Activates the focused session and displays its terminal.
    pub fn activate_focused_session(&mut self) {
        if self.focused_index >= self.session_terminals.len() {
            return;
        }
        self.active_index = self.focused_index;
        let area = self.last_terminal_area;
        if let Some(terminal) = self.active_terminal_mut() {
            terminal.resize(area.height, area.width);
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
    }

    /// Ends the active left-pane session drag operation.
    pub fn finish_session_drag(&mut self) {
        self.session_drag = None;
    }

    /// Keeps the focused session visible in the left pane viewport.
    pub fn keep_focused_session_visible(&mut self) {
        let rows = visible_session_rows(&self.session_terminals, &self.collapsed_folders);
        if let Some(row_index) = session_visible_row_index(&rows, self.focused_index) {
            self.session_scroll = clamp_visible_offset(
                row_index,
                self.session_scroll,
                usize::from(self.last_session_list_area.height),
            );
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
        if let Some(terminal) = self.active_terminal_mut() {
            terminal.resize(area.height, area.width);
        }
        self.active_terminal_area = area;
    }
}
