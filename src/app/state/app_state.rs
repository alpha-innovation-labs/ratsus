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
use ratkit::services::hotkey_service::HotkeyRegistry;

use crate::app::deletion::delete_session_confirmation_state::DeleteSessionConfirmationState;
use crate::app::deletion::delete_sessions_result::DeleteSessionsResult;
use crate::app::diagnostics::app_diagnostics::AppDiagnostics;
use crate::extensions::command_bar::data::command_bar_state::CommandBarState;
use crate::extensions::expo::card::area::ExpoCardArea;
use crate::extensions::expo::observations::conversation_preview::ConversationObservationPreview;
use crate::extensions::file_viewer::tabs::tab::MainPaneTab;
use crate::extensions::file_viewer::tree::view::FileSystemTreeView;
use crate::extensions::history_modal::data::state::ConversationPickerState;
use crate::extensions::harness::core::chat_harness::ChatHarness;
use crate::extensions::harness::core::chat_session::ChatSession;
use crate::extensions::plans::data::plan_list_state::PlanListState;
use crate::extensions::terminal::session::session_terminal::SessionTerminal;
use crate::ui::grid_layout::group::split_pane_session_group_state::SplitPaneSessionGroupState;
use crate::ui::grid_layout::persistence::persisted_multiplexer_state::PersistedMultiplexerState;
use crate::ui::layout::focus::focused_pane::FocusedPane;
use crate::ui::left_panel::input::session_drag_state::SessionDragState;
use crate::ui::left_panel::mode::left_pane_mode::LeftPaneMode;
use crate::ui::left_panel::session::visible_rows_cache::VisibleSessionRowsCache;

/// Demo app state for chat session terminals.
pub struct AppState {
    pub layout: ResizableGrid,
    pub layout_widget_state: ResizableGridWidgetState,
    pub terminal_layout: ResizableGrid,
    pub terminal_layout_widget_state: ResizableGridWidgetState,
    pub terminal_pane_sessions: BTreeMap<PaneId, String>,
    pub terminal_pane_session_bundles: BTreeMap<PaneId, Vec<String>>,
    pub split_pane_session_groups: SplitPaneSessionGroupState,
    pub suspended_multiplexer_state: Option<PersistedMultiplexerState>,
    pub terminal_pane_areas: BTreeMap<PaneId, Rect>,
    pub terminal_pane_close_buttons: BTreeMap<PaneId, Rect>,
    pub active_terminal_pane_id: PaneId,
    pub toast_manager: ToastManager,
    pub menu_bar: MenuBar,
    pub hotkey_registry: HotkeyRegistry,
    pub command_bar: CommandBarState,
    pub conversation_picker: ConversationPickerState,
    pub delete_confirmation: DeleteSessionConfirmationState,
    pub delete_session_receiver: Option<Receiver<DeleteSessionsResult>>,
    pub initial_sessions_receiver: Option<Receiver<anyhow::Result<Vec<ChatSession>>>>,
    pub diagnostics: AppDiagnostics,
    pub session_terminals: Vec<SessionTerminal>,
    pub visible_rows_cache: RefCell<VisibleSessionRowsCache>,
    pub closed_chat_session_ids: BTreeSet<String>,
    pub completed_unseen_session_ids: BTreeSet<String>,
    pub selected_conversation_ids: BTreeSet<String>,
    pub left_pane_mode: LeftPaneMode,
    pub plan_list: PlanListState,
    pub active_index: usize,
    pub focused_index: usize,
    pub session_scroll: usize,
    pub pending_left_scroll_redraw: bool,
    pub suppress_left_focus_scroll: bool,
    pub session_drag: Option<SessionDragState>,
    pub folder_drag: Option<PathBuf>,
    pub folder_drag_moved: bool,
    pub workspace_drag: Option<PathBuf>,
    pub workspace_drag_moved: bool,
    pub collapsed_folders: BTreeSet<PathBuf>,
    pub folder_order: Vec<PathBuf>,
    pub selected_workspace_path: Option<PathBuf>,
    pub workspace_focused_session_ids: BTreeMap<PathBuf, String>,
    pub workspace_view_enabled: bool,
    pub workspace_scroll: usize,
    pub focused_row: usize,
    pub pending_left_g: bool,
    pub last_layout_area: Rect,
    pub last_terminal_area: Rect,
    pub active_terminal_area: Rect,
    pub last_workspace_area: Rect,
    pub last_workspace_list_area: Rect,
    pub last_left_area: Rect,
    pub last_session_list_area: Rect,
    pub last_left_session_toggle_area: Rect,
    pub last_left_plan_toggle_area: Rect,
    pub last_left_file_toggle_area: Rect,
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
    pub session_watcher: Option<FileWatcher>,
    pub session_refresh_receiver: Option<Receiver<anyhow::Result<Vec<ChatSession>>>>,
    pub last_main_pane_area: Rect,
    pub file_system_tree_view: FileSystemTreeView,
    pub file_system_tree_expanded_paths: BTreeMap<PathBuf, Vec<PathBuf>>,
    pub loader_tick: u64,
    pub chat_harness: Arc<dyn ChatHarness>,
}
