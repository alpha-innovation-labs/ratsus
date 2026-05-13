use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::path::PathBuf;
use std::sync::mpsc::Receiver;
use std::sync::Arc;

use ratatui::layout::Rect;
use ratkit::primitives::menu_bar::MenuBar;
use ratkit::primitives::resizable_grid::{PaneId, ResizableGrid, ResizableGridWidgetState};
use ratkit::primitives::toast::ToastManager;
use ratkit::services::file_watcher::FileWatcher;

use crate::app::delete_session_confirmation_state::DeleteSessionConfirmationState;
use crate::app::delete_sessions_result::DeleteSessionsResult;
use crate::chat_sessions::spawn_session_refresh_worker::SessionRefreshResult;
use crate::conversation_picker::conversation_picker_state::ConversationPickerState;
use crate::expo::conversation_observation_preview::ConversationObservationPreview;
use crate::expo::expo_card_area::ExpoCardArea;
use crate::harness::chat_harness::ChatHarness;
use crate::layout::focused_pane::FocusedPane;
use crate::left_panel::session_drag_state::SessionDragState;
use crate::left_panel::visible_session_rows_cache::VisibleSessionRowsCache;
use crate::main_pane::file_system_tree_view::FileSystemTreeView;
use crate::main_pane::main_pane_tab::MainPaneTab;
use crate::terminal::session_terminal::SessionTerminal;

/// Demo app state for chat session terminals.
pub struct AppState {
    pub layout: ResizableGrid,
    pub layout_widget_state: ResizableGridWidgetState,
    pub terminal_layout: ResizableGrid,
    pub terminal_layout_widget_state: ResizableGridWidgetState,
    pub terminal_pane_sessions: BTreeMap<PaneId, String>,
    pub terminal_pane_session_bundles: BTreeMap<PaneId, Vec<String>>,
    pub terminal_pane_areas: BTreeMap<PaneId, Rect>,
    pub terminal_pane_close_buttons: BTreeMap<PaneId, Rect>,
    pub active_terminal_pane_id: PaneId,
    pub toast_manager: ToastManager,
    pub menu_bar: MenuBar,
    pub conversation_picker: ConversationPickerState,
    pub delete_confirmation: DeleteSessionConfirmationState,
    pub delete_session_receiver: Option<Receiver<DeleteSessionsResult>>,
    pub session_terminals: Vec<SessionTerminal>,
    pub visible_rows_cache: RefCell<VisibleSessionRowsCache>,
    pub closed_chat_session_ids: BTreeSet<String>,
    pub selected_conversation_ids: BTreeSet<String>,
    pub active_index: usize,
    pub focused_index: usize,
    pub session_scroll: usize,
    pub pending_left_scroll_redraw: bool,
    pub suppress_left_focus_scroll: bool,
    pub session_drag: Option<SessionDragState>,
    pub folder_drag: Option<PathBuf>,
    pub folder_drag_moved: bool,
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
    pub selected_expo_folder: Option<PathBuf>,
    pub expo_scroll: usize,
    pub expo_card_width: u16,
    pub expo_filter_query: String,
    pub expo_filtering: bool,
    pub expo_card_areas: Vec<ExpoCardArea>,
    pub observation_previews: HashMap<String, ConversationObservationPreview>,
    pub observation_cache_receiver:
        Option<Receiver<HashMap<String, ConversationObservationPreview>>>,
    pub observation_watcher: Option<FileWatcher>,
    pub last_main_pane_area: Rect,
    pub file_system_tree_view: FileSystemTreeView,
    pub loader_tick: u64,
    pub session_refresh_receiver: Receiver<SessionRefreshResult>,
    pub chat_harness: Arc<dyn ChatHarness>,
}
