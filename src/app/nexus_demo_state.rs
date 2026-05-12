use anyhow::Result;
use ratatui::layout::Rect;
use ratkit::primitives::resizable_grid::{ResizableGrid, ResizableGridWidgetState};
use ratkit::primitives::toast::ToastManager;
use std::collections::BTreeSet;
use std::path::PathBuf;
use std::sync::mpsc::Receiver;
use std::time::Duration;

use crate::conversation_picker::conversation_picker_state::ConversationPickerState;
use crate::layout::focused_pane::FocusedPane;
use crate::layout::pane_ids::LEFT_PANE_ID;
use crate::left_panel::apply_session_id_order::apply_session_id_order;
use crate::left_panel::load_session_order_preferences::load_session_order_preferences;
use crate::left_panel::session_drag_state::SessionDragState;
use crate::left_panel::session_visible_row_index::session_visible_row_index;
use crate::left_panel::sync_folder_order::sync_folder_order;
use crate::left_panel::visible_session_rows::visible_session_rows;
use crate::main_pane::file_system_tree_view::FileSystemTreeView;
use crate::main_pane::main_pane_tab::MainPaneTab;
use crate::nexus_sessions::load_nexus_sessions::load_nexus_sessions;
use crate::nexus_sessions::spawn_session_refresh_worker::{
    spawn_session_refresh_worker, SessionRefreshResult,
};
use crate::terminal::session_terminal::SessionTerminal;

/// How long the background worker waits after a completed session refresh.
const SESSION_REFRESH_INTERVAL: Duration = Duration::from_secs(2);

/// Demo app state for Nexus session terminals.
pub struct NexusDemo {
    pub layout: ResizableGrid,
    pub layout_widget_state: ResizableGridWidgetState,
    pub toast_manager: ToastManager,
    pub conversation_picker: ConversationPickerState,
    pub session_terminals: Vec<SessionTerminal>,
    pub active_index: usize,
    pub focused_index: usize,
    pub session_scroll: usize,
    pub session_drag: Option<SessionDragState>,
    pub folder_drag: Option<PathBuf>,
    pub collapsed_folders: BTreeSet<PathBuf>,
    pub folder_order: Vec<PathBuf>,
    pub focused_row: usize,
    pub pending_left_g: bool,
    pub last_layout_area: Rect,
    pub last_terminal_area: Rect,
    pub active_terminal_area: Rect,
    pub last_left_area: Rect,
    pub last_session_list_area: Rect,
    pub left_pane_visible: bool,
    pub focused_pane: FocusedPane,
    pub active_main_pane_tab: MainPaneTab,
    pub last_main_pane_area: Rect,
    pub file_system_tree_view: FileSystemTreeView,
    pub loader_tick: u64,
    pub session_refresh_receiver: Receiver<SessionRefreshResult>,
}

impl NexusDemo {
    /// Builds the demo by loading Nexus sessions and spawning their terminals.
    pub fn new() -> Result<Self> {
        let mut layout = ResizableGrid::new(LEFT_PANE_ID);
        let _ = layout.split_pane_vertically(LEFT_PANE_ID);
        layout.set_split_percent(20);

        let preferences = load_session_order_preferences();
        let mut session_terminals = load_nexus_sessions()?
            .into_iter()
            .map(SessionTerminal::dormant)
            .collect::<Vec<_>>();
        apply_session_id_order(&mut session_terminals, &preferences.session_ids);
        let folder_order = sync_folder_order(&preferences.folder_paths, &session_terminals);
        let active_index = preferences
            .active_session_id
            .as_deref()
            .and_then(|id| {
                session_terminals
                    .iter()
                    .position(|entry| entry.session.id == id)
            })
            .unwrap_or(0);
        let collapsed_folders = preferences.collapsed_folder_paths.into_iter().collect();
        let focused_row = session_visible_row_index(
            &visible_session_rows(&session_terminals, &collapsed_folders, &folder_order),
            active_index,
        )
        .unwrap_or(0);
        let file_system_tree_view = FileSystemTreeView::new()?;

        Ok(Self {
            layout,
            layout_widget_state: ResizableGridWidgetState::default(),
            toast_manager: ToastManager::new(),
            conversation_picker: ConversationPickerState::new(),
            session_terminals,
            active_index,
            focused_index: active_index,
            session_scroll: 0,
            session_drag: None,
            folder_drag: None,
            collapsed_folders,
            folder_order,
            focused_row,
            pending_left_g: false,
            last_layout_area: Rect::default(),
            last_terminal_area: Rect::default(),
            active_terminal_area: Rect::default(),
            last_left_area: Rect::default(),
            last_session_list_area: Rect::default(),
            left_pane_visible: true,
            focused_pane: FocusedPane::Terminal,
            active_main_pane_tab: MainPaneTab::Chat,
            last_main_pane_area: Rect::default(),
            file_system_tree_view,
            loader_tick: 0,
            session_refresh_receiver: spawn_session_refresh_worker(SESSION_REFRESH_INTERVAL),
        })
    }
}
