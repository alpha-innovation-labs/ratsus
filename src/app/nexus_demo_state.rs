use anyhow::Result;
use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet, HashMap};

use ratatui::layout::Rect;
use ratkit::primitives::menu_bar::MenuBar;
use ratkit::primitives::resizable_grid::{PaneId, ResizableGrid, ResizableGridWidgetState};
use ratkit::primitives::toast::ToastManager;
use ratkit::services::file_watcher::FileWatcher;
use std::path::PathBuf;
use std::sync::mpsc::Receiver;
use std::time::Duration;

use crate::app::delete_session_confirmation_state::DeleteSessionConfirmationState;
use crate::app::delete_sessions_result::DeleteSessionsResult;
use crate::conversation_picker::conversation_picker_state::ConversationPickerState;
use crate::expo::clamp_expo_card_width::clamp_expo_card_width;
use crate::expo::conversation_observation_preview::ConversationObservationPreview;
use crate::expo::expo_card_area::ExpoCardArea;
use crate::expo::observation_preview_requests::observation_preview_requests;
use crate::expo::spawn_observation_cache_worker::spawn_observation_cache_worker;
use crate::expo::start_observation_watcher::start_observation_watcher;
use crate::layout::focused_pane::FocusedPane;
use crate::layout::pane_ids::{LEFT_PANE_ID, TERMINAL_PANE_ID};
use crate::left_panel::apply_session_id_order::apply_session_id_order;
use crate::left_panel::load_session_order_preferences::load_session_order_preferences;
use crate::left_panel::session_drag_state::SessionDragState;
use crate::left_panel::session_visible_row_index::session_visible_row_index;
use crate::left_panel::sort_sessions_by_creation_date::sort_sessions_by_creation_date;
use crate::left_panel::sync_folder_order::sync_folder_order;
use crate::left_panel::visible_session_rows::visible_session_rows;
use crate::left_panel::visible_session_rows_cache::VisibleSessionRowsCache;
use crate::main_pane::file_system_tree_view::FileSystemTreeView;
use crate::main_pane::main_pane_tab::MainPaneTab;
use crate::menu_bar::nexus_menu_bar::nexus_menu_bar;
use crate::nexus_sessions::load_nexus_sessions::load_nexus_sessions;
use crate::nexus_sessions::spawn_session_refresh_worker::{
    spawn_session_refresh_worker, SessionRefreshResult,
};
use crate::terminal::load_normal_terminal_sessions::load_normal_terminal_sessions;
use crate::terminal::session_terminal::SessionTerminal;

/// How long the background worker waits after a completed session refresh.
const SESSION_REFRESH_INTERVAL: Duration = Duration::from_secs(2);

/// Demo app state for Nexus session terminals.
pub struct NexusDemo {
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
}

impl NexusDemo {
    /// Builds the demo by loading Nexus sessions and spawning their terminals.
    pub fn new() -> Result<Self> {
        let mut layout = ResizableGrid::new(LEFT_PANE_ID);
        let _ = layout.split_pane_vertically(LEFT_PANE_ID);
        layout.set_split_percent(20);

        let preferences = load_session_order_preferences();
        let mut sessions = load_nexus_sessions()?;
        sessions.extend(load_normal_terminal_sessions());
        let mut session_terminals = sessions
            .into_iter()
            .map(SessionTerminal::dormant)
            .collect::<Vec<_>>();
        sort_sessions_by_creation_date(&mut session_terminals);
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
            &visible_session_rows(
                &session_terminals,
                &collapsed_folders,
                &folder_order,
                Some(active_index),
            ),
            active_index,
        )
        .unwrap_or(0);
        let file_system_tree_view = FileSystemTreeView::new()?;
        let observation_cache_receiver = Some(spawn_observation_cache_worker(
            observation_preview_requests(&session_terminals),
        ));
        let observation_watcher = start_observation_watcher();
        let mut terminal_pane_sessions = BTreeMap::new();
        let mut terminal_pane_session_bundles = BTreeMap::new();
        if let Some(session) = session_terminals.get(active_index) {
            terminal_pane_sessions.insert(TERMINAL_PANE_ID, session.session.id.clone());
            terminal_pane_session_bundles
                .insert(TERMINAL_PANE_ID, vec![session.session.id.clone()]);
        }

        Ok(Self {
            layout,
            layout_widget_state: ResizableGridWidgetState::default(),
            terminal_layout: ResizableGrid::new(TERMINAL_PANE_ID),
            terminal_layout_widget_state: ResizableGridWidgetState::default(),
            terminal_pane_sessions,
            terminal_pane_session_bundles,
            terminal_pane_areas: BTreeMap::new(),
            terminal_pane_close_buttons: BTreeMap::new(),
            active_terminal_pane_id: TERMINAL_PANE_ID,
            toast_manager: ToastManager::new(),
            menu_bar: nexus_menu_bar(MainPaneTab::Chat),
            conversation_picker: ConversationPickerState::new(),
            delete_confirmation: DeleteSessionConfirmationState::default(),
            delete_session_receiver: None,
            session_terminals,
            visible_rows_cache: RefCell::new(VisibleSessionRowsCache::new()),
            closed_chat_session_ids: BTreeSet::new(),
            selected_conversation_ids: BTreeSet::new(),
            active_index,
            focused_index: active_index,
            session_scroll: 0,
            pending_left_scroll_redraw: false,
            suppress_left_focus_scroll: false,
            session_drag: None,
            folder_drag: None,
            folder_drag_moved: false,
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
            selected_expo_folder: None,
            expo_scroll: 0,
            expo_card_width: clamp_expo_card_width(preferences.expo_card_width),
            expo_filter_query: String::new(),
            expo_filtering: false,
            expo_card_areas: Vec::new(),
            observation_previews: HashMap::new(),
            observation_cache_receiver,
            observation_watcher,
            last_main_pane_area: Rect::default(),
            file_system_tree_view,
            loader_tick: 0,
            session_refresh_receiver: spawn_session_refresh_worker(SESSION_REFRESH_INTERVAL),
        })
    }
}
