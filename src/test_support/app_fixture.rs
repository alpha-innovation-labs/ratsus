use std::cell::RefCell;
use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::sync::{mpsc, Arc};

use ratatui::layout::Rect;
use ratkit::primitives::resizable_grid::{ResizableGrid, ResizableGridWidgetState};
use ratkit::primitives::toast::ToastManager;

use crate::app::app_state::AppState;
use crate::app::delete_session_confirmation_state::DeleteSessionConfirmationState;
use crate::chat_sessions::spawn_session_refresh_worker::SessionRefreshResult;
use crate::conversation_picker::conversation_picker_state::ConversationPickerState;
use crate::harnesses::stub::StubHarness;
use crate::layout::focused_pane::FocusedPane;
use crate::layout::pane_ids::{LEFT_PANE_ID, TERMINAL_PANE_ID};
use crate::left_panel::visible_session_rows_cache::VisibleSessionRowsCache;
use crate::main_pane::file_system_tree_view::FileSystemTreeView;
use crate::main_pane::main_pane_tab::MainPaneTab;
use crate::menu_bar::app_menu_bar::app_menu_bar;
use crate::terminal::session_terminal::SessionTerminal;

/// Builds a minimal AppState fixture without spawning terminal processes.
pub fn app_fixture(session_terminals: Vec<SessionTerminal>) -> anyhow::Result<AppState> {
    let (_sender, receiver) = mpsc::channel::<SessionRefreshResult>();
    let mut layout = ResizableGrid::new(LEFT_PANE_ID);
    let _ = layout.split_pane_vertically(LEFT_PANE_ID);
    layout.set_split_percent(20);
    Ok(AppState {
        layout,
        layout_widget_state: ResizableGridWidgetState::default(),
        terminal_layout: ResizableGrid::new(TERMINAL_PANE_ID),
        terminal_layout_widget_state: ResizableGridWidgetState::default(),
        terminal_pane_sessions: BTreeMap::new(),
        terminal_pane_session_bundles: BTreeMap::new(),
        terminal_pane_areas: BTreeMap::new(),
        terminal_pane_close_buttons: BTreeMap::new(),
        active_terminal_pane_id: TERMINAL_PANE_ID,
        toast_manager: ToastManager::new(),
        menu_bar: app_menu_bar(MainPaneTab::Chat),
        conversation_picker: ConversationPickerState::new(),
        delete_confirmation: DeleteSessionConfirmationState::default(),
        delete_session_receiver: None,
        session_terminals,
        visible_rows_cache: RefCell::new(VisibleSessionRowsCache::new()),
        closed_chat_session_ids: BTreeSet::new(),
        selected_conversation_ids: BTreeSet::new(),
        active_index: 0,
        focused_index: 0,
        session_scroll: 0,
        pending_left_scroll_redraw: false,
        suppress_left_focus_scroll: false,
        session_drag: None,
        folder_drag: None,
        folder_drag_moved: false,
        collapsed_folders: BTreeSet::new(),
        folder_order: vec!["/tmp/project".into()],
        focused_row: 1,
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
        expo_card_width: crate::expo::default_expo_card_width::default_expo_card_width(),
        expo_filter_query: String::new(),
        expo_filtering: false,
        expo_card_areas: Vec::new(),
        observation_previews: HashMap::new(),
        observation_cache_receiver: None,
        observation_watcher: None,
        last_main_pane_area: Rect::default(),
        file_system_tree_view: FileSystemTreeView::new()?,
        loader_tick: 0,
        session_refresh_receiver: receiver,
        chat_harness: Arc::new(StubHarness::new()),
    })
}
